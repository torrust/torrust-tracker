---
semantic-links:
  related-artifacts:
    - docs/adrs/20260926142648_adopt_self_hosted_hetzner_runner_for_container_tests.md
    - .github/workflows/container.yaml
    - .github/workflows/testing.yaml
    - docs/issues/closed/2323-1840-hetzner-self-hosted-ci-runner/runner-server-setup.md
    - docs/issues/closed/2323-1840-hetzner-self-hosted-ci-runner/runner-agent-installation.md
---

# Self-Hosted CI Runner

<!-- cspell:ignore passwordauthentication kbdinteractiveauthentication permitrootlogin publickey keyrings usermod fallocate swapfile swapon installdependencies tostring journalctl -->

How to set up and operate the self-hosted GitHub Actions runner that runs the tracker's container
test jobs. The decision, its cost rationale, and the security controls it depends on are in
[ADR 20260926142648](adrs/20260926142648_adopt_self_hosted_hetzner_runner_for_container_tests.md).
The record of how the current runner was actually built, including the problems found on the
way, is in issue #2323's
[server setup log](issues/closed/2323-1840-hetzner-self-hosted-ci-runner/runner-server-setup.md) and
[runner agent log](issues/closed/2323-1840-hetzner-self-hosted-ci-runner/runner-agent-installation.md).

## What Runs on the Runner

| Workflow         | Job             | Self-hosted when                                                                 | Otherwise       |
| ---------------- | --------------- | -------------------------------------------------------------------------------- | --------------- |
| `container.yaml` | `Test (Docker)` | PR targeting `develop` (not opened by Dependabot), or push to `develop`          | `ubuntu-latest` |
| `testing.yaml`   | `Docker E2E`    | Pushes to other branches and PRs targeting other branches in `torrust/torrust-tracker`, except Dependabot PRs and `dependabot/` branch pushes; the job is skipped for `develop`, `main`, and `releases/**` events | `ubuntu-latest` |

Fork PRs targeting `develop` can select the self-hosted runner after their workflows are approved;
the approval policy for all external contributors is the gate. The publish jobs always run on
GitHub-hosted runners and use no GitHub Actions cache. On the self-hosted runner the jobs use the
default `docker` Buildx driver, keep host-side Cargo builds in
`~runner/.cache/torrust-tracker/`, and write no GitHub Actions cache.

## Current Runner

| Property     | Value                                                                                   |
| ------------ | --------------------------------------------------------------------------------------- |
| Owner        | Nautilus Cyberneering (Hetzner account)                                                 |
| Provider     | Hetzner Cloud                                                                           |
| Location     | Falkenstein, Germany (data center `fsn1-dc8`, network zone `eu-central`)                |
| Server type  | CPX42 class, shared vCPU                                                                |
| Server ID    | `167265417` (Hetzner Cloud)                                                             |
| CPU          | 8 vCPU, AMD EPYC (Genoa), KVM virtual machine                                           |
| Memory       | 16 GB RAM, 16 GB swap file                                                              |
| Disk         | 320 GB local disk, 300 GB usable for `/`                                                |
| Traffic      | 20 TB outgoing per month included                                                       |
| Price        | 69.49 EUR per month                                                                     |
| Public IP    | Not recorded in this public repository; see the server in the Hetzner console           |
| SSH          | Key only, as `root`, with the dedicated key from step 3                                 |
| Firewall     | `torrust-runner-ssh-only`: inbound TCP 22 only                                          |
| OS           | Ubuntu 26.04.1 LTS                                                                      |
| Docker       | Engine 29.8.1, Buildx 0.37.1, Compose 5.5.1                                             |
| Hostname     | `torrust-runner-01`                                                                     |
| Runner       | `torrust-runner-01`, runner ID 23, agent 2.337.0 (updates itself)                       |
| Registration | Repository level, `torrust/torrust-tracker`                                             |
| Labels       | `self-hosted`, `Linux`, `X64`, `torrust-hetzner`                                        |
| Service      | `actions.runner.torrust-torrust-tracker.torrust-runner-01.service`                      |
| Created      | Server 2026-09-24; runner registered again on 2026-09-26                                |

Versions were checked on 2026-09-30 and change with updates; re-check them with the commands in
Check the Runner.

## Cost

A GitHub-hosted runner exists only while it runs a job. The server is billed a flat 69.49 EUR per
month whether it runs jobs or sits idle, and it is idle most of the day. GitHub bills neither
self-hosted minutes nor the standard GitHub-hosted runners of this public repository.

The paid alternative for faster container jobs is a GitHub larger runner, billed per minute from
the first minute plus GitHub Team seats. Issue #2323 estimated about 18.3 minutes per
`Test (Docker)` job on a 16-core larger runner at 0.042 USD per minute, about 0.77 USD per
`Container` run: 290 to 330 USD per month for the 433 runs of the 30 days before 2026-09-16. See
the [comparison in #2323](issues/closed/2323-1840-hetzner-self-hosted-ci-runner/ISSUE.md#alternatives-considered)
and the [ADR](adrs/20260926142648_adopt_self_hosted_hetzner_runner_for_container_tests.md).

So the server is cheaper while there are more than about 100 `Container` runs per month, before
counting Team seats. The 30 days before 2026-09-30 had 591 runs (516 pull request, 75 push). When
rechecking capacity, also recheck the volume and the current per-minute prices:

```bash
desktop$ gh api "repos/torrust/torrust-tracker/actions/workflows/container.yaml/runs?created=>=<YYYY-MM-DD>&per_page=1" -q .total_count
```

## Security

The runner is persistent and job code gets root-equivalent access to it through the `docker`
group, so a compromised job can control later jobs. The ADR accepts that on the condition that
unreviewed code rarely reaches the runner, and that the server holds no long-lived Torrust
credentials.

### Secrets and Credentials

No job that can run on the self-hosted runner references a secret. The Docker Hub credentials are
used only by the publish jobs, which always run on GitHub-hosted runners:

| Workflow         | Job                     | Runner                                     | Secrets and environment                                                                         |
| ---------------- | ----------------------- | ------------------------------------------ | ----------------------------------------------------------------------------------------------- |
| `container.yaml` | `Test (Docker)`         | Self-hosted (see What Runs on the Runner)  | None                                                                                            |
| `testing.yaml`   | `Docker E2E`            | Self-hosted (see What Runs on the Runner)  | None                                                                                            |
| `container.yaml` | `Publish (Development)` | `ubuntu-latest`                            | Environment `dockerhub-torrust`: `DOCKER_HUB_USERNAME`, `DOCKER_HUB_ACCESS_TOKEN`, `DOCKER_HUB_REPOSITORY_NAME` |
| `container.yaml` | `Publish (Release)`     | `ubuntu-latest`                            | Same as `Publish (Development)`                                                                 |

What a self-hosted job can still reach: the run's automatic `GITHUB_TOKEN` and its GitHub Actions
cache token, and, on the host, the runner registration in the install directory. The server stores
no other Torrust credentials.

To re-check after changing a workflow, list every secret and environment reference in the two
workflows that target the runner. Every match must belong to a publish job:

```bash
desktop$ grep -nE 'secrets\.|environment:' .github/workflows/container.yaml .github/workflows/testing.yaml
desktop$ grep -rln 'torrust-hetzner' .github/workflows/
```

The second command must list only `container.yaml` and `testing.yaml`.

### Rules

Keep these rules; if one cannot be kept, stop the runner and fall back to GitHub-hosted runners.

- **Review before approving.** Fork pull requests from external contributors wait for approval.
  Approve a run only after reviewing the full diff, including `.github/workflows/`, `build.rs`
  files, `contrib/` scripts, the `Containerfile`, and `Cargo.lock`.
- **Keep the GitHub settings** below (fork approval policy, organization 2FA).
- **Keep Dependabot, `main`, `releases/**`, and publishing off the runner.** Do not change the
  `runs-on` expressions or the publish jobs' cache settings without revisiting the ADR.
- **Never give the runner secrets.** The self-hosted jobs must not reference repository,
  organization, or environment secrets (see Secrets and Credentials).
- **Rebuild the server regularly** (see Operations).

## Set Up a New Runner

Conventions: `desktop$` runs on a maintainer's machine, `server#` runs on the server as `root`,
and `runner$` runs as the `runner` user (`su - runner`). `<runner-ip>` is the server's public IPv4
address and `<TOKEN>` a registration token; never commit either. Paste server commands one at a
time: pasting several lines at once has dropped lines on this setup.

### 1. Check the GitHub Settings

Both settings must be in place before any runner is registered.

```bash
desktop$ gh api repos/torrust/torrust-tracker/actions/permissions/fork-pr-contributor-approval -q .approval_policy
desktop$ gh api orgs/torrust -q .two_factor_requirement_enabled
```

Expected: `all_external_contributors` and `true`. To set the approval policy:

```bash
desktop$ gh api -X PUT repos/torrust/torrust-tracker/actions/permissions/fork-pr-contributor-approval \
  -f approval_policy=all_external_contributors
```

The 2FA requirement is set by an organization owner under **Organization settings ->
Authentication security -> Require two-factor authentication**. It removes members and outside
collaborators without 2FA, so check
`gh api "orgs/torrust/outside_collaborators?filter=2fa_disabled" -q length` (and the same for
`members`) first.

### 2. Create the Server

In the Hetzner Cloud console, create a server with 8 vCPU and 16 GB RAM (CPX42 class) running
Ubuntu 26.04 LTS. Create a firewall with one inbound rule, TCP 22 from `0.0.0.0/0` and `::/0`, no
outbound rules (Hetzner treats that as allow all), and **apply it to the server**; a firewall that
is created but not applied does not filter anything. The runner only makes outbound connections.

If the server was created without an SSH key, log in with the emailed root password. Skip the
agent's keys, otherwise the server disconnects with "Too many authentication failures":

```bash
desktop$ ssh -o PubkeyAuthentication=no -o PreferredAuthentications=password root@<runner-ip>
```

### 3. Set Up SSH Key Login

```bash
desktop$ ssh-keygen -t ed25519 -a 100 -f ~/.ssh/torrust_ci_runner_ed25519 -C "torrust-runner-01"
desktop$ ssh-copy-id -o PubkeyAuthentication=no -o PreferredAuthentications=password \
  -i ~/.ssh/torrust_ci_runner_ed25519.pub root@<runner-ip>
desktop$ ssh -i ~/.ssh/torrust_ci_runner_ed25519 -o IdentitiesOnly=yes root@<runner-ip>
```

Keep the passphrase in a password manager. Then add a host entry to `~/.ssh/config`:

```text
Host torrust-runner-01
    HostName <runner-ip>
    User root
    IdentityFile ~/.ssh/torrust_ci_runner_ed25519
    IdentitiesOnly yes
```

### 4. Disable Password Login

Only after key login works, and with a second session open as a fallback. A `00-` prefix makes
this file win over provider defaults such as `50-cloud-init.conf`.

```bash
server# cat > /etc/ssh/sshd_config.d/00-torrust-hardening.conf <<'EOF'
PasswordAuthentication no
KbdInteractiveAuthentication no
PermitRootLogin prohibit-password
EOF
server# sshd -t && systemctl reload ssh
server# sshd -T | grep -E '^(passwordauthentication|kbdinteractiveauthentication|permitrootlogin) '
desktop$ ssh -o PubkeyAuthentication=no -o PreferredAuthentications=password root@<runner-ip>
```

Expected: `sshd -T` prints `no`, `no`, and `prohibit-password`, and the last command fails with
`Permission denied (publickey).` Keep the root password for the Hetzner web console.

### 5. Update the OS and Enable Security Updates

```bash
server# apt update && apt full-upgrade -y
server# apt install -y unattended-upgrades
server# cat /etc/apt/apt.conf.d/20auto-upgrades
server# test -f /var/run/reboot-required && reboot
```

`20auto-upgrades` must set both `Update-Package-Lists` and `Unattended-Upgrade` to `"1"`; if not,
run `dpkg-reconfigure -plow unattended-upgrades`.

### 6. Install Docker Engine

The jobs need Docker Engine, the Buildx plugin, and the Compose plugin (qBittorrent E2E stacks).

```bash
server# apt install -y ca-certificates curl
server# install -m 0755 -d /etc/apt/keyrings
server# curl -fsSL https://download.docker.com/linux/ubuntu/gpg -o /etc/apt/keyrings/docker.asc
server# chmod a+r /etc/apt/keyrings/docker.asc
server# tee /etc/apt/sources.list.d/docker.sources <<EOF
Types: deb
URIs: https://download.docker.com/linux/ubuntu
Suites: $(. /etc/os-release && echo "${UBUNTU_CODENAME:-$VERSION_CODENAME}")
Components: stable
Signed-By: /etc/apt/keyrings/docker.asc
EOF
server# apt update
server# apt install -y docker-ce docker-ce-cli containerd.io docker-buildx-plugin docker-compose-plugin
server# docker run --rm hello-world
```

### 7. Install Host Build Tools

The E2E tools are compiled and run on the host with `cargo run`, and `openssl-sys` needs the
system OpenSSL headers. The Rust toolchain is installed by the workflow (`dtolnay/rust-toolchain`)
into the `runner` user's home.

```bash
server# apt install -y build-essential pkg-config git jq unzip libssl-dev
```

### 8. Create the `runner` User

The runner refuses to run as `root`. Membership in the `docker` group is root-equivalent; that is
the exposure the Security Rules exist for.

```bash
server# useradd --create-home --shell /bin/bash runner
server# usermod -aG docker runner
server# su - runner -c 'docker run --rm hello-world'
```

### 9. Add Swap

Cargo compiles one crate per vCPU, and eight parallel `rustc` processes exhausted 16 GB of RAM on
the first run. The kernel then killed the runner itself. Swap absorbs those peaks.

```bash
server# fallocate -l 16G /swapfile && chmod 600 /swapfile && mkswap /swapfile && swapon /swapfile && echo '/swapfile none swap sw 0 0' >> /etc/fstab
server# swapon --show
```

### 10. Prune Docker Storage Daily

The jobs keep Docker layers and BuildKit cache mounts on local disk. A daily timer caps the build
cache at 120 GB and removes stale containers, images, and anonymous volumes; in-use cache and
running containers are never removed.

```bash
server# printf '%s\n' '[Unit]' 'Description=Prune Docker build cache and unused objects for the CI runner' '' '[Service]' 'Type=oneshot' 'ExecStart=/usr/bin/docker builder prune --force --max-used-space 120GB' 'ExecStart=/usr/bin/docker container prune --force --filter until=24h' 'ExecStart=/usr/bin/docker image prune --force --filter until=168h' 'ExecStart=/usr/bin/docker volume prune --force' > /etc/systemd/system/docker-ci-prune.service
server# printf '%s\n' '[Unit]' 'Description=Daily Docker prune for the CI runner' '' '[Timer]' 'OnCalendar=*-*-* 04:00:00 UTC' 'Persistent=true' '' '[Install]' 'WantedBy=timers.target' > /etc/systemd/system/docker-ci-prune.timer
server# systemctl daemon-reload && systemctl enable --now docker-ci-prune.timer
server# systemctl start docker-ci-prune.service && systemctl list-timers docker-ci-prune.timer
```

### 11. Install and Register the Runner Agent

Use the latest [runner release](https://github.com/actions/runner/releases) and the SHA-256 hash
from its release notes. The values below are for `v2.337.0`.

```bash
runner$ mkdir -p ~/actions-runner && cd ~/actions-runner
runner$ curl -fsSL -o actions-runner-linux-x64-2.337.0.tar.gz https://github.com/actions/runner/releases/download/v2.337.0/actions-runner-linux-x64-2.337.0.tar.gz
runner$ echo "70920811a4f8ad4328818682bca5c6469c1c942fab52448868071d0063816613  actions-runner-linux-x64-2.337.0.tar.gz" | sha256sum -c
runner$ tar xzf actions-runner-linux-x64-2.337.0.tar.gz && rm actions-runner-linux-x64-2.337.0.tar.gz
server# cd /home/runner/actions-runner && ./bin/installdependencies.sh
```

Get a registration token from the **repository** page,
<https://github.com/torrust/torrust-tracker/settings/actions/runners/new>, not the organization
page. Register at repository level: on GitHub Free, an organization-level runner would be reachable
from every public Torrust repository. A token is valid for one hour; a `NotFound` error from
`runner-registration` means it expired. Give `config.sh` as a single line:

```bash
runner$ cd ~/actions-runner && ./config.sh --unattended --url https://github.com/torrust/torrust-tracker --token <TOKEN> --name torrust-runner-01 --labels torrust-hetzner --work _work
```

Install it as a service, one command at a time, from the install directory as `root`:

```bash
server# cd /home/runner/actions-runner
server# ./svc.sh install runner
server# ./svc.sh start
```

### 12. Restart the Runner After a Crash

The generated unit does not restart the runner if it dies. Add a drop-in; a deliberate
`systemctl stop` still stops it.

```bash
server# mkdir -p /etc/systemd/system/actions.runner.torrust-torrust-tracker.torrust-runner-01.service.d && printf '%s\n' '[Service]' 'Restart=on-failure' 'RestartSec=10' > /etc/systemd/system/actions.runner.torrust-torrust-tracker.torrust-runner-01.service.d/restart.conf && systemctl daemon-reload
server# systemctl show actions.runner.torrust-torrust-tracker.torrust-runner-01.service -p Restart -p RestartUSec
```

Replace `torrust-runner-01` in the unit name if the runner has another name.

### 13. Verify

```bash
desktop$ gh api repos/torrust/torrust-tracker/actions/runners -q '.runners[] | [.name, .os, .status, (.busy | tostring), ([.labels[].name] | join(","))] | join("  ")'
```

Expected: `torrust-runner-01  Linux  online  false  self-hosted,Linux,X64,torrust-hetzner`. Then
open or update a non-documentation PR against `develop` and check that its `Test (Docker)` job
reports `runner_name` `torrust-runner-01` and passes.

## Operations

### Check the Runner

- Status and labels: the `gh api .../actions/runners` command in step 13.
- Service: `systemctl status actions.runner.torrust-torrust-tracker.torrust-runner-01.service`.
- Out-of-memory kills: `journalctl -k | grep "Killed process"`.
- Disk and cache: `df -h /` and `docker system df`.

### When the Runner Is Offline

Symptom: a PR's `Test (Docker)` check stays queued with "Waiting for a runner". `timeout-minutes`
does not apply while a job is queued; GitHub fails it after 24 hours. On `develop` pushes the
publish jobs wait for `test`, so nothing is published until the job runs.

1. Check the service and the kernel log (see Check the Runner). If the service is `failed`, run
   `systemctl reset-failed <unit>` and `systemctl start <unit>`, and look for the cause.
2. Re-run the failed or cancelled jobs: `gh run rerun <run-id> --repo torrust/torrust-tracker --failed`.
3. If the server cannot be recovered quickly, merge a PR that sets `runs-on: ubuntu-latest` for
   the two jobs in "What Runs on the Runner", then restore the expressions once the runner is back.
   An open PR picks up the fallback only after it is rebased onto it: a re-run keeps the workflow
   file of its original commit.

### Cache Cleanup

The daily prune timer (step 10) bounds the Docker build cache. The host-side Cargo target
directories under `/home/runner/.cache/torrust-tracker/` are not pruned; if they grow too large,
delete them (`rm -rf /home/runner/.cache/torrust-tracker/*-target`) while no job is running. The
next job rebuilds them.

### Runner Agent Updates

Self-hosted runners update themselves when GitHub publishes a new runner version. Check the
current version in the service log (`Current runner version`) or the runners API (`.version`).

### Rebuild the Server

Rebuild the server regularly (for example monthly) and immediately on any suspicion of compromise,
so an implant does not survive:

1. Create a new server and repeat steps 2 to 13 with a new runner name (for example
   `torrust-runner-02`) and the same `torrust-hetzner` label.
2. When the new runner is online, remove the old registration:
   `gh api repos/torrust/torrust-tracker/actions/runners -q '.runners[] | "\(.id) \(.name)"'`,
   then `gh api -X DELETE repos/torrust/torrust-tracker/actions/runners/<runner-id>`.
3. Delete the old server in the Hetzner console, and remove its host key from `~/.ssh/known_hosts`.

Deleting the registration through the API needs no removal token. Do not reuse anything from the
old server, including its caches.

### Add Runner Capacity

Only if measured queue time shows pull requests waiting for the runner. The decision to keep one
runner, and when to recheck it, is recorded in the
[capacity specification](issues/drafts/1840-self-hosted-runner-minimum-capacity/ISSUE.md).

Add capacity as a **second server** with one runner instance: repeat steps 2 to 13 with a new
runner name and the same `torrust-hetzner` label. Do not add a second instance on the same server:
concurrent jobs on one Docker host collide on the E2E tests' fixed host ports, share the
`torrust-tracker:local` image tag (a job can test another job's image), and share the Cargo target
directories.
