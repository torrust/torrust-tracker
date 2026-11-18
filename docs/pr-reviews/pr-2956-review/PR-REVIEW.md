---
pr-number: 2956
pr-url: https://github.com/torrust/torrust-tracker/pull/2956
last-updated-utc: "2026-11-18 21:11"
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

| Finding ID | Review finding reference | Author class | Severity | Category | Relationship | Disposition | Thread state |
| ---------- | ------------------------ | ------------ | -------- | -------- | ------------ | ----------- | ------------ |
| F1 | `review-finding:pr-2956-f1` | Copilot | Major | correctness | ORIGINAL | FIXED | RESOLVED |

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

## Processing Log

- 2026-11-18 21:11 UTC - Started audit after the reply of round 1, not before it (late start): the record was written once the reply URL existed.
- 2026-11-18 21:11 UTC - Round 1: Copilot's review `5964711849` (submitted 2026-11-18T20:41:22Z, the poller cursor) with one finding. The fix was committed at 21:08:50Z (`git log --format=%cI`) and pushed (fork branch updated, `push_exit=0`). The reply was posted at 21:11:26Z (forge `created_at`), after the push; the thread was resolved by its thread ID, and a GraphQL refresh at 21:11:36Z (`date -u`) showed 0 unresolved threads. `validate-audit-record.py` was not run: it uses plain `gh` and needs the maintainer's permission.

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
