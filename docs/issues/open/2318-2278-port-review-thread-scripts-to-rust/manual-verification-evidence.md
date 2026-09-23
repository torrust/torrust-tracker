---
doc-type: manual-verification-evidence
issue-spec: docs/issues/open/2318-2278-port-review-thread-scripts-to-rust/ISSUE.md
last-updated-utc: "2026-09-23 14:47"
---

# Manual Verification Evidence

## Environment and Prerequisites

- Date and time (UTC): 2026-09-23 14:47
- Artifact under test: #2318 implementation on branch `2318-2278-port-review-thread-scripts-to-rust`
- Operating system / environment: Linux
- Rust toolchain: `rustc 1.98.1 (48a229cea 2026-09-01)`
- Test pull request: #2319

## Verification Processes

### V1 - Fetch Parity on a Live Pull Request

- Goal: Confirm that `github-review-threads fetch` preserves the legacy raw GraphQL response shape.
- Status: `DONE`

#### Steps Performed

1. Restored `get-pr-review-threads.sh` from commit `f071e20e`, the parent of the script-retirement commit, into a temporary directory.
2. Ran the restored script and `cargo run --quiet --package github-review-threads -- fetch` for PR #2319.
3. Sorted both response files with `jq -S .` and compared them with `cmp`.

#### Observed Result

Both commands exited successfully. The normalized response files were byte-identical. The two summary objects differed only in their requested output-file paths.

#### Conclusion

The Rust `fetch` command preserves the response-file shape consumed by downstream scripts. The parity check was performed from the exact retired script snapshot after deletion, rather than before deletion as initially planned.

### V2 - Read-Only Projections

- Goal: Confirm the Rust projections retain the legacy selection behavior on the parity fixture.
- Status: `DONE`

#### Steps Performed

1. Restored the retired `list-unresolved-threads.sh`, `show-unresolved-thread-bodies.sh`, and `check-thread-reply-status.sh` scripts from commit `f071e20e`.
2. Compared the legacy list JSON lines with `github-review-threads list` transformed by `jq -c '.[]'`.
3. Verified `github-review-threads show` returned two unresolved threads and retained both comments from the multi-comment thread.
4. Ran both legacy and Rust `reply-status` commands for `author` against the fixture.

#### Observed Result

The compact list outputs were byte-identical. Both implementations retained the multi-comment thread and exited with code `1` when one unresolved thread lacked an author reply. The Rust command emits a JSON diagnostic on stderr and leaves stdout empty for that failure, as required by the CLI output contract.

#### Conclusion

The data projections preserve the legacy action semantics while adopting the approved JSON-only command contract.

### V3 - Downstream Resolver Compatibility

- Goal: Confirm the existing bulk resolver accepts a Rust-generated response file.
- Status: `DONE`

#### Steps Performed

1. Ran `cargo run --quiet --package github-review-threads -- fetch --pr-number 2319 --output-file .tmp/github-review-threads-t4.json | jq -e '.status == "ok" and .pr_number == 2319'`.
2. Ran `bash .github/skills/dev/pr-reviews/resolve-review-threads/scripts/resolve-all-unresolved-threads.sh --threads-file .tmp/github-review-threads-t4.json --dry-run`.
3. Removed the temporary response file.

#### Observed Result

The Rust command returned a successful JSON result, and the resolver completed its dry run using the generated response file.

#### Conclusion

No resolver change is required for the Rust-produced file.

### V4 - CLI Output Contract

- Goal: Confirm TTY refusal and structured diagnostics required by the CLI output contract.
- Status: `DONE`

#### Steps Performed

1. Ran `cargo run --quiet --package github-review-threads -- fetch --pr-number 2319` with stdout attached to the terminal.
2. Captured stdout and stderr separately.
3. Parsed stderr as JSON with `jq -e '.kind == "tty_refusal"'`.

#### Observed Result

The command exited with code `2`, wrote no stdout data, and wrote a JSON `tty_refusal` diagnostic to stderr. Redirected and piped fetch runs emitted a JSON success result on stdout.

#### Conclusion

The command complies with the `stdout-result-data` classification registered in the CLI output-contract ADR.

### V5 - Container Workspace Integration

- Goal: Confirm the new developer-only workspace member is valid in cargo-chef recipe and container test stages.
- Status: `DONE`

#### Steps Performed

1. Ran `docker build --target recipe --file Containerfile .`.
2. Ran `docker build --target test_debug --file Containerfile .` after registering the manifest, target stubs, and archive exclusions.

#### Observed Result

Both container builds exited successfully. The recipe stage copied `github-review-threads/Cargo.toml`, and the test target completed with the tool excluded from production test archives.

#### Conclusion

The workspace member is correctly integrated with the Containerfile recipe and test-archive strategy.
