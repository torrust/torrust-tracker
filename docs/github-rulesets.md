---
semantic-links:
  skill-links:
    - merge-pull-request
    - open-pull-request
  related-artifacts:
    - docs/adrs/20261013161419_stop_requiring_pull_request_rebases.md
    - .github/skills/dev/git-workflow/merge-pull-request/SKILL.md
    - contrib/dev-tools/git/github-merge.py
    - "issue #2507"
    - "issue #2525"
---

# GitHub Rulesets

GitHub repository settings are not in Git, so this document is the version-controlled record of the
rulesets that protect `torrust/torrust-tracker` and why each exists. Compare it with the live
configuration to detect a ruleset that was disabled, added, or changed without a record.

## Policy

- **Never disable a ruleset to merge a pull request.** A disabled ruleset is easily forgotten: in
  2026 the required-checks ruleset stayed disabled for months and took the force-push ban with it
  (issue #2507). When a required check blocks a known-good pull request, use the override of that
  check instead.
- **No ruleset requires branches to be up to date.** Rebasing is recommended but not required
  ([ADR](adrs/20261013161419_stop_requiring_pull_request_rebases.md)); any required-checks rule
  keeps `strict_required_status_checks_policy` set to `false`.
- **Required checks must report on every pull request.** A skipped workflow reports nothing, and a
  required check that never reports blocks the merge forever. No status check is required today;
  a single always-reporting `CI gate` check, overridable by a maintainer label, is planned in issue
  #2525.
- **Change a ruleset and this document together.** Record the change, and the reason, in the same
  pull request or in the issue that authorized it.

## Active Rulesets

Repository role `2` in a bypass list is the Maintain role.

| ID       | Target                                                  | Rules                                                                                                              | Bypass           | Why                                                                                                                                                                                                    |
| -------- | ------------------------------------------------------- | ------------------------------------------------------------------------------------------------------------------ | ---------------- | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------ |
| 87085    | All branches except `dependabot`, `releases`, `staging` | Deny create; deny delete; require a pull request (0 approvals, dismiss stale reviews on push, merge/squash/rebase) | None             | Changes reach `develop` and `main` only through pull requests. The maintainer merge tool's signed merge push is accepted because it merges an open pull request whose recorded base is the branch tip. |
| 25068452 | All branches except `dependabot`, `staging`             | Deny force push (`non_fast_forward`)                                                                               | None             | Published history is never rewritten. Kept in its own ruleset so it is never switched off together with a checks rule.                                                                                 |
| 94133    | All branches                                            | Require signed commits                                                                                             | None             | Every commit is attributable to a verified key; GitHub's "Update branch" commits are signed by GitHub and pass.                                                                                        |
| 86945    | `releases/**`                                           | Deny create; deny delete                                                                                           | Maintain, always | Only maintainers create or delete release branches.                                                                                                                                                    |
| 94096    | `releases/**`                                           | Require deployment `dockerhub-torrust`; deny update                                                                | None             | A release branch is immutable after creation and is published to Docker Hub.                                                                                                                           |
| 90587    | `staging/**`                                            | Deny create, update, delete, and force push                                                                        | Maintain, always | Only maintainers drive the `develop` → `staging/main` → `main` release pipeline.                                                                                                                       |
| 87069    | Tags except `v*.*.*`                                    | Deny create                                                                                                        | None             | Only version tags exist.                                                                                                                                                                               |
| 94135    | All tags                                                | Require signed; deny update, delete, and force push                                                                | None             | Published tags are signed and immutable.                                                                                                                                                               |
| 94130    | `v*.*.*` tags                                           | Deny create                                                                                                        | Maintain, always | Only maintainers create version tags.                                                                                                                                                                  |

Rulesets 94237 (force-push ban with 16 required checks, disabled since 2026-05-05), 19245352
(Copilot code review, created and disabled by GitHub), and 87098 (no rules) were deleted on
2026-10-14; their full definitions are kept with the issue #2507 evidence.

## Checking for Drift

List the live rulesets and the rules that apply to a branch:

```sh
gh api repos/torrust/torrust-tracker/rulesets --jq '.[] | "\(.id) \(.enforcement) \(.name)"'
gh api repos/torrust/torrust-tracker/rules/branches/develop --jq '[.[].type] | join(",")'
```

Every listed ruleset must be `active` and appear in the table above. For `develop`, the expected
rules are `creation,pull_request,deletion,non_fast_forward,required_signatures`.
