---
pr-number: 2956
pr-url: https://github.com/torrust/torrust-tracker/pull/2956
last-updated-utc: "2026-11-19 08:27"
---

<!-- skill-link: process-pr-review -->

# PR #2956 Review Audit

Source: pull-request reviews and inline review threads for https://github.com/torrust/torrust-tracker/pull/2956.

<!-- Copied verbatim into each audit record after placeholder replacement. -->

## Ownership

The PR author owns this tracked audit record. Reviewers, including repository review agents,
deliver findings through GitHub and have no repository-artifact obligation.

- Post-merge workflow approval: N/A

<!-- Copied verbatim into each audit record. -->

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
- When the current identity cannot resolve a thread (`viewerCanResolve` is `false`), keep the
  disposition established from the current tree and record `Thread state=OPEN`. The reviewer or PR
  author resolves it; permission says nothing about whether the concern was fixed.

## Findings

Round 1 is Copilot's review `5964711849` (COMMENTED, one inline comment with the reviewer-provided ID F1) on the first head.
Round 2 is Copilot's review `5970782901` (one body finding, F2 here) and da2ce7's review `5971038339` (APPROVED, eight inline findings, F3 to F10 here; his own IDs are F1 to F8), both on head `a4d2a220b`.

| Finding ID | Review finding reference | Author class | Severity | Category | Relationship | Disposition | Thread state |
| ---------- | ------------------------ | ------------ | -------- | -------- | ------------ | ----------- | ------------ |
| F1 | `review-finding:pr-2956-f1` | Copilot | Major | correctness | ORIGINAL | FIXED | RESOLVED |
| F2 | `review-finding:pr-2956-f2` | Copilot | Minor | documentation | ORIGINAL | FIXED | NON_RESOLVABLE |
| F3 | `review-finding:pr-2956-f3` | Human | Minor | documentation | RE_RAISE_OF:F2 | FIXED | RESOLVED |
| F4 | `review-finding:pr-2956-f4` | Human | Minor | documentation | ORIGINAL | FIXED | RESOLVED |
| F5 | `review-finding:pr-2956-f5` | Human | Minor | documentation | ORIGINAL | FIXED | RESOLVED |
| F6 | `review-finding:pr-2956-f6` | Human | Minor | documentation | ORIGINAL | FIXED | RESOLVED |
| F7 | `review-finding:pr-2956-f7` | Human | Nit | documentation | ORIGINAL | FIXED | RESOLVED |
| F8 | `review-finding:pr-2956-f8` | Human | Nit | documentation | ORIGINAL | FIXED | RESOLVED |
| F9 | `review-finding:pr-2956-f9` | Human | Nit | documentation | ORIGINAL | FIXED | RESOLVED |
| F10 | `review-finding:pr-2956-f10` | Human | Nit | documentation | ORIGINAL | FIXED | RESOLVED |

## Finding Details

### F1 - Plain-text panic output breaks the JSON-only stderr rule

- PR number: 2956
- Source review ID: 5964711849
- Reviewer finding ID: F1
- Source URL: https://github.com/torrust/torrust-tracker/pull/2956#discussion_r4620489523
- Concern: A `check` configuration with a UDP tracker URL that has no port passes validation and panics at `checker/checks/udp.rs:107`; the default panic hook writes plain text to stderr before the error becomes a JSON runtime failure.
- Solution: `main` installs a panic hook first (`install_panic_hook` in `unified/output.rs`) that writes one `panic` JSON record and does not chain the default hook. A black-box test runs `check` with that input. The unwrap in `checks/udp.rs` is not changed: that would change what `check` does with the input and is outside this issue.
- Current-tree verification: Reproduced on the first head: the default hook text, then the `runtime_failure` record, exit 1. After the fix, `cargo nextest run -p torrust-tracker-client` ran 72 tests, all passed, including `it_should_report_a_panic_as_a_json_record_and_leave_stdout_empty` (two JSON stderr lines, empty stdout, exit 1).
- Resolution reference: `fix(tracker-client): [#2768] write panics as JSON records`
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2956#discussion_r4620914535

### F2 - The README `check` example has an extra `]` and its closing fence is on the `}` line, so the License section renders as code

- PR number: 2956
- Source review ID: 5970782901
- Reviewer finding ID: F2
- Source URL: https://github.com/torrust/torrust-tracker/pull/2956#pullrequestreview-5970782901
- Concern: The README `check` example has an extra `]` and its closing fence is on the `}` line, so the License section renders as code.
- Solution: Removed the duplicate bracket and put the closing fence on its own line.
- Current-tree verification: checked in the tree after the push of the round 2 commit; see the reply.
- Resolution reference: `docs(tracker-client): [#2768] fix the README example and update the skills and contract`
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2956#discussion_r4626112294

### F3 - Same README defect as F2, raised inline by da2ce7 on the same head

- PR number: 2956
- Source review ID: 5971038339
- Reviewer finding ID: F1
- Source URL: https://github.com/torrust/torrust-tracker/pull/2956#discussion_r4625668513
- Concern: Same README defect as F2, raised inline by da2ce7 on the same head.
- Solution: Same fix as F2.
- Current-tree verification: checked in the tree after the push of the round 2 commit; see the reply.
- Resolution reference: `docs(tracker-client): [#2768] fix the README example and update the skills and contract`
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2956#discussion_r4626112294

### F4 - The README `check` example is not piped, so on a terminal it is refused

- PR number: 2956
- Source review ID: 5971038339
- Reviewer finding ID: F2
- Source URL: https://github.com/torrust/torrust-tracker/pull/2956#discussion_r4625668521
- Concern: The README `check` example is not piped, so on a terminal it is refused.
- Solution: Added `| jq .`.
- Current-tree verification: checked in the tree after the push of the round 2 commit; see the reply.
- Resolution reference: `docs(tracker-client): [#2768] fix the README example and update the skills and contract`
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2956#discussion_r4626110523

### F5 - Live skills run `tracker_client` with stdout on the terminal, and `public-trackers-for-testing` links deleted files; the EPIC asks for such docs to change in the same PR

- PR number: 2956
- Source review ID: 5971038339
- Reviewer finding ID: F3
- Source URL: https://github.com/torrust/torrust-tracker/pull/2956#discussion_r4625668527
- Concern: Live skills run `tracker_client` with stdout on the terminal, and `public-trackers-for-testing` links deleted files; the EPIC asks for such docs to change in the same PR.
- Solution: Piped every invocation to `jq` in the six skills, added a refusal note to each, replaced the deleted file links.
- Current-tree verification: checked in the tree after the push of the round 2 commit; see the reply.
- Resolution reference: `docs(tracker-client): [#2768] fix the README example and update the skills and contract`
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2956#discussion_r4626110951

### F6 - The PR description's test counts and feature list were stale (9 tests, 71 crate tests, no panic hook)

- PR number: 2956
- Source review ID: 5971038339
- Reviewer finding ID: F4
- Source URL: https://github.com/torrust/torrust-tracker/pull/2956#discussion_r4625668537
- Concern: The PR description's test counts and feature list were stale (9 tests, 71 crate tests, no panic hook).
- Solution: Edited the PR description: 10 tests, 72 crate tests, panic record listed.
- Current-tree verification: checked in the tree after the push of the round 2 commit; see the reply.
- Resolution reference: `docs(tracker-client): [#2768] fix the README example and update the skills and contract`
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2956#discussion_r4626109994

### F7 - The contract asks for a schema version and migration note on a breaking change; `check` changed shape with a note only

- PR number: 2956
- Source review ID: 5971038339
- Reviewer finding ID: F5
- Source URL: https://github.com/torrust/torrust-tracker/pull/2956#discussion_r4625668547
- Concern: The contract asks for a schema version and migration note on a breaking change; `check` changed shape with a note only.
- Solution: Amended the contract's Stability text: no schema version exists yet, the breaking change is documented, a version is left to a later issue. No code change.
- Current-tree verification: checked in the tree after the push of the round 2 commit; see the reply.
- Resolution reference: `docs(tracker-client): [#2768] fix the README example and update the skills and contract`
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2956#discussion_r4626112745

### F8 - The module docs say nothing else writes to stdout or stderr, but the tracing subscriber does

- PR number: 2956
- Source review ID: 5971038339
- Reviewer finding ID: F6
- Source URL: https://github.com/torrust/torrust-tracker/pull/2956#discussion_r4625668555
- Concern: The module docs say nothing else writes to stdout or stderr, but the tracing subscriber does.
- Solution: The `output.rs` and `unified/mod.rs` docs name the tracing writer.
- Current-tree verification: checked in the tree after the push of the round 2 commit; see the reply.
- Resolution reference: `docs(tracker-client): [#2768] fix the README example and update the skills and contract`
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2956#discussion_r4626111389

### F9 - The `panic` record is not in the contract or the README's stderr list

- PR number: 2956
- Source review ID: 5971038339
- Reviewer finding ID: F7
- Source URL: https://github.com/torrust/torrust-tracker/pull/2956#discussion_r4625668563
- Concern: The `panic` record is not in the contract or the README's stderr list.
- Solution: Documented in the contract and the README.
- Current-tree verification: checked in the tree after the push of the round 2 commit; see the reply.
- Resolution reference: `docs(tracker-client): [#2768] fix the README example and update the skills and contract`
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2956#discussion_r4626109460

### F10 - The `rustix` dev-dependency has no recorded rationale

- PR number: 2956
- Source review ID: 5971038339
- Reviewer finding ID: F8
- Source URL: https://github.com/torrust/torrust-tracker/pull/2956#discussion_r4625668575
- Concern: The `rustix` dev-dependency has no recorded rationale.
- Solution: One sentence in the T1 row of the spec: why `rustix`, why the features, no new locked package.
- Current-tree verification: checked in the tree after the push of the round 2 commit; see the reply.
- Resolution reference: `docs(tracker-client): [#2768] fix the README example and update the skills and contract`
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2956#discussion_r4626111885

## Processing Log

- 2026-11-18 21:11 UTC - Started audit after the reply of round 1, not before it (late start): the record was written once the reply URL existed.
- 2026-11-18 21:11 UTC - Round 1: Copilot's review `5964711849` (submitted 2026-11-18T20:41:22Z, the poller cursor) with one finding. The fix was committed at 21:08:50Z (`git log --format=%cI`) and pushed (fork branch updated, `push_exit=0`). The reply was posted at 21:11:26Z (forge `created_at`), after the push; the thread was resolved by its thread ID, and a GraphQL refresh at 21:11:36Z (`date -u`) showed 0 unresolved threads. `validate-audit-record.py` was not run: it uses plain `gh` and needs the maintainer's permission.
- 2026-11-19 08:27 UTC - Round 2: Copilot's review `5970782901` (07:12:39Z, one body finding) and da2ce7's APPROVED review `5971038339` (07:37:22Z, eight inline findings), all on head `a4d2a220b`; the maintainer and da2ce7 had posted their ACK of that head already. One commit fixed all nine (`9e117792c`, pushed with `push_exit=0`); the PR description was edited; eight replies were posted, then the eight threads were resolved (GraphQL: 0 unresolved). The audit row for Copilot's F2 was written now, as the review had no thread. `validate-audit-record.py` was not run (it uses plain gh).

<!-- Copied verbatim into each audit record. -->

## Completion Rules

- Re-derive the reply claim against the current tree before replying or resolving a thread.
- Reply on every resolvable thread before resolving it.
- For an outdated thread whose concern was fixed, record `Disposition=FIXED` and
  `Thread state=RESOLVED`, even if GitHub marks the thread outdated after the push. For a
  duplicate, superseded, or no-change in-PR thread, reply exactly
  `Superseded by <FindingId>: <reason>.`, record `Disposition=NO_ACTION` and
  `Thread state=SUPERSEDED`, then resolve it. A post-merge `NO_ACTION` requires maintainer
  approval to decline the follow-up work. When the current identity cannot resolve a thread
  (`viewerCanResolve` is `false`), still reply, keep the disposition established from the current
  tree, and record `Thread state=OPEN`; the reviewer or PR author resolves it.
- A consolidated PR conversation response may cover multiple review rounds only when it names
  every review ID and every finding ID with its disposition and resolution reference. Record its
  durable URL in each related row.
- Cite a fix by its unique Conventional Commit subject or durable reply URL, never by a branch SHA
  that can change after a rebase.
- Refresh review threads using GraphQL and confirm that no unresolved actionable thread remains.
