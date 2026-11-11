---
pr-number: 2829
pr-url: https://github.com/torrust/torrust-tracker/pull/2829
last-updated-utc: "2026-11-11 12:23"
---

# PR #2829 Review Audit

Source: pull-request reviews and inline review threads for https://github.com/torrust/torrust-tracker/pull/2829.

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

Round 1: da2ce7 review 5858533435 (CHANGES_REQUESTED, F1 to F6) and Copilot review 5858573361 (two inline findings, both labelled R1 by the reviewer: the first keeps R1, the second is F7 with the reviewer ID R1). All eight are valid and fixed.

| Finding ID | Review finding reference | Author class | Severity | Category | Relationship | Disposition | Thread state |
| ---------- | ------------------------ | ------------ | -------- | -------- | ------------ | ----------- | ------------ |
| F1 | review-finding:pr-2829-f1 | Human | Major | correctness | ORIGINAL | FIXED | RESOLVED |
| F2 | review-finding:pr-2829-f2 | Human | Minor | documentation | ORIGINAL | FIXED | RESOLVED |
| F3 | review-finding:pr-2829-f3 | Human | Minor | correctness | ORIGINAL | FIXED | RESOLVED |
| F4 | review-finding:pr-2829-f4 | Human | Nit | documentation | ORIGINAL | FIXED | RESOLVED |
| F5 | review-finding:pr-2829-f5 | Human | Nit | metadata | ORIGINAL | FIXED | RESOLVED |
| F6 | review-finding:pr-2829-f6 | Human | Suggestion | documentation | ORIGINAL | FIXED | RESOLVED |
| R1 | review-finding:pr-2829-r1 | Copilot | Minor | correctness | RE_RAISE_OF:F3 | FIXED | RESOLVED |
| F7 | review-finding:pr-2829-f7 | Copilot | Minor | correctness | ORIGINAL | FIXED | RESOLVED |

## Finding Details

### F1 - The develop head the review cites is not on develop

- PR number: 2829
- Source review ID: 5858533435
- Reviewer finding ID: N/A
- Source URL: https://github.com/torrust/torrust-tracker/pull/2829#discussion_r4537729385
- Concern: Line 1249 cited `850e95e35`, the 2024 release merge, which is not an ancestor of develop.
- Solution: Valid. The id came from stale context and was not checked. It now cites the merge base `f8b0f26e9`.
- Current-tree verification: `git merge-base --is-ancestor f8b0f26e9 HEAD` succeeds; `git merge-base HEAD torrust/develop` prints f8b0f26e9.
- Resolution reference: docs(issues): [#1669] cite the develop commit the coupling review used
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2829#discussion_r4537787464

### F2 - Row 4 omits five normal edges with one to three imports

- PR number: 2829
- Source review ID: 5858533435
- Reviewer finding ID: N/A
- Source URL: https://github.com/torrust/torrust-tracker/pull/2829#discussion_r4537729393
- Concern: Row 4 read as the complete set but left out five edges.
- Solution: Valid. The row says "including" and lists the five missing edges.
- Current-tree verification: Compared the row with the import counts of the report (edges with 1-3 imports).
- Resolution reference: docs(issues): [#1669] list every thin normal edge in coupling finding 4
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2829#discussion_r4537787765

### F3 - The transitive-weight claim holds for udp-server, not for axum-rest-api-server

- PR number: 2829
- Source review ID: 5858533435
- Reviewer finding ID: N/A
- Source URL: https://github.com/torrust/torrust-tracker/pull/2829#discussion_r4537729398
- Concern: http-protocol and reqwest/hyper already reach the downstream packages by other paths.
- Solution: Valid. The claim is scoped to udp-server; downstream the edge adds only client-lib.
- Current-tree verification: `cargo tree -p torrust-tracker-axum-rest-api-server -e normal` lists reqwest and hyper as direct dependencies; http-protocol arrives through http-core.
- Resolution reference: docs(issues): [#1669] scope the transitive weight claim to udp-server
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2829#discussion_r4537788113

### F4 - "earlier handoffs" names nothing a reader can find

- PR number: 2829
- Source review ID: 5858533435
- Reviewer finding ID: N/A
- Source URL: https://github.com/torrust/torrust-tracker/pull/2829#discussion_r4537729404
- Concern: The sentence cited no findable source.
- Solution: Valid. It now names DEC-11 and the EPIC known smells.
- Current-tree verification: Read the New findings introduction after the edit.
- Resolution reference: docs(issues): [#1669] name the sources of the two verified claims
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2829#discussion_r4537788522

### F5 - The description has no Related to link and no validation item

- PR number: 2829
- Source review ID: 5858533435
- Reviewer finding ID: N/A
- Source URL: https://github.com/torrust/torrust-tracker/pull/2829#discussion_r4537729409
- Concern: The PR description lacked `Related to #N` and a validation line.
- Solution: Valid. The description was edited; the fix is outside the tree.
- Current-tree verification: `gh pr view 2829` body shows `Related to #1669` and the validation line.
- Resolution reference: https://github.com/torrust/torrust-tracker/pull/2829#issuecomment-6795666697
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2829#discussion_r4537810510

### F6 - Track the DEC-11 recommendation

- PR number: 2829
- Source review ID: 5858533435
- Reviewer finding ID: N/A
- Source URL: https://github.com/torrust/torrust-tracker/pull/2829#discussion_r4537729420
- Concern: The recommendation to record the asymmetry in DEC-11 had no owner.
- Solution: Valid. DEC-11 now has the trade-off line.
- Current-tree verification: Read DECISIONS.md, "Trade-offs acknowledged" of DEC-11.
- Resolution reference: docs(issues): [#1669] record the HTTP and UDP health check asymmetry in DEC-11
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2829#discussion_r4537788808

### R1 - Qualify the claimed transitive-build savings

- PR number: 2829
- Source review ID: 5858573361
- Reviewer finding ID: N/A
- Source URL: https://github.com/torrust/torrust-tracker/pull/2829#discussion_r4537761081
- Concern: Removing the edge does not remove http-protocol, reqwest and hyper from the downstream builds.
- Solution: Same change as F3, already made; the text distinguishes removing the edge from removing crates.
- Current-tree verification: Read Finding 1 after the F3 commit.
- Resolution reference: docs(issues): [#1669] scope the transitive weight claim to udp-server
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2829#discussion_r4537797674

### F7 - Include container composition in the proposed decoupling

- PR number: 2829
- Source review ID: 5858573361
- Reviewer finding ID: R1
- Source URL: https://github.com/torrust/torrust-tracker/pull/2829#discussion_r4537761155
- Concern: A stats trait alone would not remove the edge; container.rs builds and accepts the concrete UdpTrackerServerContainer.
- Solution: Valid. The Finding 2 recommendation now includes moving or abstracting that wiring.
- Current-tree verification: `sed -n 85,88p` and `sed -n 103,106p` of packages/rest-api-runtime-adapter/src/v1/container.rs show the concrete type at lines 87 and 105.
- Resolution reference: docs(issues): [#1669] include the container wiring in the udp-server decoupling note
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2829#discussion_r4537810236

## Processing Log

- 2026-11-11 12:23 UTC - Round 1 processed after the replies and thread resolutions: five fix commits for F1 to F4 and F6 were pushed first, then the commit for F7; F5 was fixed in the PR description; all eight threads replied to and resolved.

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
