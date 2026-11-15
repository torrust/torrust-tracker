---
doc-type: manual-verification-evidence
issue-spec: docs/issues/open/2866-2003-move-skills-to-claude-skills/ISSUE.md
last-updated-utc: 2026-11-15 20:55
---

# Manual Verification Evidence

## Purpose

Record real, human-oriented verification of the completed behavior. This is
evidence from commands or interactions actually performed against the artifact;
do not invent commands, output, logs, or results.

## Environment and Prerequisites

- Date and time (UTC): 2026-11-15 20:48 (V0)
- Artifact under test: V0: `develop` at `abd7a10ff` (`Merge torrust/torrust-tracker#2863`), before
  any change of this issue.
- Operating system / environment: Linux 7.0.0-43-generic; Claude Code 2.1.318
  (`claude --version`); model reported by `/context`: `claude-sonnet-5-5`, 1M context window.
- Prerequisites and setup performed: a fresh local clone of `develop` in a scratch folder with no
  `.claude/` folder (`ls .claude` failed with "No such file or directory");
  `find . -name SKILL.md -not -path './.git/*' | wc -l` printed `53`.

## Verification Processes

### V0 - Reproduce the defect before T3 (M0)

- Goal: show, in the real repository, that Claude Code does not discover the repository's skills.
- Initial state: fresh clone at `abd7a10ff`, detached `HEAD`, clean tree.
- Status: `DONE` (outcome: **Reproduced**)

#### Steps Performed

The two commands ran one after the other in one shell, at the clone's root. The times are the
output files' birth and last-modification times.

1. 20:48:11.17 to 20:48:13.86 UTC: `claude -p "/context" > m0-context.md`
2. 20:48:14.77 to 20:48:26.22 UTC: `claude -p "/create-issue" > m0-create-issue.txt`

#### Observed Result

Step 1, the category table and the whole Skills table of the output:

```text
| Memory files | 5k | 0.5% |
| Skills | 4.6k | 0.5% |
```

The Skills table has 26 rows: 17 with source `Built-in` and 9 with source `claude.ai sync`, and
none with source `Project` (`sed -n '/### Skills/,$p' m0-context.md | grep -c '| Project |'`
printed `0`). The only `Project` row in the whole output is the memory file `AGENTS.md`. The
4.6k-token Skills total equals the empty-repository baseline in the spec's pre-spec checks.

Step 2, the first two sentences of the reply:

```text
The repository has a `create-issue` skill (`.github/skills/dev/planning/create-issue/SKILL.md`). Your `/create-issue` command didn't run in this session, and the skill needs a description of the issue to draft.
```

#### Conclusion

Reproduced. No repository skill is discovered, and `/create-issue` does not run as a command. The
model found the skill file by reading the repository instead, which is step 2 of the spec's
"bug in plain terms".

## Failures and Follow-up

None.
