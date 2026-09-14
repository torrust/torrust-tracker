---
semantic-links:
  skill-links:
    - update-github-workflow-actions
  related-artifacts:
    - .github/skills/dev/maintenance/update-github-workflow-actions/SKILL.md
---

<!-- skill-link: update-github-workflow-actions -->

# GitHub Workflow Actions Update Pull Request

> Use this template only for the GitHub PR body of a GitHub Actions workflow update. Keep the update, organization-policy, validation, commit, and push process in the [`update-github-workflow-actions` skill](../../.github/skills/dev/maintenance/update-github-workflow-actions/SKILL.md). Do not commit a populated copy of this template.

## Summary

{State that the intended workflow action references were updated to current compatible releases. Note any compatibility consideration or deferred action separately.}

## Files/packages touched

- `.github/workflows/{workflow files updated}`
- `.github/skills/dev/maintenance/update-github-workflow-actions/SKILL.md` {if the process changed}

## Organization allowed-actions policy

{State whether an organization administrator confirmed the policy update. For an additive update, explicitly state that all previous entries were retained for other Torrust repositories.}

### Allowed actions before update

```text
{Replace this placeholder with the complete, unedited contents of .tmp/<timestamp>-allowed-actions-current.txt.}
```

### Allowed actions after update

```text
{Replace this placeholder with the complete, unedited contents of .tmp/<timestamp>-allowed-actions-new.txt.}
```

## Validation

- `linter yaml`
- `git diff --check`
- `TORRUST_GIT_HOOKS_LOG_DIR=.tmp ./contrib/dev-tools/git/hooks/pre-commit.sh --format=json`
- {Set-based allowlist comparison proving all prior entries were retained and each intended third-party `uses:` reference is allowed.}
