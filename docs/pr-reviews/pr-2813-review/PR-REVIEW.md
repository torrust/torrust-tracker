---
pr-number: 2813
pr-url: https://github.com/torrust/torrust-tracker/pull/2813
last-updated-utc: "2026-11-10 21:50"
---

# PR #2813 Review Audit

Source: pull-request reviews and inline review threads for https://github.com/torrust/torrust-tracker/pull/2813.

## Ownership

The PR author owns this tracked audit record. Reviewers, including repository review agents,
deliver findings through GitHub and have no repository-artifact obligation.

- Post-merge workflow approval: N/A

## Status Values

- Relationship: `ORIGINAL`, `RE_RAISE_OF:<FindingId>`
- Disposition: `FIXED`, `NO_ACTION`, `SUPERSEDED`, `FOLLOW_UP`
- Thread state: `OPEN`, `RESOLVED`, `NON_RESOLVABLE`, `SUPERSEDED`
- Severity: `Blocker`, `Major`, `Minor`, `Nit`, `Suggestion`; append `(inferred)` when derived
  from free prose.
- Author class: `Copilot`, `Human`, `Unknown`
- Category: `link-integrity`, `formatting`, `metadata`, `testing`, `correctness`,
  `documentation`, `maintainability`, `security`, `other`

## Findings

Round 1: Copilot review 5846895000 (lite review: the review body reports that the full agentic review timed out; five inline findings) and da2ce7 review 5846910258 (CHANGES_REQUESTED, eight findings) on the head `024eb565a`. All thirteen are valid and fixed in one batch. Thread state is OPEN for all: the maintainer or reviewer resolves them (the bot resolveReviewThread mutation was not attempted, as it was denied in earlier audits).

| Finding ID | Review finding reference | Author class | Severity | Category | Relationship | Disposition | Thread state |
| ---------- | ------------------------ | ------------ | -------- | -------- | ------------ | ----------- | ------------ |
| F1 | review-finding:pr-2813-f1 | Copilot | Suggestion (inferred) | testing | ORIGINAL | FIXED | OPEN |
| F2 | review-finding:pr-2813-f2 | Copilot | Suggestion (inferred) | testing | ORIGINAL | FIXED | OPEN |
| F3 | review-finding:pr-2813-f3 | Copilot | Suggestion (inferred) | security | ORIGINAL | FIXED | OPEN |
| F4 | review-finding:pr-2813-f4 | Copilot | Suggestion (inferred) | testing | ORIGINAL | FIXED | OPEN |
| F5 | review-finding:pr-2813-f5 | Copilot | Suggestion (inferred) | maintainability | ORIGINAL | FIXED | OPEN |
| F6 | review-finding:pr-2813-f6 | Human | Major | correctness | ORIGINAL | FIXED | OPEN |
| F7 | review-finding:pr-2813-f7 | Human | Minor | documentation | ORIGINAL | FIXED | OPEN |
| F8 | review-finding:pr-2813-f8 | Human | Minor | documentation | ORIGINAL | FIXED | OPEN |
| F9 | review-finding:pr-2813-f9 | Human | Minor | maintainability | ORIGINAL | FIXED | OPEN |
| F10 | review-finding:pr-2813-f10 | Human | Minor | documentation | ORIGINAL | FIXED | OPEN |
| F11 | review-finding:pr-2813-f11 | Human | Minor | documentation | ORIGINAL | FIXED | OPEN |
| F12 | review-finding:pr-2813-f12 | Human | Minor | testing | ORIGINAL | FIXED | OPEN |
| F13 | review-finding:pr-2813-f13 | Human | Nit | documentation | ORIGINAL | FIXED | OPEN |

## Finding Details

### F1 - run-probes.sh can leave the tracker running if a probe fails (no set -e, no trap).

- PR number: 2813
- Source review ID: 5846895000
- Reviewer finding ID: N/A
- Source URL: https://github.com/torrust/torrust-tracker/pull/2813#discussion_r4528624843
- Concern: run-probes.sh can leave the tracker running if a probe fails (no set -e, no trap).
- Solution: Valid. set -eu and a trap that stops the tracker on any exit.
- Current-tree verification: sed -n 1,20p evidence/run-probes.sh shows the set -eu line and the trap.
- Resolution reference: fix(axum-server): [#2776] address PR review round 1
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2813#discussion_r4530833427

### F2 - The health check test can leak the server task if the assertion panics before the token is cancelled.

- PR number: 2813
- Source review ID: 5846895000
- Reviewer finding ID: N/A
- Source URL: https://github.com/torrust/torrust-tracker/pull/2813#discussion_r4528624945
- Concern: The health check test can leak the server task if the assertion panics before the token is cancelled.
- Solution: Valid. A drop_guard on the cancellation token cancels the server on any exit.
- Current-tree verification: grep -n drop_guard packages/axum-health-check-api-server/src/server.rs.
- Resolution reference: fix(axum-server): [#2776] address PR review round 1
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2813#discussion_r4530833818

### F3 - A private key is committed (test-only); document it or generate certificates at run time.

- PR number: 2813
- Source review ID: 5846895000
- Reviewer finding ID: N/A
- Source URL: https://github.com/torrust/torrust-tracker/pull/2813#discussion_r4528625018
- Concern: A private key is committed (test-only); document it or generate certificates at run time.
- Solution: Valid. README beside the fixtures states it is a public self-signed test key; the files stay because they are byte-identical to the REST API package fixtures.
- Current-tree verification: packages/axum-server/tests/fixtures/README.md exists.
- Resolution reference: fix(axum-server): [#2776] address PR review round 1
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2813#discussion_r4530834184

### F4 - 300 ms timeouts and a 3 s deadline may be flaky on slow CI.

- PR number: 2813
- Source review ID: 5846895000
- Reviewer finding ID: N/A
- Source URL: https://github.com/torrust/torrust-tracker/pull/2813#discussion_r4528625090
- Concern: 300 ms timeouts and a 3 s deadline may be flaky on slow CI.
- Solution: Valid. 500 ms injected timeout, 5 s test deadline.
- Current-tree verification: grep -n "const SHORT\|const TEST_DEADLINE" packages/axum-server/tests/idle_connections.rs.
- Resolution reference: fix(axum-server): [#2776] address PR review round 1
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2813#discussion_r4530834621

### F5 - from_tcp_with_timeouts reads as if it takes timeouts; rename or document.

- PR number: 2813
- Source review ID: 5846895000
- Reviewer finding ID: N/A
- Source URL: https://github.com/torrust/torrust-tracker/pull/2813#discussion_r4528625143
- Concern: from_tcp_with_timeouts reads as if it takes timeouts; rename or document.
- Solution: Not renamed (public, used by three packages); doc notes on both default constructors point at the explicit ones.
- Current-tree verification: Doc comments of from_tcp_with_timeouts and from_tcp_rustls_with_timeouts.
- Resolution reference: fix(axum-server): [#2776] address PR review round 1
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2813#discussion_r4530834950

### F6 - V8 claims the wrapper left idle HTTP/2 open 10 s, which the removed code contradicts; evidence cannot show otherwise.

- PR number: 2813
- Source review ID: 5846910258
- Reviewer finding ID: F1
- Source URL: https://github.com/torrust/torrust-tracker/pull/2813#discussion_r4528636463
- Concern: V8 claims the wrapper left idle HTTP/2 open 10 s, which the removed code contradicts; evidence cannot show otherwise.
- Solution: Reran the probes on the build with the wrapper, named by subject, raw output kept: 10.0 s. Probable cause recorded. Removal stands; AC4 and AC5 reopened (F10).
- Current-tree verification: evidence/results-with-wrapper.txt and manual-verification-evidence.md V8.
- Resolution reference: fix(axum-server): [#2776] address PR review round 1
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2813#discussion_r4530835288

### F7 - The PR-branch commit id is cited in the evidence and progress log.

- PR number: 2813
- Source review ID: 5846910258
- Reviewer finding ID: F2
- Source URL: https://github.com/torrust/torrust-tracker/pull/2813#discussion_r4528636470
- Concern: The PR-branch commit id is cited in the evidence and progress log.
- Solution: Dropped; builds are named by Conventional Commit subject.
- Current-tree verification: grep -rn 167841cd3 docs/issues/open/2776-2411-axum-server-0-9-timeouts returns nothing.
- Resolution reference: fix(axum-server): [#2776] address PR review round 1
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2813#discussion_r4530835616

### F8 - EPIC citations of new lines have no stated basis while other rows resolve at an older commit.

- PR number: 2813
- Source review ID: 5846910258
- Reviewer finding ID: F3
- Source URL: https://github.com/torrust/torrust-tracker/pull/2813#discussion_r4528636481
- Concern: EPIC citations of new lines have no stated basis while other rows resolve at an older commit.
- Solution: The new A2 note and controls rows cite symbols instead of lines.
- Current-tree verification: grep -n "custom_axum_server.rs:" docs/issues/open/2411-spam-and-abuse-resistance/EPIC.md returns nothing.
- Resolution reference: fix(axum-server): [#2776] address PR review round 1
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2813#discussion_r4530835972

### F9 - The chore(deps) commit downgrades two unrelated dependency edges (tempfile and another) in Cargo.lock; axum-server was already 0.9.0.

- PR number: 2813
- Source review ID: 5846910258
- Reviewer finding ID: F4
- Source URL: https://github.com/torrust/torrust-tracker/pull/2813#discussion_r4528636490
- Concern: The chore(deps) commit downgrades two unrelated dependency edges (tempfile and another) in Cargo.lock; axum-server was already 0.9.0.
- Solution: Restored develop resolution for both edges; description remark corrected.
- Current-tree verification: git diff torrust/develop -- Cargo.lock shows only the axum-server package dependency list.
- Resolution reference: fix(axum-server): [#2776] address PR review round 1
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2813#discussion_r4530836317

### F10 - AC4 is ticked although HTTP/2 measures 10 s, and "already 10 s today" was not measured.

- PR number: 2813
- Source review ID: 5846910258
- Reviewer finding ID: F5
- Source URL: https://github.com/torrust/torrust-tracker/pull/2813#discussion_r4528636497
- Concern: AC4 is ticked although HTTP/2 measures 10 s, and "already 10 s today" was not measured.
- Solution: AC4 and AC5 unticked; the claim dropped and the missing develop measurement stated.
- Current-tree verification: grep -n "AC4\|AC5" docs/issues/open/2776-2411-axum-server-0-9-timeouts/ISSUE.md.
- Resolution reference: fix(axum-server): [#2776] address PR review round 1
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2813#discussion_r4530836683

### F11 - Red evidence has no command or real output; green run not recorded; V8 and V9 lack date and toolchain; Purpose is stale.

- PR number: 2813
- Source review ID: 5846910258
- Reviewer finding ID: F6
- Source URL: https://github.com/torrust/torrust-tracker/pull/2813#discussion_r4528636507
- Concern: Red evidence has no command or real output; green run not recorded; V8 and V9 lack date and toolchain; Purpose is stale.
- Solution: red-run.txt and green-run.txt hold commands, toolchain and real output; V8, V9 and Purpose updated.
- Current-tree verification: evidence/red-run.txt, evidence/green-run.txt.
- Resolution reference: fix(axum-server): [#2776] address PR review round 1
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2813#discussion_r4530837074

### F12 - HTTP/2 tests never check a response arrived, and an immediate close passes every close test.

- PR number: 2813
- Source review ID: 5846910258
- Reviewer finding ID: F7
- Source URL: https://github.com/torrust/torrust-tracker/pull/2813#discussion_r4528636514
- Concern: HTTP/2 tests never check a response arrived, and an immediate close passes every close test.
- Solution: Tests read a HEADERS frame on stream 1 (fail on GOAWAY) and assert the close is no earlier than the injected timeout.
- Current-tree verification: grep -n "read_http2_response_headers\|assert_closed_by_timeout" packages/axum-server/tests/idle_connections.rs.
- Resolution reference: fix(axum-server): [#2776] address PR review round 1
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2813#discussion_r4530837414

### F13 - A working note was left in probe-h2.py.

- PR number: 2813
- Source review ID: 5846910258
- Reviewer finding ID: F8
- Source URL: https://github.com/torrust/torrust-tracker/pull/2813#discussion_r4528636523
- Concern: A working note was left in probe-h2.py.
- Solution: Replaced by a statement that the request is GET /.
- Current-tree verification: sed -n 8,14p evidence/probe-h2.py.
- Resolution reference: fix(axum-server): [#2776] address PR review round 1
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2813#discussion_r4530837803

## Processing Log

- 2026-11-10 21:50 UTC - Round 1 processed: thirteen findings, all fixed in one batch, all threads replied; threads left OPEN. The audit and the review retrospective go in the same push as the fixes.

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
