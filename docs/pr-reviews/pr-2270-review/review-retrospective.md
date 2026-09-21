---
semantic-links:
  skill-links:
    - process-pr-review
    - fetch-review-threads
    - resolve-review-threads
  related-artifacts:
    - "issue #2230"
    - docs/pr-reviews/pr-2270-review/PR-REVIEW.md
    - docs/templates/PR-REVIEW-TEMPLATE.md
    - .github/skills/dev/pr-reviews/process-pr-review/scripts/validate-audit-record.py
    - review-finding:pr-2270-f10
    - review-finding:pr-2270-f11
    - review-finding:pr-2270-f20
    - review-finding:pr-2270-f23
    - review-finding:pr-2270-f24
    - review-finding:pr-2270-f25
    - review-finding:pr-2270-f26
    - review-finding:pr-2270-f27
    - review-finding:pr-2270-f28
    - review-finding:pr-2270-f37
---

# Review Retrospective — PR #2270

## Purpose

Record why the review of PR #2270 cost far more than the change it delivered, and what the
evidence says about fixing that. This is a blameless review of the *review process*, written from
the author/agent side using the audit record and branch history. It complements the issue-level
[implementation retrospective](../../issues/open/2230-add-fix-bug-skill-and-bug-spec-guardrails/implementation-retrospective.md),
which covers the deliverable; this document covers the cost of getting the deliverable merged.

It proposes improvements only. Decisions about which to adopt are deferred until the PR is merged.

## Outcome

The deliverable — the `fix-bug` skill, bug-spec guardrails, and Implementer integration — was
substantively settled after the second human review. The remaining seven reviews were about the
accuracy of the review bookkeeping itself: the audit record, the issue progress log, reply URLs,
commit references, and timestamps. Every one of those findings was correct.

## Cost

All figures are derived from `PR-REVIEW.md` and `git log`; the commands are in
[Evidence](#evidence).

| Measure                                                                        | Value                                            |
| ------------------------------------------------------------------------------ | ------------------------------------------------ |
| Source reviews with findings                                                   | 9 (1 Copilot, 8 human)                           |
| Elapsed time, first fetch to last resolution                                   | ~48 hours (2026-09-19 08:20 to 2026-09-21 08:38) |
| Findings                                                                       | 43 (8 Copilot, 35 human)                         |
| Findings about the deliverable (F1-F9, F12-F16, F18-F19)                       | 16 (37%)                                         |
| Findings about the PR description (F17, F21)                                   | 2 (5%)                                           |
| Findings about the audit record or progress logs (F10-F11, F20, F22-F43)       | 25 (58%)                                         |
| Re-raises (`RE_RAISE_OF`)                                                      | 15 (35%)                                         |
| Most re-raised findings                                                        | F26 truthful timestamps (6x), F25 missing rows (5x) |
| Commits on the branch                                                          | 42                                               |
| Commit subjects mentioning the review or audit                                 | ~30                                              |

Read together: more than half of the findings and roughly two thirds of the commits were spent
correcting the record of the review rather than the code under review, and a third of all findings
were the reviewer repeating a point already made.

## What Went Well

1. The reviewer's findings were precise, reproducible, and consistently correct. There was no
   disagreement about facts, only failure to establish them before writing.
2. The append-only evidence model held. Once the in-place rewrites were reverted and corrections
   appended (F11, F27), the record showed both the mistake and the fix, which is what it is for.
3. Small signed commits made every correction attributable. The reviewer could and did cite the
   exact commit that introduced each inaccuracy.
4. The final round produced a mechanical validator
   (`scripts/validate-audit-record.py`) that would have caught F20, F23-F26, F28, F30-F37, and
   F39-F43 before they were posted. It is now part of the skill.

## How the Cost Accumulated

The pattern was the same in every round from the fourth onward:

1. A review arrives with N findings about the audit record.
2. The agent fixes them by writing new factual claims: new rows, new reply URLs, new commit
   subjects, new timestamps.
3. Some of the new claims are wrong in a different way than the old ones (a subject that does not
   exist on the branch, a stamp earlier than the commit that carries it, a reply URL that belongs to
   a different thread).
4. The next review re-raises the original finding plus the new inaccuracies.

Concrete instances:

- **F10 to F20 to F23.** The first response resolved threads without replies or an audit (F10).
  The audit was then created, but written against an older tree (F20), and then left with stale
  "pending" claims after the fixes landed (F23). Three rounds for one artifact to become true.
- **F26 and its six re-raises.** Processing-log entries were stamped with times earlier than the
  commits that carried them. Each fix changed the stamps again, and each change was checked by hand
  and found wrong again, until round ten restamped every entry from the carrying commit's UTC time.
- **F25 and its five re-raises.** A review's findings were replied to and resolved without rows in
  the audit. Adding rows for the re-raise threads then required rows for *those* re-raises; F30-F35
  and F39-F43 are almost entirely "add audit row for the round-N re-raise of F<k>".
- **F28.** A resolution reference cited a commit subject that never existed on the branch. The
  subject had been drafted, then the commit was made with different wording, and the audit was not
  re-checked. `git log -S` finds the real one in seconds; it was not run.

The common factor: each correction was itself an unverified factual claim. The audit format asks
for IDs, URLs, commit subjects, and UTC stamps precisely because they are checkable, but the skill
provided no way to check them, so their accuracy depended on the agent's diligence across a
ten-round session with several context compactions.

## Root Causes

1. **No mechanical verification of a record that is mostly machine-checkable.** Of the eight
   fields in a finding detail entry, six (source review ID, source URL, reply URL, resolution
   reference, severity, log ordering) can be validated against GitHub data and `git log`. None
   were, until round ten.
2. **Bookkeeping was done from memory rather than from the bytes.** Commit subjects were cited as
   drafted rather than as committed; reply URLs were carried forward from earlier drafts; stamps
   were estimated. Session compaction made this worse: the agent was reconstructing state it no
   longer had.
3. **The correction loop had no exit condition.** Fixing findings about the record adds new claims
   to the record. Without verification the loop is not guaranteed to converge, and here it took
   seven rounds to do so.
4. **The format scales with findings, not with substance.** Every re-raise needs its own row and
   six verified fields, regardless of whether it adds information. Fifteen of 43 rows exist only to
   say "the reviewer said this again". This is correct under the current template and it is also
   where much of the cost went.
5. **Append-only semantics were not treated as invariants.** Two logs were rewritten in place
   (F11, F27), each costing a round to restore. The skill states the rule; nothing enforced it.

## Insights

- A review record that demands precision without providing a checker converts reviewer time into
  the checker. Cameron performed by hand the checks the script now performs in under a second.
- The first review of the *deliverable* found real defects (F9 malformed frontmatter, F13-F14
  weakened workflow ordering). The reviews of the *record* found no defects in the deliverable.
  The record's accuracy matters for future analysis, but its cost should be proportional to that
  value.
- Re-raises are a leading indicator. The moment a second `RE_RAISE_OF` for the same finding
  appears, the fix strategy is not working and should change (here: from "edit again" to "verify
  mechanically"). That signal was available from round six.
- Agents under context pressure reliably lose exactly the kind of state the audit records:
  specific IDs, exact strings, and ordering. Anything an agent must recall verbatim across
  compaction should be re-derived from the source instead.

## Proposed Improvements

These are proposals for evaluation after merge, not commitments. Roughly ordered by expected
return on effort.

### Tooling

1. **Run the validator before every reply and every audit commit**, not only at completion. The
   skill now references it in step 9; consider moving the requirement to step 5 (decide) and step
   7 (reply) so an inaccurate row can never reach GitHub.
2. **Extend the validator to the issue progress log**: check that entries are chronological,
   that each cites finding IDs that exist in the audit, and that the file only grows between
   commits (append-only check via `git diff` on the log section).
3. **Generate rather than hand-write the mechanical fields.** A helper that, given a source
   comment ID, emits the detail-entry skeleton with `Source review ID`, `Source URL`, severity
   bracket, and the reply URL once posted, removes the transcription step entirely.
4. **Derive processing-log timestamps from the carrying commit** rather than typing them. A
   pre-commit check that each new log stamp is not earlier than the previous commit's timestamp
   would have prevented F26 and all six re-raises.

### Skill workflow

1. **Add an explicit convergence rule**: if a finding is re-raised twice, stop editing and
   re-derive every field of every affected row from source before the next reply.
2. **Make "re-derive from bytes" the default for corrections.** When fixing an audit inaccuracy,
   the skill should require the command whose output supplies the value (`gh api ...`,
   `git log -S`, `git log --format=%cI`) to be recorded in the detail entry's verification line.
3. **Treat context compaction as a trigger to re-validate.** After compaction the agent should run
   the validator before writing anything, because its recollection of IDs and subjects is the
   least reliable thing it has.

### Format

To be evaluated; each trades traceability for cost.

1. **Collapse re-raise bookkeeping.** Instead of a full row per re-raise thread, allow the original
   finding's detail entry to carry a `Re-raised in:` list of thread URLs. Rows would then count
   distinct concerns rather than distinct threads. The `RE_RAISE_OF` relationship remains for
   findings that are re-raised *with new information*.
2. **Distinguish record-accuracy findings from deliverable findings** with a category (e.g.
   `audit-integrity`) so future analysis can measure this cost directly rather than reconstruct it
   as this retrospective had to.
3. **Ask whether every field earns its place.** `Resolution reference` as a commit subject is
   fragile under rewording and is the field most often wrong here; a reply URL alone might carry
   the same traceability at lower cost, since the reply is where the subject is stated and the
   thread is where the reviewer verifies it.

## Avoiding Overcorrection

This evidence does not argue for dropping the audit record, weakening append-only history, or
resolving threads without replies. Those rules found real problems (F10, F11) and the record is
what made this retrospective possible. It also does not argue for a large tooling investment: the
validator is ~200 lines and covers most of the cost; the remaining proposals are incremental.

It does not argue that the reviewer should have been more lenient. Every finding was correct, and
a record that is tolerated as approximately right has no value for the analysis it exists to
support.

The right reading is narrow: **the format was ahead of its tooling.** Precision was required and
nothing helped supply it. Closing that gap is cheap; the format questions above are worth asking
but are secondary.

## Evidence

- Audit record: `docs/pr-reviews/pr-2270-review/PR-REVIEW.md` (F1-F43, Processing Log)
- Pull request: https://github.com/torrust/torrust-tracker/pull/2270
- Validator: `.github/skills/dev/pr-reviews/process-pr-review/scripts/validate-audit-record.py`
- Deliverable retrospective:
  `docs/issues/open/2230-add-fix-bug-skill-and-bug-spec-guardrails/implementation-retrospective.md`

Reproducing the figures (run from the repository root on the PR branch):

```bash
A=docs/pr-reviews/pr-2270-review/PR-REVIEW.md
rg -c '^\| F[0-9]+ \|' "$A"                       # findings
rg -o 'RE_RAISE_OF:F[0-9]+' "$A" | sort | uniq -c  # re-raises per finding
rg -o 'Source review ID: [0-9]+' "$A" | sort -u    # source reviews
rg '^- 2026-' "$A" | sed -n '1p;$p'                # elapsed span
git log --format='%s' develop..HEAD | wc -l        # commits on branch
```

Classification of findings by target (deliverable, PR description, record) is by finding title and
is recorded in the Cost table so it can be disputed.
