---
semantic-links:
  skill-links:
    - create-adr
  related-artifacts:
    - .github/skills/dev/planning/create-adr/SKILL.md
    - .github/workflows/container.yaml
    - .github/workflows/testing.yaml
    - docs/issues/open/2323-1840-hetzner-self-hosted-ci-runner/ISSUE.md
    - docs/issues/open/2323-1840-hetzner-self-hosted-ci-runner/self-hosted-runner-security-research.md
    - docs/issues/open/2323-1840-hetzner-self-hosted-ci-runner/benchmark-results.md
---

<!-- skill-link: create-adr -->

# Adopt a Self-Hosted Hetzner Runner for the Container Test Job

## Scope

Root ADR. It governs repository-wide CI infrastructure: which runner executes the container image
build and end-to-end tests, and what state published images may be built from. No package owns it.

## Description

The `container.yaml` `Test (Docker)` job is the slowest required check on every pull request. The
T1 baseline of issue #2323 (25 runs, 6 job logs) measured a median of 37 minutes: the image build
takes 32 minutes, of which the workspace compile takes 15.5 to 18.6 minutes and the GitHub Actions
cache export another 5 to 12 minutes. The repository cache is over its 10 GB allowance, so entries
are evicted. Agents open most pull requests, so each merge makes the remaining pull requests
rebase and rerun this job, and the queue of pull requests waiting to merge grows.

Standard GitHub-hosted runners are free for public repositories but are ephemeral 4 vCPU / 16 GB
machines, so every job starts cold and pays the network cost of the GitHub cache. BuildKit cache
mounts bring no benefit there (EPIC #1840 draft on BuildKit cache mounts).

Two options were compared:

- **GitHub larger runners.** They require GitHub Team or Enterprise Cloud and are billed per minute
  from the first minute, even for public repositories. Derived from the T1 baseline, a 16-core
  runner costs about 290 USD per month for the 378 pull-request runs in 30 days and about 330 USD
  with the 55 push runs. The cache stays remote, so the export cost remains.
- **A persistent self-hosted runner.** A Hetzner server with 8 vCPU, 16 GB RAM, and 320 GB disk in
  Falkenstein (shared vCPU, CPX42 class) costs 69.49 EUR per month, flat, and keeps Docker layers,
  BuildKit cache mounts, and Cargo caches on local disk between jobs.

The self-hosted option has a security cost. All pull requests come from forks, and a
`pull_request` workflow runs the pull request's code. On a persistent runner whose `runner` user
is in the `docker` group, that code gains root-equivalent control of the host and can persist into
later jobs, including `develop` push jobs and their tokens. A fork pull request can reach any
registered runner by adding its own workflow, even if no repository workflow targets it. On
2026-09-24 the maintainers rejected that exposure and removed the runner. The analysis is in
[`self-hosted-runner-security-research.md`](../issues/open/2323-1840-hetzner-self-hosted-ci-runner/self-hosted-runner-security-research.md).

On 2026-09-25 the maintainers reviewed who actually opens pull requests. In the 12 months before
that date, organization members opened 406, Dependabot 118, and external contributors 5 (from four
accounts). Unreviewed code rarely reaches CI, so the decision was revisited on the basis of
likelihood, not impact.

## Agreement

Run the `container.yaml` `test` job, and the `testing.yaml` `docker-e2e` job, on a persistent
self-hosted runner on the Hetzner server, and accept the fork-PR exposure described above,
subject to the controls below. The 2026-09-24 rejection is reversed on the basis of the
contribution profile.

### Controls the decision depends on

1. **Approval for all external contributors.** The repository's fork pull-request approval policy
   is `all_external_contributors`, and a maintainer approves a run only after reviewing the full
   diff, including workflows, `build.rs`, `contrib/` scripts, and `Cargo.lock`.
2. **Required two-factor authentication** for organization members, whose pull requests run
   without approval.
3. **Dependabot stays on GitHub-hosted runners.** Dependabot branches live in the base repository,
   so the fork approval policy does not gate them. The jobs select `ubuntu-latest` when
   `github.event.pull_request.user.login == 'dependabot[bot]'`. The selection uses the pull request
   author, not `github.actor`, because `github.actor` becomes the maintainer who updates the
   branch. Pushes to `dependabot/` branches, which trigger `testing.yaml` in this repository
   without approval, also stay on `ubuntu-latest`.
4. **Only `develop` traffic uses the runner.** Pull requests targeting `develop` and pushes to
   `develop` use it; pull requests to `main` and pushes to `main` and `releases/**` stay on
   `ubuntu-latest`.
5. **Published images use no GitHub Actions cache.** The publish jobs stay on `ubuntu-latest`, and
   their image builds neither import nor export `type=gha` cache. A cache scope is a key name, not
   a producer identity: an implant holding a `develop` job's cache token could write any scope, and
   `develop` is the default branch, so its entries are readable from every branch. The self-hosted
   job references no repository, organization, or environment secrets.
6. **Regular server rebuild** from the setup logs, so an implant does not survive indefinitely.

With these controls, a compromise of the runner can at most fake test results. AI agents acting
with a maintainer's credentials remain a residual path: an agent manipulated by untrusted content
could push code that runs without approval.

### Runner layout

- **Owner.** The Hetzner account belongs to the Nautilus Cyberneering organization.
- **Instances.** One runner instance. The job is bound by the Rust compile, so two parallel jobs on
  8 vCPU would each run about twice as slow, and two workspace links risk exhausting 16 GB of RAM.
  A second instance is added only if measured queue time shows jobs waiting for the runner.
- **Installation.** The plain `actions/runner` agent as a systemd service under a dedicated
  `runner` user, registered at repository level with the `torrust-hetzner` label.

### Cache strategy

The self-hosted jobs build with the default `docker` driver, so layers and BuildKit cache mounts
stay in the host's Docker store, and they stop writing `type=gha` cache: no trusted build may read
cache that the self-hosted host wrote, and the export costs 5 to 12 minutes per job. Disk growth is
bounded by scheduled pruning.

### Offline runner

A job targeting an offline runner stays queued ("Waiting for a runner"), and GitHub fails it after
24 hours in the queue; `timeout-minutes` does not bound queue time. The pull request check stays
pending, so maintainers and agents notice it in their normal pull-request review. There is no
automatic fallback and no alerting service: routing a job to another runner requires a decision
before the job is queued, and doing it automatically would need a monitor workflow with an admin
token and a repository variable, which is more machinery than one runner justifies. Recovery is to
restart the runner or to merge a one-line change that points `runs-on` back to `ubuntu-latest`,
as documented in the runner operations guide. On `develop` pushes the publish jobs wait on `test`,
so an offline runner delays publishing but does not publish anything unverified.

### Revisit conditions

Fall back to GitHub larger runners, or to per-job VMs, if external contributions rise noticeably,
if any control above cannot be kept, or if the runner is compromised.

### Alternatives Considered

- **GitHub larger runners** (rejected on cost): about 290 to 330 USD per month plus Team seats,
  growing with pull-request volume, with the remote cache cost unchanged. They remain the fallback.
- **Per-job VMs on the Hetzner server** (deferred): a fresh VM per job removes the persistence
  problem but loses the local caches that motivate the change, and needs a cache service that
  untrusted jobs can read but not overwrite.
- **Rootless Docker** (rejected): removes host root but not persistence, because the runner agent
  and the job run as the same user.
- **Automatic fallback and alerting** (rejected as unjustified complexity, see Offline runner).

### Consequences

- Positive: the container job keeps warm local caches and drops the cache export; cost is flat as
  pull-request volume grows; no paid GitHub plan is needed.
- Negative: maintainers own patching, pruning, runner upgrades, and rebuilds; one server is a
  single point of failure; the security of the job depends on the controls above being kept;
  publish builds run cold.

## Date

2026-09-26

## References

- Issue #2323 and its spec:
  [`ISSUE.md`](../issues/open/2323-1840-hetzner-self-hosted-ci-runner/ISSUE.md)
- EPIC #1840: [`EPIC.md`](../issues/open/1840-improve-pr-workflow-performance-epic/EPIC.md)
- Spec pull request #2335 and its review audit:
  [`PR-REVIEW.md`](../pr-reviews/pr-2335-review/PR-REVIEW.md)
- Baseline:
  [`benchmark-results.md`](../issues/open/2323-1840-hetzner-self-hosted-ci-runner/benchmark-results.md)
- [GitHub-hosted runners reference](https://docs.github.com/en/actions/reference/runners/github-hosted-runners)
- [Approving workflow runs from forks](https://docs.github.com/en/actions/how-tos/manage-workflow-runs/approve-runs-from-forks)
- Related ADRs:
  [`20260612000000_adopt_sccache_for_ci_bare_builds.md`](20260612000000_adopt_sccache_for_ci_bare_builds.md),
  [`20260603000000_keep_unit_tests_inside_container_build.md`](20260603000000_keep_unit_tests_inside_container_build.md)

## Affected Code

- [`.github/workflows/container.yaml`](../../.github/workflows/container.yaml): `test` job runner
  selection, cache configuration, and the publish jobs' cache settings.
- [`.github/workflows/testing.yaml`](../../.github/workflows/testing.yaml): `docker-e2e` job runner
  selection and cache configuration.
