---
doc-type: manual-verification-evidence
issue-spec: docs/issues/open/2760-2758-adr-binary-table/ISSUE.md
last-updated-utc: "2026-11-09 11:27"
---

# Manual Verification Evidence

## Purpose

Record the real comparison of the ADR binary table with the EPIC #2758 audit and with the current code.

## Environment and Prerequisites

- Date and time (UTC): 2026-11-09 10:24 (rerun after the table changed; the first run was on 2026-11-06 20:51)
- Artifact under test: the `docs(adr): [#2760] align the CLI output contract binary table with the audit` change to `docs/adrs/20260519000000_define_global_cli_output_contract.md`
- Operating system / environment: Linux, Claude Code (Claude Sonnet 5.5), `linter all` from the repository checkout
- Prerequisites and setup performed: none beyond the checkout (docs-only change)

## Verification Processes

### V1 - Compare the table with the audit

- Goal: every program in the EPIC audit table has a row with the class the audit and the code support.
- Initial state: branch `2760-adr-binary-table` with the change applied.
- Status: `DONE`

#### Steps Performed

Steps 2 to 5 ran at 10:24:28 UTC on 2026-11-09. Step 1 was rerun at 10:26:01 UTC with the `awk` form, after its first form (`cut`) failed the spell check, so the stamps below are not in the listed order:

1. ``grep -n '^| `' docs/adrs/20260519000000_define_global_cli_output_contract.md | awk -F'|' '{print $1 "|" $2 "|" $3 "|"}'`` to list the table rows and classes (`awk` only drops the notes column).
2. `grep -n 'println!\|eprintln!' src/bin/http_health_check.rs` to check the `http_health_check` output claim.
3. `grep -n 'HEALTHCHECK' -A2 Containerfile` to check the container `HEALTHCHECK` claim.
4. `grep -n 'OUTPUT_FILE\|to_writer\|println!' contrib/dev-tools/analysis/workspace-coupling/src/main.rs` to decide the `workspace-coupling` class (where its report goes).
5. `ls console/tracker-client/src/bin` to confirm the deprecated clients still exist.
6. Read the EPIC audit table (`docs/issues/open/2758-cli-output-contract-tools-migration/EPIC.md`) beside the rows.

#### Observed Result

Output of steps 1 to 5, pasted from the run (a `# started` line is the start time of the command, a `# exit` line its exit code):

```text
$ grep -n '^| `' docs/adrs/20260519000000_define_global_cli_output_contract.md | awk -F'|' '{print $1 "|" $2 "|" $3 "|"}'
# started 10:26:01
79:| `torrust-tracker`               | `no-stdout-result`   |
80:| `http_health_check`             | `no-stdout-result`   |
81:| `e2e_tests_runner`              | `no-stdout-result`   |
82:| `qbittorrent_e2e_runner`        | `no-stdout-result`   |
83:| `tracker_client`                | `stdout-result-data` |
84:| `clippy-allow-reasons`          | `no-stdout-result`   |
85:| `frontmatter-validator`         | `no-stdout-result`   |
86:| `frontmatter-schema`            | `no-stdout-result`   |
87:| `shipped-features-check`        | `no-stdout-result`   |
88:| `agent-review-report-contract`  | `no-stdout-result`   |
89:| `workspace-coupling`            | `no-stdout-result`   |
90:| `github-review-threads`         | `stdout-result-data` |
91:| `ci-gate`                       | `stdout-result-data` |
92:| `wait-for-pr-review`            | `stdout-result-data` |
93:| `package-coverage-check`        | `stdout-result-data` |
94:| `http_tracker_client`, `udp_tracker_client`, `tracker_checker` | `N/A` |
# exit 0

$ grep -n 'println!\|eprintln!' src/bin/http_health_check.rs
# started 10:24:28
23:        eprintln!("Usage:   cargo run --bin http_health_check <HEALTH_URL>");
24:        eprintln!("Example: cargo run --bin http_health_check http://127.0.0.1:1313/health_check");
28:    println!("Health check ...");
37:                println!("STATUS: {}", response.status());
40:            println!("Non-success status received.");
44:            println!("ERROR: {err}");
# exit 0

$ grep -n 'HEALTHCHECK' -A2 Containerfile
# started 10:24:28
277:HEALTHCHECK --interval=5s --timeout=5s --start-period=3s --retries=3 \  
278-  CMD /usr/bin/http_health_check http://localhost:${HEALTH_CHECK_API_PORT}/health_check \
279-    || exit 1
# exit 0

$ grep -n 'OUTPUT_FILE\|to_writer\|println!' contrib/dev-tools/analysis/workspace-coupling/src/main.rs
# started 10:24:28
12://! workspace-coupling [OUTPUT_FILE]
15://! If `OUTPUT_FILE` is omitted the report is written to
49:    serde_json::to_writer(&mut stderr, event)?;
# exit 0

$ ls console/tracker-client/src/bin
# started 10:24:28
http_tracker_client.rs
tracker_checker.rs
tracker_client.rs
udp_tracker_client.rs
# exit 0
```

#### Conclusion

Every audited program is listed with a matching class (the deprecated clients read `N/A`). `workspace-coupling` is `no-stdout-result`: the report goes to a file and only NDJSON status events go to stderr. Met.

### V2 - Repository linters

- Goal: the documentation passes the repository linters.
- Initial state: ADR, spec and evidence edits made, not yet committed.
- Status: `DONE`

#### Steps Performed

1. `linter all > .tmp/linter.out 2>&1; echo exit=$?` on 2026-11-09 (the output below is `.tmp/linter.out` with the colour codes removed).

#### Observed Result

```text
2026-11-09T10:26:28.079458Z  INFO torrust_linting::cli: Running All Linters
2026-11-09T10:26:28.080079Z  INFO markdown: Scanning markdown files...
2026-11-09T10:26:36.232673Z  INFO markdown: All markdown files passed linting! (8.153s)
2026-11-09T10:26:36.233777Z  INFO lychee: Checking local Markdown links and fragments...
2026-11-09T10:26:36.341378Z  INFO lychee: All local Markdown links passed checking! (0.108s)
2026-11-09T10:26:36.342074Z  INFO yaml: Scanning YAML files...
2026-11-09T10:26:36.711754Z  INFO yaml: All YAML files passed linting! (0.370s)
2026-11-09T10:26:36.712448Z  INFO toml: Scanning TOML files...
2026-11-09T10:26:37.358865Z  INFO toml: All TOML files passed linting and formatting checks! (0.646s)
2026-11-09T10:26:37.359633Z  INFO cspell: Running spell check on all files...
2026-11-09T10:26:45.235279Z  INFO cspell: All files passed spell checking! (7.876s)
2026-11-09T10:26:45.235304Z  INFO clippy: Running Rust Clippy linter...
2026-11-09T10:26:45.480026Z  INFO clippy: Clippy linting completed successfully! (0.245s)
2026-11-09T10:26:45.480053Z  INFO rustfmt: Running Rust formatter check...
2026-11-09T10:26:45.939307Z  INFO rustfmt: Rust formatting check passed! (0.459s)
2026-11-09T10:26:45.939328Z  INFO shellcheck: Running ShellCheck on shell scripts...
2026-11-09T10:26:46.167650Z  INFO shellcheck: Found 30 shell script(s) to check
2026-11-09T10:26:46.834109Z  INFO shellcheck: shellcheck passed (0.895s)
2026-11-09T10:26:46.834136Z  INFO torrust_linting::cli: All linters passed
exit=0
```

#### Conclusion

`linter all` exited 0. Met. The final run after the later edits is recorded in the progress log of the spec.

## Failures and Follow-up

None.
