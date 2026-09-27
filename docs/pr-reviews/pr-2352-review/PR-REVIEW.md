---
semantic-links:
  skill-links:
    - process-pr-review
  related-artifacts:
    - .github/skills/dev/pr-reviews/process-pr-review/SKILL.md
    - docs/issues/open/2323-1840-hetzner-self-hosted-ci-runner/ISSUE.md
---

<!-- skill-link: process-pr-review -->

# PR #2352 Review Audit

Source: pull-request reviews and inline review threads for
<https://github.com/torrust/torrust-tracker/pull/2352>.

## Ownership

The PR author owns this tracked audit record. Reviewers, including repository review agents,
deliver findings through GitHub and have no repository-artifact obligation.

- Post-merge workflow approval: N/A

## Status Values

- Relationship: `ORIGINAL`, `RE_RAISE_OF:<FindingId>`
- Disposition: `FIXED`, `NO_ACTION`, `SUPERSEDED`, `FOLLOW_UP`
- Thread state: `OPEN`, `RESOLVED`, `NON_RESOLVABLE`, `SUPERSEDED`. `OPEN` applies prospectively
  to audits created or updated for approved follow-up work; historical audits remain valid without
  bulk migration.
- Severity: `Blocker`, `Major`, `Minor`, `Nit`, `Suggestion`; append `(inferred)` when derived
  from free prose.
- Author class: `Copilot`, `Human`, `Unknown`
- Category: `link-integrity`, `formatting`, `metadata`, `testing`, `correctness`,
  `documentation`, `maintainability`, `security`, `other`
- An outdated thread whose concern was fixed is `FIXED`/`RESOLVED`, even when GitHub marks the
  original thread outdated after the push. For in-PR feedback, use `NO_ACTION`/`SUPERSEDED` only
  for a duplicate, superseded, or no-change concern. A post-merge `NO_ACTION` requires maintainer
  approval to decline the follow-up work.

## Findings

| Finding ID | Review finding reference | Author class | Severity | Category | Relationship | Disposition | Thread state |
| ---------- | ------------------------ | ------------ | -------- | -------- | ------------ | ----------- | ------------ |
| F1 | `review-finding:pr-2352-f1` | Copilot | Minor (inferred) | maintainability | ORIGINAL | NO_ACTION | SUPERSEDED |
| F2 | `review-finding:pr-2352-f2` | Copilot | Minor (inferred) | maintainability | ORIGINAL | NO_ACTION | SUPERSEDED |
| F3 | `review-finding:pr-2352-f3` | Copilot | Minor (inferred) | documentation | ORIGINAL | NO_ACTION | SUPERSEDED |
| F4 | `review-finding:pr-2352-f4` | Copilot | Nit (inferred) | maintainability | ORIGINAL | FIXED | RESOLVED |

## Finding Details

### F1 - Empty cache inputs to `docker/build-push-action` in `container.yaml`

- PR number: 2352
- Source review ID: 5329846709
- Reviewer finding ID: N/A
- Source URL: <https://github.com/torrust/torrust-tracker/pull/2352#discussion_r4114918043>
- Concern: passing `''` for `cache-from`/`cache-to` on the self-hosted runner could be parsed
  incorrectly by some action versions; split the build into two `if:`-gated steps instead.
- Solution: no change. `docker/build-push-action` v7.4.0 declares both inputs optional with no
  default, so `''` is the value the action receives when the input is omitted. Two gated steps
  would duplicate the build configuration for no behavioral difference.
- Current-tree verification: `action.yml` at tag `v7.4.0` defines `cache-from` and `cache-to`
  with `required: false` and no `default`; the self-hosted job 108594972392 (`Container` run
  36310358131) issued `docker buildx build` with no `--cache-from` or `--cache-to` flag (0 cache
  lines in its log).
- Resolution reference: <https://github.com/torrust/torrust-tracker/pull/2352#discussion_r4116388462>
- Follow-up PR URL: N/A
- Reply URL: <https://github.com/torrust/torrust-tracker/pull/2352#discussion_r4116388462>

### F2 - Empty cache inputs to `docker/build-push-action` in `testing.yaml`

- PR number: 2352
- Source review ID: 5329846709
- Reviewer finding ID: N/A
- Source URL: <https://github.com/torrust/torrust-tracker/pull/2352#discussion_r4114918052>
- Concern: the same `''` cache inputs in the `docker-e2e` build step.
- Solution: no change, for the reasons recorded in F1; the step uses the same action version.
- Current-tree verification: `testing.yaml` `docker-e2e` uses `docker/build-push-action@v7.4.0`
  with the same `runner.environment` expressions as `container.yaml`.
- Resolution reference: <https://github.com/torrust/torrust-tracker/pull/2352#discussion_r4116388535>
- Follow-up PR URL: N/A
- Reply URL: <https://github.com/torrust/torrust-tracker/pull/2352#discussion_r4116388535>

### F3 - `docker volume prune` scope in the runner guide

- PR number: 2352
- Source review ID: 5329846709
- Reviewer finding ID: N/A
- Source URL: <https://github.com/torrust/torrust-tracker/pull/2352#discussion_r4114918069>
- Concern: `docker volume prune --force` removes all unused volumes, including named ones, while
  the guide says it removes anonymous volumes.
- Solution: no change. Since Docker Engine 23, `docker volume prune` without `--all` removes only
  unused anonymous volumes, so the guide is accurate for the command the prune service runs.
- Current-tree verification: on `torrust-runner-01` (Docker API 1.56), `docker volume prune
  --help` lists `-a, --all  Remove all unused volumes, not just anonymous ones`; the service in
  `docs/self-hosted-runner.md` step 10 runs `docker volume prune --force` without `--all`.
- Resolution reference: <https://github.com/torrust/torrust-tracker/pull/2352#discussion_r4116388589>
- Follow-up PR URL: N/A
- Reply URL: <https://github.com/torrust/torrust-tracker/pull/2352#discussion_r4116388589>

### F4 - Make `docker-e2e` routing branch on the event name

- PR number: 2352
- Source review ID: 5329846709
- Reviewer finding ID: N/A
- Source URL: <https://github.com/torrust/torrust-tracker/pull/2352#discussion_r4114918079>
- Concern: the `runs-on` expression applied `github.event.pull_request.user.login` to every event
  and relied on it being null on pushes; branching on `github.event_name` is clearer to audit.
- Solution: the expression now branches on `pull_request` and `push`, with any other event on
  `ubuntu-latest`.
- Current-tree verification: `testing.yaml` `docker-e2e` `runs-on` contains
  `github.event_name == 'pull_request'` and `github.event_name == 'push'` branches; `actionlint`
  exits 0; the workflow triggers only on `push` and `pull_request`.
- Resolution reference: ci(testing): branch docker-e2e routing on the event name
- Follow-up PR URL: N/A
- Reply URL: <https://github.com/torrust/torrust-tracker/pull/2352#discussion_r4116388663>

## Processing Log

- 2026-09-27 18:09 UTC - Fetched Copilot review 5329846709 (submitted 2026-09-27 10:01 UTC; four
  inline threads; the review body summarizes the same four findings and notes that Copilot's
  agentic review timed out). No severity brackets, so severities are inferred from the review's
  medium and low labels. Verified F1 to F3 against `action.yml` at `v7.4.0`, the self-hosted job
  log, and the runner host's Docker; fixed F4 in its own commit; rebased, pushed, and replied on
  all four threads.

## Completion Rules

- Re-derive the reply claim against the current tree before replying or resolving a thread.
- Reply on every resolvable thread before resolving it.
- For an outdated thread whose concern was fixed, record `Disposition=FIXED` and
  `Thread state=RESOLVED`, even if GitHub marks the thread outdated after the push. For a
  duplicate, superseded, or no-change in-PR thread, reply exactly
  `Superseded by <FindingId>: <reason>.`, record `Disposition=NO_ACTION` and
  `Thread state=SUPERSEDED`, then resolve it. A post-merge `NO_ACTION` requires maintainer
  approval to decline the follow-up work.
- A consolidated PR conversation response may cover multiple review rounds only when it names
  every review ID and every finding ID with its disposition and resolution reference. Record its
  durable URL in each related row.
- Cite a fix by its unique Conventional Commit subject or durable reply URL, never by a branch SHA
  that can change after a rebase.
- Refresh review threads using GraphQL and confirm that no unresolved actionable thread remains.
