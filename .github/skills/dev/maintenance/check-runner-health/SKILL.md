---
name: check-runner-health
description: Check the self-hosted CI runner (torrust-runner-01) for CPU load, memory, disk space, and whether the automatic Docker cleanup works, then write a dated report under docs/self-hosted-runner-reports/ and open a pull request. Use for periodic runner health checks, disk-full or queued-job suspicions, or when asked to report on the self-hosted runner. Triggers on "runner health", "runner status report", "check self-hosted runner", "runner disk space", or "runner cleanup check".
metadata:
  author: torrust
  version: "1.0"
semantic-links:
  related-artifacts:
    - docs/self-hosted-runner.md
    - docs/self-hosted-runner-reports/README.md
    - docs/templates/RUNNER-HEALTH-REPORT.md
    - .github/skills/dev/git-workflow/create-feature-branch/SKILL.md
    - .github/skills/dev/git-workflow/open-pull-request/SKILL.md
---

# Check Runner Health

Produce a read-only health snapshot of the self-hosted runner and publish it as a report. Setup and
operation of the runner are in [docs/self-hosted-runner.md](../../../../../docs/self-hosted-runner.md); do not
repeat them here.

## Rules

- Read-only. Do not stop containers, run prunes, delete files, or restart services while collecting.
  A fix needs the maintainer's explicit approval and an issue; propose it in the report.
- Do not record the runner's public IP, tokens, or credentials.
- Report only what you measured, and label every cause you did not prove as a hypothesis.
- The SSH key has a passphrase that only the maintainer types. Ask the maintainer to open the
  connection, then reuse it (below). Never request the passphrase in chat.

## 1. Open One SSH Connection

The `torrust-runner-01` host entry is in the maintainer's `~/.ssh/config`. Multiplex it so the
passphrase is typed once:

```bash
ssh -o ControlMaster=auto -o ControlPath=/tmp/ssh-runner-%C -o ControlPersist=15m torrust-runner-01 'hostname; uptime; nproc'
S="ssh -o ControlPath=/tmp/ssh-runner-%C torrust-runner-01"
```

Close it at the end: `ssh -O exit -o ControlPath=/tmp/ssh-runner-%C torrust-runner-01`.

## 2. Collect

Send the commands to the runner through a quoted heredoc, so single quotes, `$`, and parentheses reach
the runner unchanged and nothing needs shell-level escaping. Wrapping a command in `$S '...'` breaks
every command that contains a single quote, such as the `awk` ones.

```bash
$S bash -s <<'EOF'
df -h /
journalctl -u docker-ci-prune.service --since "3 days ago" -o short-iso | awk '{print substr($1,1,10), $NF}'
EOF
```

In the table, `\|` is Markdown escaping for a pipe; write a plain `|` in the command. Run the commands
marked local on your own machine, not through `$S`.

| Question              | Command                                                                                                                                                  |
| --------------------- | -------------------------------------------------------------------------------------------------------------------------------------------------------- |
| Runner registered?    | Local: `gh api repos/torrust/torrust-tracker/actions/runners -q '.runners[] \| [.name, .os, .status, (.busy \| tostring), ([.labels[].name] \| join(","))] \| join("  ")'` |
| Service state         | `systemctl show actions.runner.torrust-torrust-tracker.torrust-runner-01.service -p ActiveState -p ActiveEnterTimestamp -p NRestarts`                    |
| Agent, OS, uptime     | `/home/runner/actions-runner/bin/Runner.Listener --version; . /etc/os-release; echo $PRETTY_NAME; uname -r; uptime; ls /var/run/reboot-required`        |
| CPU now               | `nproc; cat /proc/loadavg; top -b -n1 \| head -5; vmstat 1 5`                                                                                            |
| CPU per day           | For each `sa<DD>` in `/var/log/sysstat`: `sar -u -f /var/log/sysstat/sa<DD>` and `sar -q -f ...` (average idle, minimum idle, peak `ldavg-5`)            |
| Memory, swap, OOM     | `free -h; swapon --show; journalctl _TRANSPORT=kernel --since "30 days ago" \| grep -c "Killed process"` (not `journalctl -k`: it implies the current boot only, so after a reboot it hides earlier kills and can print a false zero) |
| Disk                  | `df -h /; df -i /; docker system df; du -xh --max-depth=1 / \| sort -rh \| head; du -sh /home/runner/.cache/torrust-tracker/*`                           |
| Cache records         | `docker buildx du \| tail -3`; `docker system df -v \| sed -n '/Build cache usage/,$p'` (a `*` after a cache id means in use; count them); `ctr -n moby snapshots ls \| awk 'NR>1{print $3}' \| sort \| uniq -c` |
| Held containers       | `docker ps -a --format '{{.ID}} {{.Image}} {{.Names}} {{.Status}} {{.CreatedAt}}'`; `docker image ls -a`                                                 |
| Disk-full evidence    | `journalctl --since "7 days ago" \| grep -ci "no space left"` and the date of the first match                                                            |
| Cleanup timer         | `systemctl status docker-ci-prune.timer docker-ci-prune.service --no-pager`; `systemctl is-failed docker-ci-prune.service` (check by name, `systemctl --failed` lists unrelated units); `systemctl show docker.service -p ActiveEnterTimestamp` (a recent start means Docker was restarted) |
| Bytes freed per run   | `journalctl -u docker-ci-prune.service --since "30 days ago" -o short-iso \| grep -E 'docker(-ci-prune)?\[[0-9]+\]: Total:' \| awk '{print substr($1,1,10), $NF}'` |
| CI impact             | Local: `gh run list --repo torrust/torrust-tracker --workflow container.yaml --limit 15`, then `gh api repos/torrust/torrust-tracker/actions/runs/<id>/jobs` for `runner_name`, step conclusions, and times |

The prune service writes four commands to the journal. The build cache prune prints a line that
starts with `Total:` (logged as `docker[<pid>]` by the old unit or `docker-ci-prune[<pid>]` by the
script from issue 2673); the container, image, and volume prunes print `Total reclaimed space:`
instead. Do not mix them up. Runs of the cleanup script also log `docker-ci-prune[<pid>]: Disk usage after the prune: N%` and, when it restarts
Docker, a line that says so.

Reference points (from the setup guide): the build cache trim target is 120 GiB and the prune
overshoots it somewhat, so about 125 to 130 GB is normal. Treat these as WARN: disk at or above 70%,
a prune that frees 0 B for two days while the cache is far above target, a container up for more than
a day, and a pending reboot. Treat these as CRITICAL: disk at or above 85%, a failed
`docker-ci-prune.service`, a cache of hundreds of GB with every record starred in `docker system df -v`
and 0 B reclaimable (the stuck state of the 2026-10-27 incident), and failing runner jobs.

The jobs list of a run shows only the latest attempt of a re-run. When you need the first attempt of a
failed job, read `gh api repos/torrust/torrust-tracker/actions/runs/<id>/attempts/<n>/jobs`.

## 3. Write the Report

1. Copy the skeleton from [RUNNER-HEALTH-REPORT.md](../../../../../docs/templates/RUNNER-HEALTH-REPORT.md) to
   `docs/self-hosted-runner-reports/<YYYYMMDD-HHMMSS>-runner-report.md` (UTC time of the first
   measurement).
2. Fill every section with measured values. Compare with the previous report in the folder, as a
   table of the same measures. When the previous report named a wrong cause, do not edit it (reports
   are snapshots); state the correction in the new report.
3. Add a row, newest first, to the table in
   [docs/self-hosted-runner-reports/README.md](../../../../../docs/self-hosted-runner-reports/README.md).
4. Avoid `#<number>` text in Markdown (see the `write-markdown-docs` skill); write run ids and issue
   numbers without a hash unless you mean a link.

## 4. Publish

Follow `create-feature-branch` (use `chore/runner-health-report-<YYYYMMDD>` when no issue exists), run
`linter markdown` and `linter cspell` (or `linter all`), commit with
`docs(self-hosted-runner): add runner health report <YYYY-MM-DD>` using `commit-changes`, and open
the pull request with `open-pull-request`. When the report has CRITICAL or WARN findings, open an
issue with `create-issue` and link it from the report and the pull request.
