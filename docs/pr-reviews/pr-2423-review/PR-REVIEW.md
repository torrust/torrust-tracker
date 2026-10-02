---
semantic-links:
  skill-links:
    - process-pr-review
  related-artifacts:
    - .github/skills/dev/pr-reviews/process-pr-review/SKILL.md
    - "issue #2406"
---

<!-- skill-link: process-pr-review -->

# PR #2423 Review Audit

Source: pull-request reviews and inline review threads for
<https://github.com/torrust/torrust-tracker/pull/2423>.

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

The review overview rates F1, F2, and F3 `Medium severity` and F4 and F5 `Low severity` in its
badge markup; as in the PR #2397 audit, they are recorded as `Minor (inferred)` and
`Nit (inferred)`.

| Finding ID | Review finding reference | Author class | Severity | Category | Relationship | Disposition | Thread state |
| ---------- | ------------------------ | ------------ | -------- | -------- | ------------ | ----------- | ------------ |
| F1 | `review-finding:pr-2423-f1` | Copilot | Minor (inferred) | testing | ORIGINAL | FIXED | RESOLVED |
| F2 | `review-finding:pr-2423-f2` | Copilot | Minor (inferred) | link-integrity | ORIGINAL | FIXED | RESOLVED |
| F3 | `review-finding:pr-2423-f3` | Copilot | Minor (inferred) | link-integrity | ORIGINAL | FIXED | RESOLVED |
| F4 | `review-finding:pr-2423-f4` | Copilot | Nit (inferred) | maintainability | ORIGINAL | FIXED | RESOLVED |
| F5 | `review-finding:pr-2423-f5` | Copilot | Nit (inferred) | documentation | ORIGINAL | FIXED | RESOLVED |

## Finding Details

### F1 - UDP completion helper reuses one transaction id

- PR number: 2423
- Source review ID: 5395864035
- Reviewer finding ID: N/A
- Source URL: <https://github.com/torrust/torrust-tracker/pull/2423#discussion_r4169041437>
- Concern: Both announces in `udp_complete_download` used transaction id 2, so responses could not be told apart if delayed or reordered.
- Solution: The helper builds each announce with its own transaction id (2 and 3), asserts each response echoes its request id, and the scrape uses id 4. Request construction was split from sending to stay within Clippy's argument limit.
- Current-tree verification: `cargo test --test persistence-scrape-after-restart --test metrics-fixed-ports --test metrics-port-zero` passed; workspace pedantic Clippy clean.
- Resolution reference: `test(test-helpers): [#2406] use a distinct transaction id per UDP request`
- Follow-up PR URL: N/A
- Reply URL: <https://github.com/torrust/torrust-tracker/pull/2423#discussion_r4169552847>

### F2 - Relative ADR link in ScrapeHandler rustdoc

- PR number: 2423
- Source review ID: 5395864035
- Reviewer finding ID: N/A
- Source URL: <https://github.com/torrust/torrust-tracker/pull/2423#discussion_r4169041513>
- Concern: The repository-relative `../../../docs/...` link does not resolve in rendered Rust documentation.
- Solution: The link now uses the absolute GitHub URL, the form already used in `packages/configuration/src/v3_0_0/mod.rs`.
- Current-tree verification: `grep -rn 20261002173716 --include=*.rs packages` shows only absolute URLs.
- Resolution reference: `docs(tracker-core): [#2406] link the scrape ADR by absolute URL in rustdoc`
- Follow-up PR URL: N/A
- Reply URL: <https://github.com/torrust/torrust-tracker/pull/2423#discussion_r4169553043>

### F3 - Relative ADR link in persisted_downloads module docs

- PR number: 2423
- Source review ID: 5395864035
- Reviewer finding ID: N/A
- Source URL: <https://github.com/torrust/torrust-tracker/pull/2423#discussion_r4169041582>
- Concern: The repository-relative `../../../../docs/...` link in the module docs does not resolve in rendered Rust documentation.
- Solution: Same change as F2, in the same commit.
- Current-tree verification: same `grep` as F2.
- Resolution reference: `docs(tracker-core): [#2406] link the scrape ADR by absolute URL in rustdoc`
- Follow-up PR URL: N/A
- Reply URL: <https://github.com/torrust/torrust-tracker/pull/2423#discussion_r4169553261>

### F4 - Repeated info-hashes inflate the batch lookup

- PR number: 2423
- Source review ID: 5395864035
- Reviewer finding ID: N/A
- Source URL: <https://github.com/torrust/torrust-tracker/pull/2423#discussion_r4169041649>
- Concern: A request repeating an info-hash absent from memory bound it more than once in the `IN (...)` query.
- Solution: Absent info-hashes are collected in a `BTreeSet` before the batch lookup; response order is unaffected. New unit test `it_should_look_up_a_repeated_info_hash_absent_from_memory_only_once` failed before the change (the mock received the hash twice) and passes after.
- Current-tree verification: `cargo test -p torrust-tracker-core` passed (151 unit, 15 doc, 8 integration).
- Resolution reference: `perf(tracker-core): [#2406] deduplicate scrape info-hashes before the persisted lookup`
- Follow-up PR URL: N/A
- Reply URL: <https://github.com/torrust/torrust-tracker/pull/2423#discussion_r4169553531>

### F5 - Typo in in-memory repository docs

- PR number: 2423
- Source review ID: 5395864035
- Reviewer finding ID: N/A
- Source URL: <https://github.com/torrust/torrust-tracker/pull/2423#discussion_r4169041714>
- Concern: "underling" should be "underlying".
- Solution: Corrected all ten occurrences in `in_memory.rs`, including the pre-existing ones and a stray "error.s".
- Current-tree verification: `grep -c underling packages/tracker-core/src/torrent/repository/in_memory.rs` returns 0.
- Resolution reference: `docs(tracker-core): [#2406] fix 'underling' typo in in-memory repository docs`
- Follow-up PR URL: N/A
- Reply URL: <https://github.com/torrust/torrust-tracker/pull/2423#discussion_r4169553774>

## Processing Log

- 2026-10-02 19:21 UTC - Copilot review 5395864035 submitted five inline findings.
- 2026-10-02 20:28 UTC - Fixed all five findings in separate commits, rebased onto the latest `develop`, pushed after the pre-push suite passed, replied to each thread, and started this audit.

## Completion Rules

- Re-derive the reply claim against the current tree before replying or resolving a thread.
- Reply on every resolvable thread before resolving it.
- For an outdated thread whose concern was fixed, record `Disposition=FIXED` and
  `Thread state=RESOLVED`, even if GitHub marks the original thread outdated after the push. For a
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
