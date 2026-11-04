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
- Initial state: a fresh Claude Code session (not resumed) in the working copy at branch tip
  `66403fc93` (`wc -l AGENTS.md` printed 229), no history, model Sonnet 5.5 (as reported by the
  maintainer).
- Status: `DONE`

#### Steps Performed

1. The maintainer ran `git rev-parse --short HEAD && wc -l AGENTS.md` in the working copy and got
   `66403fc93` and `229 AGENTS.md`.
2. The maintainer started a fresh session with `claude` in that directory and reported the model:
   Sonnet 5.5.
3. The maintainer asked: "From your project instructions only, without reading any other file: what
   are the rules for making a commit in this repository (signing, message format), how should I name
   a branch for issue 123 about peer expiry, and what must pass before every commit?"
4. The maintainer pasted the answer, quoted below unchanged.

#### Observed Result

```text
Commit rules
- Signing: every commit must be GPG-signed (git commit -S). If the passphrase prompt times out, stop, tell you, and ask whether you want to retry manually or have me rerun the same signed command while you type the passphrase in the terminal. I never retry automatically, handle the passphrase in chat, or use --no-gpg-sign.
- Message format: Conventional Commits, type(scope): summary.
  - Types: feat, fix, chore, docs, refactor, ci, test.
  - Scope is the affected package or area, such as tracker-core, udp-protocol, ci or docs.
- History: keep intermediate commits. Don't squash, fixup, or rewrite history.
- Never commit storage/ or target/.

Branch name for issue 123 about peer expiry
The preferred pattern is <issue-number>-<short-description>, so 123-peer-expiry, or 123-fix-peer-expiry if it's a bug fix. Branches are cut from develop, and PRs target develop from a fork branch. If it's a spec-only branch, add a -spec suffix: 123-peer-expiry-spec.

What must pass before every commit
- linter all must exit 0, with no exceptions.
- ./contrib/dev-tools/git/hooks/pre-commit.sh is the mandatory quality gate. In restricted sandboxes, set TORRUST_GIT_HOOKS_LOG_DIR=.tmp so the logs stay in the workspace.
- Run cargo machete and remove any unused dependencies.
- Code must compile and pass clippy cleanly under the strict rustflags.
- Self-review is strongly recommended before each commit.

One more thing: new technical terms go in project-words.txt, or cspell will fail the linter.
```

Comparison with the baseline rules (inventory rows in brackets):

| Expected rule                                                              | In the answer | Source in the final file        |
| -------------------------------------------------------------------------- | ------------- | ------------------------------- |
| Commits signed with `git commit -S` (R066)                                 | yes           | Essential Rules 2               |
| GPG timeout handling: stop, ask, never retry, never `--no-gpg-sign` (R067) | yes, in full  | Essential Rules 2               |
| Conventional Commits, `type(scope): summary`, types and scope (R076)      | yes           | Git Workflow                    |
| Branch pattern `<issue-number>-<short-description>` (R074)                | yes           | Git Workflow                    |
| `-spec` suffix for a spec-only branch (R075)                              | yes           | Git Workflow                    |
| Cut from `develop`, PR target `develop`, from a fork branch (R077, R079, R081)| yes           | Git Workflow, Branch strategy   |
| Keep intermediate commits, no squash (R082)                               | yes           | Git Workflow, Commit history    |
| Never commit `storage/` or `target/` (R068)                               | yes           | Essential Rules 3               |
| `linter all` exits 0, no exceptions (R065)                                | yes           | Essential Rules 1               |
| `pre-commit.sh` is the mandatory gate (R033)                              | yes           | Build, Test, and Lint           |
| `TORRUST_GIT_HOOKS_LOG_DIR=.tmp` in sandboxes (R036)                      | yes           | Build, Test, and Lint           |
| `cargo machete`, remove unused dependencies (R069)                        | yes           | Essential Rules 4               |
| Strict rustflags and clippy (R042, R043)                                  | yes           | Code Style                      |
| Self-review before each commit (R071)                                     | yes           | Essential Rules 5               |
| New terms in `project-words.txt` (R048)                                   | yes           | Code Style                      |

#### Conclusion

The expected result was met: every rule asked about was stated correctly, including the absolute
GPG timeout rule with all four prohibitions, and the agent also recalled rules it was not asked
about (`project-words.txt`, clippy, `cargo machete`). One imprecision: the answer says "Don't squash,
fixup, or rewrite history", while the file says not to do so "to hide steps that were later
superseded". The file's own wording is the same as the baseline's; the agent shortened it. The
answer does not mention that PR titles and issue-linking rules are in a skill; that was not asked.
This is a single run with one model, so it shows that the rules survive in the loaded text, not that
every agent will always follow them.

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
