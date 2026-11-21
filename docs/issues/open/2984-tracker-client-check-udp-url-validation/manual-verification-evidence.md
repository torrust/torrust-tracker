---
doc-type: manual-verification-evidence
issue-spec: docs/issues/open/2984-tracker-client-check-udp-url-validation/ISSUE.md
last-updated-utc: "2026-11-21 13:15"
---

# Manual Verification Evidence

## Environment and Prerequisites

- Date and time (UTC): 2026-11-19 16:05
- Artifact under test: `tracker_client` built from `torrust/develop` at the commit that contains PR #2956 (no patches).
- Operating system / environment: Linux; `cargo build -p torrust-tracker-client --bin tracker_client` (default toolchain).
- Setup: none; no tracker is needed because the failure happens before any network traffic. Stdout is piped (`| cat`) because `tracker_client` refuses a terminal.

## Verification Processes

### V1 - Initial reproduction (before the fix)

- Goal: observe the wrong outcome for UDP URLs without a usable address.
- Initial state: `torrust/develop` after PR #2956.
- Status: `DONE`
- Outcome: **Reproduced** (the wrong outcome, a panic and exit `1`, was observed).

#### Steps Performed

For each `<url>`:

```bash
TORRUST_CHECKER_CONFIG='{"udp_trackers":["<url>"],"http_trackers":[],"health_checks":[]}' \
  ./target/debug/tracker_client check | cat
```

#### Observed Result

| `<url>`                          | Exit | Stderr (abridged)                                                                                         |
| -------------------------------- | ---- | --------------------------------------------------------------------------------------------------------- |
| `udp://127.0.0.1`                | 1    | `panic` at `checks/udp.rs:107`: `No port number in the URL`; then `runtime_failure`                       |
| `udp:///`                        | 1    | `panic` at `checks/udp.rs:107`: `No host name in the URL`; then `runtime_failure`                         |
| `ftp://127.0.0.1:6969`           | 1    | `panic` at `checks/udp.rs:107`: `No port number in the URL` (the entry becomes `udp://ftp://...`)         |
| `http://127.0.0.1`               | 1    | `panic` at `checks/udp.rs:107`: `No port number in the URL`                                               |
| `udp://nonexistent.invalid:6969` | 1    | `panic` at `checks/udp.rs:107`: `failed to lookup address information: Name or service not known`         |
| `invalid URL` (control)          | 2    | `{"error":{"kind":"invalid_configuration","source":"TORRUST_CHECKER_CONFIG","message":"Invalid URL: invalid domain character"}}` |
| `udp://[::1` (control)           | 2    | `{"error":{"kind":"invalid_configuration",...,"message":"Invalid URL: invalid IPv6 address"}}`             |

Stdout was empty in every case.

#### Conclusion

Reproduced. The `unwrap` at `checks/udp.rs:107` is reached for every UDP URL without a resolvable address; URLs that fail to parse already get the expected exit `2`.
