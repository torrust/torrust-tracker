---
pr-number: 2813
pr-url: https://github.com/torrust/torrust-tracker/pull/2813
last-updated-utc: "2026-11-11 11:07"
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

Round 1: Copilot review 5846895000 (lite review: the review body reports that the full agentic review timed out; five inline findings) and da2ce7 review 5846910258 (CHANGES_REQUESTED, eight findings) on the head `docs(issues): [#2776] record evidence, decisions and EPIC #2411 case A2 update`. All thirteen are valid and fixed in one batch. Thread state is OPEN for all: the maintainer or reviewer resolves them (the bot resolveReviewThread mutation was not attempted, as it was denied in earlier audits).

Round 2: da2ce7 reviews 5850237912, 5856329614, 5856670353, 5856843868, 5857093258 and 5857566978 on later heads (reviewer findings F9 to F29, here F14 to F34; three re-posts of their F21 and one of their F25 are F35 to F37). All are valid and fixed in one batch; thread state is OPEN for all.

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
| F14 | review-finding:pr-2813-f14 | Human | Minor | documentation | ORIGINAL | FIXED | OPEN |
| F15 | review-finding:pr-2813-f15 | Human | Minor | documentation | ORIGINAL | FIXED | OPEN |
| F16 | review-finding:pr-2813-f16 | Human | Nit | documentation | ORIGINAL | FIXED | OPEN |
| F17 | review-finding:pr-2813-f17 | Human | Nit | documentation | ORIGINAL | FIXED | OPEN |
| F18 | review-finding:pr-2813-f18 | Human | Nit | documentation | ORIGINAL | FIXED | OPEN |
| F19 | review-finding:pr-2813-f19 | Human | Nit | formatting | ORIGINAL | FIXED | OPEN |
| F20 | review-finding:pr-2813-f20 | Human | Nit | documentation | ORIGINAL | FIXED | OPEN |
| F21 | review-finding:pr-2813-f21 | Human | Minor | documentation | ORIGINAL | FIXED | OPEN |
| F22 | review-finding:pr-2813-f22 | Human | Minor | documentation | ORIGINAL | FIXED | OPEN |
| F23 | review-finding:pr-2813-f23 | Human | Minor | correctness | ORIGINAL | FIXED | OPEN |
| F24 | review-finding:pr-2813-f24 | Human | Nit | documentation | ORIGINAL | FIXED | OPEN |
| F25 | review-finding:pr-2813-f25 | Human | Nit | documentation | ORIGINAL | FIXED | OPEN |
| F26 | review-finding:pr-2813-f26 | Human | Major | correctness | ORIGINAL | FIXED | OPEN |
| F27 | review-finding:pr-2813-f27 | Human | Minor | correctness | ORIGINAL | FIXED | OPEN |
| F28 | review-finding:pr-2813-f28 | Human | Minor | maintainability | ORIGINAL | FIXED | OPEN |
| F29 | review-finding:pr-2813-f29 | Human | Minor | documentation | ORIGINAL | FIXED | OPEN |
| F30 | review-finding:pr-2813-f30 | Human | Nit | documentation | ORIGINAL | FIXED | OPEN |
| F31 | review-finding:pr-2813-f31 | Human | Nit | documentation | ORIGINAL | FIXED | OPEN |
| F32 | review-finding:pr-2813-f32 | Human | Nit | maintainability | ORIGINAL | FIXED | OPEN |
| F33 | review-finding:pr-2813-f33 | Human | Minor | documentation | ORIGINAL | FIXED | OPEN |
| F34 | review-finding:pr-2813-f34 | Human | Suggestion | security | ORIGINAL | FIXED | OPEN |
| F35 | review-finding:pr-2813-f35 | Human | Major | correctness | RE_RAISE_OF:F26 | FIXED | OPEN |
| F36 | review-finding:pr-2813-f36 | Human | Major | correctness | RE_RAISE_OF:F26 | FIXED | OPEN |
| F37 | review-finding:pr-2813-f37 | Human | Nit | documentation | RE_RAISE_OF:F30 | FIXED | OPEN |

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
- Current-tree verification: the commit-id check prints nothing for docs/issues/closed/2776-2411-axum-server-0-9-timeouts.
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
- Current-tree verification: grep -n "AC4\|AC5" docs/issues/closed/2776-2411-axum-server-0-9-timeouts/ISSUE.md.
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

### F14 - The audit cites PR-branch commit ids (the reviewed head and a refactor commit).

- PR number: 2813
- Source review ID: 5850237912
- Reviewer finding ID: F9
- Source URL: https://github.com/torrust/torrust-tracker/pull/2813#discussion_r4531153163
- Concern: The audit cites PR-branch commit ids (the reviewed head and a refactor commit).
- Solution: Names the head by the subject of its last commit; the check is stated as "prints nothing".
- Current-tree verification: grep -n "024eb565a\|167841cd3" docs/pr-reviews/pr-2813-review/PR-REVIEW.md returns nothing.
- Resolution reference: fix(axum-server): [#2776] address PR review round 2
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2813#discussion_r4537116320

### F15 - The round 1 processing entry is stamped later than its commit and describes a push still to come.

- PR number: 2813
- Source review ID: 5850237912
- Reviewer finding ID: F10
- Source URL: https://github.com/torrust/torrust-tracker/pull/2813#discussion_r4531153188
- Concern: The round 1 processing entry is stamped later than its commit and describes a push still to come.
- Solution: Stamped from the replies (21:36 UTC); says they preceded the push; last sentence dropped.
- Current-tree verification: Processing Log, first entry.
- Resolution reference: fix(axum-server): [#2776] address PR review round 2
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2813#discussion_r4537110198

### F16 - The spec log says 14 findings; round 1 had thirteen.

- PR number: 2813
- Source review ID: 5850237912
- Reviewer finding ID: F11
- Source URL: https://github.com/torrust/torrust-tracker/pull/2813#discussion_r4531153207
- Concern: The spec log says 14 findings; round 1 had thirteen.
- Solution: Corrected to 13.
- Current-tree verification: grep -n "13 findings" docs/issues/closed/2776-2411-axum-server-0-9-timeouts/ISSUE.md.
- Resolution reference: fix(axum-server): [#2776] address PR review round 2
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2813#discussion_r4537113016

### F17 - The EPIC note says add_timeouts sets the TLS handshake timeout; from_tcp_rustls_with does.

- PR number: 2813
- Source review ID: 5850237912
- Reviewer finding ID: F12
- Source URL: https://github.com/torrust/torrust-tracker/pull/2813#discussion_r4531153217
- Concern: The EPIC note says add_timeouts sets the TLS handshake timeout; from_tcp_rustls_with does.
- Solution: The note names from_tcp_rustls_with for the handshake bound.
- Current-tree verification: grep -n "from_tcp_rustls_with. also sets" docs/issues/open/2411-spam-and-abuse-resistance/EPIC.md.
- Resolution reference: fix(axum-server): [#2776] address PR review round 2
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2813#discussion_r4537111662

### F18 - green-run.txt reads as raw output but is an excerpt.

- PR number: 2813
- Source review ID: 5850237912
- Reviewer finding ID: F13
- Source URL: https://github.com/torrust/torrust-tracker/pull/2813#discussion_r4531153229
- Concern: green-run.txt reads as raw output but is an excerpt.
- Solution: Header says it is filtered and by what; every target header kept.
- Current-tree verification: head -4 evidence/green-run.txt.
- Resolution reference: fix(axum-server): [#2776] address PR review round 2
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2813#discussion_r4537115607

### F19 - run-probes.sh ends with a blank line.

- PR number: 2813
- Source review ID: 5850237912
- Reviewer finding ID: F14
- Source URL: https://github.com/torrust/torrust-tracker/pull/2813#discussion_r4531153244
- Concern: run-probes.sh ends with a blank line.
- Solution: Removed.
- Current-tree verification: git diff --check reports nothing.
- Resolution reference: fix(axum-server): [#2776] address PR review round 2
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2813#discussion_r4537116673

### F20 - The description says the wrapper never fired for HTTP/2; the runs show it did not shorten it.

- PR number: 2813
- Source review ID: 5850237912
- Reviewer finding ID: F15
- Source URL: https://github.com/torrust/torrust-tracker/pull/2813#discussion_r4531153255
- Concern: The description says the wrapper never fired for HTTP/2; the runs show it did not shorten it.
- Solution: The description now says "did not shorten".
- Current-tree verification: PR description, "What changed".
- Resolution reference: fix(axum-server): [#2776] address PR review round 2
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2813#discussion_r4537114451

### F21 - AC4 says "no worse than before the change" but only the wrapper build (V8) was measured; M4 still says 5 s.

- PR number: 2813
- Source review ID: 5856329614
- Reviewer finding ID: F16
- Source URL: https://github.com/torrust/torrust-tracker/pull/2813#discussion_r4535969665
- Concern: AC4 says "no worse than before the change" but only the wrapper build (V8) was measured; M4 still says 5 s.
- Solution: AC4 says "no worse than with the wrapper (V8)"; M4 gives AC4 figures.
- Current-tree verification: grep -n "no worse than with the wrapper" docs/issues/closed/2776-2411-axum-server-0-9-timeouts/ISSUE.md.
- Resolution reference: fix(axum-server): [#2776] address PR review round 2
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2813#discussion_r4537118022

### F22 - The log entry records a maintainer ruling with reasons that nothing on the pull request carries.

- PR number: 2813
- Source review ID: 5856329614
- Reviewer finding ID: F17
- Source URL: https://github.com/torrust/torrust-tracker/pull/2813#discussion_r4535969670
- Concern: The log entry records a maintainer ruling with reasons that nothing on the pull request carries.
- Solution: The entry says the ACK is taken as the ruling and labels the rationale as the author's recommendation.
- Current-tree verification: Progress Log entry of 2026-11-11 08:53 UTC.
- Resolution reference: fix(axum-server): [#2776] address PR review round 2
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2813#discussion_r4537112066

### F23 - The branch conflicts with develop in three server.rs files.

- PR number: 2813
- Source review ID: 5856329614
- Reviewer finding ID: F18
- Source URL: https://github.com/torrust/torrust-tracker/pull/2813#discussion_r4535969679
- Concern: The branch conflicts with develop in three server.rs files.
- Solution: Merged develop into the branch; removals kept, import without TimeoutAcceptor.
- Current-tree verification: git merge-tree --write-tree HEAD torrust/develop exits 0.
- Resolution reference: fix(axum-server): [#2776] address PR review round 2
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2813#discussion_r4537112386

### F24 - The evidence quotes the old AC4 wording and says the 5 s figure is not met.

- PR number: 2813
- Source review ID: 5856329614
- Reviewer finding ID: F19
- Source URL: https://github.com/torrust/torrust-tracker/pull/2813#discussion_r4535969684
- Concern: The evidence quotes the old AC4 wording and says the 5 s figure is not met.
- Solution: Paragraph rewritten in the past tense with the reworded AC4.
- Current-tree verification: manual-verification-evidence.md, V8 conclusion.
- Resolution reference: fix(axum-server): [#2776] address PR review round 2
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2813#discussion_r4537114148

### F25 - Stamps later than the commit that carries them (09:00 against 08:53).

- PR number: 2813
- Source review ID: 5856329614
- Reviewer finding ID: F20
- Source URL: https://github.com/torrust/torrust-tracker/pull/2813#discussion_r4535969694
- Concern: Stamps later than the commit that carries them (09:00 against 08:53).
- Solution: Stamps use the commit time.
- Current-tree verification: Progress Log, 2026-11-11 08:53 UTC.
- Resolution reference: fix(axum-server): [#2776] address PR review round 2
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2813#discussion_r4537117039

### F26 - The README says about 10 s over HTTP/2 without the condition: a client that answers pings is not bounded.

- PR number: 2813
- Source review ID: 5856670353
- Reviewer finding ID: F21
- Source URL: https://github.com/torrust/torrust-tracker/pull/2813#discussion_r4536237393
- Concern: The README says about 10 s over HTTP/2 without the condition: a client that answers pings is not bounded.
- Solution: The README states the condition and the A1 limit.
- Current-tree verification: grep -n "answers them" packages/axum-server/README.md.
- Resolution reference: fix(axum-server): [#2776] address PR review round 2
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2813#discussion_r4537110644

### F27 - README and Cargo.toml conflicts with develop; the two introductions must become one.

- PR number: 2813
- Source review ID: 5856670353
- Reviewer finding ID: F22
- Source URL: https://github.com/torrust/torrust-tracker/pull/2813#discussion_r4536237399
- Concern: README and Cargo.toml conflicts with develop; the two introductions must become one.
- Solution: Resolved by the merge of develop into the branch.
- Current-tree verification: git merge-tree reports a clean tree.
- Resolution reference: fix(axum-server): [#2776] address PR review round 2
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2813#discussion_r4537111040

### F28 - The placement ADR requires root placement for protocol behaviour; the new ADR is package-local.

- PR number: 2813
- Source review ID: 5857093258
- Reviewer finding ID: F23
- Source URL: https://github.com/torrust/torrust-tracker/pull/2813#discussion_r4536572037
- Concern: The placement ADR requires root placement for protocol behaviour; the new ADR is package-local.
- Solution: Moved to docs/adrs/ with a root index row and a repository-level Scope; the local collection removed.
- Current-tree verification: ls docs/adrs/20261111095222_*; no packages/axum-server/docs/adrs.
- Resolution reference: fix(axum-server): [#2776] address PR review round 2
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2813#discussion_r4537115213

### F29 - The log attributes a decision to a maintainer discussion that is not on the pull request.

- PR number: 2813
- Source review ID: 5857093258
- Reviewer finding ID: F24
- Source URL: https://github.com/torrust/torrust-tracker/pull/2813#discussion_r4536572049
- Concern: The log attributes a decision to a maintainer discussion that is not on the pull request.
- Solution: Attribution dropped; the ADR is accepted by this review.
- Current-tree verification: Progress Log, 2026-11-11 09:53 UTC.
- Resolution reference: fix(axum-server): [#2776] address PR review round 2
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2813#discussion_r4537112724

### F30 - Stamp slip again (10:00 against 09:53) and #2801 named where #2807 and #2812 conflicted.

- PR number: 2813
- Source review ID: 5857093258
- Reviewer finding ID: F25
- Source URL: https://github.com/torrust/torrust-tracker/pull/2813#discussion_r4536572064
- Concern: Stamp slip again (10:00 against 09:53) and #2801 named where #2807 and #2812 conflicted.
- Solution: Commit time used; the entry names #2807 and #2812.
- Current-tree verification: Progress Log, 2026-11-11 09:53 UTC.
- Resolution reference: fix(axum-server): [#2776] address PR review round 2
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2813#discussion_r4537113420

### F31 - The ADRs line still reads as an open question about a custom acceptor; the description lags.

- PR number: 2813
- Source review ID: 5857093258
- Reviewer finding ID: F26
- Source URL: https://github.com/torrust/torrust-tracker/pull/2813#discussion_r4536572081
- Concern: The ADRs line still reads as an open question about a custom acceptor; the description lags.
- Solution: Line keeps only the configuration-keys condition; description updated.
- Current-tree verification: ISSUE.md "Architectural Decisions".
- Resolution reference: fix(axum-server): [#2776] address PR review round 2
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2813#discussion_r4537113799

### F32 - "Only custom_axum_server.rs allows them" is false: signals.rs also allows bind.

- PR number: 2813
- Source review ID: 5857566978
- Reviewer finding ID: F27
- Source URL: https://github.com/torrust/torrust-tracker/pull/2813#discussion_r4536956026
- Concern: "Only custom_axum_server.rs allows them" is false: signals.rs also allows bind.
- Solution: The signals.rs test listener uses the shared constructor and its allow is removed.
- Current-tree verification: grep -rn "disallowed_methods" packages --include=*.rs shows only custom_axum_server.rs.
- Resolution reference: fix(axum-server): [#2776] address PR review round 2
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2813#discussion_r4537111392

### F33 - The log attributes a request to the maintainer and reports a red run with no output; "doc comments"; omits the signals.rs allow.

- PR number: 2813
- Source review ID: 5857566978
- Reviewer finding ID: F28
- Source URL: https://github.com/torrust/torrust-tracker/pull/2813#discussion_r4536956013
- Concern: The log attributes a request to the maintainer and reports a red run with no output; "doc comments"; omits the signals.rs allow.
- Solution: Attribution dropped; red run recorded in evidence/red-run-service-tests.txt; wording fixed.
- Current-tree verification: evidence/red-run-service-tests.txt.
- Resolution reference: fix(axum-server): [#2776] address PR review round 2
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2813#discussion_r4537117384

### F34 - clippy.toml does not forbid axum::serve; paths should be checked against the 0.9 source.

- PR number: 2813
- Source review ID: 5857566978
- Reviewer finding ID: F29
- Source URL: https://github.com/torrust/torrust-tracker/pull/2813#discussion_r4536956042
- Concern: clippy.toml does not forbid axum::serve; paths should be checked against the 0.9 source.
- Solution: axum::serve and the two unix constructors added; every path checked against axum-server 0.9.2 and accepted by clippy.
- Current-tree verification: cat clippy.toml; cargo clippy --workspace --all-targets -- -D warnings passes.
- Resolution reference: fix(axum-server): [#2776] address PR review round 2
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2813#discussion_r4537118355

### F35 - The README HTTP/2 sentence still lacks the condition after the merge.

- PR number: 2813
- Source review ID: 5856843868
- Reviewer finding ID: F21 (again)
- Source URL: https://github.com/torrust/torrust-tracker/pull/2813#discussion_r4536372527
- Concern: The README HTTP/2 sentence still lacks the condition after the merge.
- Solution: Same fix as F26.
- Current-tree verification: grep -n "answers them" packages/axum-server/README.md.
- Resolution reference: fix(axum-server): [#2776] address PR review round 2
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2813#discussion_r4537114777

### F36 - The README HTTP/2 sentence still lacks the condition after the ADR commit.

- PR number: 2813
- Source review ID: 5857093258
- Reviewer finding ID: F21 (again)
- Source URL: https://github.com/torrust/torrust-tracker/pull/2813#discussion_r4536572023
- Concern: The README HTTP/2 sentence still lacks the condition after the ADR commit.
- Solution: Same fix as F26.
- Current-tree verification: grep -n "answers them" packages/axum-server/README.md.
- Resolution reference: fix(axum-server): [#2776] address PR review round 2
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2813#discussion_r4537117714

### F37 - Stamp slip recurs (11:00 against 10:29).

- PR number: 2813
- Source review ID: 5857566978
- Reviewer finding ID: F25 (again)
- Source URL: https://github.com/torrust/torrust-tracker/pull/2813#discussion_r4536956033
- Concern: Stamp slip recurs (11:00 against 10:29).
- Solution: Commit time used.
- Current-tree verification: Progress Log, 2026-11-11 10:29 UTC.
- Resolution reference: fix(axum-server): [#2776] address PR review round 2
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2813#discussion_r4537115956

## Processing Log

- 2026-11-10 21:36 UTC - Round 1 processed: the thirteen threads were replied to between 21:36:25Z and 21:36:53Z, before the fixes were pushed, and all are left OPEN.
- 2026-11-11 11:06 UTC - Round 2 processed: twenty-four threads (reviewer findings F9 to F29, including three re-posts of F21 and one of F25, as F14 to F37) replied to between 11:06:03Z and 11:06:58Z, fixes in the commit named in each row, all threads left OPEN.

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
