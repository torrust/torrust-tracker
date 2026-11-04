---
doc-type: manual-verification-evidence
issue-spec: docs/issues/open/2691-reduce-agents-md-token-footprint/ISSUE.md
last-updated-utc: 2026-11-03 16:30
---

# Manual Verification Evidence

## Purpose

Record real, human-oriented verification of the completed behavior for issue #2691. The maintainer
ran the `/context` measurements; the agent recorded the output as pasted into the session.

## Environment and Prerequisites

- Date and time (UTC): baseline 2026-10-30; final 2026-11-03
- Artifact under test: branch `2691-reduce-agents-md-token-footprint`, tip commit
  `docs(agents): [#2691] condense the remaining AGENTS.md sections`
- Operating system / environment: Linux, Claude Code 2.1.316, model `claude-sonnet-5-5`
- Prerequisites and setup performed: the working copy has no `CLAUDE.md` or `CLAUDE.local.md`, so
  Claude Code reads `AGENTS.md` directly (see `research.md`)

## Verification Processes

### V1 - Token footprint before and after (scenario M1)

- Goal: confirm that the loaded size of the root `AGENTS.md` fell below the agreed target of about
  5k Claude Code tokens (decision D1), measured with Claude Code's own `/context`.
- Initial state: `AGENTS.md` at the baseline commit `Merge torrust/torrust-tracker#2692: docs(issues):
  [#2691] add specification to reduce the AGENTS.md token footprint` (523 lines, 35,910 bytes).
- Status: `DONE`

#### Steps Performed

1. Baseline, 2026-10-30: the maintainer ran `/context all` in the working session and pasted the
   output (the Memory files section is quoted below).
2. Trimmed the file in tasks T6 and T7 (commits on the branch, see the pull request).
3. Final, 2026-11-03: the maintainer ran `/context` in the working session, after the session's
   instruction files had been re-read with the branch tip checked out, and pasted the output
   (quoted below). The instruction file the agent received in that re-read was the trimmed one
   (229 lines), which matches the working tree (`wc -l AGENTS.md` printed 229).
4. For cross-checking, the agent measured the file with the proxy tokenizer on the same commit:
   `cl100k_base` gave 2,994 tokens for 229 lines and 13,027 bytes (baseline: 7,472 tokens).

#### Observed Result

Baseline (`/context all`, 2026-10-30):

```text
Memory files · /memory
├ ~/.claude/projects/-home-josecelano-...-agent-claude-01/memory/MEMORY.md: 59 tokens
└ AGENTS.md: 12.4k tokens
```

Final (`/context`, 2026-11-03):

```text
Memory files · /memory
└ 2 files · 4.8k tokens
Memory Files table:
| AutoMem | .../memory/MEMORY.md | 59 |
| Project | .../torrust-tracker-agent-claude-01/AGENTS.md | 4.8k |
```

| Measure                       | Baseline | Final   | Change          |
| ----------------------------- | -------- | ------- | --------------- |
| `AGENTS.md`, Claude Code      | 12.4k    | 4.8k    | -7.6k tokens, -61% |
| `AGENTS.md`, proxy tokenizer  | 7,472    | 2,994   | -4,478 tokens, -60% |
| Ratio, Claude to proxy        | 1.66     | 1.60    | n/a             |
| Lines                         | 523      | 229     | -294            |
| Bytes                         | 35,910   | 13,027  | -22,883         |

#### Conclusion

The expected result was met. `AGENTS.md` costs 4.8k Claude Code tokens against the 12.4k baseline,
which is under the agreed target of about 5k (decision D1). The estimate made in T7 (about 4.9k) was
within 3% of the measurement.

### V2 - Key rules survive in a fresh session (scenario M2)

- Goal: confirm that an agent working from the trimmed file states the commit, signing, branch and
  linting rules correctly.
- Initial state: fresh Claude Code session on the branch tip.
- Status: `TODO`

Pending: to be run by the maintainer in a fresh session, recording the agent and model used.

### V3 - Moved content is discoverable (scenario M3)

- Goal: confirm that an agent finds the package list and the container ports through the links and
  skill pointers.
- Initial state: fresh Claude Code session on the branch tip.
- Status: `TODO`

Pending: to be run by the maintainer in a fresh session, recording the agent and model used.

### V4 - Rule inventory is fully asserted (scenario M4)

- Goal: confirm that every rule in the baseline file has a final location or a recorded reason.
- Initial state: `rule-inventory.md` with 99 rows.
- Status: `TODO`

Pending: final walk-through after the last edits to `AGENTS.md`. At the time of writing every row
is `ASSERTED`, and 45 of the rows whose final home is `AGENTS.md` were checked mechanically for the
baseline phrase.

## Failures and Follow-up

- **Discarded reading, 2026-11-03.** A first `/context` run after the trims showed
  `Memory files: 12.4k tokens (1 file)`, equal to the baseline. It did not measure the trimmed file:
  the session loaded the old `AGENTS.md`, either because it ran in another checkout or because its
  instruction files were read before the branch was checked out. The cause was not isolated. The
  reading was discarded, and the measurement above was taken after the session's instruction files
  were re-read and the loaded file was confirmed to be the trimmed one.
