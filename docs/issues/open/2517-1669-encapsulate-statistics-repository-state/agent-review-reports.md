---
semantic-links:
  related-artifacts:
    - .github/agents/complexity-auditor.agent.md
    - .github/agents/task-reviewer.agent.md
    - .github/agents/pr-reviewer.agent.md
    - docs/agents/orchestration.md
---

# Agent Review Reports - Make the Statistics Repositories' Metrics Lock Private

> Append one completed independent-review entry at a time. Do not modify, reorder, or remove
> earlier entries. A correction is a new entry that names the earlier conclusion.

## Reports

### 2026-10-14 14:06 UTC - Task Reviewer (pre-PR, `review-task` skill)

- Invocation scope: Pre-push review of branch `2517-1669-encapsulate-statistics-repository-state`
  (`develop` at `c1874fac`..`392dc5e9`, 7 signed commits). Reviewed all acceptance criteria and
  workflow checkboxes in `ISSUE.md`, the five `packages/*/src/statistics/repository.rs` changes,
  the ten new doctests, commit structure and messages, and `manual-verification-evidence.md`.
- Inputs: `ISSUE.md`, `manual-verification-evidence.md`, `git diff develop...HEAD`,
  `git log --show-signature develop..HEAD`, `.github/skills/dev/task-reviews/review-task/SKILL.md`,
  `.github/skills/dev/testing/write-unit-test/SKILL.md` (rule 4: `compile_fail` doctest and a
  companion), and the `packages/udp-core/src/container.rs` `cookie_cipher` precedent.
- Evidence:
  - AC1 grep `grep -nE 'pub(\([^)]*\))? +stats *:' packages/*/src/statistics/repository.rs`
    printed nothing (exit 1).
  - `git grep -nE '\.stats\b' -- '*.rs'`, excluding the known other types and the `stats()`
    method, matched only lines inside the five repository files (method bodies and the
    `compile_fail` snippets). `git grep 'Repository *{ *stats'` found no struct literals.
  - Mutation (red) check in a disposable detached worktree at `HEAD` (removed afterwards): with
    `stats` changed back to `pub`, `cargo test --no-fail-fast --doc` on the five packages failed
    all five `compile_fail` doctests with "Test compiled successfully, but it's marked
    `compile_fail`"; the five companion doctests still passed. This shows the field access is
    the only reason each snippet fails to compile.
  - Green: `cargo test --doc` (stable) and `cargo +nightly test --doc` (pinned error code
    enforced) on the five packages, filtered to `statistics::repository`: 10 passed, 0 failed.
  - `cargo check --workspace --all-targets --all-features` passed.
  - `RUSTDOCFLAGS="-D warnings" cargo doc --no-deps` on the five packages passed (intra-doc links
    to `Repository::get_stats` / `Repository::get_metrics` resolve).
  - `linter all` exited `0`.
  - All seven commits carry a good GPG signature; each `refactor(<package>)` commit touches only
    its package's `repository.rs` plus `ISSUE.md` progress.
  - Not re-run: full stable all-target workspace tests and `pre-push.sh` (implementer claim; no
    concrete reason to doubt it after the workspace check and focused doctests passed).
  - Acceptance-criteria matrix:

    | Item                                                         | Result  | Evidence                                                                                                       |
    | ------------------------------------------------------------ | ------- | -------------------------------------------------------------------------------------------------------------- |
    | AC1: no `pub` metrics lock field                             | PASS    | AC1 grep prints nothing; diff shows `stats` private in all five files.                                         |
    | AC2: outside code uses only repository methods               | PASS    | Field access confined to the five files; workspace all-target check passes; 5 `compile_fail` doctests pass.  |
    | AC3: REST metric values unchanged for the same announces     | PENDING | Only a subset compared; swarm and tracker-core metrics not recorded (finding 1).                               |
    | `linter all` exits `0`                                       | PASS    | Re-run by reviewer, exit `0`.                                                                                  |
    | Relevant tests pass                                          | PASS    | Focused doctests (stable and nightly) and mutation check re-run; workspace check passes.                      |
    | Manual verification executed and documented                 | PENDING | M1 recorded, but not against its full expected result (finding 1).                                             |
    | Acceptance criteria re-reviewed after implementation         | PENDING | AC3 marked DONE with evidence that does not cover all four counter families (finding 1).                       |
    | Documentation updated when behavior/workflow changes         | PASS    | Rustdoc on each `Repository` documents the accessor; no runtime behavior change.                               |
    | Workflow: spec drafted, approved, issue created, spec PR merged | PASS | Progress log 06:50-08:14 UTC; spec present on `develop` at the merge base.                                     |
    | Workflow: implementation completed                           | PASS    | Diff in five files matches In Scope.                                                                           |
    | Workflow: automatic verification completed                   | PASS    | See evidence above.                                                                                            |
    | Workflow: manual verification recorded                       | PENDING | Finding 1.                                                                                                     |
    | Workflow: acceptance criteria reviewed with evidence         | PENDING | Finding 1.                                                                                                     |
    | Workflow: completion review recorded                         | PASS    | Progress log 13:12 UTC explains why no retrospective is needed; deviations are minor and documented.          |
    | Workflow: Committer verified spec progress                   | FAIL    | Ticked by the implementer, not by Committer (finding 2).                                                       |

  - Test design (doctests): each `compile_fail` snippet states one property (an external crate
    cannot read `stats`), is deterministic, and fails only because of the field access (proved by
    the mutation check and the pinned `E0616`). The companion doctests are appropriate, not scope
    creep: `write-unit-test` rule 4 requires them, the spec's Commit Points defer doctest design to
    that skill, and they prove the import path, type, and accessor compile, which isolates the
    `compile_fail` cause. Prose-first Arrange-Act-Assert comparison: not applicable to these
    compile-time doctests (no runtime Act or assertion; the compiler error is the assertion); the
    progress-log design-review entries for T2-T6 cover the equivalent intent.
- Findings:
  1. Medium (blocking): AC3/M1 evidence does not cover the scenario's stated expected result.
     `ISSUE.md` line 313 expects "the UDP, HTTP, tracker-core, and swarm counters" to reflect the
     announces as on `develop`, and line 319 asks for relevant tracker logs.
     `manual-verification-evidence.md` lines 100 and 126-131 record only
     `http_tracker_core_requests_received_total` and `udp_tracker_core_requests_received_total`
     samples plus REST stats fields. Those fields come from the `http-core` and `udp-server`
     repositories and from the in-memory swarm aggregate, not from the swarm statistics
     `Repository`. No `swarm_coordination_registry_*` or `tracker_core_*` samples, no `diff -u`
     output, and no tracker log lines are recorded, and the raw outputs were not kept. A `started`
     announce also cannot change the `tracker_core_*_downloads_total` counters, so the scenario
     must at least record them as unchanged, or add a `completed` announce.
  2. Medium (blocking, spec hygiene): `ISSUE.md` line 215 ("Committer verified spec progress is up
     to date before commit") was ticked by the implementer. Only Committer may tick it.
  3. Low (blocking, spec hygiene): `ISSUE.md` line 5 has `status: planned`, but implementation is
     complete. It should be `in-progress` until the PR is opened, then `in-review`.
  4. Low (non-blocking): the Implementation Plan table at `ISSUE.md` lines 184-187 has broken
     column padding (T4-T6 Task cells and the over-padded T7 ID cell). markdownlint accepts it; the fix
     is cosmetic.
  5. Low (non-blocking, doc quality): no lead-in sentence says what the `compile_fail` block shows.
     The `container.rs` precedent (lines 143-153) introduces each block. Suggested: add
     "The metrics lock is private:" before each `compile_fail` block and "Read through the accessor
     instead:" before each companion. Locations: `packages/udp-core/src/statistics/repository.rs`
     lines 26-27, `packages/udp-server/src/statistics/repository.rs` lines 27-28,
     `packages/http-core/src/statistics/repository.rs` lines 14-15,
     `packages/tracker-core/src/statistics/repository.rs` lines 18-19, and
     `packages/swarm-coordination-registry/src/statistics/repository.rs` lines 14-15.
  6. Info: the companion doctests are an addition the spec does not list (In Scope, `ISSUE.md`
     line 117). They are appropriate (see Test design). Optionally mention them in In Scope or
     Regression Test Strategy so the spec matches the result.
  7. Info: T1 (re-verify evidence) was recorded at 13:06 UTC, after T2-T6 (12:55-13:02 UTC). It
     ran against `develop` at `c1874fac`, so the result is valid. No action needed.
  8. Info: commit `cb076314` ("record manual verification preference") is not in the Commit Points
     table. It is a docs-only change, which is acceptable.
  9. Info (pre-existing, out of scope): the `tracker-core` `Repository` doc says "A repository for
     the torrent repository metrics" (`packages/tracker-core/src/statistics/repository.rs`
     line 16).
- Verdict: REVIEW FAILED
- Follow-up actions:
  - Implementer: re-run M1 on both `develop` and the branch, or obtain and record maintainer
    acceptance of the reduced comparison. The re-run should record the post-announce
    `swarm_coordination_registry_*`, `tracker_core_*`, and `udp_tracker_server_*` samples from
    `/api/v1/metrics`, the `diff -u` result, and relevant tracker log lines in
    `manual-verification-evidence.md` V1. Then update AC3 and the Acceptance Verification row
    (`ISSUE.md` lines 284 and 331).
  - Implementer: clear the checkbox at `ISSUE.md` line 215, and set `ISSUE.md` line 5 to `status: in-progress`.
  - Implementer (optional): fix findings 4-6.
  - Caller: request a new Task Reviewer entry after remediation. The reviewer did not edit
    `ISSUE.md` checkboxes because the caller restricted edits to this report.
  - Code changes: none required. The visibility change and doctests are correct.
