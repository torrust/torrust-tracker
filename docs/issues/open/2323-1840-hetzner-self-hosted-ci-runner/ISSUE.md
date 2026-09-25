---
schema-version: 1
doc-type: issue
issue-type: task
status: open
priority: p1
epic: 1840
github-issue: 2323
spec-path: docs/issues/open/2323-1840-hetzner-self-hosted-ci-runner/ISSUE.md
branch: "2323-1840-hetzner-self-hosted-ci-runner-spec"
related-pr: 2335
last-updated-utc: 2026-09-24 17:05
semantic-links:
  skill-links:
    - create-issue
  related-artifacts:
    - .github/workflows/container.yaml
    - .github/workflows/testing.yaml
    - .github/workflows/coverage.yaml
    - .github/skills/dev/planning/create-issue/SKILL.md
    - docs/issues/open/1840-improve-pr-workflow-performance-epic/EPIC.md
    - docs/issues/drafts/1840-workflow-performance-buildkit-cargo-cache-mounts/ISSUE.md
    - docs/adrs/20260612000000_adopt_sccache_for_ci_bare_builds.md
---

<!-- skill-link: create-issue -->

# Issue #2323 - Offload the Container Test Job to a Self-Hosted Hetzner Runner

Parent EPIC: #1840 - Improve PR Workflow Performance

> **Design status (2026-09-24): under review.** Copilot finding PERSISTENT-RUNNER-PRIVILEGE and
> reviewer finding F1 on PR #2335 showed that a
> persistent self-hosted runner lets fork-PR code gain root on the host and persistently compromise
> the runner. The maintainer ruled that unacceptable. The research in
> [`self-hosted-runner-security-research.md`](self-hosted-runner-security-research.md) found no
> safe persistent-runner setup for fork PRs, and the maintainer is leaning towards GitHub larger
> runners on GitHub Team instead. `torrust-runner-01` is stopped and disabled. The plan from T4 on
> is on hold until this specification is rewritten around the chosen approach; the sections below
> still describe the original Hetzner design.

## Goal

Run the heaviest CI job, the container image build and E2E test, on a paid self-hosted runner
hosted at Hetzner, while the remaining workflows and the image publishing jobs stay on the free
GitHub-hosted runners. The target is to reduce the wall-clock time a PR waits for required checks
from roughly 40 minutes to roughly 15 minutes.

This issue changes where the workflow runs, not what it runs. It does not optimize the build
itself. The expected gain comes from two effects:

- a larger machine that is not shared with the free-runner queue; and
- a persistent runner, unlike ephemeral GitHub-hosted runners, so Docker layers, the Cargo
  registry and git caches, and other build state can be kept between jobs.

## Background

Issue #2323 was opened on GitHub by a Torrust organization member before this specification
existed. This document formalizes it; the GitHub issue remains the source of the decision record.

Several AI agents (up to six at once) now produce PRs in parallel. Every PR must pass the required
checks before the maintainer merge tool merges it, and each merge into `develop` forces the
remaining PRs to rebase and rerun their checks. With a check cycle of about 40 minutes, this
creates a domino effect that delays every queued PR.

Decision summary from the issue:

- Upgrading to paid GitHub-hosted larger runners was studied and rejected. The organization is on
  GitHub Free. Its public-repository usage of standard runners, worth about 900-1000 USD per
  month at list price, is not billed. Moving to GitHub Team would not make that usage billable:
  standard runners stay free for public repositories on every plan. Only the larger-runner minutes
  would be billed, plus the Team seats. The estimate for running only the `Container` workflow's
  slow job on a larger runner is about 200 USD per month, well above a comparable Hetzner server.
- Workflows are split: lightweight workflows stay on the free GitHub-hosted runners, and the
  bottleneck, the container image build and test run at the end of the pipeline, moves to a
  self-hosted runner on a Hetzner server.
- Expected outcome: merge-cycle time drops from about 40 to about 15 minutes.
- Planned implementation start: 2026-09-25, by @josecelano.

GitHub billing facts (checked on 2026-09-24 against
<https://docs.github.com/en/billing/concepts/product-billing/github-actions>):

- Actions usage is free for self-hosted runners and for public repositories using standard
  GitHub-hosted runners.
- Larger runners are always charged, even for public repositories and even with plan quota
  available. They are blocked until a payment method is set up.

Hetzner candidates reviewed by the maintainer:

| Location            | vCPU | RAM   | Disk   | Price         |
| ------------------- | ---- | ----- | ------ | ------------- |
| Germany             | 8    | 16 GB | 320 GB | about €60/mo  |
| USA                 | 4    | 8 GB  | 160 GB | about €60/mo  |

For the same price, the US location offers half the resources.

Selected server (provisioning started 2026-09-24):

| Property                  | Value                        |
| ------------------------- | ---------------------------- |
| Location                  | Falkenstein, Germany         |
| vCPU                      | 8                            |
| RAM                       | 16 GB                        |
| Disk                      | 320 GB local                 |
| Included outgoing traffic | 20 TB per month              |
| Price                     | €69.49 per month             |
| Server type               | To be recorded in T2         |

An earlier cost analysis (2026-09-16) is kept in
[`larger-runner-vs-self-hosted-cost-analysis.md`](larger-runner-vs-self-hosted-cost-analysis.md).
Its timing breakdown and pricing tables remain useful input: `Build Tracker Image` took about 28
of the 35 minutes of `Test (Docker)`, and there were 378 PR-triggered `Container` runs in 30 days.
Its recommendation (a GitHub-hosted larger runner, rejecting self-hosted) was superseded by the
later review recorded in this issue, which weighed the organization's current free subsidized
usage and Hetzner pricing.

Relevant current state (on `develop` at `21017bb6`):

- `.github/workflows/container.yaml` job `test` (`Test (Docker)`) runs on `ubuntu-latest` with a
  90-minute timeout. It builds the `release` Containerfile target with BuildKit `type=gha` cache,
  then runs the persistence-transition regression, the E2E runner, and three qBittorrent E2E runs
  (SQLite, MySQL, PostgreSQL). Unit tests run inside the Containerfile build (ADR
  `20260603000000_keep_unit_tests_inside_container_build.md`).
- `.github/workflows/testing.yaml` job `docker-e2e` builds the same image, but is skipped for PRs
  targeting `develop`/`main` and for pushes to `develop`/`main`/`releases/**` (issue #1854).
- `container.yaml` ignores `**/*.md` and `project-words.txt`; `coverage.yaml` has no path filter.
- `container.yaml` publish jobs (`publish_development`, `publish_release`) are separate jobs on
  `ubuntu-latest`. They are the only jobs that use the Docker Hub credentials, through the
  `dockerhub-torrust` environment, and they only run on pushes, never on PRs. They read the
  `container-release` GHA cache scope written by the `test` job before their own scope.

Runner-load evidence posted by @da2ce7 on the issue (last 100 `develop` runs, 2026-09-22T17:43Z to
2026-09-24T09:40Z, 18 head commits):

| Workflow                    | Runs | Result                 |
| --------------------------- | ---- | ---------------------- |
| Coverage                    | 18   | 17 success, 1 failure  |
| Docs Lint                   | 18   | success                |
| CodeQL Code Quality         | 18   | success                |
| Running Copilot Code Review | 18   | 11 success, 7 canceled |
| OS Compatibility            | 6    | success                |
| Testing                     | 5    | success                |
| Container                   | 5    | success                |
| Security Scan               | 4    | success                |
| Dependabot update runs      | 8    | success                |

The comment observes that 12 of 18 heads were documentation-only (so `Container` was skipped),
that `Coverage` runs on every `develop` push including documentation-only ones, and asks whether
the Coverage failure on `78d7e0fe` (a Markdown-only merge) was a flake, an infrastructure error,
or a regression.

The Coverage observations in that comment are out of scope for this issue (see Out of Scope).

This issue is a subissue of EPIC #1840. The EPIC's draft
`docs/issues/drafts/1840-workflow-performance-buildkit-cargo-cache-mounts/ISSUE.md` found that
BuildKit cache mounts bring no CI benefit on ephemeral GitHub-hosted runners; a persistent
self-hosted runner removes that limitation.

## Alternatives Considered

Both options move only the `Test (Docker)` job; every other workflow stays on the free standard
GitHub-hosted runners. Run volume and the per-core estimates come from
[`larger-runner-vs-self-hosted-cost-analysis.md`](larger-runner-vs-self-hosted-cost-analysis.md)
(2026-09-16); the selected server price comes from the Hetzner console. The ADR must recheck both.

| Aspect               | GitHub-hosted larger runner                                                                     | Self-hosted runner on Hetzner                                                                                 |
| -------------------- | ----------------------------------------------------------------------------------------------- | ------------------------------------------------------------------------------------------------------------- |
| GitHub plan          | Con: requires GitHub Team or Enterprise Cloud, plus a per-seat cost.                            | Pro: works on the current GitHub Free plan.                                                                   |
| Billing model        | Con: billed per minute from the first minute, even for public repos. Standard runners stay free. | Pro: GitHub does not bill self-hosted minutes. Con: flat server cost, paid even when idle.                    |
| Cost as volume grows | Con: grows linearly with runs; agent-driven PR volume is rising (378 PR runs in 30 days).       | Pro: flat until capacity runs out, so the cost per run falls as volume grows.                                 |
| Estimated cost       | About 200 USD per month for the `Container` job alone (maintainer estimate).                     | €69.49 per month for the selected 8 vCPU / 16 GB server, with 20 TB outgoing traffic included.                |
| Build caches         | Con: ephemeral VM; caches are re-downloaded from the network (`type=gha`) on every job.         | Pro: persistent host keeps Docker layers, Cargo registry and git caches, and BuildKit cache mounts.           |
| Hardware             | Con: limited to GitHub's catalog.                                                               | Pro: choose dedicated vCPU, RAM, and disk.                                                                    |
| Concurrency          | Pro: scales with parallel jobs without sizing a server.                                         | Con: bounded by the number of runner instances; can become the new bottleneck.                                |
| Availability         | Pro: managed by GitHub.                                                                         | Con: single point of failure unless redundant; needs a fallback.                                              |
| Operations           | Pro: nothing to operate.                                                                        | Con: maintainers own patching, monitoring, runner upgrades, and disk cleanup.                                 |
| Security             | Pro: fork-PR code runs on a clean GitHub VM per job; no Torrust infrastructure is exposed.      | Con: fork-PR code gains root-equivalent control of a persistent host through the `docker` group, which persists into later jobs (see Risks). |
| Workflow change      | Pro: `runs-on` label swap; the existing cache setup keeps working.                              | Con: `runs-on` swap plus cache reconfiguration and publish-job cache isolation.                               |
| Network locality     | Pro: runs next to GitHub's cache and artifact services.                                         | Con: every GitHub cache import/export and artifact transfer crosses the internet from Hetzner to GitHub.     |

Decision: self-hosted on Hetzner. The selected server costs about a third of the estimated
larger-runner bill, its cost stays flat as agent-driven volume grows, it does not require a paid
GitHub plan, and a persistent host enables local caches that an ephemeral runner cannot keep. The
earlier analysis found the two options roughly on par only because it priced a more expensive
dedicated-vCPU server (CCX33, about $151 per month). The security and operations costs are
accepted and mitigated as described in Risks and Trade-offs. The ADR (T4) records this comparison
with rechecked prices.

## Scope

### In Scope

- Capture a reproducible baseline of PR wall-clock time and `Test (Docker)` job duration on the
  GitHub-hosted runner.
- Register a self-hosted GitHub Actions runner on a Hetzner server for `torrust/torrust-tracker`,
  registered at repository level (custom organization runner groups require GitHub Team) with a
  dedicated label.
- Move the `container.yaml` `test` job to the self-hosted runner label.
- Configure the persistent runner to keep build caches between jobs (Docker/BuildKit layers,
  Cargo registry and git caches), with a disk-usage cleanup policy.
- Keep the publish jobs on GitHub-hosted runners, isolated from state produced on the
  self-hosted runner (see Risks and Trade-offs).
- Document the accepted security model for running fork-PR workflows on a self-hosted runner.
- Define the fallback when the self-hosted runner is unavailable.
- Measure after the switch and record the before/after comparison.
- Document the runner for maintainers: purpose, label, operational owner, how to recover it.
- Record the decision in an ADR.

### Out of Scope

- Moving any other workflow or job to self-hosted runners (candidates may be noted as follow-ups).
- Moving the Docker Hub publish jobs to the self-hosted runner.
- A self-hosted container registry, package mirrors, or other cache services beyond the local
  caches in T5 (follow-up if scenario B appears).
- Upgrading to paid GitHub-hosted (larger) runners.
- Changing the required-check policy or the merge tool.
- Further Containerfile build optimizations tracked by EPIC #1840.
- Adding a path filter to `coverage.yaml` or diagnosing the `78d7e0fe` Coverage failure raised in
  the issue comments. They are unrelated to the runner change.

## Architectural Decisions

- Related ADRs:
  - `docs/adrs/20260612000000_adopt_sccache_for_ci_bare_builds.md` (non-sticky GHA runner cache
    assumptions)
  - `docs/adrs/20260603000000_keep_unit_tests_inside_container_build.md`
- ADRs to create:
  - Adopt a self-hosted Hetzner runner for the container test job (root `docs/adrs/`, because it
    is repository-wide CI infrastructure). It must cover the cost rationale, the persistent
    runner choice, the accepted security risk, publish-job isolation, the cache strategy, and
    the fallback.

## Design and Ownership Review

Not applicable in the sense of child processes or reusable test fixtures. The equivalent
ownership decisions for this infrastructure are:

- **Server owner**: who pays for, patches, and monitors the Hetzner server.
- **Runner lifecycle**: persistent (decided). The runner is not ephemeral, so build state survives
  between jobs. This is the source of the cache gain and also of the residual risk described in
  Risks and Trade-offs.
- **Runner concurrency**: one runner process executes one job at a time. The number of runner
  instances on the server bounds how many `Test (Docker)` jobs can run in parallel.
- **Cache ownership**: the self-hosted `test` job owns a local cache on the server (Docker/BuildKit
  layers, Cargo registry and git caches). Nothing produced on the self-hosted runner is an input
  to the publish jobs. A cleanup policy bounds disk usage.
- **Credential ownership**: Docker Hub credentials stay in the `dockerhub-torrust` environment and
  are used only by the publish jobs on GitHub-hosted runners. The server stores no Torrust
  credentials other than the runner registration.
- **Deadline**: `timeout-minutes` bounds only execution time, after a runner has picked up the
  job; it does not bound time spent queued. GitHub cancels a self-hosted job only after 24 hours
  in the queue (<https://docs.github.com/en/actions/reference/limits>, checked 2026-09-24). An
  offline runner therefore needs explicit handling: offline detection with an alert to
  maintainers, and a documented fallback procedure (for example, moving the job back to
  `ubuntu-latest`). T5 and T8 define both.
- **Network locality**: GitHub's cache and artifact services run on GitHub's own infrastructure
  (reported to be Microsoft Azure). A Hetzner runner reaches them over the public internet. The
  `test` job depends on them through `Swatinem/rust-cache` and the BuildKit `type=gha` cache
  (`cache-from` and `cache-to` with `mode=max`, which exports every intermediate layer). It also
  pulls from crates.io and Docker Hub. The design must measure this traffic before assuming the
  faster machine gives a net gain.

## Bug-Fix Process

Not applicable. This is a CI capacity and performance task, not a defect.

## Regression Test Strategy

Not applicable. This is not bug work. Verification is by workflow-run evidence (see Verification
Plan).

## Implementation Plan

Status values: `TODO`, `IN_PROGRESS`, `BLOCKED`, `DONE`.

Delivery phases:

1. **Spec PR** (branch `2323-1840-hetzner-self-hosted-ci-runner-spec`): this specification, the
   server preparation (T2), and the runner installation and registration (T3). T2 and T3 change
   infrastructure only; the repository receives their setup logs. The runner is registered but
   no workflow uses it yet.
2. **Implementation PR** (branch `2323-1840-hetzner-self-hosted-ci-runner`): ADR, all workflow
   changes, validation, measurement, and runner operations documentation (T4-T8). The baseline
   (T1) was recorded in the spec PR.
3. **Scenario follow-up** (same implementation PR, or a follow-up subissue of #1840 if the work
   is large): apply the remedies selected from the measured scenario (see Post-Switch Scenarios).

| ID  | Phase | Status | Task                                  | Notes / Expected Output                                                                                                         |
| --- | ----- | ------ | ------------------------------------- | ------------------------------------------------------------------------------------------------------------------------------- |
| T1  | 1     | DONE   | Record baseline                       | Recorded in [`benchmark-results.md`](benchmark-results.md): `Test (Docker)` median 37 min; build step median 32 min, of which workspace compile 15.5-18.6 min and GitHub cache export 5-12 min; third-party layer missed in 4 of 6 runs; cache over its 10 GB allowance; `Security Scan` (28-29 min) is the next critical path when it runs. |
| T2  | 1     | DONE   | Prepare Hetzner server                | Falkenstein, 8 vCPU, 16 GB RAM, 320 GB disk, €69.49/month (see Background). Hostname `torrust-runner-01`, Ubuntu 26.04.1 LTS, kernel `7.0.0-34-generic`; SSH key-only login; automatic security updates; Hetzner Cloud Firewall allowing only inbound SSH; Docker Engine 29.8.1 with Buildx and Compose; host build tools; `runner` user in the `docker` group. Host holds no Torrust credentials. Server type and shared or dedicated vCPU remain open (Open Question 1). Logged in [`runner-server-setup.md`](runner-server-setup.md). |
| T3  | 1     | DONE   | Install and register runner           | Runner `v2.337.0` (hash-verified) under `/home/runner/actions-runner`, registered at repository level as `torrust-runner-01` with label `torrust-hetzner`, running as the systemd service `actions.runner.torrust-torrust-tracker.torrust-runner-01` under `runner`, enabled at boot. GitHub reports it `online` and idle. One runner instance for now. Logged in [`runner-agent-installation.md`](runner-agent-installation.md). |
| T4  | 2     | TODO   | Write ADR                             | ADR in `docs/adrs/` covering the decisions listed in Architectural Decisions.                                                   |
| T5  | 2     | TODO   | Change the `container.yaml` workflow  | One change set: (a) the `test` job's `runs-on` uses the self-hosted label, with an adjusted timeout; (b) the job uses caches kept on the server (Docker/BuildKit layers, Cargo registry and git caches) instead of the GitHub Actions cache, with a disk cleanup policy; (c) the publish jobs stay on `ubuntu-latest` and read no cache produced by the self-hosted job. The baseline shows the GitHub cache export already costs 5-12 min per job inside GitHub's network, so the runner switch is not measured separately with the GitHub cache. |
| T6  | 2     | TODO   | Validate on real runs                 | At least one fork PR and one `develop` push run green on the self-hosted runner, and a publish run succeeds.                    |
| T7  | 2, 3  | TODO   | Measure and compare                   | `benchmark-results.md` records the result after T5 (cold and warm cache) and after each phase 3 remedy, against the baseline and the 15-minute target, with the same step breakdown as T1, queue time, and the data volume transferred per job. Identify the matching scenario. |
| T8  | 2     | TODO   | Document runner operations            | Maintainer-facing documentation: purpose, label, owner, cache cleanup, runner-offline detection and alerting, fallback procedure for queued jobs, and recovery steps. |

### Post-Switch Scenarios

After T5, use the T7 measurements to identify the scenario, record it in the progress log, and
plan the remedy. Scenarios can combine. Add new rows when an unexpected scenario appears.

| ID  | Observation                                                                        | Likely cause                                              | Remedy                                                                                                                                                  |
| --- | ---------------------------------------------------------------------------------- | --------------------------------------------------------- | ------------------------------------------------------------------------------------------------------------------------------------------------------- |
| A   | Job time meets the target with the local caches                                    | The bigger, dedicated machine and local caches removed the bottleneck | None required.                                                                                                                                 |
| B   | Compile time drops, but remaining network transfers still take a large share of the job | Network distance for what still leaves the server: checkout, crates.io and Docker Hub downloads, any remaining GitHub cache use | Add local mirrors or a local registry for the slow transfers. Moving to a Hetzner US location is a low-priority alternative: it offers half the resources for a similar price, and the maintainer does not expect a large transfer gain across providers. |
| C   | Compile time is still high on the self-hosted runner                               | Server too small (CPU or RAM)                             | Resize the Hetzner server to a larger type and repeat T7. Check shared versus dedicated vCPU first.                                                     |
| D   | Job time is good, but PRs wait in the queue for the runner                         | Too few runner instances for the agent PR rate            | Add runner instances on the same server if CPU and RAM allow, otherwise add a second server.                                                            |
| E   | Jobs fail or interfere when run on the same host                                   | Persistent state: leftover containers, networks, volumes, or fixed host ports from the E2E and qBittorrent Compose stacks, especially with more than one runner instance | Add pre-job and post-job cleanup; give each runner instance isolated Compose project names and ports, or keep one job per host.                          |
| F   | Tests pass on `ubuntu-latest` but fail on the self-hosted runner                   | Environment differences (Docker or Compose version, kernel, IPv6, installed tools) | Align tool versions with the GitHub-hosted image or fix the test's environment assumptions; record the difference.                                      |
| G   | `Test (Docker)` is fast, but PR wall-clock time stays high                         | Another workflow on the shared runners is now on the critical path. The baseline already shows `Security Scan` at 28-29 min on PRs that touch its trigger paths (uncached `docker build`) | Record the new critical path and open a follow-up subissue of #1840; out of scope for this issue.                                                       |
| H   | The server's disk fills up                                                          | Unbounded Docker and Cargo caches                         | Scheduled pruning with a size limit (part of T5 and T8).                                                                                                 |

## Commit Points

| Task | Coherent change set                                          | Commit policy                                        |
| ---- | ------------------------------------------------------------ | ---------------------------------------------------- |
| T1   | Baseline evidence document                                   | Commit after the data is reviewed.                   |
| T2   | Server setup log (`runner-server-setup.md`)                  | Commit on the spec branch after each completed setup stage. |
| T3   | Runner agent log (`runner-agent-installation.md`)            | Commit on the spec branch after the runner is registered.   |
| T4   | ADR                                                          | Commit after maintainer review.                      |
| T5   | `container.yaml` changes (runner, local caches, publish isolation) | Commit after focused validation and required review. |
| T6   | Validation evidence in `manual-verification-evidence.md`     | Commit after the runs complete.                      |
| T7   | Updated benchmark evidence                                   | Commit after the comparison is reviewed.             |
| T8   | Documentation                                                | Commit after maintainer review.                      |

T5 is a single change set so that no published image is ever built from self-hosted cache.

Use Conventional Commits with a narrow scope (for example `ci(container)`, `docs(adrs)`,
`docs(issues)`), and sign every commit with GPG.

## Progress Tracking

### Workflow Checkpoints

- [x] GitHub issue created (#2323, opened on GitHub before this specification)
- [x] Folder-style spec drafted in `docs/issues/open/2323-1840-hetzner-self-hosted-ci-runner/ISSUE.md`
- [ ] Spec reviewed and approved by user/maintainer
- [x] Issue linked as a GitHub sub-issue of EPIC #1840
- [x] Server prepared (T2) and runner installed and registered (T3), both logged in this folder
- [ ] Spec-only PR merged into `develop` before implementation
- [ ] Implementation completed
- [ ] Automatic verification completed (`linter all`, workflow lint, pre-push checks when applicable)
- [ ] Manual verification scenarios executed and recorded in issue-local `manual-verification-evidence.md`
- [ ] Acceptance criteria reviewed after implementation and updated with evidence
- [ ] Evidence-based implementation completion review recorded: issue-local retrospective created for material discoveries, or progress log states why none was needed
- [ ] Reviewer validated acceptance criteria and updated checkboxes
- [ ] Independent reviewer reports recorded in issue-local `agent-review-reports.md` when reviewers received this folder-style specification
- [ ] Committer verified spec progress is up to date before commit
- [ ] Issue closed and spec moved from `docs/issues/open/` to `docs/issues/closed/`

### Progress Log

Append one line per meaningful update.

- 2026-09-24 14:58 UTC - GitHub Copilot - Drafted the spec from GitHub issue #2323 and its two comments - this file
- 2026-09-24 15:20 UTC - GitHub Copilot - Maintainer review: made it a subissue of EPIC #1840, chose a persistent runner with local caches, accepted the fork-PR risk for an isolated host, and kept publishing on GitHub-hosted runners - this file
- 2026-09-24 15:35 UTC - GitHub Copilot - Added the cost analysis, the larger-runner versus self-hosted comparison, and the network-locality concern with a post-T5 network checkpoint - this file
- 2026-09-24 15:45 UTC - GitHub Copilot - Recorded the verified GitHub billing model (standard runners stay free for public repos on any plan; larger runners always billed), the ~200 USD/month larger-runner estimate, and the Hetzner candidates - this file
- 2026-09-24 15:50 UTC - josecelano - Started T2: provisioning a Falkenstein server (8 vCPU, 16 GB RAM, 320 GB disk, 20 TB traffic, €69.49/month) before the spec PR is merged; infrastructure only, no repository change - this file
- 2026-09-24 16:05 UTC - GitHub Copilot - Split delivery into three phases (spec PR with server and runner setup; implementation PR; scenario follow-up), made T7 a phase 3 remedy, and replaced the network checkpoint with the Post-Switch Scenarios table (A-H) - this file
- 2026-09-24 16:15 UTC - GitHub Copilot - Completed T1 baseline from 25 recent Container runs and 6 job logs; key findings: GitHub cache export already costs 5-12 min per job on GitHub-hosted runners, the cache is over its 10 GB allowance, and `Security Scan` is the next critical path - [`benchmark-results.md`](benchmark-results.md)
- 2026-09-24 16:25 UTC - josecelano - Decided to include local caches in this issue and to deliver all `container.yaml` changes (runner switch, local caches, publish isolation) as a single task; tasks renumbered: T5 workflow changes, T6 validation, T7 measurement, T8 runner operations docs - this file
- 2026-09-24 17:05 UTC - josecelano, GitHub Copilot - Completed T2 (firewall, Docker, build tools, `runner` user) and T3 (runner `v2.337.0` registered at repository level as `torrust-runner-01`, label `torrust-hetzner`, systemd service online and idle); no workflow uses it yet - [`runner-server-setup.md`](runner-server-setup.md), [`runner-agent-installation.md`](runner-agent-installation.md)
- 2026-09-24 17:15 UTC - josecelano, GitHub Copilot - Opened spec-only PR #2335 and linked #2323 as a GitHub sub-issue of EPIC #1840 - this file
- 2026-09-24 21:30 UTC - josecelano, GitHub Copilot - Copilot finding F2 (fork-PR root through the `docker` group and persistent runner compromise) ruled unacceptable; researched safe alternatives; stopped and disabled `torrust-runner-01`; design under review with GitHub larger runners on Team as the leading option - [`self-hosted-runner-security-research.md`](self-hosted-runner-security-research.md)
- 2026-09-25 06:23 UTC - GitHub Copilot - Review round 1 (Copilot review 5307823915, reviewer review 5310557886): rewrote the fork-PR risk to state the real exposure (root-equivalent, persistent, runner registration, later push jobs, faked publish gate). Correction: the 21:30 entry's "Copilot finding F2" means Copilot's PERSISTENT-RUNNER-PRIVILEGE, not reviewer finding F2 - audit at `docs/pr-reviews/pr-2335-review/PR-REVIEW.md`

## Acceptance Criteria

- [ ] AC1: The `container.yaml` `test` job runs on the self-hosted Hetzner runner for PRs targeting
      `develop` and for pushes to `develop`.
- [ ] AC2: The measured PR check wall-clock time is recorded for the baseline and after the
      workflow changes (cold and warm cache), with the 15-minute target either met or the gap
      explained.
- [ ] AC3: Published images are built only from GitHub-hosted runner state: the publish jobs run on
      GitHub-hosted runners, read no cache produced by the self-hosted `test` job, and the
      self-hosted job references no repository, organization, or environment secrets.
- [ ] AC4: A runner-offline condition is detected and alerts maintainers, and a documented
      fallback procedure moves or reruns queued jobs; the plan does not rely on
      `timeout-minutes`, which does not bound queue time.
- [ ] AC5: An ADR records the decision, cost rationale, persistent-runner choice, accepted security
      risk, publish-job isolation, and cache strategy.
- [ ] AC6: Maintainer-facing documentation describes the runner, its cache cleanup, and its
      operation.
- [ ] `linter all` exits with code `0`
- [ ] Manual verification scenarios are executed and documented in issue-local `manual-verification-evidence.md`
- [ ] Acceptance criteria are re-reviewed after implementation and reflect actual behavior

## Verification Plan

### Automatic Checks

- `linter all` (includes YAML lint of the changed workflow)
- Pre-push checks when applicable
- The `Container` workflow itself, run on the self-hosted runner

### Manual Verification Scenarios

Status values: `TODO`, `IN_PROGRESS`, `DONE`, `FAILED`, `BLOCKED`.

| ID  | Scenario                     | Human-oriented command/steps                                                                   | Expected Result                                                   | Status | Evidence                                     |
| --- | ---------------------------- | ---------------------------------------------------------------------------------------------- | ----------------------------------------------------------------- | ------ | -------------------------------------------- |
| M1  | PR run on self-hosted runner | Open a non-documentation PR from a fork; inspect the `Test (Docker)` job runner name           | Job runs on the Hetzner runner and passes                         | TODO   | `manual-verification-evidence.md` section V1 |
| M2  | `develop` push run           | Merge a non-documentation PR; inspect the `Container` run on `develop`                         | `test` runs on the Hetzner runner; publish job succeeds           | TODO   | `manual-verification-evidence.md` section V2 |
| M3  | Publish isolation            | Inspect the publish job's runner and build log for cache imports                               | Runs on a GitHub-hosted runner; imports no self-hosted cache      | TODO   | `manual-verification-evidence.md` section V3 |
| M4  | Warm cache reuse             | Run the `test` job twice on the same runner with only application code changed                | Second run reuses local Docker layers and Cargo caches            | TODO   | `manual-verification-evidence.md` section V4 |
| M5  | Runner offline               | Stop the runner service; trigger the workflow; wait past the job's `timeout-minutes`           | Job stays queued (not timed out), the offline alert fires, and the documented fallback procedure unblocks the PR | TODO   | `manual-verification-evidence.md` section V5 |
| M6  | Timing comparison            | `gh run list --workflow container.yaml` and job timings for cold-cache and warm-cache runs     | Durations recorded against the baseline and the 15-minute target  | TODO   | `benchmark-results.md`                       |

Notes:

- Record the runner name, job duration, and run URL for every scenario.
- Create `manual-verification-evidence.md` from `docs/templates/MANUAL-VERIFICATION-EVIDENCE.md`.
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

- **Fork-PR code execution on a persistent runner (rejected).** All PRs to this repository come
  from forks, and GitHub advises against self-hosted runners on public repositories because PR
  code runs on the host. The original draft accepted this because the server holds no critical
  data and no Torrust credentials, but that understated the exposure. With a persistent runner and
  the `runner` user in the `docker` group, any fork-PR job that reaches the runner's labels gains
  root-equivalent control of the host (for example with `docker run -v /:/host`), and whatever it
  installs persists into later jobs:
  - the runner registration lives in the install directory, owned by the same uid as job code,
    so a job can read or replace it;
  - later `push` jobs on the same host (moving the `test` job also brings pushes to `develop`,
    `main`, and `releases/**` and PRs to `main`) receive their `GITHUB_TOKEN` and Actions cache
    token, which an implant can use;
  - the `test` job gates the publish jobs through `needs:`, so an implant can fake a passing
    gate on a push.

  A fork PR can reach the runner even if no repository workflow targets it, by adding its own
  workflow. The maintainer ruled this exposure unacceptable, so this design is rejected; see the
  design-status note and
  [`self-hosted-runner-security-research.md`](self-hosted-runner-security-research.md).
- **State poisoning through the persistent cache.** Because the runner is persistent, a fork-PR job
  could leave tampered state (for example Docker layers or Cargo registry entries) that a later
  `develop` push job reuses. Today the publish jobs read the `container-release` GHA cache scope
  written by the `test` job, so moving that job to the self-hosted runner would let such state
  reach a published image. Mitigation: T5 removes that cache link, so publish jobs build only from
  GitHub-hosted state. Splitting publishing into a separate workflow, as proposed during review,
  is optional; the isolation comes from not sharing cache, not from the workflow boundary. Cost:
  publish builds get less cache reuse, but they run after merge, off the PR critical path.
- **Runner concurrency becomes the new bottleneck.** With several agents opening PRs, a single
  runner instance serializes `Test (Docker)` jobs. Mitigation: size the server and the number of
  runner instances from the baseline job rate, and measure queue time in T7.
- **Single point of failure.** One server means jobs stay queued (up to GitHub's 24-hour limit)
  when it is down; `timeout-minutes` does not help while a job waits for a runner. Mitigation:
  offline detection with alerting and a documented fallback procedure (AC4).
- **Network transfer to GitHub may cancel the gain.** The baseline shows the GitHub cache export
  already takes 5-12 minutes per job inside GitHub's own network; from Hetzner it is expected to
  be slower. Outgoing traffic also counts against the server's 20 TB monthly allowance.
  Mitigation: T5 moves the build caches onto the server, so they no longer cross the network.
  Remaining transfers (checkout, crates.io and Docker Hub downloads) are measured in T7 and
  handled by the scenario B remedy.
- **Disk growth.** Persistent caches grow without bound. Mitigation: scheduled pruning with a
  documented size limit.
- **Target may not be reached.** Other workflows (Coverage, Copilot review) also occupy the
  critical path. Mitigation: measure end-to-end wall-clock time, not only the moved job.
- **Operational cost.** Patching, disk cleanup, and monitoring become a maintainer duty.
  Mitigation: name an owner and document the routine.

## Open Questions

1. Who owns the Hetzner account, and what is the server type (shared or dedicated vCPU)?
2. How many runner instances on the server (parallel `Test (Docker)` jobs)?
3. Runner installation: plain `actions/runner` service, or containerized?
4. BuildKit cache mechanism on the persistent host: the default `docker` driver, a named
   persistent `docker-container` builder, or a `type=local` cache directory? Should the job keep
   writing `type=gha` cache at all?
5. Should `testing.yaml` `docker-e2e` (feature-branch path) also move, or stay on GitHub-hosted?
6. Fallback preference: automatic fallback to `ubuntu-latest`, or fail fast and alert? Which
   offline-detection mechanism (for example, a scheduled GitHub-hosted check of the runner status,
   or an external uptime monitor)?

## Implementation Completion Review

- Retrospective: `Not yet assessed`
- If needed, create `implementation-retrospective.md` from
  `docs/templates/IMPLEMENTATION-RETROSPECTIVE.md` in this directory.
- If no retrospective is needed, add a concise progress-log entry explaining why.
- When an independent reviewer receives this folder-style specification, it records its result in
  `agent-review-reports.md` using `docs/templates/AGENT-REVIEW-REPORTS.md`.

## References

- GitHub issue: #2323
- Issue comments: <https://github.com/torrust/torrust-tracker/issues/2323#issuecomment-5799347274>,
  <https://github.com/torrust/torrust-tracker/issues/2323#issuecomment-5814005344>
- Parent EPIC: #1840
- Cost analysis: [`larger-runner-vs-self-hosted-cost-analysis.md`](larger-runner-vs-self-hosted-cost-analysis.md)
- Server setup log: [`runner-server-setup.md`](runner-server-setup.md)
- Baseline: [`benchmark-results.md`](benchmark-results.md)
- GitHub Actions billing: <https://docs.github.com/en/billing/concepts/product-billing/github-actions>
- Related issue: #1854 (container test gating)
- Related draft: `docs/issues/drafts/1840-workflow-performance-buildkit-cargo-cache-mounts/ISSUE.md`
- Related ADRs: `docs/adrs/20260612000000_adopt_sccache_for_ci_bare_builds.md`,
  `docs/adrs/20260603000000_keep_unit_tests_inside_container_build.md`
