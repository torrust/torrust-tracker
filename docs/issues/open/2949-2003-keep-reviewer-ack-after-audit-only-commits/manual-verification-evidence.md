---
doc-type: manual-verification-evidence
issue-spec: docs/issues/open/2949-2003-keep-reviewer-ack-after-audit-only-commits/ISSUE.md
last-updated-utc: "2026-11-21 13:23"
---

# Manual Verification Evidence

## Purpose

Record real, human-oriented verification of the completed behavior. This is
evidence from commands actually performed; nothing here is invented.

## Environment and Prerequisites

- Date and time (UTC): 2026-11-21 13:22
- Artifact under test: the command documented in `merge-pull-request`, section An ACK Names One
  Commit, added by `docs(merge-pull-request): [#2949] say an ACK names one commit and how to
re-check after an audit-only commit`.
- Operating system / environment: Linux 7.0.0-43-generic, Git 2.53.0.
- Prerequisites and setup performed: `git fetch torrust`; the implementation branch was cut from
  `develop` at `4be22ee66`. Every commit named below is on `develop`.

## Verification Processes

The two processes are independent: each only reads history, and neither changes state.

### V1 - ACK followed by an audit-only commit (M1)

- Goal: on PR #2947's history, the documented command lists only that pull request's audit record.
- Initial state: `e01212151` is the commit both ACKs named on PR #2947; `1d27a67e9` is the audit
  commit pushed after them.
- Status: `DONE`

#### Steps Performed

1. Started at 13:22:04.279 UTC and ended at 13:22:04.336 UTC:
   `git log --no-renames --diff-merges=first-parent --name-only --format= e01212151..1d27a67e9`

#### Observed Result

```text
docs/pr-reviews/pr-2947-review/PR-REVIEW.md
```

The command exited 0.

#### Conclusion

Met. The only listed path starts with `docs/pr-reviews/pr-2947-review/`, so the commit after the
ACKs changed only the audit record, as the skill text says. The ACKs on `e01212151` were still
stale: an ACK counts only on the commit it names.

### V2 - ACK followed by a commit outside the folder (M2)

- Goal: on a range whose commits change a file outside the audit folder, the command lists the
  other paths.
- Initial state: the range is from PR #2950's own history. `eb07319da` is the head da2ce7 ACKed
  after round 7; `163e74560` is the head after round 8, which rewrote the specification and
  updated the EPIC row.
- Status: `DONE`

#### Steps Performed

1. Started at 13:22:04.336 UTC and ended at 13:22:04.395 UTC:
   `git log --no-renames --diff-merges=first-parent --name-only --format= eb07319da..163e74560`

#### Observed Result

```text
docs/pr-reviews/pr-2950-review/PR-REVIEW.md
docs/issues/open/2949-2003-keep-reviewer-ack-after-audit-only-commits/ISSUE.md
docs/issues/open/2003-overhaul-guardrails-and-automation/EPIC.md
docs/issues/open/2949-2003-keep-reviewer-ack-after-audit-only-commits/ISSUE.md
```

The command exited 0.

#### Conclusion

Met. Three of the four listed paths are outside `docs/pr-reviews/pr-2950-review/`, so the
reviewer has more than the audit record to read. The specification is listed twice because two
commits in the range changed it: the command lists the paths of each commit.

## Failures and Follow-up

None.
