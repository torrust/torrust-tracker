# Progressive Verification Evidence

Record baseline and post-change manual verification for each code-changing task
in the registry metadata migration.

## Task Evidence

| Task | Baseline Status | Post-change Status          | Evidence                                                                                                     |
| ---- | --------------- | --------------------------- | ------------------------------------------------------------------------------------------------------------ |
| T2   | NOT RECORDED    | Automated PASS; manual TODO | API boundary reviewed and approved before implementation.                                                    |
| T3   | NOT RECORDED    | Automated PASS; manual TODO | Server-lib tests cover acknowledgement, duplicate rejection, metadata snapshots, and deterministic ordering. |
| T4   | NOT RECORDED    | Automated PASS; manual TODO | `register().await` is the insertion acknowledgement; integration helpers await exact identities.             |
| T5   | NOT RECORDED    | Automated PASS; manual TODO | `cargo publish --dry-run` and final publication of `torrust-server-lib` 0.2.0 succeeded.                     |
| T6   | NOT RECORDED    | Automated PASS; manual TODO | Port-zero integration discovers every HTTP/UDP canonical instance identity.                                  |
| T7   | NOT RECORDED    | Automated PASS; manual TODO | Health contract tests assert preserved URL, binding, and service-type fields for HTTP, REST API, and UDP.    |
| T8   | NOT RECORDED    | Automated PASS; manual TODO | Integration helpers query roles/identities instead of raw map entries or bind IPs.                           |
| T9   | NOT RECORDED    | Automated PASS; manual TODO | Focused server, health-contract, repeated-port-zero, and scaffold tests passed.                              |

## Automated Local Verification

The issue's evidence protocol asks for manual baseline and post-change probes
before each edit. This work started before those baselines were recorded, so no
manual baseline or manual post-change result is claimed retroactively. The
following are reproducible **automated** post-change checks only. M1-M3 remain
mandatory manual scenarios before this issue can be accepted.

### T3-T5 - Generic registry API and released crate

- Baseline: Not recorded before implementation.
- Post-change revision: `torrust-server-lib` commit `d17fdb1`.
- Commands: `cargo publish --dry-run`, `cargo publish`, `cargo machete --with-metadata`, `linter all`, and `cargo test --doc --workspace`.
- Observed result: dry-run packaged and verified 18 files; `torrust-server-lib` 0.2.0 published to crates.io. Dependency, lint, and doc-test checks passed.
- Comparison: The released API replaces raw map access with metadata snapshots and acknowledged insertion.
- Result: `DONE`.

### T6-T8 - Runtime identities, health report, and integration discovery

- Baseline: Not recorded before implementation.
- Post-change revision: tracker branch `2041-migrate-runtime-service-registry-metadata`.
- Commands: `cargo test -p torrust-tracker-axum-health-check-api-server --test integration` and `cargo test --test aggregate_stats_port_zero --test scaffold`.
- Observed result: all seven health-contract tests passed; repeated port-zero HTTP/UDP blocks registered distinct non-zero final bindings for exact canonical identities; scaffold and port-zero integration scenarios passed.
- Comparison: Helper behavior now waits for exact canonical identities and finds endpoints by role, rather than registry size, map ordering, or bind-IP conventions.
- Result: `DONE`.

### T9 - Focused regression coverage

- Baseline: Not recorded before implementation.
- Post-change revision: tracker branch `2041-migrate-runtime-service-registry-metadata`.
- Commands: `cargo check --workspace --all-targets`; focused server package tests; `cargo test --test aggregate_stats_fixed_ports --test aggregate_stats_port_zero --test scaffold`; and `linter all`.
- Observed result: all invoked checks passed. Health JSON tests assert `service_binding`, `binding`, and `service_type`; port-zero coverage asserts exact identity-to-final-binding correlation.
- Comparison: Regression coverage now protects the metadata and readiness contracts introduced by this issue.
- Result: `DONE`.

## Scenario Record Template

```markdown
### T{N} - {Task title}

#### Baseline

- Configuration:
- Command/query:
- Observed result:

#### Post-change

- Commit or revision:
- Command/query:
- Observed result:
- Comparison:
- Result: `DONE` / `FAILED`
```
