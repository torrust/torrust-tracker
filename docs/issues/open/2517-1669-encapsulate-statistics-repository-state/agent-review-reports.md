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

### 2026-10-14 14:21 UTC - Task Reviewer (pre-PR, `review-task` skill, round 2)

- Invocation scope: Second pre-push review of branch
  `2517-1669-encapsulate-statistics-repository-state` (`develop` at `c1874fac`..`3e339389`, 10
  signed commits). Checked that each round 1 finding is fixed. Re-checked AC1-AC3, every
  acceptance and workflow checkbox, and whether `ISSUE.md`, `manual-verification-evidence.md`, and
  `evidence/` are accurate and consistent with each other. Included the new commit
  `docs(statistics): [#2517] introduce the repository privacy doctests`.
- Inputs: `ISSUE.md`, `manual-verification-evidence.md`, the four `evidence/` files, the round 1
  entry above, `git diff develop...HEAD`, `git log --format='%h %G? %s' develop..HEAD`,
  `.github/skills/dev/task-reviews/review-task/SKILL.md`,
  `docs/templates/MANUAL-VERIFICATION-EVIDENCE.md`, the implementer's local M1 run directories
  (`.tmp/m1-2517-baseline/` and `.tmp/m1-2517-implementation/`), and the implementer's
  session-local M1 runner.
- Evidence:
  - AC1 grep `grep -nE 'pub(\([^)]*\))? +stats *:' packages/*/src/statistics/repository.rs`
    printed nothing (exit 1).
  - `git grep -nE '\.stats\b' -- '*.rs'` found `.stats` only inside the five repository files
    (method bodies and the `compile_fail` snippets), after excluding the unrelated
    `announce_data.stats`, `domain_data.stats`, and `data.stats` fields and the `stats()` method.
    `git grep 'Repository *{ *stats'` matched only spec prose.
  - `cargo test --doc` (stable) and `cargo +nightly test --doc` (which enforces the pinned error
    code) on the five packages, filtered to `statistics::repository`: 10 passed, 0 failed on each
    toolchain (5 `compile_fail,E0616` doctests and 5 companion doctests).
  - `linter all` exited `0`. The worktree was clean apart from this review's edits.
  - All 10 commits are GPG signed (`%G?` = `U`: valid signature, key trust not set locally). Since
    the round 1 pre-push run, the only code change is the doc-comment commit `a8d4fc9f`, so the
    reviewer did not re-run the full pre-push suite.
  - M1 artifacts: in the local run directories, `client.txt`, `metrics-before.txt`,
    `metrics-after.txt`, `stats-before.normalized.json`, and `stats-after.normalized.json` are
    byte-identical between the baseline and the implementation run. `evidence/m1-metrics-before.txt`,
    `evidence/m1-metrics-after.txt`, and `evidence/m1-stats-after.json` are byte-identical to the
    implementation outputs. Both `tracker.log` files have 220 lines, contain no `WARN` or `ERROR`,
    and end with "Torrust tracker successfully shutdown." The baseline log ended at 14:12:22 UTC.
    The branch `torrust-tracker` binary was built at 14:12:38 UTC, and the implementation log
    ended at 14:12:45 UTC. This order matches the claim that the two runs used different builds.
    The configuration embedded in the evidence matches the generated `tracker.toml`, except for
    two `skill-link` comment lines and the database path.
  - Independent M1 re-run on the current branch (rebuilt binaries were already up to date, the
    run directory was removed afterwards, and no listener was left on the M1 ports):
    `metrics-before.txt`, `metrics-after.txt`, and `stats-after.normalized.json` matched the stored
    evidence exactly. `client.txt` matched the implementation run. The log had 220 lines, no
    `WARN` or `ERROR`, and ended with a successful shutdown.
  - All four evidence links in `manual-verification-evidence.md` resolve. The selected samples
    and stats quoted inline match `evidence/m1-metrics-after.txt` and `evidence/m1-stats-after.json`.
    All five metric families are covered. The `tracker_core_*_downloads_total` counters go from
    `0` to `2`, and the `swarm_coordination_registry_*` samples are present.
  - Round 1 findings:

    | #   | Round 1 finding                         | Result   | Evidence                                                                                                         |
    | --- | --------------------------------------- | -------- | ---------------------------------------------------------------------------------------------------------------- |
    | 1   | M1 evidence incomplete (Medium)         | RESOLVED | Reproduced: full-family metrics, stats, client output, `diff -u` summary, and a log summary are recorded.        |
    | 2   | Self-ticked Committer checkbox (Medium) | RESOLVED | `ISSUE.md` line 216 is not ticked.                                                                               |
    | 3   | `status: planned` (Low)                 | RESOLVED | `ISSUE.md` line 5 is `status: in-progress`.                                                                      |
    | 4   | Plan table padding (Low)                | PARTIAL  | The Task column is aligned. The Notes column is still 2-4 characters short in rows T1-T7 (finding 1 below).      |
    | 5   | Doctest lead-in sentences (Low)         | RESOLVED | Every repository has "The metrics lock is private:" and "Read through the accessor instead:" (commit `a8d4fc9f`). |
    | 6   | Companion doctests not in scope (Info)  | RESOLVED | `ISSUE.md` lines 116-118 (In Scope).                                                                             |

  - Acceptance-criteria matrix:

    | Item                                                    | Result | Evidence                                                                                             |
    | ------------------------------------------------------- | ------ | ---------------------------------------------------------------------------------------------------- |
    | AC1: no `pub` metrics lock field                        | PASS   | AC1 grep prints nothing; `stats` is private in all five files.                                       |
    | AC2: outside code uses only repository methods          | PASS   | Field access exists only in the five files; 5 `compile_fail,E0616` doctests pass (stable and nightly). |
    | AC3: REST metric values unchanged for the same announces | PASS   | Stored outputs are identical between the runs, and the reviewer's re-run matches the evidence.        |
    | `linter all` exits `0`                                  | PASS   | The reviewer re-ran it: exit `0`.                                                                    |
    | Relevant tests pass                                     | PASS   | 10 focused doctests on stable and nightly. The workspace tests were not re-run because only doc comments changed since round 1. |
    | Manual verification executed and documented             | PASS   | `manual-verification-evidence.md` V1 and `evidence/`, reproduced.                                    |
    | Acceptance criteria re-reviewed after implementation    | PASS   | AC rows and the Acceptance Verification table match the evidence.                                    |
    | Documentation updated when behavior/workflow changes    | PASS   | Each repository's rustdoc names its accessor. Runtime behavior is unchanged.                         |
    | Workflow checkpoints, lines 205-213                     | PASS   | Each is supported by the progress log, diff, checks, and evidence.                                   |
    | Workflow: Committer verified spec progress (line 216)   | N/A    | Correctly left unchecked. Committer must tick it.                                                     |

  - Test design (doctests): the round 1 assessment still holds. The new lead-in sentences now
    state each snippet's purpose, as in the `container.rs` precedent. Prose-first
    Arrange-Act-Assert comparison: not applicable to compile-time doctests, which have no runtime
    Act or assertion.
- Findings:
  1. Low (non-blocking, cosmetic): the round 1 finding 4 fix is partial. In the Implementation
     Plan table (`ISSUE.md` lines 180-188), the header's Notes cell is 177 characters wide, the T1
     row is 173, and the T2-T7 rows are 175. markdownlint and `linter all` accept the table.
  2. Info: `evidence/m1-client-responses.txt` lines 4-5 and 8 are not byte-identical to the raw
     `client.txt`. The UDP client prints no trailing newline, so in the raw output the next
     `$ http announce completed` label shares a line with the UDP JSON. The evidence copy adds the
     missing newlines, and the content is otherwise the same. Optionally say this in
     `manual-verification-evidence.md`, which calls the stored copy one of the identical outputs.
  3. Info: M1 sends two announces per protocol, while the scenario row at `ISSUE.md` line 322
     says one. `manual-verification-evidence.md` V1 step 3 discloses and justifies the extra
     `completed` announces, so this is acceptable. The scenario row may be updated to match.
  4. Info: the M1 steps ran through a session-local wrapper script so both runs used identical
     steps. The script ran the real binaries through human-oriented commands (`tracker_client`
     and `curl`). Every command is recorded, and the wrapper is disclosed in `ISSUE.md`
     (Disposable Verification Scripts). This is acceptable manual evidence, not automated-test
     output.
  5. Info (pre-existing, out of scope, carried over from round 1): the `tracker-core` row of the
     Background write-methods table (`ISSUE.md` line 65) is wider than its column on `develop`.
- Verdict: REVIEW PASSED
- Follow-up actions:
  - Implementer (optional): fix findings 1-3 in a later spec update.
  - Reviewer: ticked `ISSUE.md` lines 214 and 215 (reviewer validation and recorded independent
    reports). No other checkbox was changed.
  - Caller: ask Committer to include this report and the two checkbox updates in a focused
    documentation commit. Committer must verify and tick line 216.
