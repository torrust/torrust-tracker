# Runner Agent Installation Log

<!-- cspell:ignore installdependencies tostring -->

Step-by-step record of how the GitHub Actions runner agent was installed and registered on
`torrust-runner-01` for issue #2323 (task T3). The host preparation is in
[`runner-server-setup.md`](runner-server-setup.md); the plan is in [ISSUE.md](ISSUE.md).

Conventions:

- `server#` runs as `root`, `runner$` runs as the `runner` user (`su - runner`).
- `<TOKEN>` is the runner registration token. It is a secret: the maintainer substitutes it and
  runs the command. Never commit it.

## Settings

| Property        | Value                                                              |
| --------------- | ------------------------------------------------------------------ |
| Runner version  | `v2.337.0` (latest release, published 2026-08-26)                  |
| Install path    | `/home/runner/actions-runner`                                      |
| Service user    | `runner`                                                           |
| Registration    | Repository level: `https://github.com/torrust/torrust-tracker`     |
| Runner name     | `torrust-runner-01`                                                |
| Custom label    | `torrust-hetzner` (plus the default `self-hosted`, `Linux`, `X64`) |
| Workflow target | `runs-on: [self-hosted, torrust-hetzner]`                          |

### Why Repository-Level Registration

GitHub's generic instructions register the runner at organization level
(`--url https://github.com/torrust`). On the GitHub Free plan an organization has only the default
runner group, and creating custom groups requires GitHub Team. Serving a public repository from the
default group would require allowing public repositories on it, making the runner available to
every public Torrust repository and to fork PRs against any of them. Registering at repository
level limits the runner to `torrust/torrust-tracker`.

Reference: <https://docs.github.com/en/actions/how-tos/manage-runners/self-hosted-runners/manage-access>
(checked 2026-09-24).

### Why a Custom Label

`runs-on: self-hosted` alone would match any self-hosted runner added later. The dedicated
`torrust-hetzner` label makes the target explicit and lets more instances join the same pool
(scenario D in [ISSUE.md](ISSUE.md)).

## 1. Download and Verify the Runner

```bash
runner$ mkdir -p ~/actions-runner && cd ~/actions-runner
runner$ curl -fsSL -o actions-runner-linux-x64-2.337.0.tar.gz \
  https://github.com/actions/runner/releases/download/v2.337.0/actions-runner-linux-x64-2.337.0.tar.gz
runner$ echo "70920811a4f8ad4328818682bca5c6469c1c942fab52448868071d0063816613  actions-runner-linux-x64-2.337.0.tar.gz" \
  | sha256sum -c
runner$ tar xzf actions-runner-linux-x64-2.337.0.tar.gz && rm actions-runner-linux-x64-2.337.0.tar.gz
server# cd /home/runner/actions-runner && ./bin/installdependencies.sh
```

The expected hash comes from the `v2.337.0` release notes.

Result (2026-09-24): `actions-runner-linux-x64-2.337.0.tar.gz: OK`; extracted; the dependency
script reported nothing to install.

## 2. Register the Runner

Get a registration token from the repository: **Settings -> Actions -> Runners -> New
self-hosted runner** (Linux, x64), and copy the value after `--token` in the displayed command.
The token expires after one hour.

```bash
runner$ cd ~/actions-runner
runner$ ./config.sh --unattended \
  --url https://github.com/torrust/torrust-tracker \
  --token <TOKEN> \
  --name torrust-runner-01 \
  --labels torrust-hetzner \
  --work _work
```

The generic **New self-hosted runner** page under the organization settings shows
`--url https://github.com/torrust`; use the repository page
(`https://github.com/torrust/torrust-tracker/settings/actions/runners/new`) instead, whose command
shows `--url https://github.com/torrust/torrust-tracker`.

Result (2026-09-24): `Connected to GitHub`, `Runner successfully added`, `Settings Saved.`

## 3. Run the Runner as a Service

Use the service instead of `./run.sh`, so the runner starts on boot and restarts after failures.

```bash
server# cd /home/runner/actions-runner
server# ./svc.sh install runner
server# ./svc.sh start
server# ./svc.sh status
```

Run `svc.sh` from `/home/runner/actions-runner` as `root`, one command at a time. Pasting the block
together with the `exit` that leaves the `runner` shell drops the commands after `exit`.

Result (2026-09-24): `install` created
`/etc/systemd/system/actions.runner.torrust-torrust-tracker.torrust-runner-01.service`, running as
`runner` (uid 1000), enabled at boot. `start` reported `Active: active (running)`. The service log
shows `Connected to GitHub`, `Current runner version: '2.337.0'`, and `Listening for Jobs`.

## 4. Verify on GitHub

The runner appears as **Idle** under **Settings -> Actions -> Runners** with the labels
`self-hosted`, `Linux`, `X64`, and `torrust-hetzner`. No workflow targets it until the
implementation PR (T5).

```bash
desktop$ gh api repos/torrust/torrust-tracker/actions/runners \
  -q '.runners[] | [.name, .os, .status, (.busy | tostring), ([.labels[].name] | join(","))] | join("  ")'
```

Result (2026-09-24), fields are name, OS, status, busy, and labels:

```text
torrust-runner-01  Linux  online  false  self-hosted,Linux,X64,torrust-hetzner
```
