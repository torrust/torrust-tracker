---
semantic-links:
  related-artifacts:
    - .github/agents/task-reviewer.agent.md
    - docs/issues/open/2668-1488-si-20-configure-shutdown-policy/ISSUE.md
---

# Agent Review Reports - Issue #2668 Configure Shutdown Policy and Deployment Contract

> Append one completed independent-review entry at a time. Do not modify, reorder, or remove
> earlier entries. A correction is a new entry that names the earlier conclusion.

## Reports

### 2026-10-27 21:00 UTC - Task Reviewer (independent, read-only)

- Invocation scope: the whole branch `2668-configure-shutdown-policy` against `torrust/develop` (25 signed commits), acceptance criteria AC1 to AC11, every added or changed test, the documents and ADRs, the completion review, and the spec and evidence hygiene. The branch head reviewed was the commit `chore(issues): remove the disposable verification script of #2668`.
- Inputs: `ISSUE.md`, `manual-verification-evidence.md`, the diff, the source files named in the spec, the templates, the two new ADRs, and `tests/lifecycle/shutdown_policy.rs` with the shared native-tracker fixture.
- Evidence: the reviewer ran `cargo test --test lifecycle-shutdown-policy` (21 passed), the configuration package tests (156 passed), the root library tests (156 passed), the HTTP configured-drain test, and `linter all` (passed). It searched `src/`, `packages/` and `tests/` for the temporary fault-injection variable (none) and the added lines for `/tmp`, `.patch`, `/home/` and PR-branch commit ids (none). It ran the real binary once with a UDP tracker and `application.shutdown` set to 20/6/3/2 and SIGTERM: `request_drain_deadline=3s`, `drain_timeout=6s`, `timeout_seconds=20` and the success line.
- Findings:
  - Blocker B1: the completion review still read `Not yet assessed`, no retrospective existed, and this report file did not exist. The reviewer judged `implementation-retrospective.md` required (D5 changed the design during the work; the shared fixture `shutdown()` ignored the exit status; the drain helper panicked on an oversized `Duration` and `JobManager::wait_for_all` still adds to the clock unchecked; the chain rule is a bound, not a prediction; the H default fell from 20 s to 15 s because of the listener stage).
  - Major M1: AC5 is not met for `U` and `L` by a maintained test: swapping `udp_request_drain()` for `http_drain()` in `src/app.rs` passes every test. The wiring is correct (confirmed by the reviewer's run). Remedy proposed: record the real-binary check with distinct values as an evidence section.
  - Minor m1: stale evidence text (the introduction said the file holds only V1; a sentence said the script is still in `evidence/`; the V1 paragraph said the branch changes only documentation).
  - Minor m2: `src/AGENTS.md` still said `main` waits up to 10 seconds.
  - Minor m3: `docs/features/shutdown-process/README.md` stamp not bumped.
  - Minor m4: the M3 row said the exit takes about 3 s; V3 recorded 2.0 s.
  - Minor m5: the spec and evidence said 25 new tests in the configuration package; the final tree has 27 (T4 and the D5 follow-up added one each).
  - Minor m6: several native-binary tests are one-line wrappers around helpers, so the production Act is not visible in the test body, and `assert_rejected_before_any_service_starts(budgets, keys)` is close to a parameter bag. Acceptable as a convention if the maintainer agrees.
  - Minor m7: the source-text guard test in `src/bootstrap/jobs/mod.rs` is brittle; the author documents that no seam exists.
  - Minor m8: `src/container.rs` lost a comment and a blank line in a struct literal.
  - Minor m9: the drain helper now clamps an oversized `Duration`, but `JobManager::wait_for_all` still adds to `Instant::now()` unchecked; only the 3600 s configuration bound protects it.
  - AC11 pending: the Rollback text omits that the HTTP drain default falls from 90 s to 15 s and the health-check drain rises from 5 s to 15 s (they are in Risks and the ADR). Operator-facing changes worth the PR description and release notes: exit code 1 after an incomplete stop, the drain defaults, and the public signature changes of the server crates (0.1.0).
  - Acceptance criteria: AC1, AC2, AC3, AC4, AC6, AC7, AC8, AC9, AC10 `PASS`; AC5 and AC11 `PENDING`. No checkbox was ticked by the reviewer or the author.
- Verdict: REVIEW FAILED (documentation and evidence only; no code rework required).
- Follow-up actions:
  - Author: write the retrospective and the completion-review line (B1); record the real-binary `U` check (M1); fix m1 to m5 and m8; add the clamp gap (m9) to the retrospective; widen the Rollback text (AC11); then request a re-review entry.
  - Maintainer: decide whether the evidence suffices for AC5 and whether the wrapper-style native tests (m6) are acceptable.
