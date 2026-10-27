---
semantic-links:
  skill-links:
    - process-pr-review
  related-artifacts:
    - .github/skills/dev/pr-reviews/process-pr-review/SKILL.md
---

<!-- skill-link: process-pr-review -->

# PR #2672 Review Audit

Source: pull-request reviews and inline review threads for <https://github.com/torrust/torrust-tracker/pull/2672>.

## Ownership

The PR author owns this tracked audit record. Reviewers, including repository review agents,
deliver findings through GitHub and have no repository-artifact obligation.

- Post-merge workflow approval: N/A (the review was processed before the merge)

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

Round 1 is the Copilot review `5665072589` (https://github.com/torrust/torrust-tracker/pull/2672#pullrequestreview-5665072589), submitted 2026-10-27 09:46:19 UTC. Its body is a verdict plus a list of the same four open issues as the four inline threads, so it adds no separate row. The review was not processed when it arrived: it was found and processed at 12:41 UTC, after the maintainer reported the unresolved threads (see the Processing Log).

| Finding ID | Review finding reference | Author class | Severity | Category | Relationship | Disposition | Thread state |
| ---------- | ------------------------ | ------------ | -------- | -------- | ------------ | ----------- | ------------ |
| F1 | review-finding:pr-2672-f1 | Copilot | Major (inferred) | correctness | ORIGINAL | FIXED | RESOLVED |
| R2 | review-finding:pr-2672-r2 | Copilot | Major | correctness | ORIGINAL | FIXED | RESOLVED |
| F3 | review-finding:pr-2672-f3 | Copilot | Minor (inferred) | correctness | ORIGINAL | FIXED | RESOLVED |
| F4 | review-finding:pr-2672-f4 | Copilot | Minor (inferred) | documentation | ORIGINAL | FIXED | RESOLVED |

## Finding Details

### F1 - Use a quoted heredoc to preserve embedded shell quotes

- PR number: 2672
- Source review ID: 5665072589
- Reviewer finding ID: N/A
- Source URL: https://github.com/torrust/torrust-tracker/pull/2672#discussion_r4387191261
- Concern: The skill says to run each command as `$S '<command>'`, which breaks table entries that contain single-quoted arguments; the bytes-freed command produces a local syntax error at `substr(` before SSH runs. Copilot asks for a quoted heredoc so quotes and `$` reach the runner unchanged. The severity is inferred from the prose: the broken command prevents cleanup-history collection.
- Solution: Fixed. The skill now sends the commands through a quoted heredoc (`$S bash -s <<'EOF' ... EOF`), explains that `\|` in the table is Markdown escaping, and marks the local commands. The bytes-freed command's journal pattern was also widened to `docker(-ci-prune)?`, because since the cleanup script was installed the lines are logged as `docker-ci-prune[<pid>]`; this was found while rewriting the command and is part of the same commit.
- Current-tree verification: Reproduced the finding locally before the fix: `S=echo; bash -c "$S '<bytes-freed command>'"` printed `syntax error near unexpected token '('`. After the fix, ran the documented heredoc form on `torrust-runner-01`: `df -h /`, the `awk` timestamp command, and the bytes-freed command (with the widened pattern) all printed their results. `grep -c "Run each as"` on the skill prints 0 and `grep -n 'bash -s <<'` prints line 50.
- Resolution reference: docs(self-hosted-runner): run the health commands through a quoted heredoc
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2672#discussion_r4388848021

### R2 - Include previous boots in the 30-day OOM query

- PR number: 2672
- Source review ID: 5665072589
- Reviewer finding ID: R2
- Source URL: https://github.com/torrust/torrust-tracker/pull/2672#discussion_r4387191294
- Concern: `journalctl -k` implicitly restricts the query to the current boot, so `--since "30 days ago"` does not inspect the full period after a reboot, and a reboot (which the report recommends) could produce a false zero. Copilot asks for `_TRANSPORT=kernel`.
- Solution: Fixed. The skill now uses `journalctl _TRANSPORT=kernel --since "30 days ago"` and says why `-k` must not be used. The first report's zero is still valid and its text now explains why: the current boot began on 2026-09-24 15:55 UTC, so it covered the whole window, and the same query across all boots finds 2 kills in the whole journal, both earlier than that boot. The guide's own `journalctl -k` line in `docs/self-hosted-runner.md` has no time window, existed before this pull request, and was not changed.
- Current-tree verification: On the maintainer's desktop (11 boots), `journalctl -k --since '30 days ago'` first entry was today's boot and `journalctl _TRANSPORT=kernel --since '30 days ago'` first entry was 2026-09-27, so the finding holds. On the runner, `journalctl --list-boots` showed the current boot starting 2026-09-24 15:55:59 UTC, and the kernel-transport query over all boots counted 2 `Killed process` lines and 0 in the last 30 days.
- Resolution reference: docs(self-hosted-runner): read kernel messages across boots for the OOM check
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2672#discussion_r4388848288

### F3 - Use the final field to count parentless snapshots

- PR number: 2672
- Source review ID: 5665072589
- Reviewer finding ID: N/A
- Source URL: https://github.com/torrust/torrust-tracker/pull/2672#discussion_r4387191330
- Concern: `ctr -n moby snapshots ls` prints `KEY PARENT KIND`; a parentless snapshot leaves `PARENT` empty, awk collapses the whitespace, and `$3` counts those snapshots under a blank label instead of their kind. Copilot asks for `$NF`. The severity is inferred from the prose: the reported totals are inaccurate.
- Solution: Fixed. The skill now reads `$NF` and explains the reason. The first report's Committed count is corrected from 491 to 500 by the 9 parentless snapshots, which matches the full classification of the 621 snapshots in the issue evidence. The second report's counts had the same flaw and its 16 blank-label snapshots were not split by kind when measured, so it now says "at least 16 Active" and states the gap instead of inventing a split. The check on the runner also showed that parentless snapshots can be Active (39 Active with `$NF` against 28 with `$3`), so the blank count cannot be assumed to be Committed.
- Current-tree verification: On the runner, `awk 'NR>1{print $3}'` printed 21 blank rows, 28 Active, 308 Committed, and 5 View, while `awk 'NR>1{print $NF}'` printed 39 Active, 318 Committed, and 5 View. `grep -c 'print \$3}'` on the skill prints 0, and `grep -n '500 Committed'` on the first report prints line 85.
- Resolution reference: docs(self-hosted-runner): count containerd snapshots by their last field
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2672#discussion_r4388848562

### F4 - Correct mixed binary and decimal storage units

- PR number: 2672
- Source review ID: 5665072589
- Reviewer finding ID: N/A
- Source URL: https://github.com/torrust/torrust-tracker/pull/2672#discussion_r4387191376
- Concern: `df -h` and `du -h` print binary units (GiB, MiB) while Docker prints decimal GB, so labelling all of them GB misstates the storage baseline. The same mix affects line 94: the 120 GiB target is about 128.8 GB, so the excess is about 125 GB, not 134 GB. Copilot asks to keep the binary units for filesystem and directory figures and convert the target before comparing it with Docker's figures. The severity is inferred from the prose.
- Solution: Fixed in both reports. Filesystem, directory, journal, and Cargo cache figures are labelled GiB or MiB, Docker's figures stay GB, and each report states which source uses which unit. The target comparison is corrected to about 125 GB above a 128.8 GB target. The same mix made the second report call the 127.2 GB cache "above" the target; it is just under 128.8 GB, and the test run agrees (149.9 GB minus the 22.69 GB freed leaves 127.2 GB). The second report's "released 150 GB" is made exact (153.6 GB of cache and 144 GiB of disk). Documents outside this pull request that were merged earlier, in the issue folder, have the same unit mix and were not changed.
- Current-tree verification: `grep -c '134 GB'` on the first report prints 0, `grep -n '128.8 GB'` prints lines 98 and 176 of the first report and lines 96, 118, and 145 of the second, and `grep -n '301 GiB'` prints lines 18 and 81 of the first report. The target conversion was checked by computation: 120 GiB x 1.0737 = 128.8 GB, and 253.6 - 128.8 = 124.8 GB.
- Resolution reference: docs(self-hosted-runner): label disk figures in GiB and compare them with Docker in GB
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2672#discussion_r4388848941

## Processing Log

- 2026-10-27 09:46 UTC - Copilot review `5665072589` submitted (GitHub `created_at` 09:46:18 to 09:46:19 UTC) with four inline findings. It was not processed at that time: I had not fetched the threads of this pull request.
- 2026-10-27 12:38 UTC - After pull request 2675 merged, the branch was rebased onto `develop` and force-pushed (rebased commits dated 12:38:30 to 12:38:45 UTC, from `git log --format=%cI`). The commit ids the review saw no longer exist, so this audit cites commit subjects.
- 2026-10-27 12:41 UTC - The maintainer reported unresolved threads. Fetched them with `github-review-threads fetch` (command start 12:41:18 UTC) and found four unresolved threads, three of them already outdated.
- 2026-10-27 12:45 UTC - Verified each claim, then committed the four fixes as four signed commits (commit times 12:45:55, 12:46:46, 12:48:00, and 12:49:04 UTC, from `git log --format=%cI`) and pushed them (push output showed `ccdc21181..7aad7831c`).
- 2026-10-27 12:49 UTC - Replied on the four threads (reply `created_at` 12:49:46, 12:49:48, 12:49:50, and 12:49:52 UTC, GitHub); `reply-status --login josecelano` exited 0; each resolve mutation returned `isResolved: true` (the command's own date line printed 12:50:11 UTC). A refetch then listed all four threads as resolved by josecelano and zero unresolved threads.
- 2026-10-27 12:51 UTC - Wrote this audit and validated it with `validate-audit-record.py --pr-number 2672 --base torrust/develop` (run at 12:51:09 UTC), which exited 0 with 4 rows, 6 log entries, and 0 failures.

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
