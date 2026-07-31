---
name: run-tracker-locally
description: Run the Torrust Tracker locally for development and testing. Use this skill to start the tracker with default configuration, understand configuration loading, and interact with tracker services (UDP and HTTP). Triggers on "run tracker", "start tracker locally", "develop tracker", "test tracker locally", or "run tracker for testing".
compatibility: Requires cargo, bash, and local workspace access.
metadata:
  author: torrust
  version: "1.0"
---

# Run Tracker Locally

## Skill Links

This skill depends on these artifacts. If any of them change, review this skill.

- `src/bootstrap/config.rs`
- `share/default/config/tracker.development.sqlite3.toml`
- `src/lib.rs`
- `README.md`

Use the marker `skill-link: run-tracker-locally` in affected artifacts.

Convention reference: `docs/skills/semantic-skill-link-convention.md`

## Validation Loop

Before finalizing changes related to this workflow:

1. Run `bash ./scripts/validate-skill-links.sh`
2. If validation fails, update either artifact markers or this skill content.
3. Re-run validation until it passes.

## Quick Start

To run the tracker with default development configuration:

```bash
cargo run
```

The tracker will start and you will see console output (logs) indicating where it's loading configuration from.

## Default Development Configuration

When you run `cargo run` from the repository root, the tracker loads the default development configuration:

```text
Loading extra configuration from default configuration file: `./share/default/config/tracker.development.sqlite3.toml` ...
```

**Default database**: SQLite3  
**Default configuration file**: `./share/default/config/tracker.development.sqlite3.toml`

## Default Services

By default, the development configuration starts:

- **2 UDP trackers** on different ports
- **2 HTTP trackers** on different ports
- Health check API endpoint

Check the configuration file to see exact ports and settings.

## Viewing Configuration

To inspect or customize the tracker configuration:

```bash
# View the default development configuration
cat ./share/default/config/tracker.development.sqlite3.toml
```

You can modify this file to change:

- Tracker ports
- Database location
- Logging levels
- Tracker behavior and thresholds
- Authentication settings

## Common Ports (Default Configuration)

Check `./share/default/config/tracker.development.sqlite3.toml` for exact port assignments. Typical defaults:

- UDP tracker 1: `6969/udp`
- UDP tracker 2: `6970/udp`
- HTTP tracker 1: `7070/tcp`
- HTTP tracker 2: `7071/tcp`
- Health check API: `1212/tcp`

## Stopping the Tracker

To stop the running tracker:

```bash
# Press Ctrl+C in the terminal where the tracker is running
```

## Verifying Tracker is Running

Check if tracker services are listening:

```bash
# Using ss (Linux)
ss -ulnp 2>/dev/null | grep -E '6969|6970'
ss -tlnp 2>/dev/null | grep -E '7070|7071|1212'

# Or using netstat (older systems)
netstat -ulnp 2>/dev/null | grep -E '6969|6970'
netstat -tlnp 2>/dev/null | grep -E '7070|7071|1212'
```

## Running a Local HTTPS Tracker

For local TLS verification, create a temporary configuration and certificate
under `.tmp/`. The directory is git-ignored, so do not place test keys in
`share/` or commit them.

1. Copy or create a configuration based on the development configuration. Give
   an HTTP tracker a port-zero binding if the final runtime binding is part of
   the behavior under test:

```toml
[[http_trackers]]
bind_address = "0.0.0.0:0"

# Schema 2.0 uses the historical `tsl_config` spelling.
[http_trackers.tsl_config]
ssl_cert_path = ".tmp/localhost.crt"
ssl_key_path = ".tmp/localhost.key"
```

1. Generate a short-lived self-signed certificate for local use. Include SANs
   for both `localhost` and `127.0.0.1` so a loopback client can validate it
   when supplied with the certificate:

```bash
openssl req -x509 -out .tmp/localhost.crt -keyout .tmp/localhost.key \
  -newkey rsa:2048 -nodes -sha256 -days 1 \
  -subj '/CN=localhost' \
  -addext 'subjectAltName=DNS:localhost,IP:127.0.0.1' \
  -addext 'keyUsage=digitalSignature' \
  -addext 'extendedKeyUsage=serverAuth'
```

1. Start the tracker with the temporary configuration:

```bash
TORRUST_TRACKER_CONFIG_TOML_PATH="$PWD/.tmp/local-tls.toml" cargo run --bin torrust-tracker
```

Read the startup log to obtain the final port assigned to a `:0` binding.
It will report an `https://` URL when TLS is enabled.

1. Probe the listener. `--insecure` is appropriate only for this temporary
   self-signed local certificate:

```bash
curl --fail --silent --show-error --insecure https://127.0.0.1:<port>/health_check
```

1. Stop the tracker and remove or retain the `.tmp/` files as local-only test
   artifacts. Restore any temporary configuration edits before committing.

> **Known limitation:** the aggregate health-check service currently builds
> HTTP-tracker probes with an `http://` URL even when a registered listener is
> HTTPS. A direct HTTPS probe verifies the TLS listener; do not treat that
> separate health-check defect as a TLS-startup failure.

## Database Storage

By default, development tracker uses SQLite3. The database file is stored in:

```text
./storage/tracker/lib/
```

This directory is git-ignored. Database state persists between restarts unless you manually delete it.

## Logs Location

Tracker logs are written to:

```text
./storage/tracker/log/
```

Check these logs when debugging tracker behavior.

## Testing with UDP Tracker Client

Once the tracker is running, test it with the UDP tracker client:

```bash
# Default announce (backward compatibility)
cargo run -p torrust-tracker-client --bin tracker_client udp announce 127.0.0.1:6969 9c38422213e30bff212b30c360d26f9a02136422

# Announce with all optional parameters
# NOTE: Use '--peer-id=VALUE' syntax (with equals and single quotes) when peer-id starts with a dash
cargo run -p torrust-tracker-client --bin tracker_client udp announce \
  127.0.0.1:6969 443c7602b4fde83d1154d6d9da48808418b181b6 \
  --event completed \
  --uploaded 1234 \
  --downloaded 5678 \
  --left 0 \
  --port 6881 \
  --ip-address 10.0.0.1 \
  '--peer-id=-RC00000000000000001' \
  --key 42 \
  --peers-wanted 50
```

**Important**: Peer-id must be exactly 20 bytes. When the peer-id starts with a dash (like `-RC...`), use the `--peer-id='...'` syntax to prevent shell from interpreting it as a flag.

## Testing with HTTP Tracker Client

Test the HTTP tracker:

```bash
# Default announce
cargo run -p torrust-tracker-client --bin tracker_client http announce http://127.0.0.1:7070 9c38422213e30bff212b30c360d26f9a02136422
```

## Notes

- The tracker runs in the foreground. Use `Ctrl+C` to stop it or run it in a separate terminal.
- All runtime data (database, logs, config) is stored in `./storage/` which is git-ignored.
- Each `cargo run` reuses existing database state; delete `./storage/` to start fresh.
- Log output shows which services are active and on which ports.

## Available Scripts

- `./scripts/validate-skill-links.sh` validates that all linked artifacts exist and include the expected `skill-link` marker.
