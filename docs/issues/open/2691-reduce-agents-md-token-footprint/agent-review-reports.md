---
semantic-links:
  related-artifacts:
    - .github/agents/complexity-auditor.agent.md
    - .github/agents/task-reviewer.agent.md
    - .github/agents/pr-reviewer.agent.md
    - docs/agents/orchestration.md
    - docs/issues/open/2691-reduce-agents-md-token-footprint/ISSUE.md
---

# Agent Review Reports - Research AGENTS.md Best Practices and Reduce Its Token Footprint

> Append one completed independent-review entry at a time. Do not modify, reorder, or remove
> earlier entries. A correction is a new entry that names the earlier conclusion.

## Reports

### 2026-11-04 11:10 UTC - Task Reviewer (Claude Code, model Claude Sonnet 5.5)

- Invocation scope: pre-PR review of issue #2691, branch `2691-reduce-agents-md-token-footprint`,
  diff `c86b1c24e..HEAD` (root `AGENTS.md`, `docs/packages.md`, `packages/AGENTS.md`, the
  `open-pull-request`, `run-linters` and `setup-dev-environment` skills, `project-words.txt`, the
  issue folder). Acceptance criteria AC1 to AC7 and the generic criteria.
- Inputs: `ISSUE.md`, `rule-inventory.md`, `manual-verification-evidence.md`,
  `implementation-retrospective.md`, the baseline `AGENTS.md` (`git show c86b1c24e:AGENTS.md`), and
  the final files. The baseline rule list was built from the baseline before reading the inventory.
- Evidence:
  - `linter all` exited 0 (markdown, lychee, yaml, toml, cspell, clippy, rustfmt, shellcheck).
  - Baseline is 523 lines, 35,910 bytes, MD5 `f3516c78...`; final `AGENTS.md` is 229 lines, 13,027
    bytes, unchanged since commit `condense the remaining AGENTS.md sections`; both match the evidence.
  - `cargo metadata --no-deps --format-version 1` lists 34 packages; `docs/packages.md` lists the
    root crate, 23 `packages/` rows, 1 console tool, 8 developer tools and 1 skill-owned tool (34),
    with matching crate names. No package missing, extra or misnamed.
  - Spot-checked 40 inventory rows (all 17 `DELETE` rows, R017 to R024, R032 to R040, R075 to R086,
    R091 to R095) against the final files. Rules present, except the R030 note below.
  - `rg`/`grep` for `AGENTS.md#`, `Package Catalog`, `Language` bullet, `Quick Navigation` outside
    closed specs. Cited line numbers in V3 and in R040 and R091 were opened and are accurate.
  - The three skill diffs were read; anchors (`#keeping-the-branch-up-to-date`,
    `#commit-history-and-pull-request-scope`) resolve and `cspell.json` `ignorePaths` exists.
- Findings:
  - MEDIUM: `ISSUE.md` Implementation Plan still shows T6 as `TODO`, while the progress log, the
    Commit Points and the inventory show the moves done. Remediation: set T6 to `DONE` with a note.
  - MEDIUM: live references to removed `AGENTS.md` content in open specs. `docs/issues/open/2183-adopt-calendar-msrv-policy/ISSUE.md`
    (T4, AC6) tells the implementer to edit the "Language bullet" and the nested "MSRV policy" note,
    which no longer exist (MSRV is now in Project Overview). `docs/issues/open/1669-overhaul-packages/EPIC.md`
    lines 527 and 1107 require the "`AGENTS.md` Package Catalog" to be kept accurate; the catalog now
    lives in `docs/packages.md`. Remediation: update those lines (the issue's scope includes fixing
    cross-references). Also `deny.toml` line 11 says `packages/AGENTS.md` holds the package catalog;
    point it at `docs/packages.md#package-catalog` (LOW).
  - LOW: rule-inventory R030 reason says E2E compose usage is documented in
    `packages/e2e-tools/README.md` and `docs/containers.md`. Neither mentions the
    `compose.qbittorrent-e2e.*.yaml` files; `docs/testing.md` does. Remediation: correct the cited
    location.
  - LOW: AC4 acceptance (D8) is recorded and unambiguous, but the stated residue ("four short global
    rules") understates the overlap: the keep-intermediate-commits and ride-along rules also appear in
    both `AGENTS.md` and the `open-pull-request` skill. Remediation: name them in D8, or accept
    explicitly.
  - LOW: the retrospective says decisions "D1 to D7" in "What Went Well"; D8 and D9 exist. Also the
    `run-linters` skill changed without a version bump (the other two skills bumped). Remediation:
    cosmetic.
  - LOW (information): `docs/containers.md` has `docker compose up --build`, not the baseline
    `up -d`/`logs -f`/`down` trio; the commands are not lost in substance.
  - No lost rule found: every baseline rule and fact has a final location or a recorded reason.
- Verdict: REVIEW FAILED
- Follow-up actions:
  - Implementer: fix the T6 status, the 2183 and 1669 stale references and the `deny.toml` comment,
    correct R030, then request a re-review (or a documented acceptance of the low findings).
  - Reviewer: tick "Acceptance criteria are re-reviewed" and "Reviewer validated" after the re-review.

### 2026-11-04 11:20 UTC - Task Reviewer, re-review (Claude Code, model Claude Sonnet 5.5)

- Invocation scope: re-review of commit `60d26d33f` against findings 1 to 6 of the earlier entry.
- Evidence: `git show 60d26d33f`; `linter all` exited 0 with `TORRUST_GIT_HOOKS_LOG_DIR=.tmp`; repo-wide
  grep (excluding `docs/issues/closed`) for the removed `AGENTS.md` sections.
- Findings 1 to 4 and 6: resolved. T6 is `DONE`; issue #2183 T4 matches the current `AGENTS.md`
  (MSRV figure in Project Overview, separate MSRV policy paragraph) and AC6 is still accurate; EPIC
  #1669 lines 527 and 1107 and `deny.toml` point at `docs/packages.md`; D8 names the overlap;
  "D1 to D9" and the `run-linters` 1.2 bump are in. Remaining hits (`docs/discussions/...`,
  `docs/pr-reviews/pr-2187-review/PR-REVIEW.md`, EPIC line 1080) are dated historical records and
  are acceptable.
- Finding 5 (R030): the earlier claim about `docs/testing.md` was wrong (it only links the
  `qbittorrent_e2e_runner` binary). However the new reason "no document names them" is also
  inaccurate: `contrib/dev-tools/debugging/qbt/README.md` line 48 names
  `compose.qbittorrent-e2e.sqlite3.yaml`. LOW. Remediation: reword R030 to "no user-facing
  testing document lists them (only the qbt debugging README mentions one)", or drop the claim.
- Verdict: REVIEW FAILED (one LOW inaccuracy left; nothing else blocks). The re-review checkboxes
  in `ISSUE.md` are not ticked.

### Re-review follow-up

The one remaining LOW finding (R030 reason) was corrected in `rule-inventory.md` to cite the
`contrib/dev-tools/debugging/qbt/README.md` mention. All other findings were verified fixed by the
re-review; no further review pass was run for this one-line wording change.
