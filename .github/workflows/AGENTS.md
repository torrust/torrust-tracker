# GitHub Actions Workflows

Follow the canonical [workflow implementation skill](../skills/dev/ci/implement-workflow/SKILL.md)
when adding or materially changing a workflow.

## Local Constraints

- Pin actions to the repository's established versions.
- Use least-privilege permissions and preserve existing trust boundaries.
- Check out immutable base/head revisions when a job compares pull request
  revisions.
- Keep workflow files limited to GitHub Actions orchestration. Put non-trivial
  repository behavior in tested tools with portable command-line contracts.
- Validate workflow edits with `linter yaml` and `git diff --check`, then obtain
  hosted evidence for provider-only behavior.
