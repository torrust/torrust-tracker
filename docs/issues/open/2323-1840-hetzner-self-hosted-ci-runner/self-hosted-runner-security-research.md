# Research: Running Fork-PR CI on a Self-Hosted Runner Safely

<!-- cspell:ignore passwordless reimaged Cyclenerd Kata Sandboxed Sysbox Kaniko Ubicloud -->

Research for issue #2323, triggered by Copilot finding PERSISTENT-RUNNER-PRIVILEGE and reviewer
finding F1 on PR #2335: adding the `runner` user to
the `docker` group gives fork-PR code root-equivalent control of the host. The maintainer decision
is that **untrusted code getting root on a runner is not acceptable**, even on ephemeral runners.
See [ISSUE.md](ISSUE.md) for the plan this research feeds.

Status: in progress. Sources checked on 2026-09-24 unless stated otherwise. Findings 1-6 come from
primary documentation; finding 7 adds community reports and articles supplied by the maintainer.

## Questions

1. Can a fork PR get root on the runner without changing workflow code? Does `CODEOWNERS` help?
2. Why are GitHub-hosted runners not exposed to the same risk?
3. Does removing root from Docker (rootless Docker and similar) solve the problem?
4. How do other projects run self-hosted runners for public repositories safely?

## Repository Facts

Verified with the GitHub API on 2026-09-24:

| Setting                                         | Value                     |
| ----------------------------------------------- | ------------------------- |
| Default `GITHUB_TOKEN` permissions              | `read`                    |
| Fork PR workflow approval policy                | `first_time_contributors` |
| Self-hosted runner `torrust-runner-01`          | Persistent, repository-level, `runner` user in `docker` group |

## Findings

### 1. Fork-PR code gets root without touching any workflow file

A `pull_request` workflow checks out the PR merge branch by default: "`GITHUB_SHA` is the SHA of
the merge commit on the merge branch" and `actions/checkout` uses it
([Events that trigger workflows](https://docs.github.com/en/actions/reference/workflows-and-actions/events-that-trigger-workflows)).
The `Test (Docker)` job then executes code that the PR controls:

- the `Containerfile` and every `RUN` step in it;
- Cargo build scripts (`build.rs`), procedural macros, and tests;
- `cargo run` of the E2E tools and the shell scripts under `contrib/`.

Any of these can run `docker run --privileged -v /:/host ...` because `runner` is in the `docker`
group, which is root on the host. No workflow change is needed. A PR can also change the workflow
files themselves, because `pull_request` runs the workflow definitions from the merge commit.

**`CODEOWNERS` does not help here.** It requests reviews and, with branch protection, can block
merging ([About code owners](https://docs.github.com/en/repositories/managing-your-repositorys-settings-and-features/customizing-your-repository/about-code-owners)).
It is read from the base branch and gates the merge, not the execution of the PR's workflows,
which starts before any review.

**Removing the `docker` group alone is not enough either.** The runner agent and the job run as the
same Unix user, which owns `/home/runner/actions-runner`, including the runner binaries and its
registration files (`.runner`, `.credentials`). Without root, a job can still replace the runner
program or its configuration and control every later job on that runner, including `develop`
pushes and the check results reported for other PRs.

Reviewer finding F1 on PR #2335 adds what those later jobs expose in this repository:

- `push` jobs receive their `GITHUB_TOKEN` and Actions cache token whether or not they reference a
  secret; `container.yaml` has no `permissions:` block, so the token carries the repository default
  (verified as `read` for this repository, see Repository Facts);
- the `test` job gates `publish_development` and `publish_release` through `needs:`, so an implant
  can make the gate pass on a push;
- moving the `test` job also moves PRs to `main` and pushes to `main` and `releases/**` onto the
  runner, not only the `develop` events named in AC1.

### 2. GitHub-hosted runners grant root, but only inside a disposable VM

GitHub-hosted Linux runners run with passwordless `sudo`
([GitHub-hosted runners reference](https://docs.github.com/en/actions/reference/runners/github-hosted-runners)),
so jobs do get root. The difference is the boundary: "each GitHub-hosted runner is a new virtual
machine (VM)" and GitHub-hosted runners "execute code within ephemeral and clean isolated virtual
machines, meaning there is no way to persistently compromise this environment"
([Secure use reference](https://docs.github.com/en/actions/reference/security/secure-use#hardening-for-self-hosted-runners)).

So the property to reproduce is not "no root in the job", but:

- a hypervisor boundary between the job and anything that outlives it; and
- a clean environment for every job, destroyed afterwards.

The same page states that self-hosted runners "can be persistently compromised by untrusted code
in a workflow" and "should almost never be used for public repositories".

### 3. Approval policies are not a security boundary for self-hosted runners

GitHub's repository settings page warns that fork-PR approval policies exist to limit compute
consumption, and that with self-hosted runners "potentially malicious user-controlled workflow
code will execute automatically if the user is allowed to bypass approval ... or if the pull
request is approved"
([Managing GitHub Actions settings for a repository](https://docs.github.com/en/repositories/managing-your-repositorys-settings-and-features/enabling-features-for-your-repository/managing-github-actions-settings-for-a-repository)).

It also warns that the current `first_time_contributors` policy can be bypassed: a user with any
merged commit, such as an accepted typo fix, no longer needs approval. The strictest option,
"Require approval for all external contributors", still runs code automatically for organization
members and runs anything a maintainer approves. It lowers the likelihood of an attack but does
not bound its impact.

### 4. Rootless Docker removes host root, but not persistence

Rootless mode runs the Docker daemon and containers inside a user namespace, without root
privileges ([Docker rootless mode](https://docs.docker.com/engine/security/rootless/)). It blocks the
`docker run -v /:/host` escalation, but:

- the runner agent still shares a Unix user with the job (finding 1), so persistent compromise of
  the runner remains;
- all jobs still share one kernel, so a kernel exploit still reaches the host;
- compatibility with the workflow (Buildx, Compose stacks, published ports) is unverified.

Rootless Docker is a useful layer inside a per-job VM, not a replacement for one.

### 5. GitHub's recommended pattern: ephemeral or just-in-time runners on clean machines

- `config.sh --ephemeral` makes GitHub assign at most one job to a runner and then de-register it;
  "you can then create your own automation that wipes the runner"
  ([Self-hosted runners reference](https://docs.github.com/en/actions/reference/runners/self-hosted-runners#ephemeral-runners-for-autoscaling)).
- Just-in-time (JIT) runners, created through the REST API, do the same, and GitHub warns that
  "re-using hardware to host JIT runners can risk exposing information from the environment. Use
  automation to ensure the JIT runner uses a clean environment"
  ([Secure use reference](https://docs.github.com/en/actions/reference/security/secure-use#using-just-in-time-runners)).
- GitHub recommends ephemeral runners over persistent ones for autoscaling because it cannot
  otherwise guarantee that jobs are not assigned to compromised runners.

Ephemeral registration only helps when the machine underneath is also destroyed or reimaged after
the job. Re-registering an ephemeral runner on the same persistent host does not remove anything
a previous job left behind.

### 6. How others run self-hosted runners on Hetzner

| Project | Model | Fit for fork PRs |
| ------- | ----- | ---------------- |
| [Cyclenerd/hcloud-github-runner](https://github.com/Cyclenerd/hcloud-github-runner) | A workflow job creates a Hetzner Cloud server, registers it, runs the job, and deletes the server ("New Workflow, New Server") | No: the creating job needs a Hetzner API token and a GitHub PAT as secrets, which fork-PR workflows do not receive |
| [TestFlows GitHub Hetzner Runners](https://github.com/testflows/TestFlows-GitHub-Hetzner-Runners) | An external service watches queued jobs, creates a new Hetzner Cloud server per job with an ephemeral runner, and deletes it afterwards | Yes: the tokens live on the service host, not in workflows. Optional server recycling rebuilds the server image by default; the no-rebuild mode is documented for trusted single-tenant setups only |

Both projects state that self-hosted runners should only be used with private repositories, citing
GitHub's security guidance. TestFlows notes that Hetzner bills per started hour, so one server per
job costs a full hour per job unless servers are recycled (with rebuild).

### 7. Community experience confirms the attack and the limits of the workarounds

From the GitHub Community discussion
[Self-hosted runner security with public repositories (#26722)](https://github.com/orgs/community/discussions/26722):

- Two participants report testing the attack (2022 and 2024): a fork PR that adds a new workflow
  file with `on: pull_request` and `runs-on: self-hosted` runs on the target repository's runner
  whenever the author does not need approval. So "our workflows only use the runner on `push`" is
  not a control: the PR brings its own workflow.
- The accepted answer ("push-only workflows are safe") is contested in the thread for that reason.
- Participants confirm that `CODEOWNERS` does not stop workflows from running.
- The only controls discussed are stricter fork-PR approval with a manual review of workflow
  changes before approving, and attaching the runner to a separate private "companion" repository
  instead of the public one.

The companion-repository idea hides the runner from fork-PR workflow definitions, but it does not
make PR code safe: building and testing a PR (its `Containerfile`, `build.rs`, tests) still runs
untrusted code on the runner. It only helps for code that is already trusted, such as merged
commits on `develop`.

From [Is the GitHub Actions self-hosted runner safe for Open Source?](https://actuated.com/blog/is-the-self-hosted-runner-safe-github-actions)
(Alex Ellis, actuated, 2023; a vendor article, so read as informed but not neutral):

- A contributor account can be compromised, and GitHub cannot enforce MFA for pull-request
  authors, so "we only approve trusted people" does not bound the risk either.
- Container-based runners do not fix it for Docker workloads: mounting the host Docker socket or
  running privileged Docker-in-Docker lets any job take over the host, and Actions Runner
  Controller reuses pods by default. Kaniko avoids the daemon but usually needs root inside the
  build container.
- actuated's answer is a Firecracker microVM per job on servers the user provides, with an
  immutable OS image, erased after the job, supporting `docker build`, `docker run`, and `sudo`.
  It is a paid, flat-rate service; the article itself suggests unfunded volunteer projects may
  not fit.

Other articles supplied by the maintainer:

- [Self-hosted GitHub Actions runners: setup, gotchas, and when it's worth it](https://3h4x.github.io/tech/2026/02/12/github-actions-self-hosted-runners)
  (2026) repeats "don't do self-hosted on public repos" and describes a simple health check: a
  cron job checks the runner service and alerts through a webhook, plus the runners API for
  `status` and `busy`. Relevant to the offline detection required by AC4.
- [GitHub Actions Self-Hosted Runner: A Complete Guide](https://eastondev.com/blog/en/posts/dev/20260423-github-actions-self-hosted-runner/)
  (2026) also rules out public repositories, and suggests StepSecurity Harden-Runner to audit or
  block unexpected network egress from jobs. That is defense in depth, not an isolation boundary.
  It also reports that since March 2026 GitHub bills self-hosted runner minutes on private
  repositories and that public repositories stay free; this was not verified against GitHub.
- [GitHub Community discussion #160737](https://github.com/orgs/community/discussions/160737)
  (unanswered) shows a practical cost of rootless Docker: actions that assume
  `/var/run/docker.sock` fail because the rootless socket lives under `/run/user/<uid>/`.
- The Stack Overflow question
  [Is it really unsafe to use a GitHub self-hosted runner on a own public repository?](https://stackoverflow.com/questions/77179987/is-it-really-unsafe-to-use-a-github-self-hosted-runner-on-a-own-public-repositor)
  could not be retrieved (HTTP 403).

## Current Exposure

`torrust-runner-01` is registered to this public repository and online. Although no workflow in
the repository targets it, finding 7 means any fork PR whose author does not need approval can add
a workflow with `runs-on: [self-hosted, torrust-hetzner]` and run code on it, with root through
the `docker` group. Under the current `first_time_contributors` policy, anyone with one merged
contribution qualifies. The runner should stay stopped (or be removed) until a safe design is in
place.

Action taken: the maintainer stopped the runner service on 2026-09-24 at 20:16 UTC, and GitHub
reports `torrust-runner-01` as `offline`. The systemd unit was then disabled, so the runner does
not start after a reboot. On 2026-09-25 the service was uninstalled, the registration deleted from
the repository, and the runner install directory removed from the server, so no self-hosted
runner is attached to the repository any more.

## Alternatives to a Self-Hosted Runner

### A. A dedicated remote image builder

Proposed by the maintainer: keep the whole workflow on GitHub-hosted runners and move only the
bottleneck, the image build, to a server whose only purpose is to build images. The builder
returns the image, or the build output when it fails. Other workflows that build the image (for
example `Security Scan`, finding 7 of the baseline) could use it too.

BuildKit supports this directly: `docker buildx create --driver remote tcp://<builder>:1234`
connects Buildx to an externally managed `buildkitd`
([Remote driver](https://docs.docker.com/build/builders/drivers/remote/)). The build context can
also be a Git URL, so the builder can fetch a commit from GitHub itself instead of receiving it
from the client.

What it changes:

- The workflow job, its runner credentials, and everything outside the build stay on disposable
  GitHub-hosted VMs. Nothing a job does can modify a runner we operate.
- The build cache stays on the builder, which removes the 5 to 12 minute export to the GitHub
  cache measured in the baseline.

What it does not change: the builder still runs untrusted code. Every `RUN` step of the PR's
`Containerfile` executes on it, including the unit tests that run inside the image build. The
security questions move from the runner to the builder:

1. **Authentication for fork PRs.** Exposing BuildKit over TCP without TLS "allows arbitrary
   access to BuildKit without credentials" (Remote driver docs), so the builder needs mTLS or a
   token. But fork-PR workflows receive no secrets other than a read-only `GITHUB_TOKEN`
   ([Events that trigger workflows](https://docs.github.com/en/actions/reference/workflows-and-actions/events-that-trigger-workflows),
   "Workflows in forked repositories"). Possible designs:
   - a two-stage flow: the untrusted `pull_request` workflow finishes, and a trusted
     `workflow_run` workflow, which does receive secrets, asks the builder to build the PR commit
     by Git URL without checking out or running any PR code itself. GitHub warns that
     `workflow_run` with untrusted code risks cache poisoning and secret exposure, so the trusted
     side must only pass a commit reference;
   - a builder service triggered by GitHub webhooks (a GitHub App), which fetches the PR commit,
     builds it, reports a check run, and publishes the image where the E2E job can pull it;
   - GitHub OIDC tokens as proof of the calling workflow, if they are available to fork-PR
     workflows (not yet verified).
2. **Isolation of each build.** BuildKit runs `RUN` steps in containers, not VMs. Without the
   `security.insecure` entitlement a build cannot mount the host filesystem, so reaching root
   needs a container escape. Such escapes have happened through build tooling (the January 2024
   runc and BuildKit vulnerabilities known as "Leaky Vessels"; details to be verified). Under the
   maintainer's requirement, the builder therefore still needs a disposable VM boundary per build,
   or at least rootless BuildKit in a dedicated VM that is rebuilt regularly. Rootless BuildKit
   has costs: its network mode is always host-network (isolating it needs RootlessKit networking),
   and on Ubuntu 24.04 or later it requires disabling the AppArmor restriction on unprivileged
   user namespaces ([BuildKit rootless mode](https://github.com/moby/buildkit/blob/master/docs/rootless.md)).
3. **Cache poisoning.** PR builds and trusted `develop` builds must not share writable cache.
   Shared `--mount=type=cache` directories and layer caches written by PR builds could feed a later
   trusted build. PR builds would read a cache written only by trusted builds and never write to it.

Assessment: promising because it targets the real bottleneck and can serve several workflows, but
it is a custom service with a real security design (points 1 to 3), not just a server.

### B. GitHub larger runners on GitHub Team

The fastest safe option operationally: GitHub provides a new VM per job, fork PRs get the same
isolation as today, and nothing needs to be operated. Standard runners stay free for public
repositories; only larger-runner minutes and Team seats are billed (see [ISSUE.md](ISSUE.md)),
estimated at about 200 USD per month for the `Container` job. Costs grow with PR volume, and the
build cache remains remote (`type=gha`), so the cache export cost measured in the baseline remains
unless the build is also reworked.

### C. Managed runner services with VM isolation

Several third-party services sell GitHub Actions runners that run each job in its own VM, often on
bare-metal providers and priced below GitHub's larger runners (for example actuated, Blacksmith,
Namespace, Ubicloud, WarpBuild). Not yet researched: isolation model, support for public
repositories and fork PRs, pricing, and whether any can run on our own Hetzner server.

## Maintainer Direction

On 2026-09-24 the maintainer leaned towards alternative B (GitHub larger runners on GitHub Team),
because the self-hosted alternatives are either unsafe for fork PRs or a substantial custom
project. Points to settle before committing to it:

- **Isolation:** larger runners are GitHub-hosted VMs, one per job, so fork PRs get the same
  isolation as today. None of the root-escalation findings above apply.
- **Cost abuse:** fork PRs would consume paid minutes. Bound it with a spending budget for Actions,
  a job `timeout-minutes` (which, unlike queue time, does bound billable execution), and the
  fork-PR approval policy.
- **Access:** larger runners are assigned through runner groups, which must allow this public
  repository.
- **Offline risk:** GitHub operates the capacity, so the single-server failure mode behind
  Copilot finding OPS-001 no longer applies.
- **Hetzner server:** the runner was removed on 2026-09-25; the server itself becomes
  unnecessary and can be deleted.

## Implications for Issue #2323

- The current setup (persistent host, `runner` in the `docker` group, fork PRs) does not meet the
  maintainer's requirement and must not be used for fork PRs as is.
- Meeting the requirement means a VM boundary per job, like GitHub-hosted runners: every job runs
  in a fresh VM that is destroyed or reimaged afterwards, and the runner credentials never live
  where job code runs.
- A per-job VM loses the local caches that motivated a persistent runner. Caches would have to
  live outside the job VM, on a service that untrusted jobs can read but cannot overwrite for
  trusted `develop` builds (the same scoping GitHub's cache applies to PRs).
- Approval policies, `CODEOWNERS`, runner labels, and "push-only" workflows reduce the likelihood
  of an attack but do not bound its impact. They can complement a per-job VM, not replace it.
- A persistent runner is acceptable only for code that is already trusted, and only if fork PRs
  cannot reach it at all (for example, a runner attached to a private companion repository that
  builds merged `develop` commits). That would not speed up PR checks, which are the bottleneck.

## Open Research

- Per-job VMs on the existing Hetzner server: microVM runtimes (Firecracker, as used by actuated;
  Kata Containers) and whether the server type exposes hardware virtualization (nested
  virtualization on Hetzner Cloud is unverified). A self-managed Firecracker setup versus the
  actuated service.
- Per-job Hetzner Cloud servers (TestFlows model): cost at the measured volume of about 378 PR runs
  per 30 days, server boot time, and recycling with rebuild.
- Sandboxed container runtimes (gVisor, Sysbox): whether they provide an adequate boundary for
  running Docker-in-Docker builds, and compatibility with the workflow.
- Cache design for per-job VMs: a registry or BuildKit cache service with read-only access for PR
  jobs and write access only for trusted `develop` jobs.
- Whether trusted events only (`push` to `develop`) on the persistent runner, with PRs staying on
  GitHub-hosted runners, would still reduce the merge bottleneck enough.
- Remote builder (alternative A): whether GitHub OIDC tokens are issued to fork-PR workflows; the
  `workflow_run` two-stage flow versus a webhook-driven builder service; how to report the result
  as a required check; transfer time of the built image back to the runner; and the details of
  the 2024 runc and BuildKit escape vulnerabilities.
- Managed VM-isolated runner services (alternative C): public repository and fork-PR support,
  isolation model, and pricing.
