---
pr-number: 2785
pr-url: https://github.com/torrust/torrust-tracker/pull/2785
last-updated-utc: "2026-11-09 17:05"
---

# PR #2785 Review Audit

Source: pull-request reviews and inline review threads for https://github.com/torrust/torrust-tracker/pull/2785.

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

Round 1: da2ce7 review 5829057649 and Copilot review 5829108924 on `1faf075d7`; round 2: da2ce7 review 5829397839 on `bf9557b12`. Copilot reused the reviewer ID `F1` and da2ce7 reused `F5` and `F6`, so audit-local IDs F5 to F7 are assigned and the reviewers' IDs are recorded in the details.

| Finding ID | Review finding reference | Author class | Severity | Category | Relationship | Disposition | Thread state |
| ---------- | ------------------------ | ------------ | -------- | -------- | ------------ | ----------- | ------------ |
| F1 | review-finding:pr-2785-f1 | Human | Major | link-integrity | ORIGINAL | FIXED | RESOLVED |
| F2 | review-finding:pr-2785-f2 | Human | Minor | link-integrity | ORIGINAL | FIXED | RESOLVED |
| F3 | review-finding:pr-2785-f3 | Human | Minor | documentation | ORIGINAL | FIXED | RESOLVED |
| F4 | review-finding:pr-2785-f4 | Human | Nit | formatting | ORIGINAL | FIXED | RESOLVED |
| F5 | review-finding:pr-2785-f5 | Copilot | Minor | link-integrity | ORIGINAL | FIXED | RESOLVED |
| F6 | review-finding:pr-2785-f6 | Human | Minor | documentation | ORIGINAL | FIXED | RESOLVED |
| F7 | review-finding:pr-2785-f7 | Human | Nit | documentation | ORIGINAL | FIXED | RESOLVED |

## Finding Details

### F1 - Title claims a pass while fsf.org and Medium links still fail

- PR number: 2785
- Source review ID: 5829057649
- Reviewer finding ID: F1
- Source URL: https://github.com/torrust/torrust-tracker/pull/2785#discussion_r4515184768
- Concern: Title and subject said the check passes, but `https://www.fsf.org/` and two Medium links stayed checked and fail on the runner.
- Solution: Added exact-URL exclusions for `https://www.fsf.org/` and the two Medium Kent Beck URLs instead of rewording the title.
- Current-tree verification: `lychee --config .github/lychee-online.toml` over `README.md`, the three package READMEs, the write-unit-test skill, `docs/containers.md` and the PR 2486 audit: 0 errors, 28 excluded.
- Resolution reference: fix(ci): narrow link-check exclusions and add fsf.org and Medium
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2785#discussion_r4515264267

### F2 - Narrow the Stack Overflow exclusion to one URL

- PR number: 2785
- Source review ID: 5829057649
- Reviewer finding ID: F2
- Source URL: https://github.com/torrust/torrust-tracker/pull/2785#discussion_r4515184780
- Concern: The host-wide Stack Overflow exclusion was broader than the exception rule; the PR description had no `Related to` link.
- Solution: Narrowed the pattern to the exact answer URL `a/56768087/3012842`, with a comment naming its source, and added `Related to #2264` to the description.
- Current-tree verification: Pattern in `.github/lychee-online.toml` matches only the URL at `docs/containers.md:689`; the PR body starts with `Related to #2264`.
- Resolution reference: fix(ci): narrow link-check exclusions and add fsf.org and Medium
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2785#discussion_r4515264714

### F3 - Document the placeholder exclusion

- PR number: 2785
- Source review ID: 5829057649
- Reviewer finding ID: F3
- Source URL: https://github.com/torrust/torrust-tracker/pull/2785#discussion_r4515184792
- Concern: The `{response-R3}` placeholder exclusion was undocumented and would hide future unfilled placeholders.
- Solution: First narrowed to the exact URL with a comment; then superseded by F5, which repairs the URLs and removes the exclusion altogether.
- Current-tree verification: No `{response-R3}` pattern remains in `.github/lychee-online.toml`; see F5.
- Resolution reference: fix(ci): narrow link-check exclusions and add fsf.org and Medium
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2785#discussion_r4515265136

### F4 - Misplaced comment sentence

- PR number: 2785
- Source review ID: 5829057649
- Reviewer finding ID: F4
- Source URL: https://github.com/torrust/torrust-tracker/pull/2785#discussion_r4515184799
- Concern: The added sentence separated "also client-side" from the sentence it continues.
- Solution: Removed the added sentence; the comment lines equal the merge base.
- Current-tree verification: `git diff` of the comment block against the merge base shows no change to those lines.
- Resolution reference: fix(ci): narrow link-check exclusions and add fsf.org and Medium
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2785#discussion_r4515265554

### F5 - Repair the audit URLs instead of excluding placeholders

- PR number: 2785
- Source review ID: 5829108924
- Reviewer finding ID: F1
- Source URL: https://github.com/torrust/torrust-tracker/pull/2785#discussion_r4515223654
- Concern: The `{response-R3}` URLs in the PR 2486 audit are unresolved links in an actual record; repair them and remove the exclusion instead of hiding them.
- Solution: Replaced both URLs with the R3 response comment `pull/2486#issuecomment-6107186888` and removed the exclusion.
- Current-tree verification: Comment 6107186888 begins "R3 from review 5469325284"; lychee over `docs/pr-reviews/pr-2486-review/PR-REVIEW.md`: 0 errors; no `response-R3` string remains in the tree.
- Resolution reference: fix(docs): repair PR 2486 audit reply URLs instead of excluding placeholders
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2785#discussion_r4515457912

### F6 - Missing audit record and unanswered thread

- PR number: 2785
- Source review ID: 5829397839
- Reviewer finding ID: F5
- Source URL: https://github.com/torrust/torrust-tracker/pull/2785#discussion_r4515421313
- Concern: The review was processed without the tracked audit record, one thread was resolved with no reply, and replies cited a branch commit id instead of the commit subject.
- Solution: Added this audit record, replied on the Copilot thread, and edited the four earlier replies to cite the commit subject.
- Current-tree verification: `validate-audit-record.py --pr-number 2785` passes; the four earlier replies now name `fix(ci): narrow link-check exclusions and add fsf.org and Medium`.
- Resolution reference: docs(pr-reviews): [#2785] record the review audit for the external link check fix
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2785#discussion_r4515478668

### F7 - Name the checker as the cause for fsf.org

- PR number: 2785
- Source review ID: 5829397839
- Reviewer finding ID: F6
- Source URL: https://github.com/torrust/torrust-tracker/pull/2785#discussion_r4515421326
- Concern: The fsf.org comment named the runner as the cause where the 2264 record names the checker.
- Solution: The comment now reads "Lychee cannot complete the TLS handshake with www.fsf.org".
- Current-tree verification: Line in `.github/lychee-online.toml` at the pushed head; matches `external-link-check-residual-failures-2026-09-18.md:26`.
- Resolution reference: fix(docs): repair PR 2486 audit reply URLs instead of excluding placeholders
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2785#discussion_r4515458402

## Processing Log

- 2026-11-09 16:30 UTC - da2ce7 review 5829057649 (CHANGES_REQUESTED) on `1faf075d7`; Copilot review 5829108924 at 16:34 UTC.
- 2026-11-09 16:35 UTC - Fixes for F1 to F4 committed (`bf9557b12`, commit time 16:35:17 UTC) and pushed; replies on F1 to F4 posted 16:37:51 to 16:37:58 UTC and the threads resolved. The Copilot thread (F5) was resolved at that time without a reply: a process error, corrected below.
- 2026-11-09 16:52 UTC - da2ce7 review 5829397839 with F5 and F6 (reviewer IDs) at 16:52:18 UTC; the approval was later dismissed by GitHub when the next push landed.
- 2026-11-09 16:53 UTC - F5 and F7 fixed (commit time 16:53:22 UTC) and pushed; replies on F5 (16:55:44 UTC) and F7 (16:55:47 UTC) posted and the F7 thread resolved. The earlier replies on F1 to F4 were edited to cite the commit subject instead of the branch commit id.
- This audit record (F6) was written after those events, in the commit named in F6.

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
