---
name: setup-dev-environment
description: Set up a local development environment for torrust-tracker from scratch. Covers system dependencies, Rust toolchain, storage directories, linter binary, git hooks, and smoke tests. Use when onboarding to the project, setting up a new machine, or after a fresh clone. Triggers on "setup dev environment", "fresh clone", "onboarding", "install dependencies", "set up environment", or "getting started".
metadata:
  author: torrust
  version: "1.0"
---

# Set Up the Development Environment

Full setup guide for a fresh clone of `torrust-tracker`. Follow the steps in order.

Reference: [How to Set Up the Development Environment](https://torrust.com/blog/how-to-setup-the-development-environment)

## Step 1: System Dependencies

Install the required system packages (Debian/Ubuntu):

```bash
sudo apt-get install libsqlite3-dev pkg-config libssl-dev make
```

> For other distributions, install the equivalent packages for SQLite3 development headers, OpenSSL
> development headers, `pkg-config`, and `make`.

## Step 2: Rust Toolchain

```bash
rustup show                        # Confirm toolchain is active
rustup update                      # Update to latest stable
rustup toolchain install nightly   # Required for docs generation
```

The project MSRV is **1.72**. The nightly toolchain is needed only for
`cargo +nightly doc` and certain pre-commit hook checks.

## Step 3: Build

```bash
cargo build
```

This compiles all workspace crates and verifies that all dependencies resolve correctly.

## Step 4: Create Storage Directories

The tracker writes runtime data (databases, logs, TLS certs, config) to `storage/`, which is
git-ignored. Create the required folders once:

```bash
mkdir -p ./storage/tracker/lib/database
mkdir -p ./storage/tracker/lib/tls
mkdir -p ./storage/tracker/etc
```

## Step 5: Install the Linter Binary

```bash
cargo install --locked --git https://github.com/torrust/torrust-linting --bin linter
```

See the `install-linter` skill for external tool dependencies (markdownlint, yamllint, etc.).

## Step 6: Install Additional Cargo Tools

```bash
cargo install cargo-machete   # Unused dependency checker
```

## Step 7: Install Git Hooks

Install the project pre-commit hook (one-time, re-run after hook changes):

```bash
./scripts/install-git-hooks.sh
```

The hook runs `./scripts/pre-commit.sh` automatically on every `git commit`.

## Step 8: Smoke Test

Run the tracker with the default development configuration to confirm the build works:

```bash
cargo run
```

Expected output includes lines like:

```text
Loading configuration from default configuration file: `./share/default/config/tracker.development.sqlite3.toml`
[UDP TRACKER] Starting on: udp://0.0.0.0:6969
[HTTP TRACKER] Started on: http://0.0.0.0:7070
[API] Started on http://127.0.0.1:1212
[HEALTH CHECK API] Started on: http://127.0.0.1:1313
```

Press `Ctrl-C` to stop.

## Step 9: Verify Full Test Suite

```bash
cargo test --doc --workspace
cargo test --tests --benches --examples --workspace --all-targets --all-features
```

Both commands must exit `0` before any commit.

## Custom Configuration (Optional)

To run with a custom config instead of the default template:

```bash
cp share/default/config/tracker.development.sqlite3.toml storage/tracker/etc/tracker.toml
# Edit storage/tracker/etc/tracker.toml as needed
TORRUST_TRACKER_CONFIG_TOML_PATH="./storage/tracker/etc/tracker.toml" cargo run
```

## Useful Development Tools

- **DB Browser for SQLite** — inspect and edit SQLite databases: <https://sqlitebrowser.org/>
- **qBittorrent** — BitTorrent client for manual testing: <https://www.qbittorrent.org/>
- **imdl** — torrent file editor (`cargo install imdl`): <https://github.com/casey/intermodal>
