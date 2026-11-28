---
semantic-links:
  skill-links:
    - process-pr-review
  related-artifacts:
    - .github/skills/dev/pr-reviews/process-pr-review/SKILL.md
    - docs/discussions/2003-overhaul-guardrails-and-automation/20261007-ai-model-provenance-in-commits/README.md
---

<!-- skill-link: process-pr-review -->

# PR #2548 Review Audit

Source: pull-request reviews and inline review threads for <https://github.com/torrust/torrust-tracker/pull/2548>.

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

The review bot's review 5543973742 (COMMENTED, 2026-10-16 06:44:46 UTC, Lite effort after an agentic-review timeout) carries one inline finding, F1; its overview repeats that finding with a Medium severity label, so the body adds no row. The PR author's review 5543995433 is the reply wrapper for F1's answer and adds no row. josecelano's review 5588111703 (COMMENTED, 2026-10-20 16:19:28 UTC) carries two inline advisory findings, labelled R1 and R2 by the reviewer; they are recorded as F2 and F3, with the reviewer's IDs kept in the entries. Its body is the round verdict ("no blocker under the merge gate for a contribution round") and adds no request. The PR author's reviews 5604845742 and 5604846257 are the reply wrappers for the F2 and F3 answers and add no row.

| Finding ID | Review finding reference | Author class | Severity | Category | Relationship | Disposition | Thread state |
| ---------- | ------------------------ | ------------ | -------- | -------- | ------------ | ----------- | ------------ |
| F1 | `review-finding:pr-2548-f1` | Copilot | Minor (inferred) | link-integrity | ORIGINAL | NO_ACTION | RESOLVED |
| F2 | `review-finding:pr-2548-f2` | Human | Minor | documentation | ORIGINAL | FIXED | RESOLVED |
| F3 | `review-finding:pr-2548-f3` | Human | Nit | metadata | ORIGINAL | FIXED | RESOLVED |

## Finding Details

### F1 - The relative link to the draft issue might use one `..` too few

- PR number: 2548
- Source review ID: 5543973742
- Reviewer finding ID: N/A
- Source URL: <https://github.com/torrust/torrust-tracker/pull/2548#discussion_r4291659728>
- Concern: In the `harness-as-repository-dev-tools` paragraph, the link `../../../issues/drafts/...` resolves from the README's directory to `docs/issues/drafts/...`, while the neighboring `.github/...` link uses four `..` segments; if the draft issue lived at the repository root, the link would be broken. The review's overview labels the finding Medium; the audit scale has no Medium, so the row records `Minor (inferred)`, the nearest value, as the PR #2545 audit does for the same label.
- Solution: No change. The README sits three directories below `docs/`, so three `..` segments reach `docs/`, and the draft issue is under `docs/issues/drafts/`; the `.github/...` link needs four because its target is at the repository root. The reply 4291675225 states this, and the thread is resolved.
- Current-tree verification: `git cat-file -e HEAD:docs/issues/drafts/2003-separate-ai-harness-cargo-workspace/ISSUE.md` exits 0 at this branch's head.
- Resolution reference: <https://github.com/torrust/torrust-tracker/pull/2548#discussion_r4291675225>
- Follow-up PR URL: N/A
- Reply URL: <https://github.com/torrust/torrust-tracker/pull/2548#discussion_r4291675225>

### F2 - The Q3 correction adds a commitment, not only a wording change

- PR number: 2548
- Source review ID: 5588111703
- Reviewer finding ID: R1
- Source URL: <https://github.com/torrust/torrust-tracker/pull/2548#discussion_r4327376151>
- Concern: `docs/discussions/AGENTS.md` lets an author correct only the wording of their own earlier entries. The Q3 correction replaced "Identifiers come from the harness" with a sentence that added "never from the model's own account of itself", a clause the entry had not stated, while the commit message said only the bare word changed. The reviewer offered two ways out: keep the line to the wording change and put the clause in a new Q3 entry that names this one, or state that the clause only restates what the entry already meant.
- Solution: The first option, without the new entry: the clause is dropped, so the corrected sentence reads "Identifiers come from the runtime that ran the model, with `unknown` when that runtime cannot name the model, so that the record never looks more complete than it is." Nothing else in the entry changes, and the label stays. The reply 4340847812 states this, and the thread is resolved.
- Current-tree verification: The Q3 entry's labelled paragraph (`identifier-from-the-runtime-not-the-model`) contains the sentence quoted above, and `git grep -F "never from the model's own account"` over the discussion README finds nothing.
- Resolution reference: docs(discussions): [#2003] keep Q3 to wording and date tools by commit
- Follow-up PR URL: N/A
- Reply URL: <https://github.com/torrust/torrust-tracker/pull/2548#discussion_r4340847812>

### F3 - The tools observation's date matches neither the entry nor a commit the reader can check

- PR number: 2548
- Source review ID: 5588111703
- Reviewer finding ID: R2
- Source URL: <https://github.com/torrust/torrust-tracker/pull/2548#discussion_r4327376165>
- Concern: The `harness-as-repository-dev-tools` paragraph said "As of 2026-10-15", but the entry is dated 2026-10-16 and the commit that added the date says the observation was made as of the `develop` head it was made at, so a reader cannot tell which tree to re-check. The reviewer asked for the `develop` commit or 2026-10-16.
- Solution: The sentence now reads "As of `develop` at `fe5d8264a` (2026-10-16) they appear to use no machine-learning tool.", naming the commit the observation was made against and the date it was made. The reply 4340848212 states this, and the thread is resolved.
- Current-tree verification: At `fe5d8264a`, the 10 Cargo manifests under `contrib/dev-tools/` name 13 dependencies, none of them a machine-learning or model-API crate, and `git grep -i -E` over `contrib/dev-tools/` for model-API endpoints and inference terms finds no line.
- Resolution reference: docs(discussions): [#2003] keep Q3 to wording and date tools by commit
- Follow-up PR URL: N/A
- Reply URL: <https://github.com/torrust/torrust-tracker/pull/2548#discussion_r4340848212>

## Processing Log

- 2026-10-16 06:44 UTC - The review bot's review 5543973742 left one inline finding, recorded as F1 (`Minor (inferred)` from its overview's Medium label).
- 2026-10-16 06:47 UTC - Replied on the F1 thread with `NO_ACTION` (reply 4291675225); the thread capture taken on 2026-10-21 shows the thread resolved.
- 2026-10-20 16:19 UTC - josecelano's review 5588111703 left two inline advisory findings, recorded as F2 (the reviewer's R1, Minor) and F3 (R2, Nit).
- 2026-10-21 18:53 UTC - Fixed F2 and F3 in `docs(discussions): [#2003] keep Q3 to wording and date tools by commit`.
- 2026-10-21 18:56 UTC - Recorded F1 to F3 here. The replies on the F2 and F3 threads are not posted yet, so their Reply URLs are placeholders and both threads are `OPEN`.
- 2026-10-21 19:08 UTC - Replied on the F2 thread with `FIXED` and the fix's commit subject (reply 4340847812, created 19:08:12 UTC).
- 2026-10-21 19:08 UTC - Replied on the F3 thread with `FIXED` and the fix's commit subject (reply 4340848212, created 19:08:14 UTC).
- 2026-10-21 19:08 UTC - Resolved the F2 and F3 threads at 19:08:18 UTC; the thread capture taken after the replies shows all three threads resolved.
- 2026-10-21 19:10 UTC - Correction to the 18:56 entry, which stays as written: its sentence "The replies on the F2 and F3 threads are not posted yet, so their Reply URLs are placeholders and both threads are `OPEN`." no longer holds. The F2 and F3 Reply URLs now name the replies 4340847812 and 4340848212, and both rows are `FIXED`/`RESOLVED`.

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
