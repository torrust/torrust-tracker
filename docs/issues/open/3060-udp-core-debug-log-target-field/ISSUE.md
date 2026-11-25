---
schema-version: 1
doc-type: issue
issue-type: bug
status: planned
priority: p2
epic: null
github-issue: 3060
spec-path: docs/issues/open/3060-udp-core-debug-log-target-field/ISSUE.md
branch: "3060-udp-core-debug-log-target-field-spec"
related-pr: null
last-updated-utc: "2026-11-25 17:06"
semantic-links:
  skill-links:
    - create-issue
    - fix-bug
  related-artifacts:
    - .claude/skills/fix-bug/SKILL.md
    - .claude/skills/create-issue/SKILL.md
---

<!-- skill-link: create-issue -->

# Issue #3060 - Two UDP core DEBUG lines record the log target as a field instead of setting it

Related to #2387

## Goal

The two DEBUG lines that announce the UDP core statistics events (`UdpAnnounce`, `UdpScrape`) are logged under the `UDP TRACKER` log target, like every other UDP core log line, so per-target log filtering and the `target` of structured logs are correct.

## Background

Found while working on #3031 (EPIC #2387); it was not part of that issue.

**What happens, step by step.** `packages/udp-core/src/services/announce.rs:178` and `packages/udp-core/src/services/scrape.rs:140` are written as:

```rust
tracing::debug!(target = crate::UDP_TRACKER_LOG_TARGET, "Sending UdpAnnounce event: {event:?}");
```

In `tracing`, `target = value` (an equals sign) is an ordinary **field** named `target`; the log target is set only with `target: value` (a colon). So:

1. `send_event` publishes a statistics event and logs the DEBUG line.
2. `tracing` records the target as the default, the module path (`torrust_tracker_udp_core::services::announce`).
3. The intended value `UDP TRACKER` is recorded as a stray field `target="UDP TRACKER"` at the end of the line.

**Expectation violated.** Every other call site in the repository sets the target with the colon form: 111 call sites use `target: ..._LOG_TARGET`, including the other `UDP_TRACKER_LOG_TARGET` users in `packages/udp-core` (`banning.rs`, `statistics/event/listener.rs`). The constant is exported precisely so UDP core lines share one target.

**Impact.** Only DEBUG output is affected, and nothing is functionally wrong. A consumer that selects log lines by target (a `grep` for `UDP TRACKER`, the `target` key of the JSON log style, a log shipper's filter) misses these two lines, and for the JSON style the record carries a wrong `target` plus a stray `target` field. The EPIC #2387 logging conventions rely on named targets.

**Verified scope (2026-11-25, `torrust/develop` at the merge of #3053).** two scans find exactly these two sites: the single-line `git grep -nE '\(\s*target\s*=' -- '*.rs'` and the multi-line `git grep -n -P -z -o '(?s)\b(?:trace|debug|info|warn|error|event|span|trace_span|debug_span|info_span|warn_span|error_span)!\s*\(\s*target\s*=[^=]' -- '*.rs'`, whose `-z -P (?s)` form matches a call written across lines. The multi-line command is the check for AC3; both outputs are recorded in `manual-verification-evidence.md` (section V1).

## Scope

### In Scope

- Change the two call sites to `target: crate::UDP_TRACKER_LOG_TARGET`.
- Two maintained regression tests (announce and scrape) asserting the captured line carries the `UDP TRACKER` target.
- Decide whether to add a repository-wide guard against the mistake coming back (open question, see Design Decision); none is planned by default.

### Out of Scope

- Changing log levels, messages or other targets (EPIC #2387 owns those).
- Making this a sub-issue of #2387: it is standalone; `Related to #2387` is context only.

## Design Decision

The fix is the two-character change from `=` to `:`. Whether to also add a lint to catch the mistake is open for the maintainer: `clippy` does not flag it (the macro accepts the field form as valid). Proposed default: **no new lint**; the two regression tests and the `git grep` in AC3 are enough for two sites. A repository-wide guard (for example a `git grep` step in `linter`) can be a separate issue if wanted.

## Architectural Decisions

- Related ADRs: none known. The logging convention is in EPIC #2387's ADR work (SI-2).
- ADRs to create: None known.

## Design and Ownership Review

Not applicable.

## Bug-Fix Process

Follows `.claude/skills/fix-bug/SKILL.md`:

- **Analysis:** the field-versus-target macro syntax in the two call sites (above).
- **Reproduction:** V1, done on 2026-11-25: a temporary test captured the real DEBUG line of both services. Outcome: **Reproduced**.
- **Regression-test boundary:** see below.
- **Red test:** recorded before the fix.
- **Fix:** `=` to `:` at both sites.
- **Green and recheck:** regression tests green, then V1 rerun after the last production change.

## Regression Test Strategy

- **Boundary:** a unit test in each of `announce.rs` and `scrape.rs` (the existing `#[cfg(test)]` modules, next to `it_should_publish_..._event`). It builds the service with a mock event sender, starts `torrust_tracker_test_helpers::logging::capture_logs_in_scope()` (added by #3031 and already used by the listener tests in `statistics/event/listener.rs`) under `#[tokio::test]`, calls `handle_announce` / `handle_scrape`, and reads `capture.lines()`.
- **Why `capture_logs_in_scope` and not `capture_logs_while`:** both exist in `packages/test-helpers/src/logging.rs`, but only `capture_logs_in_scope` calls `keep_a_second_dispatcher_alive`. While `tracing` knows a single dispatcher, a callsite first reached from another thread is cached as never interested and a capture on this thread then misses its records. The two callsites are also reached by the existing parallel tests `it_should_publish_an_announce_event_with_the_connection_context_and_the_announced_peer` and `it_should_publish_a_scrape_event_with_the_connection_context`, so a test built on `capture_logs_while` could capture nothing and be red for the wrong reason.
- **Assertions (each test):** the capture is not empty and has exactly one line with `Sending UdpAnnounce event` (resp. `UdpScrape`) (so an empty capture cannot pass the negative checks); that line contains `UDP TRACKER: Sending UdpAnnounce event` (the `fmt` subscriber prints `<target>: <message>`); it does not contain the module-path target (`torrust_tracker_udp_core::services::announce`, resp. `scrape`); and it does not contain `target="UDP TRACKER"` (the stray field).
- **Why this boundary:** it is the smallest deterministic test that exercises the causal macro call; the symptom is only visible in the emitted record.
- **Evidence:** red and green output in `manual-verification-evidence.md`.

## Implementation Plan

Status values: `TODO`, `IN_PROGRESS`, `BLOCKED`, `DONE`.

| ID  | Status | Task                                                         | Notes / Expected Output                                         |
| --- | ------ | ------------------------------------------------------------ | --------------------------------------------------------------- |
| T1  | DONE   | Reproduce on both services and record it                     | V1 recorded in `manual-verification-evidence.md`                |
| T2  | TODO   | Add the announce and scrape regression tests and observe them red | Red run recorded on the unfixed code, both tests failing   |
| T3  | TODO   | Change `=` to `:` at both call sites                        | Regression tests green                                          |
| T4  | TODO   | Rerun V1 after the last production change; run `linter all`  | Evidence names the commit by Conventional Commit subject        |

## Commit Points

| Task | Coherent change set                                  | Commit policy                                        |
| ---- | ---------------------------------------------------- | ---------------------------------------------------- |
| T2   | The two regression tests plus their red evidence     | Commit after focused validation and required review. |
| T3   | The two-site production fix                          | Commit after focused validation and required review. |
| T4   | Final evidence and spec progress                     | Commit after focused validation and required review. |

Record a justified no-change decision in the task's evidence without creating an empty commit. For test-producing work, use the `write-unit-test` skill and complete an explicit design review after each passing test increment, before maintainer review and commit, using the mandatory prose-first Arrange-Act-Assert comparison. Use a Conventional Commit message with the narrow affected scope, and sign every commit with GPG.

## Progress Tracking

### Workflow Checkpoints

- [x] Folder-style spec drafted in `docs/issues/open/3060-udp-core-debug-log-target-field/ISSUE.md`
- [x] Spec reviewed and approved by user/maintainer
- [x] GitHub issue created (#3060) and issue number added to this spec
- [ ] Spec-only PR merged into `develop` before implementation
- [ ] Implementation completed
- [ ] Automatic verification completed (`linter all`, relevant tests, and any pre-push checks)
- [ ] Manual verification scenarios executed and recorded in issue-local `manual-verification-evidence.md`
- [ ] Acceptance criteria reviewed after implementation and updated with evidence
- [ ] Evidence-based implementation completion review recorded
- [ ] Reviewer validated acceptance criteria and updated checkboxes
- [ ] Independent reviewer reports recorded in issue-local `agent-review-reports.md` when reviewers received this folder-style specification
- [ ] Committer verified spec progress is up to date before commit
- [ ] Issue closed and spec moved from `docs/issues/open/` to `docs/issues/closed/`

### Progress Log

- 2026-11-25 17:01 UTC - Claude Code (Claude Sonnet 5.5) - Verified the bug on current `develop`, reproduced it on both services (V1), drafted the spec - `manual-verification-evidence.md` V1
- 2026-11-25 17:06 UTC - Claude Code (Claude Sonnet 5.5) - Maintainer approved the draft in session; GitHub issue #3060 created (type Bug could not be set by the bot, maintainer to set it); spec moved to open/

## Acceptance Criteria

- [ ] AC1: The `UdpAnnounce` DEBUG line is logged with target `UDP TRACKER` and has no `target` field (announce regression test).
- [ ] AC2: The `UdpScrape` DEBUG line is logged with target `UDP TRACKER` and has no `target` field (scrape regression test).
- [ ] AC3: the multi-line scan `git grep -n -P -z -o '(?s)\b(?:trace|debug|info|warn|error|event|span|trace_span|debug_span|info_span|warn_span|error_span)!\s*\(\s*target\s*=[^=]' -- '*.rs'` prints nothing, so no `tracing` macro call, one-line or spread over several lines, uses the field form (before the fix it printed exactly the two sites).
- [ ] AC4: Both regression tests were observed red before the fix and green after.
- [ ] `linter all` exits with code `0`
- [ ] Relevant tests pass
- [ ] Manual verification scenarios are executed and documented in issue-local `manual-verification-evidence.md`
- [ ] Acceptance criteria are re-reviewed after implementation and reflect actual behavior
- [ ] Documentation is updated when behavior/workflow changes (none expected)

## Verification Plan

### Automatic Checks

- `linter all`
- `cargo test -p torrust-tracker-udp-core` (stable toolchain)
- Pre-push checks

### Manual Verification Scenarios

| ID  | Scenario                                      | Human-oriented command/steps                                                                          | Expected Result                                                  | Status | Evidence                                     |
| --- | --------------------------------------------- | ----------------------------------------------------------------------------------------------------- | ---------------------------------------------------------------- | ------ | -------------------------------------------- |
| V1  | Captured DEBUG lines of announce and scrape   | Temporary test: run each service once inside `capture_logs_in_scope` and print the captured text          | Before fix: module-path target and a `target="UDP TRACKER"` field. After fix: `UDP TRACKER:` prefix, no field | DONE (before-fix result); after-fix rerun TODO | `manual-verification-evidence.md` section V1 |

### Disposable Verification Scripts

None. The reproduction is a temporary Rust test (code recorded verbatim in the evidence file); it only prints and never asserts. The maintained tests (T2) are separate asserting tests, and the red evidence comes from them.

### Acceptance Verification

| AC ID | Status | Evidence |
| ----- | ------ | -------- |
| AC1   | TODO   | -        |
| AC2   | TODO   | -        |
| AC3   | TODO   | -        |
| AC4   | TODO   | -        |

## Risks and Trade-offs

- **Log text changes for operators:** at DEBUG the two lines now start with `UDP TRACKER:` instead of the module path. Intended; DEBUG output is not a stable interface.
- **No lint:** the mistake could recur elsewhere; accepted for two sites (see Design Decision).

## Implementation Completion Review

After implementation, compare the result with this specification. Record invalidated assumptions, material design changes, unexpected validation findings, and reusable lessons.

- Retrospective: `Not yet assessed`
- If needed, create `implementation-retrospective.md` from the repository template at `docs/templates/IMPLEMENTATION-RETROSPECTIVE.md` in this issue specification's directory.
- If no retrospective is needed, add a concise progress-log entry explaining why the work had no material discovery.
- When an independent reviewer receives this folder-style specification, it records its result in `agent-review-reports.md` using `docs/templates/AGENT-REVIEW-REPORTS.md`.

## References

- Related issues: #3031, EPIC #2387 (context only)
- Related ADRs: none
