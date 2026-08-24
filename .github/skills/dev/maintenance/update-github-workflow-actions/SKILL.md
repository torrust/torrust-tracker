---
name: update-github-workflow-actions
description: Update GitHub Actions workflow dependencies safely in Torrust Tracker, including synchronizing the Torrust organization allowlist. Use when updating workflow action versions, Dependabot GitHub Actions updates, or allowed-actions settings.
metadata:
  author: torrust
  version: "1.0"
semantic-links:
  skill-links:
    - update-dependencies
  related-artifacts:
    - .github/dependabot.yaml
    - .github/skills/dev/maintenance/update-dependencies/SKILL.md
    - docs/skills/semantic-skill-link-convention.md
---

# Updating GitHub Workflow Actions

Use this skill to update `uses:` action references in `.github/workflows/`.
For Cargo dependency updates, use
`.github/skills/dev/maintenance/update-dependencies/SKILL.md` instead.

## Delivery Policy

- Never push directly to `develop` or `main`.
- Open a pull request to `torrust/torrust-tracker:develop` from a branch in the configured fork remote.
- Keep actions on explicit versions. Do not replace an exact action version with a moving major tag solely to work around an allowlist failure.

## Update Workflow

1. Start from an up-to-date `develop` branch and create a dedicated branch.
2. Identify every matching action reference and review the action's release notes for compatibility or security implications.
3. Update all intended `.github/workflows/*.yaml` references consistently. Dependabot manages GitHub Actions updates through `.github/dependabot.yaml`; preserve its explicit version format.
4. Before opening the pull request, update the Torrust organization allowed-actions policy at [Organization Actions settings](https://github.com/organizations/torrust/settings/actions).
   - Add an allowlist pattern that permits the versioned reference, such as `owner/action@v2.*`.
   - Prefer a scoped, stable pattern over a moving `owner/action@v2` tag when Dependabot updates exact versions.
   - Confirm that the configured pattern matches the full `uses:` reference, including its version.
5. Add one semantic `skill-link: update-github-workflow-actions` comment near the workflow's top-level metadata and review the related skills when updating the workflow policy.
6. Run `linter yaml`, `git diff --check`, and the relevant repository checks before committing.
7. Commit with a signed Conventional Commit, push the branch to the fork remote, and open a PR targeting `develop`.
8. Confirm affected workflow runs are queued and pass. If a run is blocked by the allowlist, correct the organization policy and rerun the failed jobs; do not weaken the workflow pin.

## Allowlist Failure Diagnosis

An error such as "The action `owner/action@vX.Y.Z` is not allowed" means the organization policy does not match the action reference exactly enough. Check the configured allowed patterns at the organization settings URL above against the workflow's `uses:` value.

For example, an allowlist entry `taiki-e/install-action@v2` does not permit `taiki-e/install-action@v2.85.5`. Configure `taiki-e/install-action@v2.*` to allow Dependabot-managed versioned v2 updates.

## Skill Links

- `.github/dependabot.yaml` controls automated GitHub Actions update proposals.
- `.github/skills/dev/maintenance/update-dependencies/SKILL.md` is the corresponding workflow for Cargo dependencies.
- `docs/skills/semantic-skill-link-convention.md` defines the required semantic-link syntax.
