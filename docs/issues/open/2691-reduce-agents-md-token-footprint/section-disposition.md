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
| 5  | Package Catalog                 | 1,692  | MOVE + COMMAND   | `docs/packages.md` (single catalog); `cargo metadata --no-deps` | 90    | Largest section; derivable from the manifests (P2). Merge crate names, publish status and tool tables into the existing catalog, reverse the three back-links. |
| 6  | Package Naming Conventions      | 183    | LINK             | `docs/packages.md#package-conventions`                         | 15    | Exact duplicate exists already.                                                                                                        |
| 7  | Key Configuration Files         | 480    | KEEP (trim) + MOVE | `run-linters` skill (verify in T6)                           | 120   | Keep the two non-obvious facts: `linter` has no config of its own, and `.gitignore` does not exclude files from linters. Move the file-by-tool table. |
| 8  | Build, Test, and Lint           | 607    | KEEP (trim)      | test commands to `docs/testing.md` (verify in T6)              | 200   | Keep `linter all`, the mandatory pre-commit command, and the `TORRUST_GIT_HOOKS_LOG_DIR=.tmp` tip. The nine-step list is printed by the script itself (P2). |
| 9  | Code Style                      | 317    | KEEP (trim)      | n/a                                                            | 100   | Keep the import rule and the strict-rustflags note. rustfmt, clippy, taplo, yamllint, markdownlint and cspell are enforced by the linter, so the prose is redundant (P6). |
| 10 | Collaboration Principles        | 284    | KEEP             | n/a                                                            | 270   | Always-applicable behavioral rules, including the no-invented-process gate. High consequence if missed (P5).                           |
| 11 | Engineering Policies            | 600    | KEEP (trim) + MOVE | policies 1 and 2 to `update-dependencies` and `add-rust-dependency` skills (verify in T6) | 300 | Keep policy numbers, because issues cite "Engineering Policy 6/7". Shorten each to one line plus a link. Dependency and base-image freshness matter only for those tasks (P3). |
| 12 | Essential Rules                 | 436    | KEEP             | n/a                                                            | 380   | Absolute rules (GPG signing, linting gate, `storage/` and `target/`, security reporting). Drop rule 5, a duplicate of Code Style (P5). |
| 13 | Git Workflow                    | 1,067  | KEEP (trim) + MOVE | PR-title and merge-tool detail to `open-pull-request` and `merge-pull-request` skills | 450 | Keep branch naming, Conventional Commits, no direct pushes, fork-branch PRs and the "keep intermediate commits" rule (finding 4). Move the rest (P3). |
| 14 | Development Principles          | 287    | KEEP (trim)      | n/a                                                            | 100   | Generic principles an agent already knows (P2). Keep the four core principles in one line each; drop the quality-standards and Beck lists, or link them from `docs/`. |
| 15 | Container / Docker              | 206    | LINK             | `docs/containers.md`                                           | 10    | Ports and volumes already documented there.                                                                                            |
| 16 | Auto-Invoke Skills              | 119    | KEEP (trim)      | n/a                                                            | 80    | Keep where skills live and that they supplement the rules. Drop the VS Code setting and the spec link.                                 |
| 17 | Documentation                   | 356    | MOVE + KEEP      | `docs/index.md` (already holds the index)                      | 60    | The quick-navigation table duplicates `docs/index.md` (P7). Keep one pointer, the security reporting line, and the skills pointer.    |
|    | **Total (estimate)**            | 7,472  |                  |                                                                | **2,489** |                                                                                                                                    |

Estimated result: about 2.5k proxy tokens, or about 4.1k Claude Code tokens, a reduction of about
67% against the 12.4k baseline. The estimate is optimistic: if every "KEEP (trim)" row is
trimmed half as much as proposed, the total is about 3.0k proxy (about 5.0k Claude), a reduction
of about 60%. Both fit the roughly 200-line guidance in the Claude Code documentation.

## Prerequisites Before Moving (T6)

Each move is done in its own commit, and a row is not removed from `AGENTS.md` until its
destination is verified to contain the content.

| Source row | Verify before removing                                                                                       | Status |
| ---------- | ------------------------------------------------------------------------------------------------------------ | ------ |
| 5          | Catalog merged into `docs/packages.md`; links in `docs/packages.md` and `packages/AGENTS.md` (2) reversed     | TODO   |
| 6          | `docs/packages.md#package-conventions` matches the table (one difference: `*-protocol` wording)                | TODO   |
| 7          | Configuration-file table present in `run-linters` or another canonical doc                                    | TODO   |
| 8          | Build and test commands present in `docs/testing.md` or a skill                                               | TODO   |
| 11         | Policies 1 and 2 present in the dependency skills                                                             | TODO   |
| 13         | PR-title and merge-tool text present in the skills; "keep intermediate commits" rule placed (finding 4)        | TODO   |
| 15         | `docs/containers.md` covers ports and volumes (ports confirmed; volumes confirmed in "Volumes")               | DONE   |
| all        | `grep` for anchors into `AGENTS.md` finds no link to a removed heading                                        | TODO   |

## Open Questions for the Maintainer (T5)

1. **Target for AC2.** Choose between the estimate range above, for example "about 5k Claude
   tokens", and a stricter target. The estimates are not measured.
2. **Git Workflow depth.** Is the proposed trim acceptable, with procedures behind skills, given
   the risk that an agent does not load the skill? The alternative is to keep more inline.
3. **Catalog home.** `docs/packages.md` (proposed, single catalog, loaded only when linked) or
   `packages/AGENTS.md` (loaded on demand when working in `packages/`)?
4. **Development Principles.** Delete the quality-standards and Beck lists outright, or move them
   to a `docs/` page?
5. **Row 11.** Moving policies 1 and 2 into dependency skills changes where those policies are
   authoritative. Acceptable, or keep all eight policies inline?
