---
semantic-links:
  related-artifacts:
    - .github/agents/task-reviewer.agent.md
    - docs/issues/closed/2413-2410-si-22-1-document-event-flows-and-draft-adr/ISSUE.md
---

# Agent Review Reports - Document Event Flows and Draft the Shutdown-Order ADR

> Append one completed independent-review entry at a time. Do not modify, reorder, or remove
> earlier entries. A correction is a new entry that names the earlier conclusion.

## Reports

### 2026-10-19 17:12 UTC - Task Reviewer (after merge)

- Invocation scope: the sub-EPIC's Shared Delivery Gates require this independent report before
  push. It was skipped then, and this report was produced after merge, on 2026-10-19, against
  #2413's acceptance criteria (AC1 to AC3 and `linter all`), tasks T1 and T2a to T2c, and M1. PR
  #2553 merged into `develop` at 15:18 UTC as the merge commit
  `Merge torrust/torrust-tracker#2553: docs(shutdown): [#2413] document event flows and draft the shutdown-order ADR`,
  and the spec was archived by PR #2564. The review was read-only.
- Inputs:
  - the archived spec and its `manual-verification-evidence.md`;
  - the sub-EPIC `EPIC.md` (Glossary, "D Versus F", record entry 27, Implementation Steps, Commit
    Points, Shared Delivery Gates, progress log, and D1, D7, D8, D9, D11, D12, D13, D15),
    `adr-draft.md`, and the sub-EPIC evidence;
  - the deliverables `docs/features/shutdown-process/task-inventory.md`,
    `docs/architecture/glossary.md`, `docs/architecture/application-jobs.md`, and the
    cancellation-tree ADR;
  - the PR diff against the merge's first parent (20 files, 54 commits);
  - `docs/pr-reviews/pr-2553-review/PR-REVIEW.md` and `review-retrospective.md`;
  - the `review-task` Test Design checklist and the `create-adr` skill.
- Evidence (code facts read from source, not run; only `linter all`, one targeted Clippy run, and
  one test binary were executed):
  - PR shape: 20 files and 54 commits, all signed; T1, T2a (move and link updates only), T2b, and
    T2c are separate commits. The ADR draft commit is at 08:26:54 UTC, consistent with the
    progress-log record of the maintainer's approval at 08:27 UTC; the approval itself was checked
    only against the record.
  - AC1, producers: `rg` for `.send(`, `stats_event_sender|event_sender|.sender()`, and
    `publish_event_if_sender_available` across `packages/` and `src/`. The publishing files match
    the inventory, including the four UDP receive-loop publish sites in `launcher.rs`; the other
    hits are tests, unrelated channels, doc comments, or sender pass-through. Torrent cleanup
    publishes indirectly through `remove_inactive_peers` and `remove_peerless_torrents`. No
    producer sends from a detached task: the request processors go into a `JoinSet` that
    `drain_request_processors_on_shutdown` drains within 5 seconds.
  - AC1, consumers: production receivers are created only in the six `src/bootstrap/jobs/`
    files (two in `tracker_core.rs` and two in `udp_tracker_server.rs`); every other hit is under
    `#[cfg(test)]`. The listeners' conditions and the start order in `start_jobs_with_manager`
    match the inventory. The HTTP test environment subscribes one receiver and the UDP one three,
    both before start, both hold a `DropGuard`, and neither runs a swarm-bus listener.
  - Entry 27 and counts: `main` keeps `_app_container` and discards the result of
    `wait_for_all(Duration::from_secs(10))`; `CHANNEL_CAPACITY = 65536`; one production `Receiver`
    implementation and six `ScriptedReceiver` implementations; seven `it_should_prioritize_*`
    tests in six files; 13 `job_manager.spawn` sites in `src/app.rs` (a plain `spawn` search finds
    14 lines, the extra one under `cfg(test)`), 14 token sites and 4 `child_token` calls; tokio
    1.53.3 in `Cargo.lock`; budgets of 90 s (HTTP drain), 5 s (UDP request drain), and 10 s
    (shared deadline) as the glossary and ADR draft state. Entry 27's text does not mention torrent
    cleanup, the REST API, or `udp_ban_cleanup`, checked with `git show` before the PR, at its
    first parent, and at its merge.
  - AC2: the jobs doc and glossary are at their new paths, the old "Terms" section links to the
    glossary, the sub-EPIC links to the glossary, and `rg 'application-jobs\.md'` finds the old
    path only in the named historical records and in plain-text prose describing the move. Every
    live link uses the new path.
  - AC3: `adr-draft.md` follows the `create-adr` section order (with a Scope section), covers D8,
    D7, D1, D9, and D13 by content, restates the old ADR's Agreement points 1 and 3 to 6, its
    Consequences, and its rejected alternatives, replaces point 2, states full supersession (D11),
    and leaves the move to sub-issue 4 (D15).
  - `linter all` exited 0 on a clean export of the archive merge
    (`Merge torrust/torrust-tracker#2564: chore(issues): archive closed issue #2413 spec to docs/issues/closed`),
    not on the PR head. Because the Clippy step was probably served from cache,
    `cargo clippy -q -p clippy-allow-reasons --all-targets --all-features -- -D warnings` was run
    after touching its sources (exit 0), and `cargo test -q -p clippy-allow-reasons --test cli`
    gave `ok. 10 passed` with the export repository unchanged afterwards.
  - Spec bookkeeping: frontmatter, task and M1 statuses, ticked criteria, and the Acceptance
    Verification table are consistent; the two delivery-gate checkpoints were unchecked with the
    reason logged; `PR-REVIEW.md` records F1 to F32, all fixed and resolved. No prose-first
    Arrange-Act-Assert comparison is recorded for the changed tests.
- Findings:
  - F1 (should-fix): the "Recheck of record entry 27" table in `task-inventory.md` credits entry
    27 with "`udp_ban_cleanup` has no events" and "Torrent cleanup and the REST API publish no
    event of their own", and reports the second as "Differs". Entry 27 states neither; "D Versus
    F" already listed torrent cleanup as a swarm producer, so the reported difference does not
    exist although the code fact is right. The sub-EPIC progress log of 2026-10-16 07:56 repeats
    it. No maintainer acknowledgement of the real differences (`events.md` omits the swarm bus;
    tokio is 1.53.3) is recorded.
  - F2 (should-fix): no `implementation-retrospective.md` and no logged reason why none is needed,
    although the work produced reusable lessons (a `.send(` search misses publish sites behind a
    helper; Git's hook environment leaked into fixture Git commands).
  - F3 (should-fix): no prose-first Arrange-Act-Assert comparison is recorded for
    `contrib/dev-tools/checks/clippy-allow-reasons/tests/cli.rs`. In
    `it_should_not_let_fixture_commands_inherit_the_repository_environment` the expected list is the
    same constant the helper loops over; `it_should_list_every_variable_git_reports_as_repository_local`
    compensates by pinning that constant to `git rev-parse --local-env-vars`, with a documented
    subset check for the machine's Git. Name, visible Act, and visible assertions are fine.
  - F4 (should-fix): the spec puts any code change out of scope, but the PR also carried the
    `clippy-allow-reasons` test-isolation fix (with no tracking issue) and edits to the
    `create-issue` and `process-pr-review` skills and four templates. All are disclosed in the
    progress logs, and the template and skill edits were requested by the maintainer.
  - F5 (nit): the #2413 and sub-EPIC progress logs stop at the F28 rename; the later review
    rounds (F29 to F32) are only in `PR-REVIEW.md`.
  - F6 (nit): the Verification Plan says only the named historical records keep the old jobs-doc
    path; plain-text prose describing the move also contains it. Not a link, so acceptable.
  - Process: the Task Reviewer gate was skipped before push; this report closes it after merge.
- Verdict: POST_MERGE_FINDINGS
- Follow-up actions:
  - Maintainer decisions of 2026-10-19, recorded by the caller (GitHub Copilot, Claude Opus 5.5):
    - F1: fix in the prep pull request, in its own commit, with dated corrections in the logs.
    - F2: write `implementation-retrospective.md` in this folder, covering what the PR review
      retrospective does not.
    - F3: record the gap in the retrospective; no retroactive Arrange-Act-Assert review.
    - F4: the maintainer accepts the extra changes; no tracking issue.
    - F5: add a #2413 progress-log entry for the later rounds. The report calls them "rounds 8-9";
      the audit numbers them 9 (F29 to F31) and 10 (F32), and round 8 (F28) is already logged.
    - F6: leave as is.
  - A Task Reviewer re-review of these fixes is appended before the prep pull request opens.
