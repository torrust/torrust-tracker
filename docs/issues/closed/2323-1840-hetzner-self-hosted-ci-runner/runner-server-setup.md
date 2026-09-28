# Runner Server Setup Log

<!-- cspell:ignore passwordauthentication kbdinteractiveauthentication permitrootlogin publickey keyrings usermod fallocate swapfile swapon -->

Step-by-step record of how the self-hosted GitHub Actions runner server for issue #2323 was set
up, so it can be reproduced or rebuilt. See [ISSUE.md](ISSUE.md) for the plan (task T2 and T3).
This log keeps the history, including problems and their fixes; the current procedure, without
the history, is [`docs/self-hosted-runner.md`](../../../self-hosted-runner.md).

Conventions:

- `<runner-ip>` stands for the server's public IPv4 address. Do not commit the real address,
  passwords, private keys, or runner registration tokens to this repository.
- `desktop$` marks commands run on the maintainer's machine; `server#` marks commands run on the
  server as `root`.

## Server

| Property   | Value                                             |
| ---------- | ------------------------------------------------- |
| Provider   | Hetzner Cloud                                     |
| Location   | Falkenstein, Germany                              |
| Resources  | 8 vCPU, 16 GB RAM, 320 GB local disk              |
| Traffic    | 20 TB outgoing per month included                 |
| Price      | €69.49 per month                                  |
| Hostname   | `torrust-runner-01`                               |
| OS         | Ubuntu 26.04.1 LTS, kernel `7.0.0-34-generic`     |
| Disk (`/`) | 300 GB usable                                     |
| Network    | Public IPv4 and IPv6                              |
| SSH access | Dedicated Ed25519 key; password login disabled    |

## 1. First Login

The server was created without an SSH key, so Hetzner emailed a root password and enabled
password login.

A plain `ssh root@<runner-ip>` failed with:

```text
Received disconnect from <runner-ip> port 22:2: Too many authentication failures
```

Cause: the local SSH agent offered every loaded key before trying the password, and the server
disconnected after reaching its authentication-attempt limit. Fix: disable public-key
authentication for this login so the client goes straight to the password:

```bash
desktop$ ssh -o PubkeyAuthentication=no -o PreferredAuthentications=password root@<runner-ip>
```

The Hetzner web console is an alternative when SSH is not reachable at all.

## 2. Create a Dedicated SSH Key

Use a key only for this server, protected by a passphrase. `-a 100` increases the key-derivation
rounds that protect the private key file.

```bash
desktop$ ssh-keygen -t ed25519 -a 100 -f ~/.ssh/torrust_ci_runner_ed25519 -C "torrust-runner-01"
```

Store the passphrase in a password manager. The private key never leaves the desktop.

## 3. Install the Public Key on the Server

```bash
desktop$ ssh-copy-id -o PubkeyAuthentication=no -o PreferredAuthentications=password \
  -i ~/.ssh/torrust_ci_runner_ed25519.pub root@<runner-ip>
```

Result (2026-09-24): `Number of key(s) added: 1`. The follow-up login command that
`ssh-copy-id` prints repeats `-o PubkeyAuthentication=no`, so it would not test the new key; use
the check in step 4 instead. Confirm on the server that only the intended key is installed:

```bash
server# cat /root/.ssh/authorized_keys
```

Expected: one `ssh-ed25519` line ending in `torrust-runner-01`. Verified on 2026-09-24.

Manual alternative, if `ssh-copy-id` is not available:

```bash
desktop$ ssh -o PubkeyAuthentication=no -o PreferredAuthentications=password root@<runner-ip> \
  'umask 077; mkdir -p ~/.ssh; cat >> ~/.ssh/authorized_keys' < ~/.ssh/torrust_ci_runner_ed25519.pub
```

## 4. Configure the SSH Client

Test the key directly first, in a **new** terminal, keeping the password session open as a
fallback:

```bash
desktop$ ssh -i ~/.ssh/torrust_ci_runner_ed25519 -o IdentitiesOnly=yes root@<runner-ip>
```

Result (2026-09-24): key login works and shows the Ubuntu 26.04.1 LTS welcome banner.

Then add a host entry to `~/.ssh/config` on the desktop. `IdentitiesOnly yes` makes the client
offer only this key, which avoids the "Too many authentication failures" error.

```text
Host torrust-runner-01
    HostName <runner-ip>
    User root
    IdentityFile ~/.ssh/torrust_ci_runner_ed25519
    IdentitiesOnly yes
```

Verify the alias:

```bash
desktop$ ssh torrust-runner-01
```

Result (2026-09-24): host entry added.

## 5. Disable Password Login

Only after step 4 succeeds. OpenSSH uses the first value it reads for each option, and the
`sshd_config.d/` files are read in lexical order, so a `00-` prefix takes precedence over
provider defaults such as `50-cloud-init.conf`.

```bash
server# cat > /etc/ssh/sshd_config.d/00-torrust-hardening.conf <<'EOF'
PasswordAuthentication no
KbdInteractiveAuthentication no
PermitRootLogin prohibit-password
EOF
server# sshd -t && systemctl reload ssh
server# sshd -T | grep -E '^(passwordauthentication|kbdinteractiveauthentication|permitrootlogin) '
```

Result (2026-09-24, Ubuntu 26.04.1 LTS):

```text
permitrootlogin prohibit-password
passwordauthentication no
kbdinteractiveauthentication no
```

Older OpenSSH versions print `without-password` for `prohibit-password`; both mean the same. On
distributions where the service is named `sshd`, use `systemctl reload sshd`.

Confirm from the desktop that password login is now refused:

```bash
desktop$ ssh -o PubkeyAuthentication=no -o PreferredAuthentications=password root@<runner-ip>
```

Expected: `Permission denied (publickey).`

Result (2026-09-24): `root@<runner-ip>: Permission denied (publickey).` Password login is
disabled.

The root password is still needed for the Hetzner web console; keep it in the password manager.

## 6. Update the OS and Enable Automatic Security Updates

```bash
server# apt update && apt full-upgrade -y
server# apt install -y unattended-upgrades
server# cat /etc/apt/apt.conf.d/20auto-upgrades
server# systemctl is-enabled unattended-upgrades
server# test -f /var/run/reboot-required && cat /var/run/reboot-required
```

Expected: `20auto-upgrades` contains `APT::Periodic::Update-Package-Lists "1";` and
`APT::Periodic::Unattended-Upgrade "1";`, and the service is `enabled`. If it is missing, run
`dpkg-reconfigure -plow unattended-upgrades`. Reboot if `/var/run/reboot-required` exists.

Result (2026-09-24):

- `full-upgrade` installed kernel `7.0.0-34-generic` (running kernel was `7.0.0-30-generic`).
- `unattended-upgrades` 2.12ubuntu9 was already installed; `20auto-upgrades` has both settings
  set to `"1"`, and the service is `enabled`.
- `apt` reported `Not Upgrading: 1` (one package held back) and `*** System restart required ***`.

Reboot and confirm the new kernel and the held-back package:

```bash
server# reboot
desktop$ ssh torrust-runner-01
server# uname -r
server# apt list --upgradable
```

Expected: `uname -r` prints `7.0.0-34-generic`.

Result after reboot (2026-09-24): key login with `ssh torrust-runner-01` works, and the login
banner reports kernel `7.0.0-34-generic`. The held-back package is `rust-coreutils`
(`0.8.0-0ubuntu3` installed, `0.10.0-1ubuntu2~26.04.1` available in `resolute-updates`). It is
left for `unattended-upgrades` to install; `apt-cache policy rust-coreutils` shows whether it is
held by Ubuntu's phased updates.

## 7. Restrict Inbound Traffic with a Hetzner Cloud Firewall

Created in the Hetzner Cloud console on 2026-09-24.

| Property  | Value                                                             |
| --------- | ----------------------------------------------------------------- |
| Name      | `torrust-runner-ssh-only`                                         |
| Inbound   | TCP 22 from `0.0.0.0/0` and `::/0`                                |
| Outbound  | No rules, which Hetzner treats as allow all                       |
| Applied to | `torrust-runner-01`                                              |

The runner only makes outbound HTTPS connections (to GitHub, crates.io, and Docker Hub); it never
accepts inbound connections. SSH stays open to any source because password login is disabled, and
restricting it to one IP would lock maintainers out when that IP changes.

Verify from the desktop that SSH still works and another port is filtered:

```bash
desktop$ ssh torrust-runner-01 true && echo ssh-ok
desktop$ nc -zv -w 5 <runner-ip> 80
```

Expected: `ssh-ok`, and `nc` times out instead of reporting `Connection refused`.

Result (2026-09-24, first check): `ssh-ok`, but `nc` reported
`connect to <runner-ip> port 80 (tcp) failed: Connection refused`. The server itself answered,
so the firewall was not yet filtering traffic. The firewall had been created but not yet applied
to the server.

Result (2026-09-24, after applying the firewall to `torrust-runner-01`): `ssh-ok`, and `nc`
reported `connect to <runner-ip> port 80 (tcp) timed out`. Inbound traffic other than SSH is
dropped.

## 8. Install Docker Engine

The `Test (Docker)` job needs Docker Engine, the Buildx plugin (`docker/setup-buildx-action`),
and the Compose plugin (qBittorrent E2E stacks). Docker's official apt repository supports Ubuntu
26.04 (`resolute`), checked on 2026-09-24.

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
server# docker version && docker buildx version && docker compose version
server# docker run --rm hello-world
```

Result (2026-09-24): Docker Engine client and server `29.8.1`, Buildx `v0.37.1`, Docker Compose
`v5.5.1`; the `docker` service is `enabled`; `hello-world` printed `Hello from Docker!`.

## 9. Install Host Build Tools

The job also runs `cargo run` directly on the host (E2E runners), so the host needs a C toolchain
and common CLI tools. The Rust toolchain itself is installed per job by `dtolnay/rust-toolchain`.

```bash
server# apt install -y build-essential pkg-config git jq unzip
```

Further missing tools will surface during validation (scenario F in [ISSUE.md](ISSUE.md)).

Result (2026-09-24): `gcc 15.2.0`, `git 2.53.0`, `jq 1.8.1`.

The E2E tools depend on `openssl-sys` without the `vendored` feature, so compiling them on the
host needs the system OpenSSL headers (found through `pkg-config`). `libsqlite3-sys` is built with
its `bundled` feature and needs no system package. Installed on 2026-09-26, while implementing T5:

```bash
server# apt install -y libssl-dev
```

Result (2026-09-26): `libssl-dev 3.5.5-1ubuntu3.5`; no service restart needed.

## 10. Create the `runner` User

The GitHub runner refuses to run as `root` by default. Membership in the `docker` group is
equivalent to root on this host. The original plan accepted this because the host is dedicated to
CI; review of PR #2335 showed the exposure is wider (persistent host compromise, the runner
registration, later push jobs), and the design was rejected, then accepted with controls on
2026-09-25 (T9). See the risks in
[ISSUE.md](ISSUE.md) and
[`self-hosted-runner-security-research.md`](self-hosted-runner-security-research.md).

```bash
server# useradd --create-home --shell /bin/bash runner
server# usermod -aG docker runner
server# id runner
server# su - runner -c 'docker run --rm hello-world'
```

Result (2026-09-24): `uid=1000(runner) gid=1000(runner) groups=1000(runner),983(docker)`, and
`runner` can run containers (`Hello from Docker!`).

From step 8 onward the commands were run remotely from the maintainer's desktop with
`ssh -o BatchMode=yes torrust-runner-01 '...'`. Prefix remote commands with
`export LC_ALL=C.UTF-8` to avoid locale warnings caused by the desktop's forwarded `LC_*`
variables.

## 11. Prune Docker Storage Daily

The self-hosted jobs keep Docker layers and BuildKit cache mounts on local disk (T5(b)), so the
cache needs a bound. A root systemd timer runs daily at 04:00 UTC. It caps the build cache at
120 GB (least recently used records go first), removes stopped containers older than a day,
unused images older than a week, and anonymous volumes left by E2E runs. Build cache in use by a
running job and running containers are never removed. Each file is written with a one-line
`printf`, because pasted multi-line commands lost lines earlier.

```bash
server# printf '%s\n' '[Unit]' 'Description=Prune Docker build cache and unused objects for the CI runner' '' '[Service]' 'Type=oneshot' 'ExecStart=/usr/bin/docker builder prune --force --max-used-space 120GB' 'ExecStart=/usr/bin/docker container prune --force --filter until=24h' 'ExecStart=/usr/bin/docker image prune --force --filter until=168h' 'ExecStart=/usr/bin/docker volume prune --force' > /etc/systemd/system/docker-ci-prune.service
server# printf '%s\n' '[Unit]' 'Description=Daily Docker prune for the CI runner' '' '[Timer]' 'OnCalendar=*-*-* 04:00:00 UTC' 'Persistent=true' '' '[Install]' 'WantedBy=timers.target' > /etc/systemd/system/docker-ci-prune.timer
server# systemctl daemon-reload && systemctl enable --now docker-ci-prune.timer
server# systemctl start docker-ci-prune.service && systemctl list-timers docker-ci-prune.timer
```

Result (2026-09-26): the manual run exited `0/SUCCESS` for all four commands, and the timer is
enabled with its next run at 2026-09-27 04:00 UTC. The host-side Cargo target directories under
`/home/runner/.cache/torrust-tracker/` are not pruned; T7 measures their growth.

## 12. Add Swap and Restart the Runner After a Crash

The first self-hosted `Test (Docker)` run (PR #2352, 2026-09-27) was cancelled after six minutes.
The kernel log shows a global out-of-memory kill at 06:02 UTC: about eight concurrent `rustc`
processes of the workspace compile (Cargo uses one job per vCPU, so twice the parallelism of a
4 vCPU GitHub-hosted runner) exhausted the 16 GB of RAM on a host with no swap. The kernel killed
processes in the runner's service cgroup (`docker-buildx`, `Runner.Worker`, `Runner.Listener`),
the job ended with "The runner has received a shutdown signal", and the service stayed `failed`,
so the runner was offline until restarted.

Two changes: a 16 GB swap file absorbs compile peaks, and a systemd drop-in restarts the runner
service after a crash. `Restart=on-failure` does not restart it after a deliberate
`systemctl stop`.

```bash
server# fallocate -l 16G /swapfile && chmod 600 /swapfile && mkswap /swapfile && swapon /swapfile && echo '/swapfile none swap sw 0 0' >> /etc/fstab
server# mkdir -p /etc/systemd/system/actions.runner.torrust-torrust-tracker.torrust-runner-01.service.d && printf '%s\n' '[Service]' 'Restart=on-failure' 'RestartSec=10' > /etc/systemd/system/actions.runner.torrust-torrust-tracker.torrust-runner-01.service.d/restart.conf && systemctl daemon-reload
server# systemctl reset-failed actions.runner.torrust-torrust-tracker.torrust-runner-01.service && systemctl start actions.runner.torrust-torrust-tracker.torrust-runner-01.service
```

Result (2026-09-27): `swapon --show` lists `/swapfile` (16G), `systemctl show` reports
`Restart=on-failure` and `RestartUSec=10s`, the service is `active (running)` with the
`restart.conf` drop-in, and GitHub reports
`torrust-runner-01  Linux  online  false  self-hosted,Linux,X64,torrust-hetzner`. If swap is not
enough, the next step is to cap Cargo's build jobs in the image build.

## Next Steps

- Install and register the GitHub Actions runner (T3): see
  [`runner-agent-installation.md`](runner-agent-installation.md).
