---
name: review-task
description: "Review a finished task on a local branch before its PR: acceptance criteria, scope, tests, docs, lint. Use when verifying an issue is complete and no PR exists yet."
metadata:
  author: torrust
  version: "1.0"
---

# Reviewing A Task (Pre-PR)

Use this skill when there is no pull request yet and the goal is to verify that implementation for
an issue/task is complete and ready to be pushed.

## Preconditions

- An issue spec exists (typically under `docs/issues/open/`).
- Local changes are available on the branch.
- No PR review workflow is required yet.

## Workflow

1. Read the issue spec and extract acceptance criteria.
2. Map each criterion to concrete evidence in changed files/tests.
3. Run relevant validation checks (`linter all` minimum, plus focused tests when applicable).
4. Classify each criterion as `PASS` (implemented and verified), `FAIL` (not implemented or
   incorrect) or `PENDING` (partial, unclear, or without evidence).
5. Update only verified checklist items in the issue spec.
6. Review the implementation completion evidence. Require an issue-local
   `implementation-retrospective.md` when the work revealed reusable lessons,
   material design changes, or meaningful deviations from the plan. Otherwise,
   require a concise issue progress-log entry explaining why no retrospective
   was needed.
7. Require a folder-style specification before accepting an issue-local
   retrospective. When a touched legacy single-file specification needs one,
   require migration to the documented folder layout first. `ISSUE.md` and
   `EPIC.md` are allowed primary-file exceptions in that layout.
8. Confirm that every mandatory manual scenario was executed against the finished artifact and is
   recorded in `manual-verification-evidence.md` (see the checklist).
9. Record the review (see [Recording The Review](#recording-the-review)), then report the overall
   result with remediation for any gaps.

## Task Review Checklist

### Scope And Criteria

- [ ] Issue spec path is identified.
- [ ] Acceptance criteria are fully listed.
- [ ] Claimed implementation scope matches actual changes.
- [ ] No scope creep beyond what the issue asks.

### Verification

- [ ] Each acceptance criterion has objective evidence.
- [ ] Required tests/lint checks pass.
- [ ] Docs updates are present when behavior changed.
- [ ] New terms are added to `project-words.txt` when needed.
- [ ] Every mandatory manual scenario was executed against the finished artifact and is recorded
      in `manual-verification-evidence.md` with the actual commands or interactions, the observed
      output, the relevant logs and the conclusions. Output of automated tests or of a disposable
      script is not manual evidence.

### Function Length And Nesting

Checked once, on the code the task adds or changes (code it did not touch is reported only when it
sits next to changed code and adds risk). Clippy's `cognitive_complexity = "deny"` in `Cargo.toml`
already fails the build on a function that is too complex, so this item is only about what that
lint does not see:

- [ ] No changed function is longer than 50 lines; a longer one is a sign of missing
      decomposition.
- [ ] No changed function nests more than 3 levels (`if`, `match`, loops, closures); deep nesting
      hides intent.

Do not penalise an idiomatic `match` that is the primary control flow of a function. Report each
violation as a repository-convention finding with the function and the simplification that would
fix it. A follow-up issue evaluates Clippy's `too_many_lines` and `excessive_nesting`
lints to replace this item.

### Test Design

When the reviewed changes add or modify tests, inspect each changed test against
`.claude/skills/write-unit-test/SKILL.md` and report a finding for every unchecked
item below:

- [ ] The test's name states one observable behavior and relevant condition.
- [ ] Arrange makes the causal initial-state difference visible.
- [ ] Any builder or scenario fixture is named for that state and owns only incidental mechanics;
      it is not a parameter bag mirroring the production call.
- [ ] Every helper names a coherent action, capability, or state and keeps the caller at one
      abstraction level. Do not treat a single-use helper as a defect solely because it has one
      caller; flag it only when it is vague, hides behavior, or mixes responsibilities.
- [ ] The production Act remains visible in the test body.
- [ ] Expected results are independently specified and assertions remain visible.
- [ ] The test does not duplicate a better-owned protocol, domain, integration, or end-to-end
      contract.
- [ ] Execution is deterministic: no uncontrolled I/O, wall-clock dependency, sleep, polling, or
      shared mutable state is introduced.
- [ ] The test evidence records a prose-first Arrange-Act-Assert comparison, or the reviewer
      records why it was not applicable. The final code expresses the temporary prose; redundant
      comments were removed and retained comments provide irreducible context.

### Spec Hygiene

- [ ] Only verified checklist items are marked done.
- [ ] Workflow checkpoints reflect pre-PR status correctly.
- [ ] Progress log includes meaningful, factual updates.
- [ ] Completion review records a retrospective or a rationale that one was unnecessary.
- [ ] Any retrospective belongs to a folder-style issue specification; the retrospective is lowercase while `ISSUE.md` and `EPIC.md` are allowed primary-file exceptions.

## Output

Return:

1. Scope reviewed
2. Acceptance criteria matrix (`PASS`/`FAIL`/`PENDING` + evidence)
3. Repository-convention findings
4. Completion-review finding
5. Issue spec updates made
6. Overall result: `REVIEW PASSED` when no required criterion fails and no blocking issue
   remains, `REVIEW FAILED` when any required criterion fails or a blocking issue remains. A
   criterion left `PENDING` because its evidence comes from a later task is listed with that
   reason and does not by itself fail the review

## Recording The Review

Where the review is recorded depends on the task and on the branch. The task you were given tells
you which case applies.

### A Reply File

When the task carries the line that names a reply file (decision D17 of issue #2904), write the
review there as one JSON file, after you update the verified checkboxes and before you finish. The
path is the one the line gives: `agent-replies/<agent>-<subject>-r<round>.json` in the folder of
the issue specification, with `task-reviewer` as the agent, the task in lower case (`t5`) or
`pr<number>` as the subject, and the round counted from 1 (decision D18). The fields:

- `schema-version`: 1; `agent`: `task-reviewer`; `model`: the full model ID you ran on;
  `subject` and `round`: the same as in the file name; `created-utc`: the time, from `date -u`.
- `verdict`: `REVIEW_PASSED` for the result `REVIEW PASSED`, `REVIEW_FAILED` for `REVIEW FAILED`,
  `QUESTION` when only the contributor can answer something, or `BLOCKED` when you cannot review.
- `summary`: the overall result in a few sentences.
- `findings`: one for each `FAIL`, `PENDING` and repository-convention finding, with a `severity`
  (`Blocker`, `Major`, `Minor`, `Nit` or `Suggestion`), a `location` and a `description` that
  carries the remediation.
- `questions`: what the contributor must answer, one text each, or an empty list.
- `evidence`: the commands you ran and what they showed, one text each.

Check the file with the validator before you finish:

```sh
cargo run --quiet --package agent-reply-validator --bin agent-reply-validator -- <reply file>
```

The stop hook of the `task-reviewer` agent runs the same checks and refuses to let you finish
while the file is missing or invalid, at most three times for one request. A reply file is
committed by the caller with the work it describes. It is also the record that satisfies the
checkpoint of a specification created from the earlier template, which names
`agent-review-reports.md`.

### No Folder-Style Specification

When the task names no reply file, because no folder-style issue specification was supplied (a
direct diff or package review, or a legacy standalone specification), write no file. Do not create,
migrate or modify an issue-local report. State in the result: `Issue-local report skipped: no
folder-style issue specification was supplied.`

### A Branch Without The `task-reviewer` Agent

A review made on a branch that does not have the `task-reviewer` agent yet is recorded in
`agent-review-reports.md` in the specification's folder, and closed specifications keep theirs.
Create the file from `docs/templates/AGENT-REVIEW-REPORTS.md` when it is absent; otherwise read it
whole and append one complete entry after the last one. Preserve every earlier entry unchanged and
in order: a correction is a new, timestamped entry that names the earlier conclusion. Record the
acceptance-criteria matrix, the repository-convention findings, the completion-review finding, the
specification updates, the overall result, and the remediation for every `FAIL` or `PENDING` item.
Do not commit the report; the caller includes it in the reviewed change set or commits it on its
own, and a failed report may be committed as durable evidence of the failure.

## Not In Scope

- Reviewing an open pull request (use `review-pr` for that).
- Publishing review comments to a PR.
- Merging or closing PRs.
