---
semantic-links:
  skill-links:
    - process-pr-review
  related-artifacts:
    - .github/skills/dev/pr-reviews/process-pr-review/SKILL.md
    - "issue #2521"
---

<!-- skill-link: process-pr-review -->

# PR #2522 Review Audit

Source: pull-request reviews and inline review threads for <https://github.com/torrust/torrust-tracker/pull/2522>.

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

Review 5515225239 (round 1, `copilot-pull-request-reviewer`, state `COMMENTED`, submitted
2026-10-14 08:39 UTC on the head whose tip commit was
`docs(issues): [#2521] add issue specification for #2521`,
<https://github.com/torrust/torrust-tracker/pull/2522#pullrequestreview-5515225239>) left five
inline findings with reviewer-provided IDs F1 to F5 and severity brackets. Its body lists the same
five findings and summarizes the pull request; it is not a further finding.

Review 5515320310 (round 1, `da2ce7`, state `CHANGES_REQUESTED`, submitted 2026-10-14 08:47 UTC on
the same head, <https://github.com/torrust/torrust-tracker/pull/2522#pullrequestreview-5515320310>)
left three inline findings, F7, F8, and F10. Its body assesses the bot's F1 to F5 against the
bytes: it agrees with F1 and marks it blocking, disagrees with F2's severity, partly agrees with F3,
and agrees with F4 and F5, extending F4 to the truncated completion checkpoint and F5 to the two
`closed/` paths. The reviewer states that his own round-one points F6 and F9 are merged into F4 and
F5. These assessments refine the scope of the bot's findings rather than request separate changes,
so they are recorded in the F1 to F5 detail entries, not as extra rows. The body's note that pull
request #2520 also edits `EPIC.md` is informational and not a finding.

| Finding ID | Review finding reference | Author class | Severity | Category | Relationship | Disposition | Thread state |
| ---------- | ------------------------ | ------------ | -------- | -------- | ------------ | ----------- | ------------ |
| F1 | `review-finding:pr-2522-f1` | Copilot | Major | correctness | ORIGINAL | FIXED | RESOLVED |
| F2 | `review-finding:pr-2522-f2` | Copilot | Major | documentation | ORIGINAL | FIXED | RESOLVED |
| F3 | `review-finding:pr-2522-f3` | Copilot | Major | testing | ORIGINAL | FIXED | RESOLVED |
| F4 | `review-finding:pr-2522-f4` | Copilot | Minor | metadata | ORIGINAL | FIXED | RESOLVED |
| F5 | `review-finding:pr-2522-f5` | Copilot | Minor | link-integrity | ORIGINAL | FIXED | RESOLVED |
| F7 | `review-finding:pr-2522-f7` | Human | Minor | documentation | ORIGINAL | FIXED | RESOLVED |
| F8 | `review-finding:pr-2522-f8` | Human | Minor | documentation | ORIGINAL | FIXED | RESOLVED |
| F10 | `review-finding:pr-2522-f10` | Human | Nit | correctness | ORIGINAL | FIXED | RESOLVED |

## Finding Details

### F1 - The manual scenario checked one REST surface that cannot show the udp-core metrics

- PR number: 2522
- Source review ID: 5515225239
- Reviewer finding ID: N/A
- Source URL: <https://github.com/torrust/torrust-tracker/pull/2522#discussion_r4268560591>
- Concern: M1 curled a single "REST stats endpoint" and AC3 expected both UDP core and UDP server
  metrics there. `TrackerStatsAdapter::get_stats()` serves `/api/v1/stats` from the udp-server
  repository only; the udp-core collection is merged only by `get_labeled_stats()`, which serves
  `/api/v1/metrics`. Cameron agreed and marked it blocking: the only manual scenario could not
  produce its own expected result.
- Solution: Split the scenario into M1 (`/api/v1/stats`) and M2 (`/api/v1/metrics`), each with
  exact JSON and Prometheus `curl` commands, before/after capture, and the fields or metric names
  expected on that surface. AC3 now names both surfaces and points at M1 and M2.
- Current-tree verification: Read `packages/axum-rest-api-server/src/v1/context/stats/routes.rs`
  and `handlers.rs` (`/stats` -> `get_stats`, `/metrics` -> `get_labeled_stats`, `format` accepts
  `json` and `prometheus`), `packages/rest-api-runtime-adapter/src/v1/adapters/stats.rs`, the
  udp-core metric constant `udp_tracker_core_requests_received_total`, the `Stats` resource field
  names in `rest-api-protocol`, and the development config (`0.0.0.0:1212`, token
  `MyAccessToken`).
- Resolution reference: `docs(issues): [#2521] verify the stats and metrics REST surfaces separately`
- Follow-up PR URL: N/A
- Reply URL: <https://github.com/torrust/torrust-tracker/pull/2522#discussion_r4268917129>

### F2 - The spec did not apply the EPIC #1669 package-boundary checklist

- PR number: 2522
- Source review ID: 5515225239
- Reviewer finding ID: N/A
- Source URL: <https://github.com/torrust/torrust-tracker/pull/2522#discussion_r4268560411>
- Concern: Design and Ownership Review said "Not applicable" although the work removes public
  interfaces and the `async-trait` dependency from two packages; the EPIC's subissue architecture
  checklist asks for current and target edges, symbol evidence, and a before/after dependency
  diff. Cameron disagreed with the severity: no workspace edge changes and `async-trait` is not a
  layer, so a one-line note would suffice.
- Solution: Added a short package-boundary review: current and target workspace edges are
  identical and respect the layer model, the only change is the external `async-trait` crate, and
  a `cargo tree -e normal --depth 1` before/after comparison for both packages (also listed under
  Automatic Checks) shows it. This records Cameron's point and gives the bot's verification step at
  little cost.
- Current-tree verification: `cargo tree -e normal --depth 1 -p torrust-tracker-udp-core` and the
  same for `torrust-tracker-udp-server` both list `async-trait v0.1.92 (proc-macro)`;
  `rest-api-runtime-adapter/src/v1/adapters/stats.rs` holds both concrete `Repository` types.
- Resolution reference: `docs(issues): [#2521] record the package-boundary review`
- Follow-up PR URL: N/A
- Reply URL: <https://github.com/torrust/torrust-tracker/pull/2522#discussion_r4268915537>

### F3 - The test-change review gates were not enforceable

- PR number: 2522
- Source review ID: 5515225239
- Reviewer finding ID: N/A
- Source URL: <https://github.com/torrust/torrust-tracker/pull/2522#discussion_r4268560464>
- Concern: T3 modifies a test, so `create-issue` requires the smallest increment, focused
  validation, a recorded design review, and a maintainer stop; the spec only asked for a "brief"
  Arrange-Act-Assert review. Cameron partly agreed: `write-unit-test` covers most of it, but the
  explicit maintainer stop was missing and "brief" should read "prose-first".
- Solution: Regression Test Strategy now lists three gates: `write-unit-test` with a focused
  command, the recorded prose-first Arrange-Act-Assert review with its four checks, and the stop
  for maintainer review before final verification, commit, or pull request. The T2 and T3 commit
  policies require the review and maintainer review.
- Current-tree verification: `cargo test -q -p torrust-tracker-udp-server --lib statistics::repository`
  (stable Rust toolchain) runs 17 tests, all passing.
- Resolution reference: `docs(issues): [#2521] make the test-change review gates explicit`
- Follow-up PR URL: N/A
- Reply URL: <https://github.com/torrust/torrust-tracker/pull/2522#discussion_r4268916247>

### F4 - Template completion checkpoints were missing

- PR number: 2522
- Source review ID: 5515225239
- Reviewer finding ID: N/A
- Source URL: <https://github.com/torrust/torrust-tracker/pull/2522#discussion_r4268560529>
- Concern: The Workflow Checkpoints omitted the template's reviewer-validation,
  independent-review-report, and committer-progress checkpoints. Cameron agreed (his round-one
  F6) and added that the completion-review checkpoint dropped the tail of the template wording.
- Solution: Restored the three checkpoints and the full completion-review wording from
  `docs/templates/ISSUE.md`.
- Current-tree verification: Compared the spec's Workflow Checkpoints with
  `docs/templates/ISSUE.md`; all thirteen template checkpoints are present, with the draft path
  filled in and the spec-only PR checkpoint stated without the template's "(Optional, ...)"
  prefix because this issue uses a spec-only PR.
- Resolution reference: `docs(issues): [#2521] restore the template's completion checkpoints`
- Follow-up PR URL: N/A
- Reply URL: <https://github.com/torrust/torrust-tracker/pull/2522#discussion_r4268916692>

### F5 - Issue specifications were linked by paths that go stale

- PR number: 2522
- Source review ID: 5515225239
- Reviewer finding ID: N/A
- Source URL: <https://github.com/torrust/torrust-tracker/pull/2522#discussion_r4268560305>
- Concern: `related-artifacts` named the #1669 and #2505 specifications by `open/` paths, which go
  stale when the issues close; the semantic-link convention prefers `issue #NNNN`. Cameron agreed
  (his round-one F9) and extended it to the two `closed/` paths, because `closed/` is only a
  temporary buffer.
- Solution: Replaced all four issue-specification paths with quoted `issue #1669`,
  `issue #2505`, `issue #1924`, and `issue #1942`; stable file paths stay as paths.
- Current-tree verification: `frontmatter-validator --staged` exited 0 on the changed spec, and
  `docs/skills/semantic-skill-link-convention.md` documents the quoted `issue #NNNN` form.
- Resolution reference: `docs(issues): [#2521] use stable issue references in related artifacts`
- Follow-up PR URL: N/A
- Reply URL: <https://github.com/torrust/torrust-tracker/pull/2522#discussion_r4268914976>

### F7 - The documentation acceptance criterion was missing

- PR number: 2522
- Source review ID: 5515320310
- Reviewer finding ID: N/A
- Source URL: <https://github.com/torrust/torrust-tracker/pull/2522#discussion_r4268636526>
- Concern: The template's "Documentation is updated when behavior/workflow changes" box was
  missing, although T4 changes the EPIC #1669 SI-30 row.
- Solution: Added the box after the re-review box.
- Current-tree verification: The spec's Acceptance Criteria list ends with the same box as
  `docs/templates/ISSUE.md`.
- Resolution reference: `docs(issues): [#2521] add the documentation acceptance criterion`
- Follow-up PR URL: N/A
- Reply URL: <https://github.com/torrust/torrust-tracker/pull/2522#discussion_r4268917590>

### F8 - Commit Points did not map T5's evidence or T1's record

- PR number: 2522
- Source review ID: 5515320310
- Reviewer finding ID: N/A
- Source URL: <https://github.com/torrust/torrust-tracker/pull/2522#discussion_r4268636547>
- Concern: T5 writes documentation (`manual-verification-evidence.md` and the Acceptance
  Verification evidence) but had no commit point, and the spec did not say which commit carries
  T1's re-verification record.
- Solution: Added a T1 row (no separate commit; the progress-log entry goes in with T2) and a T5
  row for the verification evidence and spec progress updates.
- Current-tree verification: The Commit Points table maps T1 to T5.
- Resolution reference: `docs(issues): [#2521] map the evidence and re-verification tasks to commits`
- Follow-up PR URL: N/A
- Reply URL: <https://github.com/torrust/torrust-tracker/pull/2522#discussion_r4268918064>

### F10 - The evidence heading named an older commit

- PR number: 2522
- Source review ID: 5515320310
- Reviewer finding ID: N/A
- Source URL: <https://github.com/torrust/torrust-tracker/pull/2522#discussion_r4268636558>
- Concern: The evidence heading named `395cd3ca0`, while the Progress Log and PR body say the
  evidence was re-checked at `cbdbce9c1`.
- Solution: The heading now names `cbdbce9c1`.
- Current-tree verification: `git diff --stat 395cd3ca0 cbdbce9c1` lists only `Cargo.lock`.
- Resolution reference: `docs(issues): [#2521] name the commit where the evidence was re-checked`
- Follow-up PR URL: N/A
- Reply URL: <https://github.com/torrust/torrust-tracker/pull/2522#discussion_r4268918432>

## Processing Log

- 2026-10-14 09:13 UTC - Started audit. Fetched reviews 5515225239 and 5515320310 and all eight
  threads with `github-review-threads`. Committed one fix per finding, pushed, and replied on
  every thread.

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
