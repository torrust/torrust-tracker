---
semantic-links:
  skill-links:
    - process-pr-review
  related-artifacts:
    - .github/skills/dev/pr-reviews/process-pr-review/SKILL.md
---

<!-- skill-link: process-pr-review -->

# PR #2561 Review Audit

Source: pull-request reviews and inline review threads for <https://github.com/torrust/torrust-tracker/pull/2561>.

## Ownership

The PR author owns this tracked audit record. Reviewers, including repository review agents,
deliver findings through GitHub and have no repository-artifact obligation.

- Post-merge workflow approval: N/A

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

Round 1: Copilot's review 5571188743 (COMMENTED, 2026-10-19 11:59:52 UTC, on the PR's first head,
whose newest commit is `docs(issues): [#2560] record the verification evidence and the completed
spec`) carries three inline findings, F1 to F3. Its body is an overview and adds no further row.
Copilot marked F1 with a medium and F2 and F3 with a low severity icon, so the three severities are
inferred, with medium as `Minor` and low as `Nit`.

Round 2: da2ce7's review 5571266146 (APPROVED, 2026-10-19 12:05:39 UTC, on the same head) carries
two inline findings. da2ce7 numbered them F1 and F2, which collide with the rows above, so they are
F4 and F5 here with the original number kept as the reviewer finding ID. The review body is a
verification report and adds no other row. It does not rate round 1's findings, so F1 to F3 keep
the bot's own icons. The approval was dismissed at 12:09:34 UTC by the PR author's push whose newest
commit is `docs(issues): [#2560] set the spec in review and rerun the evidence on the final code`,
which triggers the repository's stale-approval rule. Its ACK comment
(<https://github.com/torrust/torrust-tracker/pull/2561#issuecomment-6266790165>) names the first
head, so neither covers the commits that fix F1 to F5.

Round 3: da2ce7's review 5571606463 (APPROVED, 2026-10-19 12:35:28 UTC, on the head whose newest
commit is `docs(pr-reviews): [#2560] add the PR #2561 review audit`) carries one inline finding.
da2ce7 numbered it F1, which collides again, so it is F6 here with the original number kept as the
reviewer finding ID. The review body is a verification report that resolves round 1's and round 2's
findings and adds no other row. It agrees with all three of Copilot's items and, at its capture
time, reports no review from the bot after 11:59:52 UTC. The approval was dismissed at 12:39:10 UTC by the PR author's push
whose newest commit is `docs(statistics): [#2560] say that counter_total reads a gauge as zero`,
which is the commit that fixes F6. Its ACK comment
(<https://github.com/torrust/torrust-tracker/pull/2561#issuecomment-6267341946>) names the head
before that push, so neither covers it.

Round 3 also has Copilot's second review 5571626600 (COMMENTED, 2026-10-19 12:37:07 UTC, on the head
whose newest commit is `docs(pr-reviews): [#2560] add the PR #2561 review audit`), which landed after
da2ce7's capture. It reports no open inline issue and marks its three earlier items resolved. Its
body carries one independently actionable finding with no inline thread, F7, so F7 has the
submitted review's URL as its source and the thread state `NON_RESOLVABLE`. The bot marked it with a
low severity icon, so the severity is inferred.

| Finding ID | Review finding reference | Author class | Severity | Category | Relationship | Disposition | Thread state |
| ---------- | ------------------------ | ------------ | -------- | -------- | ------------ | ----------- | ------------ |
| F1 | `review-finding:pr-2561-f1` | Copilot | Minor (inferred) | maintainability | ORIGINAL | FIXED | RESOLVED |
| F2 | `review-finding:pr-2561-f2` | Copilot | Nit (inferred) | metadata | ORIGINAL | FIXED | RESOLVED |
| F3 | `review-finding:pr-2561-f3` | Copilot | Nit (inferred) | testing | ORIGINAL | FIXED | RESOLVED |
| F4 | `review-finding:pr-2561-f4` | Human | Nit | documentation | ORIGINAL | FIXED | RESOLVED |
| F5 | `review-finding:pr-2561-f5` | Human | Suggestion | documentation | ORIGINAL | FIXED | RESOLVED |
| F6 | `review-finding:pr-2561-f6` | Human | Nit | documentation | ORIGINAL | FIXED | RESOLVED |
| F7 | `review-finding:pr-2561-f7` | Copilot | Nit (inferred) | documentation | ORIGINAL | FIXED | NON_RESOLVABLE |

## Finding Details

### F1 - The `sum_counter(...).unwrap_or_default()` read is repeated across many accessors

- PR number: 2561
- Source review ID: 5571188743
- Reviewer finding ID: N/A
- Source URL: <https://github.com/torrust/torrust-tracker/pull/2561#discussion_r4313536415>
- Concern: every counter accessor repeated the same two calls, 17 times in the UDP server alone, so
  a later change to the defaulting or the telemetry of a counter read would touch every accessor. The
  reviewer suggested a small private helper dedicated to counters, which keeps the goal of the PR
  because it has no float cast.
- Solution: each of HTTP core, UDP core, and the UDP server now has one private `counter_total`
  helper whose body is the only `sum_counter` call in its file. The reviewer cited the UDP server,
  and the other two packages got the same helper so the three `Metrics` types keep the same shape.
  The helper has no cast and no `#[expect]`. Behavior is unchanged.
- Current-tree verification: `grep -c "\.sum_counter("` reads 1 in each of the three files, the UDP
  server calls `counter_total` 17 times, and the tests of the three packages and Clippy with
  `-D warnings` pass with the same counts as before.
- Resolution reference: refactor(statistics): [#2560] share the counter read behind a cast-free helper
- Follow-up PR URL: N/A
- Reply URL: <https://github.com/torrust/torrust-tracker/pull/2561#discussion_r4313672534>

### F2 - The spec's `status` is `in-progress` although its tasks and criteria are done

- PR number: 2561
- Source review ID: 5571188743
- Reviewer finding ID: N/A
- Source URL: <https://github.com/torrust/torrust-tracker/pull/2561#discussion_r4313536525>
- Concern: the frontmatter said `status: in-progress` while the document marked every task and
  acceptance criterion done, which makes the metadata inconsistent for a reader or a tool.
- Solution: a finished spec whose pull request is under review is `in-review` in the repository's
  lifecycle for `open/`. The status is now `in-review`, and `related-pr` is `2561`, which the review
  did not mention but which was also still `null`. It stays `in-review` until the issue closes and
  the spec is archived.
- Current-tree verification: the frontmatter of the spec reads `status: in-review` and
  `related-pr: 2561`.
- Resolution reference: docs(issues): [#2560] set the spec in review and rerun the evidence on the final code
- Follow-up PR URL: N/A
- Reply URL: <https://github.com/torrust/torrust-tracker/pull/2561#discussion_r4313672931>

### F3 - The exact-total test uses `CurrentClock::now()` for the counter timestamp

- PR number: 2561
- Source review ID: 5571188743
- Reviewer finding ID: N/A
- Source URL: <https://github.com/torrust/torrust-tracker/pull/2561#discussion_r4313536573>
- Concern: the UDP server test took its counter timestamp from the clock, which makes it less
  deterministic than necessary and could in principle make it flaky.
- Solution: the timestamp is not asserted, so it could not flake in practice, but a fixed value takes
  the clock out of the test for no cost. The test now passes `DurationSinceUnixEpoch::ZERO`, as the
  HTTP core and UDP core tests of the same behavior already did.
- Current-tree verification: the test passes `DurationSinceUnixEpoch::ZERO`, no `CurrentClock::now`
  remains inside it, and `cargo test -p torrust-tracker-udp-server --lib f64_cannot_represent` passes.
- Resolution reference: test(udp-server): [#2560] use a fixed timestamp for the exact-total test
- Follow-up PR URL: N/A
- Reply URL: <https://github.com/torrust/torrust-tracker/pull/2561#discussion_r4313673229>

### F4 - The Background still describes a helper and an "only" cast that the code does not have

- PR number: 2561
- Source review ID: 5571266146
- Reviewer finding ID: F1
- Source URL: <https://github.com/torrust/torrust-tracker/pull/2561#discussion_r4313591228>
- Concern: the Background said the decision was to keep the gauge cast in one small helper, but the
  scope, task T3, and AC3 keep it in the `udp_banned_ips_total` accessor with no helper. It also said
  the cast was the only one left in the counter and gauge read path, but the averaged accessors keep
  their own documented cast.
- Solution: the Background now says the cast stays in the accessor because it is the only gauge
  reader and a helper would wrap a single call, and that it is the only cast left for the integer
  totals, while the averaged accessors keep their own documented cast, which is out of scope.
- Current-tree verification: `grep -c "one small helper"` over the spec reads 0, and the last
  Background bullet reads "the only cast left for the integer totals".
- Resolution reference: docs(issues): [#2560] make the Background say where the gauge cast stays
- Follow-up PR URL: N/A
- Reply URL: <https://github.com/torrust/torrust-tracker/pull/2561#discussion_r4313673562>

### F5 - The template's independent-reviewer-reports checkpoint is missing

- PR number: 2561
- Source review ID: 5571266146
- Reviewer finding ID: F2
- Source URL: <https://github.com/torrust/torrust-tracker/pull/2561#discussion_r4313591243>
- Concern: the issue template has a checkpoint for independent reviewer reports in
  `agent-review-reports.md` when reviewers received a folder-style specification, and this spec
  travels with the implementation to its reviewers, so the spec should carry the obligation to the
  end of the review.
- Solution: the checkpoint is back in the Workflow Checkpoints, unchecked, directly under the
  reviewer checkpoint. It is not ticked, because no independent reviewer agent has received the spec
  so far and the pull-request reviews are recorded in this audit. It will be ticked only if a report
  is actually recorded.
- Current-tree verification: the spec has the checkpoint line, unchecked, below
  "Reviewer validated acceptance criteria and updated checkboxes".
- Resolution reference: docs(issues): [#2560] keep the independent-reviewer-reports checkpoint open
- Follow-up PR URL: N/A
- Reply URL: <https://github.com/torrust/torrust-tracker/pull/2561#discussion_r4313673940>

### F6 - The `counter_total` doc comment omits the gauge case

- PR number: 2561
- Source review ID: 5571606463
- Reviewer finding ID: F1
- Source URL: <https://github.com/torrust/torrust-tracker/pull/2561#discussion_r4313865069>
- Concern: the doc comment said the helper returns `0` when the metric does not exist yet, which is
  one of two cases. `sum_counter` also returns `None` for a gauge name, so the helper reads a gauge
  as a silent `0`. That is why `udp_banned_ips_total` stays on `sum`, and a later caller could route
  a gauge through the helper without noticing. The same two lines are in HTTP core and UDP core.
- Solution: the doc comment of `counter_total` in HTTP core, UDP core, and the UDP server now says
  it also returns `0` when the metric is a gauge, because `sum_counter` returns `None` for a gauge,
  and that it must be called only with counter names.
- Current-tree verification: `grep -c "Call it only with counter"` reads 1 in each of the three
  files, and Clippy and `cargo doc` with `-D warnings` are clean for the three packages.
- Resolution reference: docs(statistics): [#2560] say that counter_total reads a gauge as zero
- Follow-up PR URL: N/A
- Reply URL: <https://github.com/torrust/torrust-tracker/pull/2561#discussion_r4313901817>

### F7 - The spec and the PR description misstate counter overflow and floating-point rounding

- PR number: 2561
- Source review ID: 5571626600
- Reviewer finding ID: N/A
- Source URL: <https://github.com/torrust/torrust-tracker/pull/2561#pullrequestreview-5571626600>
- Concern: the Risks section of the spec and the "One behavior difference" section of the PR
  description said the old path saturated through `f64` on overflow, so that `sum_counter` changes
  overflow behavior. The upstream `Sum` implementation already calls `Metric<Counter>::sum`, a
  `u64` addition, before it converts to `f64`, so both paths panic on overflow in a debug build and
  the old final saturating cast never protected that addition. The reviewer asked for the paragraph
  to distinguish the removed floating-point rounding from the unchanged integer overflow.
- Solution: the reviewer is right, and the claim was wrong. A throwaway test with two counter
  samples at `u64::MAX` panics with an overflow on both `Sum::sum` and `sum_counter` in a debug
  build. It is saved with its output as `evidence/overflow-check.txt`, and the test file was deleted.
  The spec's Risks bullet now says overflow is unchanged and only the rounding of totals above 2^53
  goes away, and the PR description section is replaced by one that says the same.
- Current-tree verification: the spec has the corrected bullet and the evidence file, the PR
  description no longer says the old path saturated on overflow, and the evidence output shows both
  tests passing as `should panic` tests.
- Resolution reference: docs(issues): [#2560] correct the overflow claim: only the f64 rounding goes away
- Follow-up PR URL: N/A
- Reply URL: <https://github.com/torrust/torrust-tracker/pull/2561#issuecomment-6267553195>

## Processing Log

- 2026-10-19 12:15 UTC - Started audit after Copilot's review 5571188743 and da2ce7's review
  5571266146. Fixed F1 to F5 in separate commits, pushed them, and replied on the five threads. The
  approval of review 5571266146 was dismissed at 12:09:34 UTC by the push of the first fixes, as
  recorded in the Findings section.
- 2026-10-19 12:39 UTC - Round 3: da2ce7's review 5571606463 (APPROVED and ACK on the head that
  ended with the first audit commit) carried one finding, F6 here. Fixed it, pushed, and replied on
  its thread. The approval was dismissed at 12:39:10 UTC by that push, as recorded in the Findings
  section.
- 2026-10-19 12:47 UTC - Round 3 also had Copilot's second review 5571626600 (COMMENTED, 12:37:07
  UTC), whose body carried one finding without an inline thread, F7 here. Fixed it in the spec and
  the PR description, answered it with a PR conversation comment, and recorded it as
  `NON_RESOLVABLE`. No approval was standing when the fix was pushed.

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
