---
semantic-links:
  skill-links:
    - process-pr-review
  related-artifacts:
    - .github/skills/dev/pr-reviews/process-pr-review/SKILL.md
    - "issue #2517"
---

<!-- skill-link: process-pr-review -->

# PR #2520 Review Audit

Source: pull-request reviews and inline review threads for <https://github.com/torrust/torrust-tracker/pull/2520>.

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

Copilot review 5514549824 (submitted 2026-10-14 07:37:11 UTC, before merge) left two inline
findings headed `[Major][F1]` and `[Minor][F2]`. The reviewer-provided IDs do not collide with any
earlier audit finding, so they are used unchanged. The review overview repeats those two findings
and summarizes the PR; it adds no other request, so it adds no row.

Human review 5515221413 by `da2ce7` (submitted 2026-10-14 08:38:38 UTC, before merge, state
`CHANGES_REQUESTED`) left three inline findings headed `[Minor][F3]`, `[Minor][F4]`, and
`[Nit][F5]`. Their IDs do not collide with F1 or F2, so they are used unchanged. The review body
repeats those findings and adds two remarks that request no change: the F2 form is valid, and
whether siblings #2454 and #2475 also switch to `issue #1669` is the maintainer's call; and the
08:17 log entry below records replies that GitHub dates 08:19:37-38 UTC (marked "not raised"). They
add no row; the timing remark is corrected in the Processing Log. The empty-bodied reviews
5515013134 and 5515013449 are the containers GitHub created for the author's F1 and F2 replies.

| Finding ID | Review finding reference | Author class | Severity | Category | Relationship | Disposition | Thread state |
| ---------- | ------------------------ | ------------ | -------- | -------- | ------------ | ----------- | ------------ |
| F1 | `review-finding:pr-2520-f1` | Copilot | Major | correctness | ORIGINAL | FIXED | RESOLVED |
| F2 | `review-finding:pr-2520-f2` | Copilot | Minor | metadata | ORIGINAL | FIXED | RESOLVED |
| F3 | `review-finding:pr-2520-f3` | Human | Minor | testing | ORIGINAL | FIXED | RESOLVED |
| F4 | `review-finding:pr-2520-f4` | Human | Minor | testing | ORIGINAL | FIXED | RESOLVED |
| F5 | `review-finding:pr-2520-f5` | Human | Nit | link-integrity | ORIGINAL | FIXED | RESOLVED |

## Finding Details

### F1 - The visibility-only change does not remove the lock type from the public API

- PR number: 2520
- Source review ID: 5514549824
- Reviewer finding ID: N/A
- Source URL: <https://github.com/torrust/torrust-tracker/pull/2520#discussion_r4268014012>
- Concern: The spec said the public field "makes the lock type part of each crate's public API",
  implying that making it private removes the lock from the API. All five public read accessors
  (`get_stats()` / `get_metrics()`) still return `RwLockReadGuard<'_, Metrics>`, and the spec
  excluded changing method signatures. The goal and the semver rationale contradicted the planned
  change.
- Solution: The maintainer chose to narrow the spec rather than broaden it. The Goal now states a
  visibility-only change that keeps the read guards. The Background no longer claims the lock type
  leaves the API, and names callers outside the five crates that rely on the guards. The semver
  note is limited to the field. Out of Scope records replacing the guards (per-metric accessors or
  a cheap snapshot) as a possible separate #1669 sub-issue; cloning the whole `Metrics` per read is
  too costly on event-handler paths. The PR body and the #2517 issue body were corrected the same
  way.
- Current-tree verification: `grep -n "RwLockReadGuard" packages/*/src/statistics/repository.rs`
  shows the five read accessors returning the guard; the spec's Goal, Background, Semver impact,
  and Out of Scope sections now describe the change as visibility-only. Outside callers confirmed
  with a search for `.get_stats()` / `.get_metrics()` (for example
  `packages/rest-api-runtime-adapter/src/v1/adapters/stats.rs` and the `axum-http-server`
  contract tests).
- Resolution reference: `docs(issues): [#2517] narrow the spec to a visibility-only change`
- Follow-up PR URL: N/A
- Reply URL: <https://github.com/torrust/torrust-tracker/pull/2520#discussion_r4268384235>

### F2 - Cite the EPIC by stable issue reference instead of its path

- PR number: 2520
- Source review ID: 5514549824
- Reviewer finding ID: N/A
- Source URL: <https://github.com/torrust/torrust-tracker/pull/2520#discussion_r4268014067>
- Concern: The spec frontmatter listed `docs/issues/open/1669-overhaul-packages/EPIC.md` in
  `semantic-links.related-artifacts`. That path becomes stale when the EPIC spec moves to
  `docs/issues/closed/`; the semantic link convention requires a quoted `issue #NNNN` value.
- Solution: Replaced the path with `"issue #1669"`.
- Current-tree verification: the spec frontmatter's `related-artifacts` lists `"issue #1669"` and
  no `docs/issues/` path; the staged-frontmatter pre-commit check passed.
- Resolution reference: `docs(issues): [#2517] cite EPIC #1669 by issue number in spec frontmatter`
- Follow-up PR URL: N/A
- Reply URL: <https://github.com/torrust/torrust-tracker/pull/2520#discussion_r4268384509>

### F3 - A test can observe field visibility from outside the crate

- PR number: 2520
- Source review ID: 5515221413
- Reviewer finding ID: N/A
- Source URL: <https://github.com/torrust/torrust-tracker/pull/2520#discussion_r4268557245>
- Concern: Regression Test Strategy said no test was planned "because a test cannot observe field
  visibility from outside the crate". A doctest compiles as a separate crate, and a
  `compile_fail,E0616` doctest pins a private field; `packages/udp-core/src/container.rs` already
  does it for `cookie_cipher`. Now that the change is visibility-only, one such doctest per
  repository is its direct regression guard and a named check for AC2.
- Solution: Planned the doctests. Regression Test Strategy now describes one
  `rust,compile_fail,E0616` doctest per `Repository` that reads `repository.stats` from outside the
  crate, with an example, a red-first step, and a note that `pub(crate)` / `pub(super)` are left to
  the AC1 grep. In Scope, T2-T6, and Commit Points include the doctest, with a test-design review
  before each commit. The #2517 issue body scope was updated to match.
- Current-tree verification: `packages/udp-core/src/container.rs` has the cited
  `rust,compile_fail,E0616` doctest on `UdpTrackerCoreServices`. Each of the five crates declares
  `pub mod statistics` in `src/lib.rs` and `pub mod repository` in `src/statistics/mod.rs`, with no
  custom `[lib] name`, so `torrust_tracker_<crate>::statistics::repository::Repository` is
  reachable from a doctest.
- Resolution reference: `docs(issues): [#2517] plan compile_fail doctests that pin the private field`
- Follow-up PR URL: N/A
- Reply URL: <https://github.com/torrust/torrust-tracker/pull/2520#discussion_r4268875552>

### F4 - The AC1 grep misses restricted visibility, and AC2 has no named check

- PR number: 2520
- Source review ID: 5515221413
- Reviewer finding ID: N/A
- Source URL: <https://github.com/torrust/torrust-tracker/pull/2520#discussion_r4268557251>
- Concern: `grep -n "pub stats"` prints nothing for `pub(crate) stats:` or `pub(super) stats:`, so
  AC1's check would pass while same-crate code could still take the lock, which AC2 rules out. AC2
  had no named check.
- Solution: The AC1 check now uses the reviewer's pattern
  `grep -nE 'pub(\([^)]*\))? +stats *:' packages/*/src/statistics/repository.rs`. A new AC2 checks
  bullet names the workspace build and tests with the field private and the five F3 doctests under
  `cargo test --doc --workspace`.
- Current-tree verification: the pattern matches the five current `pub stats:` lines (one per
  repository file); on a sample it matches `pub(crate) stats:` and `pub(super) stats:` but not a
  private `stats:` or `pub stats_x:`.
- Resolution reference: `docs(issues): [#2517] make the AC1 check catch restricted visibility and name the AC2 checks`
- Follow-up PR URL: N/A
- Reply URL: <https://github.com/torrust/torrust-tracker/pull/2520#discussion_r4268875818>

### F5 - The cited SI-30 traits draft is not in the tree

- PR number: 2520
- Source review ID: 5515221413
- Reviewer finding ID: N/A
- Source URL: <https://github.com/torrust/torrust-tracker/pull/2520#discussion_r4268557254>
- Concern: Out of Scope and Risks cited "a separate #1669 sub-issue draft" for the SI-30 traits,
  but no folder under `docs/issues/drafts/` covers them. The only in-tree note is the #2505 spec's
  Out of Scope.
- Solution: Out of Scope now names the traits, says their draft is not committed yet, and points at
  `docs/issues/open/2505-udp-banned-ips-metric/ISSUE.md`. The Risks bullet refers to that entry.
- Current-tree verification: `docs/issues/drafts/` has no SI-30 traits folder; the #2505 spec's Out
  of Scope names `UdpCoreStatsRepository` and `UdpServerStatsRepository` as possibly leftover.
- Resolution reference: `docs(issues): [#2517] say where the SI-30 traits work is recorded`
- Follow-up PR URL: N/A
- Reply URL: <https://github.com/torrust/torrust-tracker/pull/2520#discussion_r4268876077>

## Processing Log

- 2026-10-14 08:12 UTC - Started audit: fetched review 5514549824 and its two unresolved threads
  with the `github-review-threads` tool. Asked the maintainer whether to narrow or broaden the
  spec for F1; the maintainer chose to narrow it.
- 2026-10-14 08:17 UTC - Pushed the signed fix commits for F2 and F1; the pre-push checks passed.
  Corrected the same inaccurate claim in the PR body and the #2517 issue body. Replied on both threads
  with the disposition, verification, and resolution reference.
- 2026-10-14 08:19 UTC - Recorded this audit; the threads are resolved after it is committed and
  pushed.
- 2026-10-14 09:05 UTC - Started round 2: fetched review 5515221413 (`da2ce7`) and its three
  unresolved threads. Correction to the 08:17 entry: GitHub dates the F1 and F2 replies 08:19:37
  and 08:19:38 UTC, after that entry's stamp; the entry grouped them with the push.
- 2026-10-14 09:12 UTC - Pushed the signed fix commits for F5, F3, and F4; the pre-push checks
  passed. Added the doctests to the #2517 issue body scope. Replied on the three threads with the
  disposition, verification, and resolution reference.
- 2026-10-14 09:13 UTC - Recorded round 2 in this audit; the threads are resolved after it is
  committed and pushed.

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
