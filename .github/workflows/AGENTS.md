---
semantic-links:
  skill-links:
    - implement-workflow
---

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

## Tracked File Encoding

`docs-lint.yaml` runs `contrib/dev-tools/checks/check-utf8-files.sh` and its shell
integration tests on every push and pull request, including documentation-only changes.
The portable check requires Git and Python 3, both provided by the Ubuntu runner.
It validates index contents as UTF-8, except paths explicitly marked `binary` in
the staged `.gitattributes`, symbolic links, and submodules. `-text` alone is not
an exemption. Run the same command locally; no CI-specific environment is needed.
