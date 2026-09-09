---
name: update-github-workflow-actions
description: Update GitHub Actions workflow dependencies safely in Torrust Tracker, including synchronizing the Torrust organization allowlist. Use when updating workflow action versions, Dependabot GitHub Actions updates, or allowed-actions settings.
metadata:
  author: torrust
  version: "1.1"
semantic-links:
  skill-links:
    - update-dependencies
  related-artifacts:
    - .github/dependabot.yaml
    - .github/workflows/
    - .github/skills/dev/maintenance/update-dependencies/SKILL.md
    - docs/templates/GITHUB-WORKFLOW-ACTIONS-UPDATE-PR.md
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
- Keep workflow actions updated to current safe versions to receive their security fixes.
- When this work accompanies a Cargo dependency update, use its dedicated branch and update workflow actions only after the Cargo update has been validated.
- When an organization allowed-actions policy update is required, capture the complete old and new allowlists in `.tmp/`. Include both lists verbatim in the signed commit body and pull-request description. State whether the change added, replaced, or retained patterns. Do not claim the policy was updated unless it has been confirmed.

## Quick Reference

```bash
# Use one nanosecond-resolution timestamp for this invocation's temporary evidence.
TIMESTAMP=$(date +%Y%m%d-%H%M%S-%N)
UPDATE_BRANCH="${TIMESTAMP}-update-github-workflow-actions"
ALLOWLIST_CURRENT=".tmp/${TIMESTAMP}-allowed-actions-current.txt"
ALLOWLIST_NEW=".tmp/${TIMESTAMP}-allowed-actions-new.txt"

git checkout develop && git pull --ff-only
git checkout -b "$UPDATE_BRANCH"
mkdir -p .tmp

# Save the complete current organization allowlist, then prepare the complete
# new list. The new list must retain every existing entry.
# Obtain administrator confirmation before claiming that the policy was updated.

linter yaml
git diff --check
TORRUST_GIT_HOOKS_LOG_DIR=.tmp ./contrib/dev-tools/git/hooks/pre-commit.sh --format=json

# Include both complete captured lists in the signed commit body.
git commit -S -m "ci(workflows): update GitHub Actions" \
  -m "$(printf 'Allowed actions before update:\n%s\n\nAllowed actions after update:\n%s' "$(cat "$ALLOWLIST_CURRENT")" "$(cat "$ALLOWLIST_NEW")")"
git push {your-fork-remote} "$UPDATE_BRANCH"

# Open a PR using docs/templates/GITHUB-WORKFLOW-ACTIONS-UPDATE-PR.md.
# Replace both allowlist placeholders with the complete captured lists verbatim.
```

## Update Workflow

1. Generate one nanosecond-resolution timestamp and use it for the dedicated branch and `.tmp/` evidence filenames shown in the quick reference. This prevents same-second collisions between independent invocations. Concurrent update workflows must not share a working tree because Git branches and the index are shared.
2. Start from an up-to-date `develop` branch and create that dedicated branch.
3. Identify every matching action reference and review the action's release notes for compatibility or security implications. If no workflow reference needs an update, stop without committing.
4. Update all intended `.github/workflows/*.yaml` references consistently. Dependabot manages GitHub Actions updates through `.github/dependabot.yaml`; preserve its explicit version format.
5. Before opening the pull request, obtain the complete current Torrust organization allowed-actions list and save it in `"$ALLOWLIST_CURRENT"`. Prepare the complete revised list in `"$ALLOWLIST_NEW"`, retaining every current entry. The allowlist is organization-wide: never replace it with an inventory from this repository alone. A missing entry from the configured list may be authorized by a broader organization policy, such as GitHub-owned or verified Marketplace actions; do not infer that it must be added from a repository scan. If a complete replacement list is requested, obtain an organization-wide inventory first; otherwise, provide only the required additions and replacements. If the required reference is not allowed and the agent cannot change the organization policy, tell the user that a GitHub organization administrator must update the allowed-actions list before the workflow can run.
   - Add an allowlist pattern that permits the versioned reference, such as `owner/action@v2.*`.
   - Prefer a scoped, stable pattern over a moving `owner/action@v2` tag when Dependabot updates exact versions.
   - Confirm that the configured pattern matches the full `uses:` reference, including its version.

- Compare the lists with a set-based comparison. Verification must prove that `"$ALLOWLIST_CURRENT"` is a subset of `"$ALLOWLIST_NEW"` for an additive change and that every third-party `uses:` reference is matched by the new list.
- Have an organization administrator apply the revised list at [Organization Actions settings](https://github.com/organizations/torrust/settings/actions), then confirm the update. Do not remove old entries merely because this repository no longer uses them; other Torrust repositories may still rely on them.

6. Add one semantic `skill-link: update-github-workflow-actions` comment near the workflow's top-level metadata and review the related skills when updating the workflow policy.
7. When a workflow command compares Git revisions, configure `actions/checkout` with
  `fetch-depth: 0` so its merge base is available in CI.
8. Run `linter yaml`, `git diff --check`, and the mandatory pre-commit checks before committing.
9. Commit with a signed Conventional Commit and push the branch to the fork remote. When step 5 required an organization policy update, include the complete contents of both `"$ALLOWLIST_CURRENT"` and `"$ALLOWLIST_NEW"` verbatim in the commit body. Use headings that identify the lists as before and after the update; do not summarize or omit unchanged entries.
10. Open a PR targeting `develop` with [the GitHub workflow-actions update PR template](../../../../../docs/templates/GITHUB-WORKFLOW-ACTIONS-UPDATE-PR.md). Replace both allowlist placeholders with the complete captured contents verbatim. State the administrator's confirmation only when it was received.
11. Confirm affected workflow runs are queued and pass. If a run is blocked by the allowlist, correct the organization policy and rerun the failed jobs; do not weaken the workflow pin.

## Allowlist Failure Diagnosis

An error such as "The action `owner/action@vX.Y.Z` is not allowed" means the organization policy does not match the action reference exactly enough. Check the configured allowed patterns at the organization settings URL above against the workflow's `uses:` value.

For example, an allowlist entry `taiki-e/install-action@v2` does not permit `taiki-e/install-action@v2.85.5`. Configure `taiki-e/install-action@v2.*` to allow Dependabot-managed versioned v2 updates.

## Skill Links

- `.github/dependabot.yaml` controls automated GitHub Actions update proposals.
- `.github/skills/dev/maintenance/update-dependencies/SKILL.md` is the corresponding workflow for Cargo dependencies.
- `docs/skills/semantic-skill-link-convention.md` defines the required semantic-link syntax.
