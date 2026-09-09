---
name: process-pr-review-feedback
description: Process pull-request reviews that may contain multiple independent findings from maintainers, collaborators, or agents acting for them. Use when asked to address reviewer feedback, maintainer PR comments, contributor review summaries, or non-Copilot suggestions.
metadata:
  author: torrust
  version: "1.0"
  semantic-links:
    related-artifacts:
      - docs/pr-review-feedback/README.md
      - docs/templates/PR-REVIEW-FEEDBACK-TEMPLATE.md
      - .github/skills/dev/pr-reviews/fetch-review-threads/SKILL.md
      - .github/skills/dev/pr-reviews/resolve-review-threads/SKILL.md
---

# Processing PR Review Feedback

Use this workflow for review feedback from one or more maintainers,
collaborators, or agents acting for them. Use `process-copilot-suggestions` for
Copilot-generated review threads instead.

## Model

A submitted **review** and an inline **review thread** are different GitHub
resources:

- A review has a numeric review ID, state, body, reviewed commit, and URL. Its
  body may contain multiple findings. GitHub has no resolved state for it.
- An inline review thread has a GraphQL node ID and can be replied to and
  resolved. A reviewer can create these just as Copilot can.

Create `docs/pr-review-feedback/pr-<PR_NUMBER>-review-feedback.md` from
`docs/templates/PR-REVIEW-FEEDBACK-TEMPLATE.md`. Treat its review-response
state as the durable completion status for a review-level summary.

## Procedure

1. **Fetch reviews and threads.** Query submitted reviews and inline comments
   by review ID. Fetch all review threads separately, including their IDs,
   author, paths, bodies, and resolved state. Do not assume review-comment IDs
   are thread IDs.
2. **Create the audit record.** Add one row per review. Decompose each review
   body and inline comment into one row per independent finding, with a decision
   of `ACTION`, `NO_ACTION`, or `FOLLOW_UP`.
3. **Implement each action independently.** For every `ACTION`, make the
   smallest correct change, run relevant validation, and create a separate GPG
   signed Conventional Commit. Do not combine feature fixes with this workflow's
   docs, template, or audit records.
4. **Handle inline suggestions.** After the relevant action/no-action decision
   is complete, reply directly on each inline thread. Record the reply URL,
   then resolve the thread using `resolve-review-threads`. Do not resolve before
   replying.
5. **Reply to the review summary.** Once all findings from one submitted review
   are done or explicitly deferred, post one consolidated PR conversation
   comment. Include the review ID, each finding's outcome, associated commit,
   validation, and any follow-up. Store that comment URL in the review row.
6. **Update progressively.** Update the audit record immediately after each
   commit, PR reply, or resolution. Preserve the historical GitHub review state
   (including `DISMISSED`) and record the current disposition in the audit
   fields.
7. **Complete.** Verify every finding status and every inline-thread state from
   the current PR. Commit the workflow/audit documentation separately with a
   signed `docs(review): ...` commit.

## GitHub CLI Queries

Fetch an individual review and its review-specific inline comments:

```bash
gh api repos/torrust/torrust-tracker/pulls/<PR_NUMBER>/reviews/<REVIEW_ID>
gh api repos/torrust/torrust-tracker/pulls/<PR_NUMBER>/reviews/<REVIEW_ID>/comments?per_page=100
```

Fetch all review threads with GraphQL before resolving inline feedback. Use the
repository `fetch-review-threads` skill for the supported scripts and query
shape.

Post a consolidated response as a PR conversation comment:

```bash
gh pr comment <PR_NUMBER> --repo torrust/torrust-tracker --body-file <FILE>
```

## Completion Checklist

- [ ] Reviews and their inline comments fetched by review ID
- [ ] Audit record has one row per review and one row per independent finding
- [ ] Each action validated and committed independently
- [ ] Each inline thread replied to and resolved, with reply URL recorded
- [ ] Each review summary has one consolidated PR response, with URL recorded
- [ ] Historical review states and current audit statuses are both recorded
- [ ] Workflow/audit documentation committed separately from product fixes
