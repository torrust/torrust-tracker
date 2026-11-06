# Manual verification evidence - #2760

## V1 - Compare the table with the audit

Toolchain: Claude Code (Claude Sonnet 5.5), 2026-11-06 UTC, branch `2760-adr-binary-table`.

Each program of the EPIC #2758 audit table was checked against the ADR table:

| Program | Row | Class | Verified against code |
| --- | --- | --- | --- |
| `torrust-tracker`, `e2e_tests_runner`, `qbittorrent_e2e_runner` | yes | `no-stdout-result` | audit |
| `http_health_check` | yes | `no-stdout-result` | `src/bin/http_health_check.rs` prints `Health check ...` and `STATUS:` with `println!`; `Containerfile` `HEALTHCHECK ... \|\| exit 1` |
| `tracker_client`, `package-coverage-check`, `github-review-threads` | yes | `stdout-result-data` | audit |
| deprecated clients | yes | deprecated | `console/tracker-client/src/bin/{http_tracker_client,udp_tracker_client,tracker_checker}.rs` exist |
| `agent-review-report-contract`, `frontmatter-schema` | added | `no-stdout-result` | crates exist under `contrib/dev-tools/checks/` |
| `workspace-coupling` | added | `no-stdout-result` | `contrib/dev-tools/analysis/workspace-coupling/src/main.rs`: report written to `OUTPUT_FILE` (default under `docs/issues/open/1669-overhaul-packages/`); stderr gets NDJSON; no stdout print |
| `clippy-allow-reasons`, `frontmatter-validator`, `shipped-features-check`, `ci-gate`, `wait-for-pr-review` | yes | unchanged | audit says consistent |

Result: every audited program is listed with a matching class. `linter all` exits 0.
