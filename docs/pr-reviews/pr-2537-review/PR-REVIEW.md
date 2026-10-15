---
semantic-links:
  skill-links:
    - process-pr-review
  related-artifacts:
    - .github/skills/dev/pr-reviews/process-pr-review/SKILL.md
    - docs/issues/open/2536-1669-hide-statistics-repository-read-lock/ISSUE.md
---

<!-- skill-link: process-pr-review -->

<!-- cspell:ignore unreassigned -->

# PR #2537 Review Audit

Source: pull-request reviews and inline review threads for <https://github.com/torrust/torrust-tracker/pull/2537>.

## Ownership

The PR author owns this tracked audit record. Reviewers, including repository review agents,
deliver findings through GitHub and have no repository-artifact obligation.

- Post-merge workflow approval: <https://github.com/torrust/torrust-tracker/pull/2537#issuecomment-6186327939>

## Status Values

- Relationship: `ORIGINAL`, `RE_RAISE_OF:<FindingId>`
- Disposition: `FIXED`, `NO_ACTION`, `SUPERSEDED`, `FOLLOW_UP`
- Thread state: `OPEN`, `RESOLVED`, `NON_RESOLVABLE`, `SUPERSEDED`. `OPEN` applies prospectively
  to audits created or updated for approved follow-up work; historical audits remain valid without
  bulk migration.
- Severity: `Blocker`, `Major`, `Minor`, `Nit`, `Suggestion`; append `(inferred)` when derived
  from free prose.
- Author class: `Copilot`, `Human`, `Unknown`
- Category: `link-integrity`, `formatting`, `metadata`, `testing`, `correctness`,
  `documentation`, `maintainability`, `security`, `other`
- An outdated thread whose concern was fixed is `FIXED`/`RESOLVED`, even when GitHub marks the
  original thread outdated after the push. For in-PR feedback, use `NO_ACTION`/`SUPERSEDED` only
  for a duplicate, superseded, or no-change concern. A post-merge `NO_ACTION` requires maintainer
  approval to decline the follow-up work.

## Findings

Cited heads are named by their last commit subject, not by commit id, so the record stays true
after a rebase:

- spec head: "docs(issues): [#2536] add specification for hiding the statistics read lock", which
  Copilot's review and da2ce7's first review read;
- audit head: "docs(pr-reviews): add the PR #2537 review audit", which da2ce7's second review read;
- Copilot-fix head: "docs(issues): [#2536] keep the debug-log example's output unchanged";
- round-1-fix head: "docs(issues): [#2536] cover the rustdoc links to the removed accessors";
- round-2-fix head: "docs(issues): [#2536] hold T7's commit until the maintainer review";
- second audit head: "docs(pr-reviews): record the second da2ce7 review in the PR #2537 audit",
  which da2ce7's third review read;
- round-3-fix head: "docs(issues): [#2536] normalize the activity job's gauges in M1";
- rebase head: "docs(pr-reviews): record the rebase onto develop in the PR #2537 audit", the first
  head after the rebase onto `develop`, which da2ce7's fourth review read and which merged.

Copilot review 5521920535 (COMMENTED, submitted 2026-10-14 17:25:40 UTC, at the spec head)
carries three inline findings with `[<Severity>][<FindingId>]` headers: F1 and F2 (`Major`) and F3
(`Minor`). Its overview rates them medium, low, and low; the rows use the inline headers. The
overview's summary and change table contain no further request, so they add no row.

da2ce7's review 5522278878 (submitted 2026-10-14 17:55:13 UTC, at the spec head; GitHub later
marked it DISMISSED after the fix pushes) carries five inline findings with
`[<Severity>][<FindingId>]` headers: F4-F7 (`Minor`) and F8 (`Nit`). Its IDs do not collide with
Copilot's F1-F3, so the rows keep them, and each detail's `Reviewer finding ID` is `N/A` because
no ID was reassigned. The body upholds Copilot's F1 and F2 at Minor and F3 as
raised. Those are the same concerns, already fixed and replied to on their own threads, so they
add no row, and the F1 and F2 rows keep Copilot's own `Major` header. The body also notes that
the #2521 quick-list item and EPIC row still read TODO. That is outside this diff and is handled by
PR #2532, so it adds no row.

da2ce7's second review 5523889096 (submitted 2026-10-14 20:01:19 UTC, at the audit head; also
later DISMISSED) carries four inline `Nit` findings, F9-F12. Its body says round 1's F4-F8 still
stood at that head, which predates their fixes, and raises nothing else, so it adds no further row.

da2ce7's third review 5524288522 (APPROVED, submitted 2026-10-14 20:33:17 UTC, at the second audit
head) carries four inline findings: F13, F14, and F16 (`Nit`) and F15 (`Minor`). Its body finds
F4-F12 fixed and raises nothing else. Fixing F13-F16 meant new pushes after the approval, which the
maintainer chose knowing the approval would likely be dismissed. The second review's F12 comment
also noted that the `open-pull-request` check cannot catch such ids. That skill gap was fixed in
this PR (see F12), as `AGENTS.md` allows for small process fixes, so it adds no row.

da2ce7's fourth review 5525009643 (APPROVED, submitted 2026-10-14 21:38:28 UTC, at the rebase
head) carries three inline `Nit` findings, F17-F19, and raises nothing else in its body. The PR
merged at the rebase head at 2026-10-15 06:44:22 UTC while they were still unprocessed. Their
fixes were pushed to this PR's branch and replied to after the merge, so none reached `develop`.
Under the post-merge approval above, follow-up PR #2539 carried them. It merged at 2026-10-15
07:21:44 UTC as "Merge torrust/torrust-tracker#2539: chore(pr-reviews): land the PR #2537 review
fixes that missed its merge", so the three rows are now `FIXED`/`RESOLVED`.

| Finding ID | Review finding reference | Author class | Severity | Category | Relationship | Disposition | Thread state |
| ---------- | ------------------------ | ------------ | -------- | -------- | ------------ | ----------- | ------------ |
| F1 | `review-finding:pr-2537-f1` | Copilot | Major | correctness | ORIGINAL | FIXED | RESOLVED |
| F2 | `review-finding:pr-2537-f2` | Copilot | Major | testing | ORIGINAL | FIXED | RESOLVED |
| F3 | `review-finding:pr-2537-f3` | Copilot | Minor | correctness | ORIGINAL | FIXED | RESOLVED |
| F4 | `review-finding:pr-2537-f4` | Human | Minor | documentation | ORIGINAL | FIXED | RESOLVED |
| F5 | `review-finding:pr-2537-f5` | Human | Minor | testing | ORIGINAL | FIXED | RESOLVED |
| F6 | `review-finding:pr-2537-f6` | Human | Minor | documentation | ORIGINAL | FIXED | RESOLVED |
| F7 | `review-finding:pr-2537-f7` | Human | Minor | testing | ORIGINAL | FIXED | RESOLVED |
| F8 | `review-finding:pr-2537-f8` | Human | Nit | link-integrity | ORIGINAL | FIXED | RESOLVED |
| F9 | `review-finding:pr-2537-f9` | Human | Nit | testing | ORIGINAL | FIXED | RESOLVED |
| F10 | `review-finding:pr-2537-f10` | Human | Nit | testing | ORIGINAL | FIXED | RESOLVED |
| F11 | `review-finding:pr-2537-f11` | Human | Nit | documentation | ORIGINAL | FIXED | RESOLVED |
| F12 | `review-finding:pr-2537-f12` | Human | Nit | documentation | ORIGINAL | FIXED | RESOLVED |
| F13 | `review-finding:pr-2537-f13` | Human | Nit | correctness | ORIGINAL | FIXED | RESOLVED |
| F14 | `review-finding:pr-2537-f14` | Human | Nit | testing | ORIGINAL | FIXED | RESOLVED |
| F15 | `review-finding:pr-2537-f15` | Human | Minor | documentation | ORIGINAL | FIXED | RESOLVED |
| F16 | `review-finding:pr-2537-f16` | Human | Nit | documentation | ORIGINAL | FIXED | RESOLVED |
| F17 | `review-finding:pr-2537-f17` | Human | Nit | documentation | ORIGINAL | FIXED | RESOLVED |
| F18 | `review-finding:pr-2537-f18` | Human | Nit | documentation | ORIGINAL | FIXED | RESOLVED |
| F19 | `review-finding:pr-2537-f19` | Human | Nit | maintainability | ORIGINAL | FIXED | RESOLVED |

## Finding Details

### F1 - A synchronous closure does not prevent blocking

- PR number: 2537
- Source review ID: 5521920535
- Reviewer finding ID: N/A
- Source URL: <https://github.com/torrust/torrust-tracker/pull/2537#discussion_r4274060593>
- Concern: the Design and Ownership Review said a synchronous `FnOnce(&Metrics) -> R` prevents the
  closure from blocking. It prevents only `.await`; blocking I/O, `thread::sleep`, or long CPU work
  can still stall writers while the read lock is held.
- Solution: the section now separates what the API enforces (no `.await`) from the caller
  obligation (no blocking, no nested `read_metrics()` on the same repository). The proposed
  `read_metrics()` rustdoc states the obligation, and the design rationale points to it.
- Current-tree verification: at the Copilot-fix head, `ISSUE.md`:180-189 lists the enforced rule
  and the caller obligation, :94-97 is the proposed rustdoc, and :123-125 is the updated rationale.
  These ranges were corrected under F11; the F1 reply cites the earlier ranges, and the F11 reply
  gives the corrected ones.
- Resolution reference: docs(issues): [#2536] record non-blocking read closures as a caller obligation
- Follow-up PR URL: N/A
- Reply URL: <https://github.com/torrust/torrust-tracker/pull/2537#discussion_r4275421008>

### F2 - Required test-design and maintainer checkpoints are missing

- PR number: 2537
- Source review ID: 5521920535
- Reviewer finding ID: N/A
- Source URL: <https://github.com/torrust/torrust-tracker/pull/2537#discussion_r4274060692>
- Concern: T3-T7 add doctests, but the plan omitted the `create-issue` skill's per-increment
  test-design review with the prose-first Arrange-Act-Assert comparison and the maintainer stop
  after the final increment.
- Solution: a Test-Development Workflow subsection applies the `write-unit-test` skill to each of
  T3-T7: red/green for each `compile_fail` doctest, then a recorded design review and prose-first
  comparison, finished before the next task. For a `compile_fail` doctest the comparison is recorded
  in compile-and-error terms rather than skipped. A maintainer stop follows T7, T8's notes repeat
  the gate, and two workflow checkpoints track both.
- Current-tree verification: at the Copilot-fix head, `ISSUE.md`:227-247 holds the subsection, the T8
  row names the gate, and :257-258 are the new checkpoints.
- Resolution reference: docs(issues): [#2536] require the per-increment test-design review and the maintainer stop
- Follow-up PR URL: N/A
- Reply URL: <https://github.com/torrust/torrust-tracker/pull/2537#discussion_r4275421321>

### F3 - The debug-log example changes the log text

- PR number: 2537
- Source review ID: 5521920535
- Reviewer finding ID: N/A
- Source URL: <https://github.com/torrust/torrust-tracker/pull/2537#discussion_r4274060789>
- Concern: formatting the `String` returned by the closure with `{:?}` adds quotes and escapes, so
  the example would change `stats: Metrics { .. }` into a quoted string.
- Solution: the example formats the returned `String` with `{}`, and the risk explains why.
- Current-tree verification: at the Copilot-fix head, `ISSUE.md`:361 uses `"stats: {}"`. In tokio
  1.53.3, the read guard's `impl fmt::Debug` (in its `read_guard.rs`) delegates to the inner value,
  so today's log already prints the `Metrics` debug text that the fixed example keeps.
- Resolution reference: docs(issues): [#2536] keep the debug-log example's output unchanged
- Follow-up PR URL: N/A
- Reply URL: <https://github.com/torrust/torrust-tracker/pull/2537#discussion_r4275421568>

### F4 - Current callers omits the in-crate unit tests

- PR number: 2537
- Source review ID: 5522278878
- Reviewer finding ID: N/A
- Source URL: <https://github.com/torrust/torrust-tracker/pull/2537#discussion_r4274350543>
- Concern: Current callers listed the adapter, the event handlers, and the integration tests, but
  not the unit tests inside the five crates, which are the largest group of calls (57 by the
  reviewer's count).
- Solution: Current callers lists the in-crate unit-test calls per crate (58 by our count: udp-server
  31, udp-core 12, http-core 9, swarm-coordination-registry 4, tracker-core 2). It also names the
  helpers and the guard-accessor test that need their own shape. In Scope covers every test caller,
  and T4 records the udp-server size.
- Current-tree verification: at the round-1-fix head, `ISSUE.md`:83-95 holds the count and the named
  callers, :168-170 the In Scope bullet, and :230 the T4 row.
- Resolution reference: docs(issues): [#2536] list the in-crate unit-test callers
- Follow-up PR URL: N/A
- Reply URL: <https://github.com/torrust/torrust-tracker/pull/2537#discussion_r4275660364>

### F5 - M1 names neither its stored files nor its full normalization

- PR number: 2537
- Source review ID: 5522278878
- Reviewer finding ID: N/A
- Source URL: <https://github.com/torrust/torrust-tracker/pull/2537#discussion_r4274350550>
- Concern: M1 compared before and after REST outputs without naming the files to store or the full
  mask for values that differ between runs, so its pass result could not be reproduced.
- Solution: M1 names the five outputs stored per run (ten in total) and the normalization (sorted
  samples, `recorded_at` ignored, processing-time values masked). Its expected result is
  byte-identical normalized outputs, and T2 and T8 each store their five outputs.
- Current-tree verification: at the round-1-fix head, `ISSUE.md`:392 is the M1 row, :399-411 the
  stored outputs and the normalization, and :228 and :234 the T2 and T8 rows.
- Resolution reference: docs(issues): [#2536] name the stored M1 outputs and their normalization
- Follow-up PR URL: N/A
- Reply URL: <https://github.com/torrust/torrust-tracker/pull/2537#discussion_r4275660710>

### F6 - The overlap risk names no sibling spec

- PR number: 2537
- Source review ID: 5522278878
- Reviewer finding ID: N/A
- Source URL: <https://github.com/torrust/torrust-tracker/pull/2537#discussion_r4274350554>
- Concern: Risks warned about specs that touch the removed accessors without naming any, so their
  authors could not be told.
- Solution: the risk names the three open sibling specs outside #1669 that touch the accessors:
  #2414, #1348, and #2518.
- Current-tree verification: at the round-1-fix head, `ISSUE.md`:446-457 names the three specs and
  their paths.
- Resolution reference: docs(issues): [#2536] name the sibling specs that overlap the accessor removal
- Follow-up PR URL: N/A
- Reply URL: <https://github.com/torrust/torrust-tracker/pull/2537#discussion_r4275661115>

### F7 - The AC1 call grep does not give a pass or fail result

- PR number: 2537
- Source review ID: 5522278878
- Reviewer finding ID: N/A
- Source URL: <https://github.com/torrust/torrust-tracker/pull/2537#discussion_r4274350564>
- Concern: the grep had no path operand, so it searched `docs/` and `target/` and matched calls
  that are not on a statistics repository. Its result could only be judged by sorting the output
  by hand.
- Solution: the grep runs over Rust sources in `packages`, `src`, `tests`, `console`, and
  `contrib`, excluding `torrent-repository-benchmarking`. It must print exactly the two REST-port
  lines, which the spec lists.
- Current-tree verification: at the round-1-fix head, `ISSUE.md`:354-372 holds the command, the
  expected output, and the scope note. Run at that head, the grep prints 84 lines. The only ones
  that are not statistics-repository calls are the two listed REST-port lines.
- Resolution reference: docs(issues): [#2536] make the AC1 call grep give an exact result
- Follow-up PR URL: N/A
- Reply URL: <https://github.com/torrust/torrust-tracker/pull/2537#discussion_r4275661470>

### F8 - The doctest update leaves out the rustdoc link to the removed accessor

- PR number: 2537
- Source review ID: 5522278878
- Reviewer finding ID: N/A
- Source URL: <https://github.com/torrust/torrust-tracker/pull/2537#discussion_r4274350571>
- Concern: each repository's rustdoc lead-in links to `Repository::get_stats` or
  `Repository::get_metrics`. With warnings denied, that link breaks the library docs step and the
  pre-push nightly doc build once the accessor is removed. The plan did not cover it.
- Solution: Current callers names the five lead-ins, and the In Scope doctests bullet, T3, and AC3
  repoint them to `Repository::read_metrics`. An AC3 check greps for the old link and runs the
  nightly library docs build.
- Current-tree verification: at the round-1-fix head, `ISSUE.md`:96-101 names the lead-ins, :177-178
  is the In Scope sub-bullet, :229 the T3 row, :330-332 AC3, and :375-381 the AC3 check. Run at
  that head, the check's grep prints the five lead-ins.
- Resolution reference: docs(issues): [#2536] cover the rustdoc links to the removed accessors
- Follow-up PR URL: N/A
- Reply URL: <https://github.com/torrust/torrust-tracker/pull/2537#discussion_r4275661770>

### F9 - The test-design review leaves out the helper-boundary check

- PR number: 2537
- Source review ID: 5523889096
- Reviewer finding ID: N/A
- Source URL: <https://github.com/torrust/torrust-tracker/pull/2537#discussion_r4275626312>
- Concern: `create-issue` and the issue template require the test-design review to judge helper
  boundaries by meaningful named actions and abstraction-level alignment, not caller count. The
  review bullets stopped before that rule, although this migration reshapes guard-wrapping helpers.
- Solution: a fourth review bullet states the rule and applies it to `udp-server`'s
  `metrics_snapshot` and the swarm job test's `inactive_peers_total`.
- Current-tree verification: at the round-2-fix head, `ISSUE.md`:259-263 is the new bullet.
- Resolution reference: docs(issues): [#2536] add the helper-boundary check to the test-design review
- Follow-up PR URL: N/A
- Reply URL: <https://github.com/torrust/torrust-tracker/pull/2537#discussion_r4275779046>

### F10 - T7's own commit lands before the maintainer stop

- PR number: 2537
- Source review ID: 5523889096
- Reviewer finding ID: N/A
- Source URL: <https://github.com/torrust/torrust-tracker/pull/2537#discussion_r4275626334>
- Concern: `create-issue` puts the maintainer stop before the final increment's commit, but the
  plan committed every package, T7 included, right after its test-design review.
- Solution: T3-T6 still commit after their own review; T7's commit waits for the maintainer
  review. Step 4, the stop, the T3-T7 commit policy, and the workflow checkpoint say so.
- Current-tree verification: at the round-2-fix head, `ISSUE.md`:240 is the commit policy, :268-273
  are step 4 and the stop, and :284 is the checkpoint.
- Resolution reference: docs(issues): [#2536] hold T7's commit until the maintainer review
- Follow-up PR URL: N/A
- Reply URL: <https://github.com/torrust/torrust-tracker/pull/2537#discussion_r4275779388>

### F11 - The F1 current-tree sentence cites the wrong rationale lines

- PR number: 2537
- Source review ID: 5523889096
- Reviewer finding ID: N/A
- Source URL: <https://github.com/torrust/torrust-tracker/pull/2537#discussion_r4275626352>
- Concern: the F1 sentence cited :120-122, which hold the "Consistent reads" and "No copies"
  bullets, and :179-188, which is off by one; the F1 reply repeated both.
- Solution: the F1 sentence cites :180-189 and :123-125 and notes that the F1 reply carries the
  earlier ranges. The F11 reply states the corrected ranges, so the thread record is complete.
- Current-tree verification: in this record, the F1 Current-tree verification entry cites the
  corrected ranges. At the Copilot-fix head, `ISSUE.md`:180-189 is the Design and Ownership text
  and :123-125 the "No lock held across an `.await`" rationale.
- Resolution reference: docs(pr-reviews): correct the F1 line ranges in the PR #2537 audit
- Follow-up PR URL: N/A
- Reply URL: <https://github.com/torrust/torrust-tracker/pull/2537#discussion_r4275779665>

### F12 - The record cites PR-branch commits by id

- PR number: 2537
- Source review ID: 5523889096
- Reviewer finding ID: N/A
- Source URL: <https://github.com/torrust/torrust-tracker/pull/2537#discussion_r4275626368>
- Concern: the record cited PR-branch commits by id, which a rebase rewrites, against
  `open-pull-request`'s rule for durable references.
- Solution: the Findings section opens with a legend that names each cited head by its last commit
  subject, and every current-tree and log sentence uses those names, including the F4-F8 rows added
  before this review arrived.
- Current-tree verification: in this record, no PR-branch commit id remains. The only commit id is
  the `develop` commit `48c4d3013`, cited in this sentence; it was the merge base until the branch
  was rebased onto the PR #2532 merge. The legend opens the Findings section.
- Resolution reference: docs(pr-reviews): name the PR #2537 audit's cited heads by commit subject
- Follow-up PR URL: N/A
- Reply URL: <https://github.com/torrust/torrust-tracker/pull/2537#discussion_r4275779954>

The comment also noted that the `open-pull-request` check scans only the issue folder and drops
all-digit ids. "docs(skills): check every changed file for PR-branch commit ids" fixes that in
this PR; a follow-up reply on this thread describes it.

### F13 - The 58 counts one companion doctest as an in-crate unit test

- PR number: 2537
- Source review ID: 5524288522
- Reviewer finding ID: N/A
- Source URL: <https://github.com/torrust/torrust-tracker/pull/2537#discussion_r4275956597>
- Concern: counting calls after each file's first `#[cfg(test)]` also counted the `udp-server`
  companion doctest call at `statistics/repository.rs`:35, which compiles as its own crate. The
  in-crate unit tests number 57, with `udp-server` at 30.
- Solution: Current callers gives 57 (udp-server 30) and explains the 58; T4 says 30.
- Current-tree verification: at the round-3-fix head, `ISSUE.md`:83-88 gives the count and the
  explanation, and :232 is the T4 row. Counting only after each file's `mod tests` at `48c4d3013`
  gives udp-server 30, udp-core 12, http-core 9, swarm-coordination-registry 4, tracker-core 2.
- Resolution reference: docs(issues): [#2536] correct the in-crate unit-test count to 57
- Follow-up PR URL: N/A
- Reply URL: <https://github.com/torrust/torrust-tracker/pull/2537#discussion_r4276361015>

### F14 - The normalization does not cover the activity job's two gauges

- PR number: 2537
- Source review ID: 5524288522
- Reviewer finding ID: N/A
- Source URL: <https://github.com/torrust/torrust-tracker/pull/2537#discussion_r4275956612>
- Concern: only the activity job writes the two inactive gauges. It skips the first tick and runs
  every 15 s, so a reading's samples depend on the time since startup, which could break
  "byte-identical" without a defect.
- Solution: M1's normalization replaces both gauges' lines with `name <activity-job>` and says
  why. The REST stats output does not include them.
- Current-tree verification: at the round-3-fix head, `ISSUE.md`:435-440 is the new rule.
  `activity_metrics_updater.rs` in `swarm-coordination-registry` sets the 15 s interval and awaits
  the first tick before its loop.
- Resolution reference: docs(issues): [#2536] normalize the activity job's gauges in M1
- Follow-up PR URL: N/A
- Reply URL: <https://github.com/torrust/torrust-tracker/pull/2537#discussion_r4276361275>

### F15 - Two Processing Log stamps are later than the commits that carry them

- PR number: 2537
- Source review ID: 5524288522
- Reviewer finding ID: N/A
- Source URL: <https://github.com/torrust/torrust-tracker/pull/2537#discussion_r4275956627>
- Concern: the 20:10 and 20:25 UTC entries were stamped after the commits that carry them, so
  neither stamp is the time of an event or of the writing.
- Solution: the log stays append-only; a correction entry gives the actual reply and commit times
  and cites both commits by subject.
- Current-tree verification: in this record, the 21:14 UTC Processing Log entry is the correction.
  Its times match the GitHub reply timestamps and the two commits' committer dates.
- Resolution reference: docs(pr-reviews): correct two Processing Log stamps in the PR #2537 audit
- Follow-up PR URL: N/A
- Reply URL: <https://github.com/torrust/torrust-tracker/pull/2537#discussion_r4276361547>

### F16 - `Reviewer finding ID` repeats IDs that were not reassigned

- PR number: 2537
- Source review ID: 5524288522
- Reviewer finding ID: N/A
- Source URL: <https://github.com/torrust/torrust-tracker/pull/2537#discussion_r4275956637>
- Concern: `process-pr-review` records a reviewer's ID only when it was reassigned, otherwise
  `N/A`; F4-F12 repeated their own IDs.
- Solution: F4-F12 record `N/A`, and the Findings prose says no ID was reassigned.
- Current-tree verification: in this record, every detail entry's `Reviewer finding ID` is `N/A`.
- Resolution reference: docs(pr-reviews): record unreassigned finding IDs as N/A in the PR #2537 audit
- Follow-up PR URL: N/A
- Reply URL: <https://github.com/torrust/torrust-tracker/pull/2537#discussion_r4276361850>

### F17 - The PR body does not mention the skill change

- PR number: 2537
- Source review ID: 5525009643
- Reviewer finding ID: N/A
- Source URL: <https://github.com/torrust/torrust-tracker/pull/2537#discussion_r4276548236>
- Concern: the merge tool copies the PR body into the merge commit, and the body's Changes list
  named only the spec and the EPIC rows, so `develop` would record the skill change without saying
  so.
- Solution: the body edit on this PR came after the merge commit was made, so the merge commit
  keeps the original body. The follow-up PR's body describes both `open-pull-request` skill
  changes, and its merge commit carries that description into `develop`.
- Current-tree verification: on `develop`, the PR #2539 merge commit's body names
  "docs(skills): check every changed file for PR-branch commit ids" and the F18 change. The
  earlier `FOLLOW_UP` reply is
  <https://github.com/torrust/torrust-tracker/pull/2537#discussion_r4279987154>.
- Resolution reference: <https://github.com/torrust/torrust-tracker/pull/2537#discussion_r4280210547>
- Follow-up PR URL: <https://github.com/torrust/torrust-tracker/pull/2539>
- Reply URL: <https://github.com/torrust/torrust-tracker/pull/2537#discussion_r4280210547>

### F18 - The check still runs only after a rebase

- PR number: 2537
- Source review ID: 5525009643
- Reviewer finding ID: N/A
- Source URL: <https://github.com/torrust/torrust-tracker/pull/2537#discussion_r4276548245>
- Concern: the rule bars PR-branch ids at any time, and the ids this PR's audit cited were written
  in a normal review round, but the skill ran the check only after a rebase.
- Solution: the follow-up PR lands "docs(skills): run the PR-branch commit-id check before every
  documentation push". The skill then says the rule holds whether or not the branch is rebased,
  and runs the check before every push that changes an issue spec, evidence file, or review
  audit, and after a rebase.
- Current-tree verification: on `develop`, `open-pull-request` `SKILL.md`:78-81 holds the new
  wording. The earlier `FOLLOW_UP` reply is
  <https://github.com/torrust/torrust-tracker/pull/2537#discussion_r4279987332>.
- Resolution reference: docs(skills): run the PR-branch commit-id check before every documentation push
- Follow-up PR URL: <https://github.com/torrust/torrust-tracker/pull/2539>
- Reply URL: <https://github.com/torrust/torrust-tracker/pull/2537#discussion_r4280210834>

### F19 - An ordinary English word was added to the dictionary

- PR number: 2537
- Source review ID: 5525009643
- Reviewer finding ID: N/A
- Source URL: <https://github.com/torrust/torrust-tracker/pull/2537#discussion_r4276548252>
- Concern: `project-words.txt` holds technical terms, and the PR #2423 review already ruled
  `unreassigned` out of it in favor of a file-local cspell directive.
- Solution: the follow-up PR lands "docs(pr-reviews): ignore unreassigned in the PR #2537 audit,
  not the dictionary". It removes the word from `project-words.txt`, and this record carries
  `<!-- cspell:ignore unreassigned -->`, as the PR #2344 and PR #2423 audits do.
- Current-tree verification: on `develop`, `project-words.txt` has no `unreassigned` line, and
  this record's line 12 is the directive. The earlier `FOLLOW_UP` reply is
  <https://github.com/torrust/torrust-tracker/pull/2537#discussion_r4279987545>.
- Resolution reference: docs(pr-reviews): ignore unreassigned in the PR #2537 audit, not the dictionary
- Follow-up PR URL: <https://github.com/torrust/torrust-tracker/pull/2539>
- Reply URL: <https://github.com/torrust/torrust-tracker/pull/2537#discussion_r4280211116>

## Processing Log

- 2026-10-14 17:30 UTC - Started audit: read Copilot review 5521920535 (three inline findings) and
  fixed F1. Its signed commit first failed with a GPG passphrase timeout; at the maintainer's
  choice the same signed commit was retried and succeeded.
- 2026-10-14 19:40 UTC - Fixed F2 and F3 in separate commits, pushed through the pre-commit and
  pre-push hooks, re-verified the three claims at the Copilot-fix head, and replied on all three
  threads. The threads are resolved after this audit is pushed.
- 2026-10-14 20:10 UTC - Read da2ce7's review 5522278878: five inline findings (F4-F8), plus body
  notes that uphold Copilot's F1-F3 and point at #2521's TODO status (no new rows; see Findings).
  Fixed each finding in its own commit, pushed through the hooks, re-verified the claims at the
  round-1-fix head, and replied on all five threads. They are resolved after this update is pushed.
- 2026-10-14 20:25 UTC - After pushing the F4-F8 rows, a thread check found da2ce7's second review
  5523889096 (F9-F12, all `Nit`). Fixed each finding in its own commit, two in the spec and two in
  this record, and pushed. Replied on all four threads, then corrected the F11 reply, whose first
  version named the wrong commit subject. Resolved the five F4-F8 threads; the four F9-F12 threads
  are resolved after this update is pushed.
- 2026-10-14 21:14 UTC - Correction (F15): the stamps of the two entries above are later than the
  commits that carry them, and neither is the time of an event. The 20:10 UTC entry covers the push
  and the F4-F8 replies (20:04:31-20:04:39 UTC); its commit, "docs(pr-reviews): record the da2ce7
  review in the PR #2537 audit", is at 20:06:47 UTC. The 20:25 UTC entry covers the F9-F12 replies
  (20:15:45-20:15:51 UTC) and the F11 reply edit (20:15:58 UTC); its commit, "docs(pr-reviews):
  record the second da2ce7 review in the PR #2537 audit", is at 20:16:52 UTC. Later entries take
  their stamp from the clock when written.
- 2026-10-14 21:19 UTC - Read da2ce7's third review 5524288522 (APPROVED; F13-F16). At the
  maintainer's choice, fixed all four, each in its own commit (two in the spec, two in this
  record), and added the `open-pull-request` skill fix for the gap noted under F12. Pushed through
  the hooks, replied on the four threads and on the F12 thread, edited that reply to drop quoted
  branch-commit ids, and checked every reply and the PR body for such ids. The four threads are
  resolved after this update is pushed.
- 2026-10-14 21:24 UTC - Rebased the branch onto `develop` after PR #2532 merged (archive of the
  #2521 spec). The only conflict was in the #1669 EPIC: develop's #2521 lines (DONE, closed-spec
  paths) were kept, the #2536 entries were added after them, and the #2536 log entry follows
  develop's #2521 entry. No Rust source changed on `develop`, so the counts and grep results cited
  at `48c4d3013` still hold. Re-ran the validator and the PR-branch commit-id check after the
  rebase.
- 2026-10-15 06:58 UTC - The maintainer reported open threads. Fetching every thread with the
  `github-review-threads` tool found da2ce7's fourth review 5525009643 (APPROVED; F17-F19), which
  had arrived after the rebase. At the maintainer's choice, the F19 and F18 fixes were committed
  on this PR's branch at 06:41:13 and 06:42:03 UTC, and the PR body was edited. The PR then merged
  at the rebase head at 06:44:22 UTC, by accident, before the push. The push, the replies
  (06:44:50-06:44:53 UTC), the thread resolutions, and the audit rows recorded on the merged branch
  came after the merge and never reached `develop`, and the merge commit kept the original body. Under the
  process-pr-review post-merge rules, the maintainer approved a follow-up (retrospective record:
  <https://github.com/torrust/torrust-tracker/pull/2537#issuecomment-6186327939>). The three
  threads were reopened. Follow-up PR #2539, branched from `develop`, carries the two fix commits
  and this update. The rows stay `FOLLOW_UP`/`OPEN` until it merges; then this record is closed
  out.
- 2026-10-15 07:25 UTC - PR #2539 merged at 07:21:44 UTC. Re-verified F17-F19 on `develop`, replied
  `FIXED` with the merge reference on the three threads, and moved the rows to `FIXED`/`RESOLVED`.
  The threads are resolved after this update is pushed. The earlier `FOLLOW_UP` replies stay on the
  threads, and the detail entries name them.

## Completion Rules

- Re-derive the reply claim against the current tree before replying or resolving a thread.
- Reply on every resolvable thread before resolving it.
- For an outdated thread whose concern was fixed, record `Disposition=FIXED` and
  `Thread state=RESOLVED`, even if GitHub marks the thread outdated after the push. For a
  duplicate, superseded, or no-change in-PR thread, reply exactly
  `Superseded by <FindingId>: <reason>.`, record `Disposition=NO_ACTION` and
  `Thread state=SUPERSEDED`, then resolve it. A post-merge `NO_ACTION` requires maintainer
  approval to decline the follow-up work.
- A consolidated PR conversation response may cover multiple review rounds only when it names
  every review ID and every finding ID with its disposition and resolution reference. Record its
  durable URL in each related row.
- Cite a fix by its unique Conventional Commit subject or durable reply URL, never by a branch SHA
  that can change after a rebase.
- Refresh review threads using GraphQL and confirm that no unresolved actionable thread remains.
