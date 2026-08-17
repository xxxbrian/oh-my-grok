//! Oh My Grok-only configuration.
//!
//! This file is deliberately separate from the upstream-compatible
//! `$GROK_HOME/config.toml` hierarchy.

use std::path::{Path, PathBuf};
use std::sync::OnceLock;

use ipnet::IpNet;
use serde::Deserialize;

use crate::user_grok_home;

pub const OMG_CONFIG_FILENAME: &str = "omg.toml";

static OMG_CONFIG: OnceLock<OmgConfig> = OnceLock::new();

#[derive(Debug, Clone, Default, Deserialize)]
#[serde(default, deny_unknown_fields)]
pub struct OmgConfig {
    pub web_fetch: OmgWebFetchConfig,
}

#[derive(Debug, Clone, Default, Deserialize)]
#[serde(default, deny_unknown_fields)]
pub struct OmgWebFetchConfig {
    pub ssrf: OmgWebFetchSsrfConfig,
}

#[derive(Debug, Clone, Default, Deserialize)]
#[serde(default, deny_unknown_fields)]
pub struct OmgWebFetchSsrfConfig {
    pub allowed_cidrs: Vec<IpNet>,
}

#[derive(Debug, thiserror::Error)]
pub enum OmgConfigError {
    #[error("failed to read OMG config at {path}: {source}")]
    Read {
        path: PathBuf,
        #[source]
        source: std::io::Error,
    },
    #[error("invalid OMG config at {path}: {detail}")]
    Parse { path: PathBuf, detail: String },
    #[error("OMG configuration was initialized more than once")]
    AlreadyInitialized,
}

/// Load and install the process-wide OMG configuration.
///
/// A missing file is equivalent to the default config. When no real user home
/// can be resolved, do not fall back to a cwd-relative `.grok/omg.toml`: a
/// project must not be able to relax the process SSRF policy.
pub fn initialize_omg_config() -> Result<(), OmgConfigError> {
    let config = match user_grok_home() {
        Some(home) => load_omg_config_file(&home.join(OMG_CONFIG_FILENAME))?,
        None => OmgConfig::default(),
    };
    OMG_CONFIG
        .set(config)
        .map_err(|_| OmgConfigError::AlreadyInitialized)
}

/// Return the installed OMG config, or the upstream-compatible default for
/// library/test entry points that do not run the `omg` composition root.
pub fn omg_config() -> &'static OmgConfig {
    OMG_CONFIG.get_or_init(OmgConfig::default)
}

fn load_omg_config_file(path: &Path) -> Result<OmgConfig, OmgConfigError> {
    let source = match std::fs::read_to_string(path) {
        Ok(source) => source,
        Err(error) if error.kind() == std::io::ErrorKind::NotFound => {
            return Ok(OmgConfig::default());
        }
        Err(source) => {
            return Err(OmgConfigError::Read {
                path: path.to_path_buf(),
                source,
            });
        }
    };

    toml::from_str(&source).map_err(|error| OmgConfigError::Parse {
        path: path.to_path_buf(),
        detail: crate::toml_error_detail(&source, &error),
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn missing_file_uses_upstream_compatible_defaults() {
        let dir = tempfile::tempdir().unwrap();
        let config = load_omg_config_file(&dir.path().join(OMG_CONFIG_FILENAME)).unwrap();
        assert!(config.web_fetch.ssrf.allowed_cidrs.is_empty());
    }

    #[test]
    fn parses_ipv4_and_ipv6_cidrs() {
        let dir = tempfile::tempdir().unwrap();
        let path = dir.path().join(OMG_CONFIG_FILENAME);
        std::fs::write(
            &path,
            r#"
[web_fetch.ssrf]
allowed_cidrs = ["198.18.0.0/15", "fd12:3456:789a::/48"]
"#,
        )
        .unwrap();

        let config = load_omg_config_file(&path).unwrap();
        assert_eq!(config.web_fetch.ssrf.allowed_cidrs.len(), 2);
        assert!(
            config.web_fetch.ssrf.allowed_cidrs[0]
                .contains(&"198.18.1.1".parse::<std::net::IpAddr>().unwrap())
        );
        assert!(
            config.web_fetch.ssrf.allowed_cidrs[1]
                .contains(&"fd12:3456:789a::1".parse::<std::net::IpAddr>().unwrap())
        );
    }

    #[test]
    fn rejects_invalid_cidr() {
        let dir = tempfile::tempdir().unwrap();
        let path = dir.path().join(OMG_CONFIG_FILENAME);
        std::fs::write(
            &path,
            "[web_fetch.ssrf]\nallowed_cidrs = [\"not-a-cidr\"]\n",
        )
        .unwrap();

        assert!(matches!(
            load_omg_config_file(&path),
            Err(OmgConfigError::Parse { .. })
        ));
    }

    #[test]
    fn rejects_unknown_keys() {
        let dir = tempfile::tempdir().unwrap();
        let path = dir.path().join(OMG_CONFIG_FILENAME);
        std::fs::write(&path, "[web_fetch.ssrf]\nallow_all = true\n").unwrap();

        assert!(matches!(
            load_omg_config_file(&path),
            Err(OmgConfigError::Parse { .. })
        ));
    }
}
