---
doc-type: manual-verification-evidence
issue-spec: docs/issues/open/2413-2410-si-22-1-document-event-flows-and-draft-adr/ISSUE.md
last-updated-utc: "2026-10-16 11:25"
---

# Manual Verification Evidence

## Purpose

Record the verification of #2413 (SI-22.1), a documentation-only task. The reproduction of the bug the sub-EPIC fixes is in the sub-EPIC's [manual-verification-evidence.md](../2410-1488-si-22-process-queued-events-before-listeners-stop/manual-verification-evidence.md), the EPIC baseline; this file does not repeat it.

## Non-applicability of executable-boundary verification

Issue #2413 changes only Markdown: an inventory, a glossary, a moved document, and an ADR draft. No runtime behaviour changes, so there is no executable boundary to run and the V0/V1 reproduction does not apply. EPIC #1488 allows a pure documentation or test-only task to record a reasoned non-applicability decision instead (`docs/issues/open/1488-overhaul-tracker-shutdown/ISSUE.md`, the Manual Verification task rule), and the sub-EPIC's Shared Delivery Gates ask each child to keep its evidence in its own `manual-verification-evidence.md`.

## M1 - Inventory against the code

- Date and time (UTC): 2026-10-16 07:55; `develop` at `702cff6cc`.
- Method: `rg` for `.send(`, `event_bus`, `.receiver()` and `spawn` across `src/` and `packages/`, then reading each hit, including `packages/axum-http-server/src/testing/environment.rs` and `packages/udp-server/src/testing/environment.rs`. Read from source, not run.
- Result: every producer and consumer found is listed in the inventory in `docs/features/shutdown-process/task-inventory.md`. No unlisted producer or consumer was found. The reviewer ticks the acceptance criteria.
- Correction (2026-10-16 10:44 UTC): the result above was wrong. The method searched for `.send(` call sites, so it missed the four publish sites in the UDP receive loop (`packages/udp-server/src/server/launcher.rs`), which reach `send` through the awaited helper `publish_event_if_sender_available`. Copilot found them in review (`review-finding:pr-2553-f2`). Re-swept on 2026-10-16 by reading every file that holds a bus sender (`rg 'stats_event_sender|event_sender|publish_event_if_sender_available'`); the launcher was the only file missing from the inventory. Fixed in `docs/features/shutdown-process/task-inventory.md`. Read from source, not run.

## Tooling defect found while pushing: pre-push tests leaked into the real repository

No separate issue exists for this defect; it blocked the push of #2413, so it is recorded here and fixed in its own commit, `fix(dev-tools): [#2413] isolate clippy-allow-reasons fixture Git commands from hook environment`.

- Symptom (2026-10-16 09:3x UTC): the first `git push` ran the pre-push hook, and `cargo +stable test --tests --benches --examples --workspace --all-targets --all-features` failed in `clippy-allow-reasons --test cli` (6 of 8 tests). The tests had written into the real repository: a commit by "Validator tests" (`test: establish documented baseline`) on the #2413 branch, a new branch `feature`, `user.name` and `user.email` in the shared `.git/config`, and `.git/config` lock errors. The branch, the extra branch and the config entries were restored by hand; nothing was pushed.
- Hypothesis (read from source): Git exports `GIT_DIR` and other repository-selecting variables to hooks, and the fixture helpers in `contrib/dev-tools/checks/clippy-allow-reasons/tests/cli.rs` (`git()` and the binary runners) did not clear them, so the fixture's `git init`, `git config` and `git commit` acted on the real repository. `frontmatter-validator`'s tests already clear them.
- Reproduction (real test binary, throwaway repository, never the real one): `S=$(mktemp -d); git init -q --initial-branch=main $S; git -C $S -c user.email=a@b -c user.name=seed commit -q --allow-empty -m seed; GIT_DIR=$S/.git GIT_WORK_TREE=$S cargo test -q -p clippy-allow-reasons --test cli`. Result before the fix: `test result: FAILED. 2 passed; 6 failed`, and the throwaway repository's local config gained `user.name = Validator tests`.
- Regression-test boundary: a deterministic unit test at the seam, `it_should_not_let_fixture_commands_inherit_the_repository_environment`, which checks that a fixture command removes every variable listed by `git rev-parse --local-env-vars`. It is smaller and safer than a test that sets `GIT_DIR` (setting process environment in parallel tests is unsafe).
- Red: with `fixture_command` not clearing the variables, `cargo test -q -p clippy-allow-reasons --test cli` gave `fixture commands inherit: ["GIT_ALTERNATE_OBJECT_DIRECTORIES", "GIT_CONFIG", ... "GIT_COMMON_DIR"]` and `test result: FAILED. 8 passed; 1 failed`.
- Fix: every fixture `Command` goes through `fixture_command`, which calls `env_remove` for each of those variables.
- Green: `cargo test -q -p clippy-allow-reasons --test cli` gave `test result: ok. 9 passed`.
- Like-for-like recheck: the reproduction command above, on a new throwaway repository, gave `test result: ok. 9 passed`, and the throwaway repository kept one commit (`seed`), one branch (`main`), a clean status, and no `user.name`. The real repository was unchanged. This ran on the working tree before the fix commit; it is repeated by the pre-push hook before the push.
