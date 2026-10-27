# Deferred Proposals: Runner Alerting and Prevention

<!-- cspell:ignore Hetzner moby -->

Proposals for issue [2673](https://github.com/torrust/torrust-tracker/issues/2673) that the maintainer chose **not** to implement now: no notifications and no scheduled workflows. They are recorded so they can be picked up if a full or broken runner becomes a recurring pain. Nothing here is implemented, and the ranking is an opinion, not a measurement.

Prepared on 2026-10-27 by GitHub Copilot (Claude Opus 5.5) after the first runner disk-full incident. See [manual-verification-evidence.md](manual-verification-evidence.md) for the evidence and [ISSUE.md](ISSUE.md) for what was implemented instead: an idle-guarded Docker restart in the daily prune, and a failed `docker-ci-prune.service` when the disk stays at or above 85%.

## What Is Missing Today

The incident was found when pull request checks stopped working, not before. After this issue the runner repairs itself in the common case, and a failure leaves a failed systemd unit, but nobody is told. A person finds out only by running `systemctl --failed` on the host, reading a health report, or noticing failed checks. Two different failures need detecting:

1. The runner is offline, stuck, or its jobs fail or queue for a long time. GitHub can see this from outside.
2. The host disk fills or the prune keeps failing. Only the host can see this.

## Constraint That Shapes Every Option

The runner gives job code root-equivalent access to the host through the `docker` group (see Security in [docs/self-hosted-runner.md](../../../self-hosted-runner.md)), and the guide's rule is that the server never holds Torrust credentials. Any secret placed on the host, such as a webhook URL, an SMTP password, or an API token, can be read by a job. So prefer options that keep secrets off the runner, or accept a secret that is worthless if leaked (for example a ping URL that can only mark a check as alive).

## Proposals

### P1. Scheduled GitHub-hosted workflow that watches the runner (recommended first)

A workflow on `ubuntu-latest` with a `schedule:` trigger, every 15 to 30 minutes, calls the GitHub API with the repository `GITHUB_TOKEN` and fails or opens an issue when:

- `torrust-runner-01` is `offline`, or
- a job with the `torrust-hetzner` label has been `queued` for more than 15 minutes (the threshold the guide already uses for adding capacity), or
- the last N runs of `Test (Docker)` on the runner failed within seconds.

Catches failure 1, which is exactly the symptom seen on 2026-10-27. Needs no secret on the host and runs outside the untrusted-code host. Cannot see disk usage. Needs `actions: read` and, to open an issue, `issues: write`; set permissions explicitly, as issue 2495 asks. Cost: GitHub-hosted minutes, and scheduled workflows are disabled after 60 days without repository activity, which is not a risk on this repository.

### P2. Host-side failure hook that sends a message

Add `OnFailure=docker-ci-prune-failed.service` to `docker-ci-prune.service`; the failure unit posts to a webhook, a mail relay, or a chat channel. Catches failure 2 at the moment it happens. Needs a secret on the host, which job code can read, so use a webhook that can only post to one channel, rotate it, and record the exception in the ADR. It also cannot report that the whole host is down.

### P3. Dead-man's-switch ping from the daily prune

The prune ends by calling a heartbeat URL on success, and an external service alerts when the ping is missing. Covers both a failed prune and a dead host. The URL is a weak secret: a leak lets someone fake a success ping, not read anything. Adds a third-party dependency and an account to maintain.

### P4. Fail-fast disk check at the start of the Docker jobs

A first step in `Test (Docker)` and `Docker E2E` that runs `df` and fails with a clear message such as "runner disk is above 85%, see When the Disk Is Full" instead of a cryptic `No space left on device` deep in a build. This is a job step, not a scheduled workflow. It does not notify anyone, but it makes the failing check explain itself. The step runs on the runner, so it must read only host state and never touch secrets.

### P5. Regular manual health reports

Keep running the `check-runner-health` skill on a fixed cadence (for example weekly, and after any incident). No automation, no secrets, and it already measures CPU, disk, and prune effectiveness. Its weakness is that it only finds a problem when someone runs it.

## Prevention Proposals

These attack the cause rather than the detection.

### R1. Research upstream and reproduce the trigger

The cause of the leaked build state is still a correlation with cancelled builds (see V2). Before building more automation: reproduce a cancelled build on the runner's stack (Docker 29.8.1, containerd snapshotter, BuildKit 0.33.0, `docker/build-push-action` with `load: true`) on an idle runner, and search the `moby/moby` and `moby/buildkit` issue trackers for the same symptom. If an upstream fix exists, updating Docker may remove the cause. Not done yet: the maintainer did not approve a test build on the production runner.

### R2. Use the `docker-container` BuildKit driver on the runner

`.github/workflows/container.yaml` selects `driver: docker` for self-hosted runs so builds use the daemon's own BuildKit. A `docker-container` builder runs BuildKit in its own container, so leaked state could be released by removing that container without restarting dockerd and without stopping other containers. It needs a benchmark first, because the cache then lives in a volume and `--load` copies the image into the daemon.

### R3. Restart Docker on a schedule while idle

Restart `docker.service` every night when no job runs, regardless of disk usage. Deterministic and simple, but it throws away in-memory BuildKit state needlessly and the current threshold-based restart already does the job when it is needed. Choose it only if the threshold logic proves unreliable.

### R4. Larger disk

The 301 GB disk was full at a 253 GB cache. A larger disk only delays the failure if build state keeps leaking. Choose it only after R1 or R2 bound the leak.

## Suggested Order and Triggers

1. Run P5 on a fixed cadence now; it costs nothing.
2. Implement P1 the first time a second incident is found by failed pull request checks rather than by a person looking, or if a check stays queued for over 15 minutes. It is the cheapest option that detects the symptom the maintainer actually saw.
3. Implement P4 together with P1 if developers keep losing time to the cryptic `No space left on device`.
4. Do R1 before any larger prevention work (R2, R3, R4); it decides which of them is needed.
5. Consider P2 or P3 only if the host can fail without a visible effect on checks, and only after deciding how to treat the secret.
