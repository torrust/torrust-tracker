---
doc-type: manual-verification-evidence
issue-spec: docs/issues/closed/2765-2758-remove-deprecated-tracker-clients/ISSUE.md
last-updated-utc: "2026-11-17 17:36"
---

# Manual Verification Evidence

## Purpose

Real verification of issue #2765 (remove the deprecated tracker client binaries). Both runs below
were executed on the implementation branch after the removal and the e2e helper migration.

## Environment and Prerequisites

- Date and time (UTC): 2026-11-17 09:07
- Artifact under test: the PR patches "remove the deprecated tracker client binaries" and
  "run `tracker_client check` from the e2e runner and capture its output".
- Operating system / environment: Linux, rustc 1.99.1 (2026-10-13).
- Setup: the tracker built from the branch, started from a scratch directory with
  `TORRUST_TRACKER_CONFIG_TOML_PATH=share/default/config/tracker.development.sqlite3.toml`
  (UDP 6969, HTTP 7070, health check API on 1212).

## Verification Processes

### V1 - Unified checker (M1)

- Goal: `tracker_client check` works against a local tracker as the README documents.
- Status: `DONE`

Command (stdin from `/dev/null`, stdout and stderr redirected to files; ran 09:07:35.123 to 09:07:35.136):

```text
TORRUST_CHECKER_CONFIG='{"udp_trackers":["127.0.0.1:6969"],"http_trackers":["http://127.0.0.1:7070"],"health_checks":["http://127.0.0.1:1212/api/health_check"]}' target/debug/tracker_client check
```

Result: exit code `0`, stderr empty, stdout:

```json
[{"Udp":{"Ok":{"remote_addr":"127.0.0.1:6969","results":[["Setup",{"Ok":null}],["Connect",{"Ok":null}],["Announce",{"Ok":null}],["Scrape",{"Ok":null}]]}}},{"Health":{"Ok":{"url":"http://127.0.0.1:1212/api/health_check","result":{"Ok":"200 OK"}}}},{"Http":{"Ok":{"url":"http://127.0.0.1:7070/","results":[["Announce",{"Ok":null}],["Scrape",{"Ok":null}]]}}}]
```

### V2 - E2E checker launch (M2, smoke test)

- Goal: the helper `tracker_checker::run` builds and launches `tracker_client check` with
  `TORRUST_CHECKER_CONFIG`, captures the child's stdout and stderr, and succeeds.
- Status: `DONE`
- Scope: this called the helper directly from a throwaway integration test, `tests/tmp_m2.rs`
  (not committed; it was deleted after the run and is not part of the PR), against the tracker of V1.
  It is a smoke test of the helper; it is not a run of the e2e runner. The full runner needs Docker;
  the PR's container test job runs `e2e_tests_runner`, and its result is cited in the acceptance
  verification of the spec. The interactive-terminal case is verified in issue #2768 once the TTY
  refusal exists.
- This section records the second run, on the helper after the review round of PR 2901 (the helper
  then built the binary in a separate step). The first run, on the first version of the helper
  (no build step), also passed and logged the same `cargo run ... -- check` command line.

Harness (`tests/tmp_m2.rs`, verbatim):

```rust
#[test]
fn m2() {
    drop(tracing_subscriber::fmt().with_test_writer().with_ansi(false).try_init());
    let cfg = r#"{"udp_trackers":["127.0.0.1:6969"],"http_trackers":["http://127.0.0.1:7070"],"health_checks":["http://127.0.0.1:1212/api/health_check"]}"#;
    torrust_tracker_lib::console::ci::e2e::tracker_checker::run(cfg).expect("ok");
}
```

Command (ran 10:46:33.534 to 10:46:35.684 UTC, with the tracker of V1 running):

```text
cargo test --test tmp_m2 -- --nocapture
```

Logged lines (trimmed to the relevant ones; the checker output is the JSON array of V1, cut here):

```text
Running: cargo build -p torrust-tracker-client --bin tracker_client
Running: TORRUST_CHECKER_CONFIG=[config] cargo run -p torrust-tracker-client --bin tracker_client -- check
stdout: [{"Udp":{"Ok":{"remote_addr":"127.0.0.1:6969", ... }}}, {"Health":{"Ok":...}}, {"Http":{"Ok":...}}]
stderr: Running `target/debug/tracker_client check`
test m2 ... ok
```

### V3 - No document or script tells the reader to run a removed binary (AC2)

- Goal: AC2 holds at this head, outside the historical records.
- Status: `DONE`
- Scope: the search skips `docs/issues/closed`, `docs/issues/drafts`, `docs/pr-reviews`, `docs/adrs`,
  `docs/refactor-plans` (historical records) and this spec's `ISSUE.md`, which describes the
  command as it was before the change. The pattern uses bracketed letters so this file does not
  match its own text.

Command (ran at 2026-11-17 10:46 UTC):

```text
git grep -nE -e '--bin (h[t]tp_tracker_client|u[d]p_tracker_client|tracker_che[c]ker)' -e '^[[:space:]]*(h[t]tp_tracker_client|u[d]p_tracker_client|tracker_che[c]ker) ' -e '(run|Run|use) `(h[t]tp_tracker_client|u[d]p_tracker_client|tracker_che[c]ker)' -- . ':!docs/issues/closed' ':!docs/issues/drafts' ':!docs/pr-reviews' ':!docs/adrs' ':!docs/refactor-plans' ':!docs/issues/open/2765-2758-remove-deprecated-tracker-clients/ISSUE.md'
```

(The spec was in `docs/issues/open/` when the search ran; the exclusion names that path. The spec has since moved to `docs/issues/closed/`.)

Result: no output, exit code `1`. The same search without the last exclusion matches only line 47
of this spec's `ISSUE.md` (exit code `0`). The open spec of issue #2594 named `tracker_checker` as a
command in three places; review finding F3 of PR 2901 found them, and that spec now names
`tracker_client check`.

### V4 - Runner-level proof of AC4

Proven by the container test job of PR 2901, which runs `e2e_tests_runner`: it passed on the head that carries the commit `docs(pr-reviews): [#2765] audit review round 7 of PR 2901` (job https://github.com/torrust/torrust-tracker/actions/runs/44115063923/job/134153598863), and its log shows the helper running `cargo build -p torrust-tracker-client --bin tracker_client` and then `TORRUST_CHECKER_CONFIG=[config] cargo run -p torrust-tracker-client --bin tracker_client -- check`, which ran `target/debug/tracker_client check`. The job of the head before it (the commit `docs(pr-reviews): [#2765] audit review round 6 of PR 2901`) failed in the image test stage, before the e2e step, on a racy unit test that the round-7 commit `test(e2e): [#2765] drop the racy byte-delta assertion of the receiver-gone test` fixed. Two later commits followed: `fix(e2e): [#2765] retry interrupted reads and bound the output logged on success` changed the reader, its logging and their tests (and added their prose-first rows to this spec), and the audit commit `docs(pr-reviews): [#2765] audit review round 8 of PR 2901` (the head merged) changed only the audit; the Docker job of that final head was still in progress when the PR was merged. `BUILD_ARGS`, `CHECKER_ARGS` and the `TORRUST_CHECKER_CONFIG` variable did not change after the passing head.
