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

## 5. Stop and Disable the Runner

The persistent runner was found unsafe for fork PRs (see
[`self-hosted-runner-security-research.md`](self-hosted-runner-security-research.md)). A fork PR
can add its own workflow targeting the runner's labels, so a registered, online runner is exposed
even when no workflow in the repository uses it.

```bash
server# cd /home/runner/actions-runner && ./svc.sh stop
server# systemctl disable actions.runner.torrust-torrust-tracker.torrust-runner-01.service
```

Result (2026-09-24): the service stopped at 20:16 UTC (`Active: inactive (dead)`), GitHub reports
the runner as `offline`, and the unit is `disabled`. The registration still exists on GitHub; remove
it under **Settings -> Actions -> Runners** if the runner is not going to be used.

## 6. Remove the Runner

The maintainer decided to clean up the runner rather than keep it for later use.

```bash
server# cd /home/runner/actions-runner && ./svc.sh uninstall
desktop$ gh api repos/torrust/torrust-tracker/actions/runners -q '.runners[] | "\(.id) \(.name)"'
desktop$ gh api -X DELETE repos/torrust/torrust-tracker/actions/runners/<runner-id>
server# rm -rf /home/runner/actions-runner
```

The registration is deleted through the REST API with the maintainer's own `gh` login, so no
removal token is needed. The install directory is deleted because it holds the runner's
registration files (`.runner`, `.credentials`).

Result (2026-09-25): the systemd unit is gone (`0 unit files listed`), the repository has no
self-hosted runners (`total_count` 0), and `/home/runner/actions-runner` no longer exists. The
`runner` user, Docker, and the rest of the host preparation remain on the server.

## 7. Apply Access Controls Before Registering Again (T9)

On 2026-09-25 the maintainers decided to keep the persistent runner with controls (see the design
decision in [ISSUE.md](ISSUE.md)). Two repository and organization settings must be in place
before the runner is registered again.

### Fork-PR Approval for All External Contributors

```bash
desktop$ gh api -X PUT repos/torrust/torrust-tracker/actions/permissions/fork-pr-contributor-approval \
  -f approval_policy=all_external_contributors
desktop$ gh api repos/torrust/torrust-tracker/actions/permissions/fork-pr-contributor-approval \
  -q .approval_policy
```

The equivalent UI setting is **Settings -> Actions -> General -> Approval for running fork pull
request workflows from contributors -> Require approval for all external contributors**.

Result (2026-09-26): `all_external_contributors` (previously `first_time_contributors`).

### Two-Factor Authentication for Organization Members

Enabled by an organization owner under **Organization settings -> Authentication security ->
Require two-factor authentication**. Enabling it removes every member and outside collaborator
who does not use 2FA, so check first:

```bash
desktop$ gh api "orgs/torrust/members?filter=2fa_disabled" -q length
desktop$ gh api "orgs/torrust/outside_collaborators?filter=2fa_disabled" -q length
desktop$ gh api orgs/torrust -q .two_factor_requirement_enabled
```

Result (2026-09-26): 0 members and 3 outside collaborators without 2FA;
`two_factor_requirement_enabled` is `false`. Pending: the outside collaborators are asked to
enable 2FA before the requirement is turned on.

Result (2026-09-26, after an owner enabled the requirement): `two_factor_requirement_enabled` is
`true`, and no member or outside collaborator lacks 2FA. All 4 members remain. The 3 outside
collaborators without 2FA were removed by the requirement; they can be invited again once they
enable 2FA. The organization setting also
required the owner to disable SMS as a 2FA method on their own account.
