//! User-facing product identity for this fork.
//!
//! Data paths and upstream protocol identifiers stay compatible (`~/.grok`,
//! `GROK_HOME`). The executable and strings that tell users what to run use
//! this identity.

/// Invoked command name (`omg login`, `omg --resume`, clap `name`, …).
pub const CLI_NAME: &str = "omg";

/// Short product label for `--help`, `--version`, welcome, update copy.
pub const PRODUCT_NAME: &str = "Oh My Grok";

/// clap `about` line.
pub const PRODUCT_ABOUT: &str = "Oh My Grok TUI";

/// Filename of the managed install under `$GROK_HOME/bin/`.
pub fn managed_bin_name() -> &'static str {
    if cfg!(windows) { "omg.exe" } else { "omg" }
}

/// Format a user-facing invocation, e.g. `omg login` / `omg --resume abc`.
pub fn cli_invocation(args: &str) -> String {
    if args.is_empty() {
        CLI_NAME.to_string()
    } else {
        format!("{CLI_NAME} {args}")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn cli_name_is_omg() {
        assert_eq!(CLI_NAME, "omg");
        assert!(managed_bin_name().starts_with(CLI_NAME));
    }

    #[test]
    fn cli_invocation_joins_args() {
        assert_eq!(cli_invocation(""), CLI_NAME);
        assert_eq!(cli_invocation("login"), "omg login");
        assert_eq!(cli_invocation("--resume abc"), "omg --resume abc");
    }
}
