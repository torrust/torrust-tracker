---
doc-type: manual-verification-evidence
issue-spec: docs/issues/closed/2917-2912-si-5-startup-and-udp-server-errors/ISSUE.md
last-updated-utc: "2026-11-23 18:39"
---

# Manual Verification Evidence

Run on branch `2917-startup-and-udp-server-errors` with the stable Rust toolchain, building the
`torrust-tracker` binary (`cargo build`) before the fix (develop at `ea1ff9fcd` plus the red tests)
and after it.

Artifact under test: the first "after" captures (`evidence/startup-after.txt`,
`evidence/tests-green.txt`) were taken at a working tree matching the commit "fix(udp-server): [#2917] remove
source locations from the UDP server errors". Two production-affecting commits followed:
"test(udp-server): [#2917] compare the authentication classification with the error's own message"
and "refactor(udp-server): [#2917] match the unit variant AuthRequired directly" (it changes
`From<Error> for ErrorKind`). After them both runs were repeated at the build of the last of these
(the commits after it change documentation only): `evidence/startup-after-final.txt` (V1 and V2, no
`.rs:` in either message) and `evidence/tests-green-final.txt` (the three packages' tests, all
green). The observed outcome is unchanged; V1 and V2 do not reach the changed arm, which handles a
variant production never builds.

## V1 - Bad configuration file (M1)

`bad.toml` is the mandatory configuration with `[core] listed = "yes"` (a string where a boolean is
expected). Command: `TORRUST_TRACKER_CONFIG_TOML_PATH=$PWD/bad.toml target/debug/torrust-tracker`.

Before (the path is the `?` line of the loader):

```text
...Failed processing the configuration: invalid type: found string "yes", expected a boolean for key "default.core.listed" in bad.toml TOML file, packages/configuration/src/v3_0_0/mod.rs:409:28
```

After:

```text
...Failed processing the configuration: invalid type: found string "yes", expected a boolean for key "default.core.listed" in bad.toml TOML file
```

The message still names the file and the key.

## V2 - Invalid TLS files (M2)

`tls.toml` enables `[http_trackers.tls_config]` with `cert.pem` and `key.pem` that hold
`not a valid certificate` and `not a valid key`.

Before:

```text
...Verify the configured certificate and key paths: bad tls config: The private key file contained no keys, packages/axum-server/src/tls.rs:53:49
```

After:

```text
...Verify the configured certificate and key paths: bad tls config: The private key file contained no keys
```

Full captures: `evidence/startup-before.txt`, `evidence/startup-after.txt`.

## Automatic evidence

- Red: `evidence/tests-red-configuration.txt`, `tests-red-axum-server.txt`, `tests-red-udp-server.txt`.
- Green: `evidence/tests-green.txt`.
