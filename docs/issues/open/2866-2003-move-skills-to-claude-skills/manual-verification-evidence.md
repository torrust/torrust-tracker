---
doc-type: manual-verification-evidence
issue-spec: docs/issues/open/2866-2003-move-skills-to-claude-skills/ISSUE.md
last-updated-utc: 2026-11-15 21:35
---

<!-- cspell:ignore Autocompact dataviz -->

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

Step 1, the category table, the Memory Files table and the whole Skills table of the output,
verbatim; the long scratch path of the clone in the Memory Files row is shortened to `<clone>`:

```text
### Estimated usage by category

| Category | Tokens | Percentage |
|----------|--------|------------|
| System prompt | 2.6k | 0.3% |
| System tools | 18.4k | 1.8% |
| MCP tools | 559 | 0.1% |
| MCP server instructions | 1.6k | 0.2% |
| MCP tools (deferred) | 13.1k | 1.3% |
| System tools (deferred) | 22.7k | 2.3% |
| Memory files | 5k | 0.5% |
| Skills | 4.6k | 0.5% |
| Messages | 10 | 0.0% |
| Free space | 934.2k | 93.4% |
| Autocompact buffer | 33k | 3.3% |

### Memory Files

| Type | Path | Tokens |
|------|------|--------|
| Project | <clone>/AGENTS.md | 5k |

### Skills

| Skill | Source | Tokens |
|-------|--------|--------|
| dataviz | Built-in | ~480 |
| artifact-design | Built-in | ~70 |
| artifact-diagramming | Built-in | ~70 |
| artifact-capabilities | Built-in | ~250 |
| update-config | Built-in | ~240 |
| keybindings-help | Built-in | ~80 |
| code-review | Built-in | ~280 |
| simplify | Built-in | ~60 |
| fewer-permission-prompts | Built-in | ~60 |
| loop | Built-in | ~120 |
| schedule | Built-in | ~130 |
| claude-api | Built-in | ~360 |
| workflow-authoring | Built-in | ~80 |
| run | Built-in | ~120 |
| plugin-authoring | Built-in | ~90 |
| init | Built-in | ~20 |
| security-review | Built-in | ~30 |
| docs | claude.ai sync | ~340 |
| docx | claude.ai sync | ~320 |
| google-workspace | claude.ai sync | ~330 |
| import-memory | claude.ai sync | ~60 |
| morning | claude.ai sync | ~120 |
| pdf | claude.ai sync | ~150 |
| pptx | claude.ai sync | ~330 |
| skill-creator | claude.ai sync | ~120 |
| xlsx | claude.ai sync | ~320 |
```

The Skills table has 26 rows: 17 with source `Built-in` and 9 with source `claude.ai sync`, and
none with source `Project` (`sed -n '/### Skills/,$p' m0-context.md | grep -c '| Project |'`
printed `0`). The only `Project` row in the whole output is the memory file `AGENTS.md`
(`grep -c 'Project' m0-context.md` printed `1`). The MCP Tools table between the category table
and the Memory Files table is left out: it lists connector tools only. The
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
