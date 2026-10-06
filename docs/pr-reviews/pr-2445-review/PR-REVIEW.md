---
semantic-links:
  skill-links:
    - process-pr-review
  related-artifacts:
    - .github/skills/dev/pr-reviews/process-pr-review/SKILL.md
    - docs/issues/open/2435-remove-misleading-panics-in-in-memory-torrent-repository/ISSUE.md
---

<!-- skill-link: process-pr-review -->

# PR #2445 Review Audit

Source: pull-request reviews and inline review threads for <https://github.com/torrust/torrust-tracker/pull/2445>.

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
- Category: `link-integrity`, `formatting`, `metadata`, `testing`, `correctness`, `documentation`,
  `maintainability`, `security`, `other`
- An outdated thread whose concern was fixed is `FIXED`/`RESOLVED`, even when GitHub marks the
  original thread outdated after the push. For in-PR feedback, use `NO_ACTION`/`SUPERSEDED` only
  for a duplicate, superseded, or no-change concern. A post-merge `NO_ACTION` requires maintainer
  approval to decline the follow-up work.

## Findings

Round 1 had two reviews. Human review 5426703978 (da2ce7) supplied finding IDs F1 to F7, which are
kept. Copilot review 5426586537 supplied none, so its four findings take the next audit-local IDs,
F8 to F11, in source order. Copilot severities are inferred from its overview badges: `Medium`
maps to Minor and `Low` to Nit. Neither review body adds an assertion beyond its inline threads.
The maintainer approved the dispositions before any change.

| Finding ID | Review finding reference | Author class | Severity | Category | Relationship | Disposition | Thread state |
| ---------- | ------------------------ | ------------ | -------- | -------- | ------------ | ----------- | ------------ |
| F1 | `review-finding:pr-2445-f1` | Human | Major | documentation | ORIGINAL | FIXED | RESOLVED |
| F2 | `review-finding:pr-2445-f2` | Human | Minor | documentation | ORIGINAL | FIXED | RESOLVED |
| F3 | `review-finding:pr-2445-f3` | Human | Minor | documentation | ORIGINAL | FIXED | RESOLVED |
| F4 | `review-finding:pr-2445-f4` | Human | Suggestion | documentation | ORIGINAL | FIXED | RESOLVED |
| F5 | `review-finding:pr-2445-f5` | Human | Suggestion | documentation | ORIGINAL | FIXED | RESOLVED |
| F6 | `review-finding:pr-2445-f6` | Human | Nit | documentation | ORIGINAL | FIXED | RESOLVED |
| F7 | `review-finding:pr-2445-f7` | Human | Nit | documentation | ORIGINAL | FIXED | RESOLVED |
| F8 | `review-finding:pr-2445-f8` | Copilot | Minor (inferred) | link-integrity | ORIGINAL | FIXED | RESOLVED |
| F9 | `review-finding:pr-2445-f9` | Copilot | Minor (inferred) | metadata | ORIGINAL | FIXED | RESOLVED |
| F10 | `review-finding:pr-2445-f10` | Copilot | Nit (inferred) | maintainability | ORIGINAL | NO_ACTION | SUPERSEDED |
| F11 | `review-finding:pr-2445-f11` | Copilot | Nit (inferred) | maintainability | RE_RAISE_OF:F10 | NO_ACTION | SUPERSEDED |

## Finding Details

### F1 - ADR no-expect rule contradicted the expect policy

- PR number: 2445
- Source review ID: 5426703978
- Reviewer finding ID: F1
- Source URL: <https://github.com/torrust/torrust-tracker/pull/2445#discussion_r4193919021>
- Concern: The ADR bullet "Never `expect` or document `# Panics` for failures that cannot occur"
  contradicted the `handle-errors-in-code` skill, which allows production `expect` when failure is
  logically impossible. Together with `clippy::missing_panics_doc`, it forbade legitimate call-site
  `expect` uses on APIs the workspace does not own.
- Solution: Scoped the bullet. It now forbids `expect`ing a workspace API's impossible error (change
  that API instead), while a call-site `expect` on an API that can fail in general but not for this
  input stays under the skill's policy, with the required `# Panics` section.
- Current-tree verification: re-read the ADR's Related rules and the skill's Unwrap and Expect
  Policy table; they no longer conflict.
- Resolution reference: `docs(adrs): scope the no-expect rule to workspace APIs that cannot fail`
- Follow-up PR URL: N/A
- Reply URL: <https://github.com/torrust/torrust-tracker/pull/2445#discussion_r4194370016>

### F2 - Affected code did not link back to the ADR

- PR number: 2445
- Source review ID: 5426703978
- Reviewer finding ID: F2
- Source URL: <https://github.com/torrust/torrust-tracker/pull/2445#discussion_r4193919033>
- Concern: `create-adr` Step 3.5 asks for module-level doc links from the affected code back to the
  ADR; neither `registry.rs` nor `in_memory.rs` had one.
- Solution: Added a module doc link and an `// adr:` marker to both files, matching
  `persisted_downloads.rs`.
- Current-tree verification: `grep -n 'adr' packages/swarm-coordination-registry/src/swarm/registry.rs packages/tracker-core/src/torrent/repository/in_memory.rs`
  shows the link and marker in both; clippy and nightly fmt are clean.
- Resolution reference: `docs(tracker-core): link the registry and in-memory repository to their ADR`
- Follow-up PR URL: N/A
- Reply URL: <https://github.com/torrust/torrust-tracker/pull/2445#discussion_r4194372464>

### F3 - Independent task review was not persisted

- PR number: 2445
- Source review ID: 5426703978
- Reviewer finding ID: F3
- Source URL: <https://github.com/torrust/torrust-tracker/pull/2445#discussion_r4193919044>
- Concern: The spec recorded a task review outcome but the folder had no `agent-review-reports.md`,
  and the template's checkpoint for it was missing.
- Solution: Added `agent-review-reports.md` from the template with the review's scope, evidence,
  findings, verdict, and follow-up, noting it was recorded after the fact; added and ticked the
  checkpoint.
- Current-tree verification: the spec folder lists `ISSUE.md`, `agent-review-reports.md`, and
  `implementation-retrospective.md`; the checkpoint line is present.
- Resolution reference: `docs(issues): record the #2435 task review and fix spec review findings`
- Follow-up PR URL: N/A
- Reply URL: <https://github.com/torrust/torrust-tracker/pull/2445#discussion_r4194372733>

### F4 - "Port" undefined and REST query ports not placed

- PR number: 2445
- Source review ID: 5426703978
- Reviewer finding ID: F4
- Source URL: <https://github.com/torrust/torrust-tracker/pull/2445#discussion_r4193919048>
- Concern: The ADR used "port" without a definition. Read with the contract-first REST API ADR,
  `StatsQueryPort` is a swappable port, so condition 3 and the retrospective seemed to require the
  `Result` that this PR reverted.
- Solution: The maintainer chose to tighten condition 3: a trait or port qualifies only with an
  existing or planned backend that can fail; being swappable alone does not. The ADR links the
  port definition and places the REST ports (auth-key and whitelist return `Result` through
  database I/O, stats and torrent query ports return plain values). The retrospective, skill, and
  index use the same wording.
- Current-tree verification: re-read ADR condition 3, the retrospective's Avoiding Overcorrection
  section, the skill rule, and the index row.
- Resolution reference: `docs(adrs): define ports and require a real or planned failing backend`
- Follow-up PR URL: N/A
- Reply URL: <https://github.com/torrust/torrust-tracker/pull/2445#discussion_r4194373008>

### F5 - False lock-panic docs in torrent services

- PR number: 2445
- Source review ID: 5426703978
- Reviewer finding ID: F5
- Source URL: <https://github.com/torrust/torrust-tracker/pull/2445#discussion_r4193919058>
- Concern: `get_torrent_info`, `get_torrents_page`, and `get_torrents` documented a panic when a
  lock cannot be obtained, but they only await `tokio::sync::Mutex::lock`, which cannot fail.
- Solution: Deleted the three `# Panics` sections (docs-only change).
- Current-tree verification: `grep -n 'panics if the lock' packages/tracker-core/src/torrent/services.rs`
  returns nothing; the function bodies contain no `expect`, `unwrap`, or indexing; clippy is clean.
- Resolution reference: `docs(tracker-core): remove false lock-panic docs from torrent services`
- Follow-up PR URL: N/A
- Reply URL: <https://github.com/torrust/torrust-tracker/pull/2445#discussion_r4194373363>

### F6 - AC5 evidence command was not reproducible

- PR number: 2445
- Source review ID: 5426703978
- Reviewer finding ID: F6
- Source URL: <https://github.com/torrust/torrust-tracker/pull/2445#discussion_r4193919062>
- Concern: Run from the repository root, the AC5 `rg` command matched prose about the reverted work
  in three docs files, so the "finds nothing" claim did not reproduce.
- Solution: Scoped the command to `-- packages src` and noted what the unscoped run matches.
- Current-tree verification: `rg 'registry::Error|SwarmRegistry|StatsError' -- packages src` exits
  1 with no matches.
- Resolution reference: `docs(issues): record the #2435 task review and fix spec review findings`
- Follow-up PR URL: N/A
- Reply URL: <https://github.com/torrust/torrust-tracker/pull/2445#discussion_r4194373668>

### F7 - EPIC checklist placed under Open Questions

- PR number: 2445
- Source review ID: 5426703978
- Reviewer finding ID: F7
- Source URL: <https://github.com/torrust/torrust-tracker/pull/2445#discussion_r4193919073>
- Concern: The Pre-publish API checklist is a decided procedure but sat under "Open Questions".
- Solution: Moved it to a subsection of Delivery Strategy with an EPIC progress-log entry. The
  rebase onto `develop` after #2441 merged resolved the EPIC stamp and log conflicts, keeping both
  log histories in chronological order.
- Current-tree verification: `grep -nE '^#{2,3} ' docs/issues/open/1669-overhaul-packages/EPIC.md`
  shows the checklist between "Subsequent cycles" and "Open Questions".
- Resolution reference: `docs(issues): move the EPIC #1669 pre-publish checklist to Delivery Strategy`
- Follow-up PR URL: N/A
- Reply URL: <https://github.com/torrust/torrust-tracker/pull/2445#discussion_r4194373894>

### F8 - T3 row cited a superseded ADR filename

- PR number: 2445
- Source review ID: 5426586537
- Reviewer finding ID: N/A
- Source URL: <https://github.com/torrust/torrust-tracker/pull/2445#discussion_r4193820694>
- Concern: The spec's T3 row named an ADR file that does not exist in this PR.
- Solution: The T3 row now explains the rename history and names the final ADR file.
- Current-tree verification: re-read the T3 row; `git ls-files docs/adrs/ | grep 20261005145329`
  lists only the final file.
- Resolution reference: `docs(issues): record the #2435 task review and fix spec review findings`
- Follow-up PR URL: N/A
- Reply URL: <https://github.com/torrust/torrust-tracker/pull/2445#discussion_r4194374137>

### F9 - Spec said pre-push checks had not run

- PR number: 2445
- Source review ID: 5426586537
- Reviewer finding ID: N/A
- Source URL: <https://github.com/torrust/torrust-tracker/pull/2445#discussion_r4193820740>
- Concern: The PR body said pre-push checks passed, but the spec checkpoint said they had not run.
- Solution: The PR body was accurate; the spec line was stale. The checkpoint is now ticked and
  records the pre-commit, full test suite, and pre-push results.
- Current-tree verification: re-read the Workflow Checkpoints in `ISSUE.md`; the pre-push hook
  passed again on this round's push.
- Resolution reference: `docs(issues): record the #2435 task review and fix spec review findings`
- Follow-up PR URL: N/A
- Reply URL: <https://github.com/torrust/torrust-tracker/pull/2445#discussion_r4194374459>

### F10 - Remove-method counts lack `#[must_use]`

- PR number: 2445
- Source review ID: 5426586537
- Reviewer finding ID: N/A
- Source URL: <https://github.com/torrust/torrust-tracker/pull/2445#discussion_r4193820785>
- Concern: `remove_inactive_peers` and `remove_peerless_torrents` return counts without
  `#[must_use]`; either mark them or return `()`.
- Solution: No change, approved by the maintainer. The counts are informational: the registry
  logs them, and every caller discards them on purpose. `#[must_use]` would force unused bindings,
  and `()` would drop a count a caller may legitimately want. Review 5426703978 independently found
  the split matches the callers.
- Current-tree verification: `grep -rnE 'swarms\s*\.\s*remove_(inactive_peers|peerless_torrents)\(' packages --include=*.rs`
  (scoped to the registry receiver; corrected per F14) matches eight lines: `in_memory.rs` (two)
  and six registry test lines, each discarding the count.
- Resolution reference: <https://github.com/torrust/torrust-tracker/pull/2445#discussion_r4194374698>
- Follow-up PR URL: N/A
- Reply URL: <https://github.com/torrust/torrust-tracker/pull/2445#discussion_r4194374698>

### F11 - Same `#[must_use]` concern on the second remove method

- PR number: 2445
- Source review ID: 5426586537
- Reviewer finding ID: N/A
- Source URL: <https://github.com/torrust/torrust-tracker/pull/2445#discussion_r4193820817>
- Concern: The same "These methods..." comment as F10, anchored on `remove_peerless_torrents`.
- Solution: No change; F10's reasoning covers both methods.
- Current-tree verification: same as F10.
- Resolution reference: <https://github.com/torrust/torrust-tracker/pull/2445#discussion_r4194374968>
- Follow-up PR URL: N/A
- Reply URL: <https://github.com/torrust/torrust-tracker/pull/2445#discussion_r4194374968>

## Processing Log

- 2026-10-06 10:06 UTC - Started audit. Fetched 11 threads (all unresolved) and both round-1
  review bodies with `github-review-threads` (time from the thread file).
- 2026-10-06 10:33 UTC - First fix committed after the maintainer approved the dispositions in
  chat: F1 scope the ADR bullet, F4 tighten condition 3, F10 and F11 no change, all others fixed as
  proposed. The approval preceded this commit; its exact time was not recorded.
- 2026-10-06 10:47 UTC - Rebased all fix commits (authored 10:33 to 10:43 UTC) onto `develop`,
  resolving the EPIC conflicts with #2441, and pushed; the pre-push hook passed.
- 2026-10-06 10:51 UTC - Replied to all 11 threads; recorded reply URLs.

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
