---
doc-type: issue
issue-type: bug
status: planned
priority: p2
epic: null
github-issue: 1029
spec-path: docs/issues/open/1029-do-not-publish-docker-tags-with-v-prefix.md
branch: "1029-do-not-publish-docker-tags-with-v-prefix"
related-pr: 2111
last-updated-utc: 2026-08-28 14:59
semantic-links:
  skill-links:
    - create-issue
  related-artifacts:
    - .github/workflows/container.yaml
    - docs/containers.md
    - docs/release_process.md
---

# Issue #1029 - Do not publish Docker tags with the `v` prefix

## Goal

Publish each release container image with the intended unprefixed semantic-version tags only.
Do not publish additional tags that retain the release branch's `v` prefix.

## Background

The release-container workflow derives Docker Hub tags from a release branch version, whose
format is `releases/v<semver>`. The Docker Hub repository currently contains duplicate image
tags for the same release: one set without the `v` prefix and another with it.

The current `publish_release` job in `.github/workflows/container.yaml` configures
`docker/metadata-action` with both `pattern={{raw}}` and `pattern={{version}}`. For a version
such as `v3.0.0`, the raw pattern preserves the prefix (`v3.0.0`) while the version pattern
produces the unprefixed tag (`3.0.0`). This configuration is the likely source of the duplicate
versioned tags reported in the original GitHub issue.

## Scope

### In Scope

- Update the release Docker metadata configuration so it does not create a full-version tag with
  the `v` prefix.
- Preserve the intended release-tag policy for unprefixed full-version, major-version, and
  major-minor-version tags, plus `latest` for the newest stable release.
- Publish major (`<major>`) and major-minor (`<major>.<minor>`) tags only for stable releases;
  prereleases publish only their unprefixed full-version tag.
- Document the release Docker-tag policy in `docs/release_process.md` and add a concise,
  adjacent explanation to the workflow metadata configuration.

### Out of Scope

- Deleting, changing tags on, or otherwise modifying already-published Docker Hub images.
- Changes to development image tags such as `develop`.
- Removing the existing `latest` tag or changing its meaning as the newest stable release.
- Redesigning release branch naming or the broader release process.
- Publishing additional Docker registries or multi-architecture images.

## Architectural Decisions

- Related ADRs: None known.
- ADRs to create: None expected. This is a CI configuration correction; create an ADR only if
  implementation reveals a broader, durable container-versioning decision.

## Implementation Plan

Status values: `TODO`, `IN_PROGRESS`, `BLOCKED`, `DONE`.

| ID  | Status | Task                                   | Notes / Expected Output                                                                                                       |
| --- | ------ | -------------------------------------- | ----------------------------------------------------------------------------------------------------------------------------- |
| T1  | DONE   | Correct release metadata configuration | Removed `{{raw}}`, changed `v{{major}}` to `{{major}}`, and retained the unprefixed full-version and major-minor rules.       |
| T2  | DONE   | Document the tag policy                | Added the canonical tag matrix and mutable-tag guidance to `docs/release_process.md` and a concise adjacent workflow comment. |
| T3  | DONE   | Validate generated metadata            | Verified the configured SemVer patterns against the metadata-action's documented stable and prerelease behavior.              |
| T4  | TODO   | Verify the next publication            | Inspect Docker Hub after the next stable release and record the published tags.                                               |
| T5  | DONE   | Run quality gates                      | The mandatory pre-commit gate passed.                                                                                         |

## Progress Tracking

### Workflow Checkpoints

- [x] Specification reconstructed from existing GitHub issue #1029.
- [x] Specification reviewed and approved by user/maintainer.
- [x] Spec-only PR opened: https://github.com/torrust/torrust-tracker/pull/2110
- [x] Spec-only PR merged into `develop` before implementation.
- [ ] Implementation PR opened: https://github.com/torrust/torrust-tracker/pull/2111
- [ ] Implementation completed.
- [ ] Automatic verification completed (`linter all`, relevant tests, and pre-push checks when applicable).
- [ ] Manual verification scenarios executed and recorded (status + evidence).
- [ ] Acceptance criteria reviewed after implementation and updated with evidence.
- [ ] Reviewer validated acceptance criteria and updated checkboxes.
- [ ] Committer verified spec progress is up to date before commit.
- [ ] GitHub issue closed and spec moved from `docs/issues/open/` to `docs/issues/closed/`.

### Progress Log

- 2026-08-28 00:00 UTC - GitHub Copilot - Reconstructed this repository-backed specification
  from GitHub issue #1029 and current `.github/workflows/container.yaml` metadata rules.
- 2026-08-28 09:38 UTC - User - Confirmed that major and major-minor tags are reserved for
  stable releases; existing `v`-prefixed Docker Hub tags remain historical artifacts.
- 2026-08-28 11:34 UTC - GitHub Copilot - Verified Docker Hub publishes `latest`; it was last
  updated by the `v3.0.0` stable release on 2024-10-02. Retained `latest` as the newest stable
  release tag because it is an existing public contract and is not part of the duplicate-tag fix.
- 2026-08-28 11:34 UTC - User - Approved refining the specification with the verified tag policy
  and implementation sequence.
- 2026-08-28 12:13 UTC - GitHub Copilot - Opened spec-only PR #2110; the Docs Lint workflow
  completed successfully.
- 2026-08-28 14:37 UTC - User - Merged spec-only PR #2110 into `develop`.
- 2026-08-28 14:39 UTC - GitHub Copilot - Began implementation on branch
  `1029-do-not-publish-docker-tags-with-v-prefix-implementation`; corrected the release metadata
  rules and documented the published image-tag policy.
- 2026-08-28 14:46 UTC - GitHub Copilot - Verified the configured SemVer patterns against the
  metadata-action documentation and ran the mandatory pre-commit gate successfully. The first
  subsequent stable release remains required to verify the published Docker Hub tags.
- 2026-08-28 14:59 UTC - GitHub Copilot - Opened implementation PR #2111 targeting `develop`.

## Acceptance Criteria

- [ ] AC1: A stable release input version of `v3.0.0` produces `3.0.0`, `3.0`, `3`, and `latest`
      for the release image.
- [ ] AC2: The release workflow does not produce a `v3.0.0` Docker tag.
- [ ] AC3: The release workflow does not produce `v`-prefixed major or major-minor Docker tags
      such as `v3` or `v3.0`.
- [ ] AC4: A prerelease input version such as `v3.1.0-rc.1` produces only `3.1.0-rc.1`; it does
      not update the `3`, `3.1`, or `latest` tags.
- [ ] AC5: Development branch image tagging remains `develop` for `develop` and `main` for `main`.
- [ ] AC6: `docs/release_process.md` defines the release Docker-tag policy, including stable,
      prerelease, development, and `latest` behavior; the workflow contains a concise adjacent
      explanation of the `v`-prefix translation.
- [ ] `linter all` exits with code `0`.
- [ ] Relevant tests pass.
- [ ] Manual verification scenarios are executed and documented (status + evidence).
- [ ] Acceptance criteria are re-reviewed after implementation and reflect actual behavior.

## Verification Plan

Define verification before implementation starts and execute it before closing the issue.

### Automatic Checks

- `linter all`
- Validate `.github/workflows/container.yaml` syntax and the release tag-generation logic.
- Run pre-push checks when preparing the implementation branch for push.

### Manual Verification Scenarios

Status values: `TODO`, `IN_PROGRESS`, `DONE`, `FAILED`, `BLOCKED`.

| ID  | Scenario                      | Command/Steps                                                                    | Expected Result                                                                    | Status | Evidence                                             |
| --- | ----------------------------- | -------------------------------------------------------------------------------- | ---------------------------------------------------------------------------------- | ------ | ---------------------------------------------------- |
| M1  | Stable release tag generation | Review the configured patterns against the metadata-action SemVer documentation. | Tags are `3.0.0`, `3.0`, `3`, and `latest`; no tag has a `v` prefix.               | DONE   | https://github.com/docker/metadata-action#typesemver |
| M2  | Prerelease tag generation     | Review the configured patterns against the metadata-action SemVer documentation. | Tags contain only `3.1.0-rc.1`; `3`, `3.1`, and `latest` are absent.               | DONE   | https://github.com/docker/metadata-action#typesemver |
| M3  | Development tag generation    | Review the unchanged development metadata configuration.                         | Generated tags remain `develop` for `develop` and `main` for `main`.               | DONE   | `.github/workflows/container.yaml`                   |
| M4  | Published release inspection  | After the next stable release, inspect Docker Hub's tag list.                    | The release publishes the stable tag matrix with no new `v`-prefixed version tags. | TODO   |                                                      |

Notes:

- Manual verification is mandatory even when automated tests pass.
- If a scenario fails, record the failure and diagnosis in the progress log before proceeding.

### Acceptance Verification

| AC ID | Status (`TODO`/`DONE`) | Evidence |
| ----- | ---------------------- | -------- |
| AC1   | TODO                   |          |
| AC2   | TODO                   |          |
| AC3   | TODO                   |          |
| AC4   | TODO                   |          |
| AC5   | TODO                   |          |
| AC6   | TODO                   |          |

## Risks and Trade-offs

- Removing the wrong metadata rule could unintentionally remove useful unprefixed tags.
  - Mitigation: capture and validate the expected tag matrix before and after the change.
- `latest`, major, and major-minor tags are mutable and do not provide repeatable deployments.
  - Mitigation: document that users requiring repeatability must select a full version tag or
    immutable image digest; define `latest` as the newest stable release only.
- The workflow can validate generated tags without proving registry publication behavior.
  - Mitigation: inspect Docker Hub after the first release using the corrected workflow.

## References

- GitHub issue: https://github.com/torrust/torrust-tracker/issues/1029
- Release container workflow: `.github/workflows/container.yaml`
- Container documentation: `docs/containers.md`
