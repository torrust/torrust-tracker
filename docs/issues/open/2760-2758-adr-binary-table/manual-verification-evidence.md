---
doc-type: manual-verification-evidence
issue-spec: docs/issues/open/2760-2758-adr-binary-table/ISSUE.md
last-updated-utc: "2026-11-06 20:55"
---

# Manual Verification Evidence

## Purpose

Record the real comparison of the ADR binary table with the EPIC #2758 audit and with the current code.

## Environment and Prerequisites

- Date and time (UTC): 2026-11-06 20:51
- Artifact under test: the `docs(adr): [#2760] align the CLI output contract binary table with the audit` change to `docs/adrs/20260519000000_define_global_cli_output_contract.md`
- Operating system / environment: Linux, Claude Code (Claude Sonnet 5.5), `linter all` from the repository checkout
- Prerequisites and setup performed: none beyond the checkout (docs-only change)

## Verification Processes

### V1 - Compare the table with the audit

- Goal: every program in the EPIC audit table has a row with the class the audit and the code support.
- Initial state: branch `2760-adr-binary-table` with the change applied.
- Status: `DONE`

#### Steps Performed

Run one at a time, in this order (started 20:51:56 UTC):

1. `grep -n '^| \`' docs/adrs/20260519000000_define_global_cli_output_contract.md` to list the table rows and classes.
2. `grep -n 'println!\|eprintln!' src/bin/http_health_check.rs` and `grep -n 'HEALTHCHECK' -A2 Containerfile` to check the `http_health_check` claim.
3. `grep -n 'OUTPUT_FILE\|to_writer\|println!' contrib/dev-tools/analysis/workspace-coupling/src/main.rs` to decide the `workspace-coupling` class (where its report goes).
4. `ls console/tracker-client/src/bin` to confirm the deprecated clients still exist.
5. Read the EPIC audit table (`docs/issues/open/2758-cli-output-contract-tools-migration/EPIC.md`) beside the rows.

#### Observed Result

```text
78 torrust-tracker              no-stdout-result
79 http_health_check            no-stdout-result
80 e2e_tests_runner             no-stdout-result
81 qbittorrent_e2e_runner       no-stdout-result
82 tracker_client               stdout-result-data
83 clippy-allow-reasons         no-stdout-result
84 frontmatter-validator        no-stdout-result
85 frontmatter-schema           no-stdout-result
86 shipped-features-check       no-stdout-result
87 agent-review-report-contract no-stdout-result
88 workspace-coupling           no-stdout-result
89 github-review-threads       stdout-result-data
90 ci-gate                     stdout-result-data
91 wait-for-pr-review          stdout-result-data
92 package-coverage-check      stdout-result-data
93 http_tracker_client, udp_tracker_client, tracker_checker (deprecated)

src/bin/http_health_check.rs: eprintln! at 23, 24; println! at 28, 37, 40, 44
Containerfile:277-279: HEALTHCHECK ... CMD /usr/bin/http_health_check http://localhost:${HEALTH_CHECK_API_PORT}/health_check || exit 1

workspace-coupling/src/main.rs: usage `workspace-coupling [OUTPUT_FILE]` (line 12), report written to a file (line 15),
  serde_json::to_writer(&mut stderr, event) (line 49); no println! on stdout

console/tracker-client/src/bin: http_tracker_client.rs tracker_checker.rs tracker_client.rs udp_tracker_client.rs
```

#### Conclusion

Every audited program is listed with a matching class. `workspace-coupling` is `no-stdout-result`: the report goes to a file and only NDJSON status events go to stderr. Met.

### V2 - Repository linters

- Goal: the documentation passes the repository linters.
- Initial state: change committed.
- Status: `DONE`

#### Steps Performed

1. `linter all` (run twice, after the table edit and after the evidence files were added).

#### Observed Result

```text
markdown, lychee (local links), yaml, toml, cspell, clippy, rustfmt, shellcheck: all passed
All linters passed
```

#### Conclusion

`linter all` exited 0. Met.

## Failures and Follow-up

None.
