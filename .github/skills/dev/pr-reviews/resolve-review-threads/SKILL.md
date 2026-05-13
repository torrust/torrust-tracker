---
name: resolve-review-threads
description: Resolve addressed GitHub pull request review threads for the torrust-tracker project. Use when asked to mark PR suggestions as resolved, resolve review comments, close addressed review threads, or clear Copilot review feedback after fixes are pushed. Triggers on "resolve PR threads", "mark suggestions as resolved", "resolve review comments", or "close addressed review threads".
metadata:
  author: torrust
  version: "1.0"
  semantic-links:
    related-artifacts:
      - .github/skills/dev/pr-reviews/resolve-review-threads/scripts/resolve-all-unresolved-threads.sh
---

# Resolving PR Review Threads

This is a component skill within the **process-copilot-suggestions** workflow.
Use this skill after the requested code or documentation changes are already implemented,
validated, committed, and pushed.

**Part of larger workflow**: See **process-copilot-suggestions** for the full end-to-end process.

## Preconditions

- The feedback has actually been addressed in the branch.
- Validation has been run for the touched scope (`linter all`, tests, or a targeted executable check).
- You have the target PR number and unresolved review thread IDs.

Do not resolve a thread just because a suggestion exists. Resolve it only when the underlying
concern is fixed or intentionally declined with a clear reason.

## Workflow

1. Read the active PR and collect unresolved review threads.
2. Group threads by file and confirm each one is truly addressed.
3. Implement and validate any missing fixes before resolving anything.
4. Resolve the addressed threads.
5. Re-check the PR state if needed.

## Preferred Resolution Path

If PR tools are available, first gather thread IDs from the active pull request metadata.

- Use the active PR tools to identify unresolved `reviewThreads`.
- Resolve only threads where `isResolved == false` and the fix is already on the branch.

## GitHub CLI GraphQL Command

Use GitHub CLI GraphQL when you need to resolve a thread directly from the terminal:

```bash
gh api graphql \
  -F threadId=THREAD_ID \
  -f query='mutation($threadId: ID!) { resolveReviewThread(input: { threadId: $threadId }) { thread { isResolved } } }'
```

Successful output should report `isResolved: true`.

## Batch Pattern

For multiple threads, resolve them one by one and check each result:

Preferred script usage:

```bash
bash scripts/resolve-all-unresolved-threads.sh \
  --threads-file /tmp/pr_threads_<PR_NUMBER>.json
```

Use `--dry-run` to preview without mutating GitHub state.

```bash
for thread_id in \
  THREAD_ID_1 \
  THREAD_ID_2
do
  gh api graphql \
    -F threadId="$thread_id" \
    -f query='mutation($threadId: ID!) { resolveReviewThread(input: { threadId: $threadId }) { thread { isResolved } } }'
done
```

## Notes

- Thread IDs are GraphQL node IDs, not PR numbers or comment IDs.
- This resolves the review thread, not the entire review.
- If a thread should remain open, leave it open and explain why.
- If you do not know the thread IDs yet, query the active PR first instead of guessing.

## Completion Checklist

- [ ] All targeted threads were verified against the current branch state
- [ ] Validation passed before resolution
- [ ] Each resolved mutation returned `isResolved: true`
- [ ] Any intentionally unresolved feedback is documented with reasoning
