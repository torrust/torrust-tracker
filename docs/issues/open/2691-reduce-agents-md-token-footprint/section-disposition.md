---
semantic-links:
  skill-links:
    - create-issue
  related-artifacts:
    - docs/issues/open/2691-reduce-agents-md-token-footprint/ISSUE.md
    - docs/issues/open/2691-reduce-agents-md-token-footprint/research.md
    - AGENTS.md
---

<!-- skill-link: create-issue -->

# Section Disposition: Root AGENTS.md

Task T4 of issue #2691. This is a **proposal for maintainer review (T5)**. Nothing in `AGENTS.md`
has been changed. Practice identifiers (P1 to P8) refer to the adopted practices in
[`research.md`](research.md#practices-adopted-for-this-issue).

Token columns use the proxy `cl100k_base` count from the baseline in `research.md`. Claude Code
counts about 1.65 times more (12.4k against 7.5k in total), so multiply by 1.65 for an estimate of
`/context` tokens. The "After" column is an **estimate** made before any edit, not a measurement.

## Verdict Vocabulary

| Verdict  | Meaning                                                                                |
| -------- | -------------------------------------------------------------------------------------- |
| KEEP     | Stays inline, possibly trimmed                                                         |
| MOVE     | Content moves to a canonical home; `AGENTS.md` keeps at most a one-line pointer        |
| LINK     | Already lives elsewhere; replace with a link                                           |
| COMMAND  | Replace with a command that produces the data on demand                                |
| DELETE   | Derivable from the repository, enforced by tooling, or generic knowledge               |

## Findings That Shape the Plan

1. **Nested `AGENTS.md` files already exist** (`src/`, `docs/`, `packages/`, `tests/`,
   `.github/workflows/`, `docs/issues/open/` and others). Claude Code loads them on demand when a
   file in that directory is opened, so the repository already uses progressive disclosure, and
   this issue needs no new mechanism.
2. **The Package Catalog is canonical in the wrong place.** `docs/packages.md` (section "Package
   Catalog") and `packages/AGENTS.md` (two places) point back to the root `AGENTS.md` anchor
   `#-package-catalog` as the canonical source. Moving the catalog requires reversing those
   three links.
3. **Several sections are duplicates**: the package-prefix table already exists in
   `docs/packages.md` ("Package Conventions"); container ports and volumes already exist in
   `docs/containers.md`; the PR title convention already exists in the `open-pull-request` skill.
4. **A rule has no other home.** The "keep intermediate commits, never squash" rule (and its
   history-rewrite constraints) in Git Workflow is not in any skill. It must stay inline, or be
   moved into the `open-pull-request` skill as part of T6 before it is removed.
5. **Rule duplication inside the file**: the import rule appears in both Code Style and Essential
   Rules; the linter tool list appears in Tech Stack, Key Configuration Files and Code Style.

## Dispositions

| #  | Section                         | Tokens | Verdict          | Destination                                                    | After | Rationale                                                                                                                              |
| -- | ------------------------------- | ------ | ---------------- | -------------------------------------------------------------- | ----- | -------------------------------------------------------------------------------------------------------------------------------------- |
| 1  | Title                           | 34     | KEEP             | n/a                                                            | 34    | Needed.                                                                                                                                |
| 2  | Project Overview                | 251    | KEEP (trim)      | n/a                                                            | 130   | Keep repository link, one-line description, language/MSRV, license. Drop the framework, runtime and database bullets (P2).             |
| 3  | Tech Stack                      | 161    | DELETE           | n/a                                                            | 0     | Derivable from `Cargo.toml` and the code. The linter list is enforced by `linter all` and described in `run-linters` (P2).             |
| 4  | Key Directories                 | 392    | KEEP (trim)      | n/a                                                            | 150   | Keep only directories an agent cannot infer: git-ignored `storage/` and `.tmp/`, `docs/issues/`, `.github/skills/`, `contrib/dev-tools/`. |
| 5  | Package Catalog                 | 1,692  | MOVE + COMMAND   | `docs/packages.md` (single catalog); `packages/AGENTS.md` points to it; `cargo metadata --no-deps` | 90    | Largest section; derivable from the manifests (P2). Merge crate names, publish status and tool tables into the existing catalog, reverse the three back-links. Maintainer decision D3. |
| 6  | Package Naming Conventions      | 183    | LINK             | `docs/packages.md#package-conventions`                         | 15    | Exact duplicate exists already.                                                                                                        |
| 7  | Key Configuration Files         | 480    | KEEP (trim) + MOVE | `run-linters` skill (verify in T6)                           | 120   | Keep the two non-obvious facts: `linter` has no config of its own, and `.gitignore` does not exclude files from linters. Move the file-by-tool table. |
| 8  | Build, Test, and Lint           | 607    | KEEP (trim)      | test commands to `docs/testing.md` (verify in T6)              | 200   | Keep `linter all`, the mandatory pre-commit command, and the `TORRUST_GIT_HOOKS_LOG_DIR=.tmp` tip. The nine-step list is printed by the script itself (P2). |
| 9  | Code Style                      | 317    | KEEP (trim)      | n/a                                                            | 100   | Keep the import rule and the strict-rustflags note. rustfmt, clippy, taplo, yamllint, markdownlint and cspell are enforced by the linter, so the prose is redundant (P6). |
| 10 | Collaboration Principles        | 284    | KEEP             | n/a                                                            | 270   | Always-applicable behavioral rules, including the no-invented-process gate. High consequence if missed (P5).                           |
| 11 | Engineering Policies            | 600    | KEEP (trim)      | n/a                                                            | 380   | All eight policies stay inline (decision D5), shortened to one line plus a link each. Policy numbers stay, because issues cite "Engineering Policy 6/7". |
| 12 | Essential Rules                 | 436    | KEEP             | n/a                                                            | 380   | Absolute rules (GPG signing, linting gate, `storage/` and `target/`, security reporting). Drop rule 5, a duplicate of Code Style (P5). |
| 13 | Git Workflow                    | 1,067  | KEEP (trim) + MOVE | Procedure and detail used only while running a skill go to that skill (`open-pull-request`, `merge-pull-request`, `create-feature-branch`, `commit-changes`) | 450 | Keep branch naming, Conventional Commits, no direct pushes, fork-branch PRs and the "keep intermediate commits" rule (finding 4). Apply decision D2: skill-only rules to skills (P3). |
| 14 | Development Principles          | 287    | KEEP             | n/a                                                            | 287   | Maintainer decision D4: leave as is. |
| 15 | Container / Docker              | 206    | LINK             | `docs/containers.md`                                           | 10    | Ports and volumes already documented there.                                                                                            |
| 16 | Auto-Invoke Skills              | 119    | KEEP (trim)      | n/a                                                            | 80    | Keep where skills live and that they supplement the rules. Drop the VS Code setting and the spec link.                                 |
| 17 | Documentation                   | 356    | MOVE + KEEP      | `docs/index.md` (already holds the index)                      | 60    | The quick-navigation table duplicates `docs/index.md` (P7). Keep one pointer, the security reporting line, and the skills pointer.    |
|    | **Total (estimate)**            | 7,472  |                  |                                                                | **2,756** |                                                                                                                                    |

Estimated result after the maintainer decisions below: about 2.8k proxy tokens, or about 4.5k
Claude Code tokens, a reduction of about 63% against the 12.4k baseline. If every "KEEP (trim)" row
is trimmed half as much as proposed, the total is about 3.3k proxy (about 5.4k Claude, about 56%).
The agreed target (D1) is **about 5k Claude Code tokens or fewer**, measured with `/context`.

## Prerequisites Before Moving (T6)

Each move is done in its own commit, and a row is not removed from `AGENTS.md` until its
destination is verified to contain the content.

| Source row | Verify before removing                                                                                       | Status |
| ---------- | ------------------------------------------------------------------------------------------------------------ | ------ |
| 5          | Catalog merged into `docs/packages.md`; links in `docs/packages.md` and `packages/AGENTS.md` (2) reversed     | DONE   |
| 6          | `docs/packages.md#package-conventions` matches the table (one difference: `*-protocol` wording)                | DONE   |
| 7          | Configuration-file table present in `run-linters` or another canonical doc                                    | DONE   |
| 8          | Build and test commands present in `docs/testing.md` or a skill                                               | DONE   |
| 11         | Policies 1 and 2 present in the dependency skills                                                             | DONE   |
| 13         | PR-title and merge-tool text present in the skills; "keep intermediate commits" rule placed (finding 4)        | DONE   |
| 15         | `docs/containers.md` covers ports and volumes (ports confirmed; volumes confirmed in "Volumes")               | DONE   |
| all        | `grep` for anchors into `AGENTS.md` finds no link to a removed heading                                        | TODO   |

## Stale Baseline Content Found

Moving content exposed statements in the baseline `AGENTS.md` that were already wrong. They are
not carried forward.

| Baseline statement                                      | Reality                                                                    |
| ------------------------------------------------------- | -------------------------------------------------------------------------- |
| `cargo test --test integration` is a common command     | `Cargo.toml` has no `integration` test target; `tests/` has separate binaries |
| `.cspell.gitignore` is the cspell ignore file           | The file does not exist; cspell uses `ignorePaths` in `cspell.json`        |
| Cargo aliases are `cov`, `cov-lcov`, `cov-html`, `time` | `.cargo/config.toml` defines five; `cov-codecov` was missing               |
| `tests/` holds `integration.rs` and `servers/`          | Neither exists; `tests/` has separate binaries described in `tests/AGENTS.md` |

## Maintainer Decisions (T5)

Recorded 2026-10-30 from the maintainer's answers to the five questions raised in T4.

| ID | Decision                                                                                                                                                                                         |
| -- | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------ |
| D1 | Accept the conservative target: about 5k Claude Code tokens or fewer in `/context` (AC2).                                                                                                          |
| D2 | **Placement rule.** Rules that apply only while an agent runs a skill move into that skill. Rules that are not global move to the `AGENTS.md` in the subfolder whose context is closest to where they are used. Only global rules stay in the root file. |
| D3 | The package catalog lives in `docs/packages.md`; `packages/AGENTS.md` points to it.                                                                                                              |
| D4 | Keep Development Principles unchanged. The maintainer considers it very important.                                                                                                               |
| D5 | Keep all eight Engineering Policies inline, as agreed with the recommendation to move only duplicates. Policies 1 and 2 (dependency and base-image freshness) fit D2 and can be reconsidered later. |
| D6 | Approved on 2026-11-03, after the maintainer asked whether the trims were documented and approved: shorten the eight Engineering Policies to one or two lines each (numbers and substance kept), and apply the remaining trims of rows 2, 3, 4, 9, 11, 12, 16 and 17. The maintainer's words: "I approve, they look reasonable". |
| D7 | Decided on 2026-11-04: do not add a regrowth guard (neither a documented budget nor an automated check) now, because it may change in the future. T8 is closed without implementation and AC6 is waived. |
| D8 | Decided on 2026-11-04: AC4 is met. The maintainer first suggested that procedural guidance should live only in skills, with `AGENTS.md` pointing to them. After the agent noted the small saving (about 0.35k Claude tokens) and the adherence risk, the maintainer withdrew it: the branch, commit and pre-commit rules concern very common tasks, so they stay inline and the token cost is low. |
| D9 | Decided on 2026-11-04: no ADR is needed for this work (task T10). The placement rule (D2) remains recorded here. |

Applying D2 changes the destination of some rows: where this document says "move", prefer an
existing nested `AGENTS.md` in the closest subfolder (for example `tests/AGENTS.md` for test
commands, `.github/workflows/AGENTS.md` for CI guidance, `docs/AGENTS.md` for documentation
navigation) over a new page, unless a skill owns the workflow.

## Result After the Trims

Measured on 2026-11-03 with the proxy tokenizer `cl100k_base`; the Claude figure applies the 1.65
ratio from the baseline and is an estimate until `/context` is re-run.

| Measure                    | Baseline | Estimated in T4 | Actual   |
| -------------------------- | -------- | --------------- | -------- |
| Proxy tokens               | 7,472    | 2,756           | 2,994    |
| Claude Code tokens (est.)  | 12,400   | about 4,500     | about 4,940 |
| Lines                      | 523      | n/a             | 229      |
| Bytes                      | 35,910   | n/a             | 13,027   |

The actual result is above the T4 estimate because Engineering Policies (525 against an estimated
380) and Essential Rules (398 against 380) were trimmed less than planned: their wording is
maintainer-owned and the absolute rules stay verbatim. The skill lists were shortened to names plus
one location pattern to compensate.
