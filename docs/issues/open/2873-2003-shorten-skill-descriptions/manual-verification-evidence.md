---
doc-type: manual-verification-evidence
issue-spec: docs/issues/open/2873-2003-shorten-skill-descriptions/ISSUE.md
last-updated-utc: 2026-11-16 08:47
---

# Manual Verification Evidence

<!-- cspell:ignore splitlines startswith -->

## Purpose

Record real, human-oriented verification for issue #2873. At the spec stage it holds only the
reproduction of the cut `write-markdown-docs` description (V0), as the `fix-bug` skill asks before
review. Sections V1 to V4 are added during implementation, one per manual scenario in the spec.

## Environment and Prerequisites

- Date and time (UTC): 2026-11-16, steps stamped below
- Artifact under test: the skill files on `develop` `a35960402` (this spec PR changes no skill)
- Operating system / environment: Linux; Python 3.14.4 with PyYAML 6.0.3; Claude Code 2.1.318
- Prerequisites and setup performed: none beyond a checkout of the commit

## Verification Processes

### V0 - Reproduce the cut `write-markdown-docs` description

- Goal: show that a YAML parser, and Claude Code, read only part of the description.
- Initial state: `.github/skills/dev/planning/write-markdown-docs/SKILL.md` as on `develop`
  `a35960402`; its `description` is a plain YAML scalar containing a space followed by `#NUMBER`.
- Status: `DONE`
- Classification: **Reproduced** (the wrong outcome, the cut value, was observed in the parser and
  in Claude Code's `/context`).

#### Steps Performed

1. 08:21 UTC. In a scratch repository holding the 53 `SKILL.md` files flat under
   `.claude/skills/<name>/` (copied from `.github/skills/` at `a35960402`), ran
   `claude -p "/context"` with Claude Code 2.1.318.
2. 08:45:16 UTC. In the checkout at `a35960402`, parsed the frontmatter with PyYAML and compared
   the raw text after `description:` with the parsed value:

   ```python
   import yaml
   p='.github/skills/dev/planning/write-markdown-docs/SKILL.md'
   fm=open(p).read().split('---')[1]
   raw=[l for l in fm.splitlines() if l.startswith('description:')][0][len('description: '):]
   d=yaml.safe_load(fm)['description']
   print('raw', len(raw)); print('parsed', len(d)); print('parsed text:', d)
   ```

#### Observed Output

Step 1, the lines of the `/context` output that bear on this issue (model `claude-sonnet-5-5`,
`27.8k / 1m (3%)` in total):

```text
| Skills | 9.9k | 1.0% |
| write-markdown-docs | Project | ~50 |
```

Step 2:

```text
raw 578
parsed 123
parsed text: Guide for writing Markdown documentation in this project. Covers GitHub Flavored Markdown pitfalls, especially the critical
```

#### Result

The parser keeps 123 of the 578 characters and stops before `#NUMBER pattern`: a space
followed by `#` starts a YAML comment. The `/context` row of about 50 tokens matches 123 characters plus the name at the measured
rate of about 3 characters per token; the whole 578 characters would cost about 200. The text
Claude Code lists has no `Use when` clause and none of the trigger phrases after it.
