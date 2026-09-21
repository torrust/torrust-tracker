---
semantic-links:
  skill-links:
    - process-pr-review
  related-artifacts:
    - docs/pr-reviews/pr-2271-review/PR-REVIEW.md
    - docs/pr-reviews/pr-2269-review/PR-REVIEW.md
    - docs/templates/PR-REVIEW-RETROSPECTIVE.md
    - .github/skills/dev/pr-reviews/process-pr-review/SKILL.md
    - contrib/dev-tools/checks/agent-review-report-contract/src/main.rs
---

# PR Review Retrospective — PR #2271

## Purpose

Record why processing the reviews of PR #2271 cost eight review rounds and fourteen audit-only
commits for a change whose substantive content was complete after five commits, and what to
change so the next audit does not repeat it. This is a blameless review of the process, not of
the reviewer or the author.

## Review Summary

| Metric | Value |
| ------ | ----- |
| Review rounds | 8 (7 human, 1 Copilot) |
| Audit findings | 46 |
| Re-raised findings (`RE_RAISE_OF`) | 17 |
| Findings about the audit record itself | 31 of 42 human findings |
| Commits on the branch | 23 (14 audit-only) |
| First review to last thread resolution | 2026-09-19 11:45 UTC to 2026-09-21 08:01 UTC |

Derivation: rows and `RE_RAISE_OF` counted over `PR-REVIEW.md`; review rounds and finding
paths from the pull-request `reviews` and `comments` REST endpoints filtered to root comments;
commit counts from `git log develop..HEAD`, audit-only meaning a `docs(pr-reviews)` subject.
"About the audit record" means the finding's source path is under `docs/pr-reviews/`.

## What Went Well

1. The reviewer re-derived every claim from the bytes at each head and named the exact command,
   line, and commit that contradicted the record. Every re-raise was mechanically checkable, so
   no round was spent disputing whether a defect existed.
2. Collision-safe audit-local IDs with `Reviewer finding ID` worked: seven rounds reused the
   reviewer's `F<k>` numbers for different concerns and the record never lost track of which row
   answered which comment.
3. Splitting documentation fixes from audit updates into separate commits, once adopted in
   round five, made "cite the commit that contains the change" straightforward to satisfy.
4. The rounds surfaced real contract gaps that the original PR had missed: the skill allowed a
   follow-up PR URL as a resolution reference while the template gave it a separate field (F19,
   F33); the template required a per-finding approval URL that no finding could ever fill (F30);
   the historical PR #2269 audit did not carry the newly mandatory field (F45).

## What Made the Review Costly

Three failure modes account for most of the 31 audit-record findings, and each recurred across
rounds because the round that "fixed" it introduced the next instance.

1. **Claims recorded before verification.** `Current-tree verification` and `Solution` lines
   described the intended edit rather than the tree. The AGENTS.md indentation (record F12) was
   claimed fixed in rounds 2, 5, and 6 before the file was changed in round 6's fix push; the
   skill's follow-up-URL sentence was claimed aligned (F19) two rounds before it was edited. Nine
   verification lines were false at the round-six head (F41); three more were false at the
   round-seven head (F46).
2. **Resolution references chosen by proximity, not content.** Six entries cited the nearest
   audit commit instead of the commit containing the described change (F26); the PR #2269 F5
   reference was "corrected" to a commit that could not contain it (F29); F12 and F27 cited a
   commit that did not touch AGENTS.md (F42).
3. **Self-referential claims that rot on every push.** Counts ("40 fields"), universals ("every
   line was re-derived"), and log timestamps written from memory became false as soon as the next
   commit added rows or events (F28, F31, F38, F39, F44, F46).

Two further costs were structural rather than repeated:

- `RE_RAISE_OF` targets on F17-F24 copied the reviewer's IDs instead of the record's own (F25),
  which made eight plausible-looking relationships false at once.
- A tooling fallback during round five (regex-driven section moves after the editor's replace
  tool was unavailable) duplicated `## Processing Log` and `## Completion Rules` and left eight
  detail entries outside `## Finding Details` (F35).

## Root Causes

1. **No verification step between editing and recording.** The skill's step 5 says "re-derive
   every reply claim from the current tree", but nothing in the workflow forces the author to run
   a command and paste its result before writing the line. Memory of an edit substituted for
   evidence of it.
2. **False confidence from `agent-review-report-contract`.** The crate validates the skill and
   template literals and ignores its path argument, so "the contract checker passes" was cited as
   evidence about the audit record at least ten times while proving nothing about it. No
   repository check validates an audit record's row/detail parity, order, `RE_RAISE_OF` targets,
   field roster, or whether cited commits exist and touch the described files.
3. **Batching replies and resolution ahead of the tree.** Threads were replied to with "Fixed"
   and resolved in the same pass that wrote the audit, before the fix commit was inspected. The
   skill's "reply before resolving" was honored; "verify before replying" was not.
4. **Mixed-scope commits.** Early rounds combined doc fixes and audit edits, so the question
   "which commit contains this fix" had no clean answer and the nearest subject was used.
5. **Reviewer cost was invisible to the author loop.** Each human round was a full byte-level
   re-derivation of every claim in a 40-plus-row record. Nothing in the process made the author
   perform the same re-derivation before requesting re-review.

## What We Learnt

1. Write the verification command and its output first; write the `Current-tree verification`
   sentence from the output, never from the intent.
2. A resolution reference is chosen by `git show --stat <commit> -- <path>`; if the described
   path is absent, the reference is wrong regardless of how close the commit is.
3. Any claim about the record's own size or completeness must be an equality or a scoped list,
   never a count or a universal.
4. Record log events from `git log --format=%cI` and the GitHub `created_at` fields, truncated to
   the minute, not from recollection; add the commit that records the log in the same commit.
5. One commit per concern: repository fixes in `fix(...)`/`docs(...)` commits, audit updates in
   `docs(pr-reviews)` commits, so every citation is unambiguous.
6. "Checker passes" is only evidence about what the checker reads; state what it reads.

## Improvements for Future Reviews

1. Add a self-audit pass to `process-pr-review` before requesting re-review: for every row,
   re-run the verification command, confirm the cited commit touches the described path, confirm
   `RE_RAISE_OF` targets exist, and confirm row/detail parity and order —
   `.github/skills/dev/pr-reviews/process-pr-review/SKILL.md` — `PROPOSED`.
2. Make `agent-review-report-contract` (or a sibling check) validate audit records: one heading
   each for the fixed sections, `### F<k>` order equal to table order, ten fields per entry,
   audit-local `RE_RAISE_OF` targets, cited commit subjects present on the branch —
   `contrib/dev-tools/checks/agent-review-report-contract/` — `FOLLOW_UP_ISSUE`.
3. Add a rule to the audit template that `Current-tree verification` names a command or
   inspection whose result is independent of row count — `docs/templates/PR-REVIEW-TEMPLATE.md`
   — `PROPOSED`.
4. Add a rule that the author re-derives all claims and records a self-audit log entry before
   each re-review request — `process-pr-review` — `PROPOSED`.
5. Document that `agent-review-report-contract` validates a fixed target and takes no path
   argument, so it is not cited as audit evidence — crate README or skill — `PROPOSED`.
6. This retrospective and its template — `docs/templates/PR-REVIEW-RETROSPECTIVE.md` —
   `APPLIED_IN_THIS_PR`.

## Avoiding Overcorrection

- Do not add more per-finding fields. The cost came from false values, not missing ones; the
  ten-field roster is sufficient.
- Do not require a retrospective for every PR review. This one is warranted by 31 findings
  about the record itself; a two-round review with no re-raises has nothing to retrospect.
- Do not make the reviewer's byte-level re-derivation the author's mandatory format for replies.
  A reply needs the disposition, one verifying command or inspection, and the commit subject.
- Do not block on the audit validator before adopting the self-audit pass; the manual pass is
  cheap and would have prevented rounds four through seven on its own.

## Evidence

- Audit record: `docs/pr-reviews/pr-2271-review/PR-REVIEW.md` (F1-F46)
- Pull request: <https://github.com/torrust/torrust-tracker/pull/2271>
- Human review IDs: 5255829901, 5255975253, 5256752506, 5260313082, 5260426384, 5260561839,
  5263382878; Copilot review 5255633262
- Round-six review body documenting that the contract crate ignores its path argument:
  <https://github.com/torrust/torrust-tracker/pull/2271#pullrequestreview-5260561839>
- Commit list: `git log --format='%h %cI %s' develop..HEAD` on branch
  `2264-address-post-merge-review`
