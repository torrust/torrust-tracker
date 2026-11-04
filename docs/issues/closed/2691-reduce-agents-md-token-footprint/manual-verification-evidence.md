---
doc-type: manual-verification-evidence
issue-spec: docs/issues/closed/2691-reduce-agents-md-token-footprint/ISSUE.md
last-updated-utc: 2026-11-04 15:00
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
  skill pointers that replaced the removed text.
- Initial state: a second fresh Claude Code session (not resumed) in the working copy at branch tip
  `66403fc93`, model Sonnet 5.5 (as reported by the maintainer). The `AGENTS.md` loaded had 229 lines.
- Status: `DONE`

#### Steps Performed

1. The maintainer started a new session with `claude` in the working copy.
2. The maintainer asked, forbidding tools: "Without using any tool or reading any file yet: where
   would you find the list of this repository's workspace packages with their crate names, and which
   ports does the tracker container use by default? Tell me what you already know from your loaded
   instructions and where you would look for the rest."
3. The maintainer asked, in the same session: "Go ahead and look them up now. Tell me the crate name
   of the tracker-client package under packages/, and the default UDP, HTTP and API ports, and name
   each file you read to find them."
4. The agent's answers were pasted by the maintainer and are summarized below. The agent's claims
   were then checked against the repository: `docs/packages.md` line 264 is the row
   `tracker-client` (`packages/`) with crate `torrust-tracker-client-lib`; line 289 is the console
   tool row with crate `torrust-tracker-client`; `docs/containers.md` lines 220 to 223 give the four
   defaults and lines 256 to 258 the publish examples.

#### Observed Result

Step 2 (no tools allowed), summarized from the agent's answer:

- It said the loaded instructions do not list the packages or ports, and named the correct pointers:
  `docs/packages.md#package-catalog`, `cargo metadata --no-deps --format-version 1` as the
  authoritative list, `docs/packages.md#package-conventions`, and `docs/containers.md` for the ports.
- It offered a recollection of UDP 6969, HTTP 7070, API 1212 and health check 1313, explicitly
  labelled as general knowledge and unverified.

Step 3 (tools allowed), summarized from the agent's answer:

```text
Crate name of the packages/ tracker-client: torrust-tracker-client-lib (docs/packages.md:264).
A second tracker-client row at line 289 is the console tools, crate torrust-tracker-client.
Default ports (docs/containers.md, lines 220-223 and 253-258):
UDP 6969, HTTP 7070, API 1212, Health Check API 1313.
Files read: docs/packages.md, docs/containers.md (grepped for matching lines, not read in full).
```

#### Conclusion

The expected result was met. With the removed text gone, the agent found the right canonical sources
from the loaded pointers alone, retrieved the correct crate name and all four ports from them, and
told apart two packages with the same name. Its earlier unverified recollection was also correct, and
it labelled it as unverified. The health check port 1313 is in `docs/containers.md` but was not in
the removed `AGENTS.md` text, so nothing was lost. Limits: one run, one model, and the agent grepped
the two files instead of reading them in full.

### V4 - Rule inventory is fully asserted (scenario M4)

- Goal: confirm that every rule in the baseline file has a final location or a recorded reason.
- Initial state: `rule-inventory.md` with 99 rows, `AGENTS.md` at 229 lines.
- Status: `DONE`

#### Steps Performed

1. Parsed every row of `rule-inventory.md` and counted statuses, `DELETE` rows without a recorded
   reason, and rows with an empty final location.
2. For the 61 rows whose final home is `AGENTS.md`, searched the current file for the baseline
   phrase (whitespace folded).
3. Read the current text for every row whose phrase was not found.

#### Observed Result

```text
rows 99 | ASSERTED 99 | PENDING 0
DELETE rows without reason: []
rows with empty final location: []
AGENTS.md rows 61 baseline phrase absent: ['R008', 'R031', 'R039', 'R043', 'R064', 'R081', 'R082', 'R099']
```

The eight rows where the phrase is absent were read against the current file:

| Row  | Why the phrase is absent                                                                              | Rule still present |
| ---- | ----------------------------------------------------------------------------------------------------- | ------------------ |
| R008 | Reworded: the torrust-linting clause in Build, Test, and Lint                                          | yes                |
| R031 | The baseline sentence "policy-level summary" was dropped; the section points to the skills             | yes (pointer)      |
| R039 | Reworded: "Skills for these workflows (each is `.github/skills/dev/<area>/<name>/SKILL.md`)"           | yes                |
| R043 | Reworded: "pass clippy without warnings" in the compile-flags bullet                                   | yes                |
| R064 | Reworded: "Implementation workflow skills (same location)"                                             | yes                |
| R081 | Split: "PRs target develop" and "PRs must pass all CI" stay; the release pipeline is in `docs/release_process.md`, linked from the file | yes |
| R082 | Letter case only: "keep intermediate commits" now follows the bold lead-in "Commit history"           | yes                |
| R099 | Reworded: custom agents are described in `.github/agents/README.md`                                   | yes                |

#### Conclusion

The expected result was met: no row is pending, every deletion has a reason, every final location is
filled in, and 53 of the 61 rows kept in `AGENTS.md` still contain the baseline phrase verbatim, with
the other eight confirmed by reading. The check is repeatable by running the same count against
`rule-inventory.md`.

## Failures and Follow-up

- **Discarded reading, 2026-11-03.** A first `/context` run after the trims showed
  `Memory files: 12.4k tokens (1 file)`, equal to the baseline. It did not measure the trimmed file:
  the session loaded the old `AGENTS.md`, either because it ran in another checkout or because its
  instruction files were read before the branch was checked out. The cause was not isolated. The
  reading was discarded, and the measurement above was taken after the session's instruction files
  were re-read and the loaded file was confirmed to be the trimmed one.
