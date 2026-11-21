---
semantic-links:
  skill-links:
    - process-pr-review
  related-artifacts:
    - .claude/skills/process-pr-review/SKILL.md
---

<!-- skill-link: process-pr-review -->

<!-- cspell:ignore keye expirede -->

# PR #2991 Review Audit

Source: pull-request reviews and inline review threads for https://github.com/torrust/torrust-tracker/pull/2991.

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
- When the current identity cannot resolve a thread (`viewerCanResolve` is `false`), keep the
  disposition established from the current tree and record `Thread state=OPEN`. The reviewer or PR
  author resolves it; permission says nothing about whether the concern was fixed.

## Findings

F1 and F2 come from Copilot's review 6006219064 (COMMENTED on the first head, "docs(issues): [#2916] record the reproduction, red and green runs and the manual verification"), in source order.

| Finding ID | Review finding reference | Author class | Severity | Category | Relationship | Disposition | Thread state |
| ---------- | ------------------------ | ------------ | -------- | -------- | ------------ | ----------- | ------------ |
| F1 | review-finding:pr-2991-f1 | Copilot | Minor | documentation | ORIGINAL | FIXED | OPEN |
| F2 | review-finding:pr-2991-f2 | Copilot | Nit | documentation | ORIGINAL | FIXED | OPEN |

## Finding Details

### F1 - The doc comment of `verify_auth_key` reads as if a found key returns an error

- PR number: 2991
- Source review ID: 6006219064
- Reviewer finding ID: N/A
- Source URL: https://github.com/torrust/torrust-tracker/pull/2991#discussion_r4653326514
- Concern: Copilot (rated Medium) read the reworded doc comment of `AuthenticationService::verify_auth_key` as saying that a found key "returns an error that names the key", while the found case goes on to the expiration check and can succeed. It asked for wording that follows the control flow.
- Solution: The sentence split across a line break made the two cases easy to misread. The comment now says that a key that is not found yields `Error::UnableToReadKey`, and that a found key is checked for expiration, the function returning `Ok(())` when the check passes or is disabled.
- Current-tree verification: `sed -n 74,82p packages/tracker-core/src/authentication/service.rs` shows the new wording; `cargo doc -p torrust-tracker-core --no-deps` (stable Rust toolchain) resolves the `Error::UnableToReadKey` link without a warning.
- Resolution reference: docs(tracker-core): [#2916] describe the key verification outcomes and the bencode terminator in samples
- Follow-up PR URL: N/A
- Reply URL: N/A

### F2 - The trailing bencode `e` in the extractor samples reads like a typo

- PR number: 2991
- Source review ID: 6006219064
- Reviewer finding ID: N/A
- Source URL: https://github.com/torrust/torrust-tracker/pull/2991#discussion_r4653326542
- Concern: Copilot (rated Low) noted that the module-doc samples of `authentication_key.rs` end with the bencode dictionary terminator right after the message (`keye`, `expirede`), which can read as a typo, and suggested decoding the samples or saying what the `e` is.
- Solution: The samples stay raw, as they are what a client receives and they match the recorded replies byte for byte; a sentence before them now says that each sample is the raw bencoded body and that the final `e` closes the dictionary and is not part of the `failure reason`.
- Current-tree verification: `sed -n 15,20p packages/axum-http-server/src/v1/extractors/authentication_key.rs` shows the note before the samples.
- Resolution reference: docs(tracker-core): [#2916] describe the key verification outcomes and the bencode terminator in samples
- Follow-up PR URL: N/A
- Reply URL: N/A

## Processing Log

- 2026-11-21 15:15 UTC - Started audit for Copilot's review 6006219064 (COMMENTED, submitted 2026-11-21 15:12:35Z on the first head). Copilot noted that its full agentic review did not start before its timeout; the review has two inline findings, F1 (Medium, recorded as Minor) and F2 (Low, recorded as Nit). Both fixed in "docs(tracker-core): [#2916] describe the key verification outcomes and the bencode terminator in samples". Not pushed yet; replies and thread resolution follow the push. da2ce7 is a requested reviewer and has not reviewed yet.

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
