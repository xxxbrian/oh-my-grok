# Oh My Grok

Oh My Grok (`omg`) is a personal hard fork of
[Grok Build](https://github.com/xai-org/grok-build). It keeps the upstream
runtime and storage behavior while giving the fork a distinct executable,
release identity, and user-facing name.

This project is not affiliated with or supported by xAI.

## Install

The release workflow publishes an unsigned Linux x86_64 binary. Install the
latest published release with [mise](https://mise.jdx.dev/dev-tools/backends/github.html):

```sh
mise use -g github:xxxbrian/oh-my-grok@latest
```

Replace `latest` with a release version to pin it. Run `mise upgrade` to install
a newer release.

To build from source, install the Rust toolchain from `rust-toolchain.toml` and
[DotSlash](https://dotslash-cli.com), then run:

```sh
cargo install dotslash --locked
cargo build --locked \
  --package xai-grok-pager-bin \
  --bin omg \
  --profile release-dist \
  --features release-dist
```

The binary is written to `target/release-dist/omg`.

## Compatibility

The fork intentionally retains upstream-compatible state and interfaces:

- `~/.grok`, `GROK_HOME`, and other `GROK_*` environment variables
- configuration, database, session, and storage formats
- agent, leader, ACP, MCP, and service protocols
- the upstream Grok version used by internal compatibility logic

The executable and user-facing product name are `omg` and Oh My Grok. The xAI
self-updater is disabled because it installs upstream `grok` binaries rather
than this fork; use mise or build from source to update OMG.

## OMG Configuration

OMG-only settings live in `$GROK_HOME/omg.toml` (normally
`~/.grok/omg.toml`) so the upstream configuration remains compatible. For
example, allow Surge's IPv4 fake-IP range through the `web_fetch` SSRF check:

```toml
[web_fetch.ssrf]
allowed_cidrs = ["198.18.0.0/15"]
```

The list accepts IPv4 and IPv6 CIDRs. It is empty by default, and only matching
IP addresses bypass the existing SSRF address check; all other URL, domain,
DNS, and redirect checks remain unchanged.

## Versions

`omg --version` reports the fork release, upstream compatibility version, and
exact fork commit:

```text
omg 0.20260816.1 (grok 1.0.0, abc1234)
```

- OMG releases use `0.<UTC commit date>.<GitHub Actions run number>`.
- The Grok version comes from `upstream.lock` and remains available to upstream
  protocol and telemetry code.
- The commit identifies the exact OMG source tree.

Releases are created manually through `.github/workflows/release.yml`. A release
contains `omg`, `LICENSE`, `THIRD-PARTY-NOTICES`, and `SHA256SUMS`.

## Upstream Governance

This repository is a hard fork with history independent from upstream. Upstream
updates are applied from the tree diff between the old and new commits recorded
in `upstream.lock`, then the small OMG delta is reapplied.

`upstream.lock` records two values:

```text
commit <public xai-org/grok-build commit>
version <official Grok CLI version>
```

When updating it:

1. Read the version from the candidate upstream tree's
   `crates/codegen/xai-grok-version/Cargo.toml`.
2. Require the first entry in
   `crates/codegen/xai-grok-shell/CHANGELOG.md` to match.
3. Confirm that exact version in the official
   [xAI changelog](https://x.ai/build/changelog). Never copy the website's latest
   version without matching it to the source tree.
4. Update `commit` and `version` together. Do not guess when they cannot be
   verified.
5. After applying the upstream diff, confirm that the remaining changes are only
   the documented OMG delta.

`SOURCE_REV` is separate: it is copied from the upstream snapshot and identifies
the internal monorepo revision from which that public snapshot was produced.

## Fork Delta

The maintained differences are:

- the binary, command examples, and user-facing brand use `omg` / Oh My Grok
- OMG has an independent release version while retaining the Grok compatibility
  version
- `omg.toml` can allow selected IPv4 or IPv6 CIDRs through `web_fetch` SSRF
  address filtering
- upstream state paths, environment variables, protocols, and formats remain
  unchanged
- the upstream self-update and reinstall paths are disabled

## License

First-party code is licensed under the [Apache License 2.0](LICENSE). Third-party
and vendored code retains its original licensing; see
[THIRD-PARTY-NOTICES](THIRD-PARTY-NOTICES) and [third_party/NOTICE](third_party/NOTICE).
