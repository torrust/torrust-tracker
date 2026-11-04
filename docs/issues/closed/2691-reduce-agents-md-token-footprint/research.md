---
semantic-links:
  skill-links:
    - create-issue
  related-artifacts:
    - docs/issues/closed/2691-reduce-agents-md-token-footprint/ISSUE.md
    - AGENTS.md
    - docs/adrs/20260821172000_establish_ai_agent_context_capability_and_portability_governance.md
---

<!-- skill-link: create-issue -->

# Research: AGENTS.md Best Practices and Token Baseline

Research for tasks T1 to T3 of issue #2691. Research date: 2026-10-30. Primary sources are
distinguished from secondary ones, and claims that could not be traced to a primary source are
marked unverified.

## Baseline Token Footprint (T1)

Measured on `AGENTS.md` at the `develop` commit that contains the merged specification
(`Merge torrust/torrust-tracker#2692`, the spec-only PR for this issue).

Claude's own tokenizer is not available offline, so two public OpenAI tokenizers are used as
proxies. They are comparable to each other and good for before/after ratios, but their absolute
numbers are not Claude token counts.

| Section                          | Bytes  | Lines | Words | cl100k | o200k |
| -------------------------------- | ------ | ----- | ----- | ------ | ----- |
| **Total**                        | 35,910 | 523   | 3,740 | 7,472  | 7,594 |
| Package Catalog                  | 9,201  | 76    | 718   | 1,692  | 1,744 |
| Git Workflow                     | 4,503  | 76    | 613   | 1,067  | 1,069 |
| Key Configuration Files          | 3,460  | 28    | 229   | 480    | 476   |
| Engineering Policies             | 2,969  | 40    | 331   | 600    | 607   |
| Build, Test, and Lint            | 2,357  | 70    | 259   | 607    | 621   |
| Essential Rules                  | 1,943  | 27    | 269   | 436    | 446   |
| Documentation                    | 1,738  | 28    | 132   | 356    | 358   |
| Key Directories                  | 1,588  | 24    | 193   | 392    | 395   |
| Collaboration Principles         | 1,449  | 24    | 220   | 284    | 284   |
| Development Principles           | 1,321  | 27    | 180   | 287    | 297   |
| Package Naming Conventions       | 1,282  | 12    | 99    | 183    | 182   |
| Code Style                       | 1,166  | 18    | 149   | 317    | 324   |
| Project Overview                 | 948    | 19    | 121   | 251    | 259   |
| Container / Docker               | 774    | 26    | 91    | 206    | 206   |
| Tech Stack                       | 548    | 12    | 63    | 161    | 170   |
| Auto-Invoke Skills               | 527    | 12    | 64    | 119    | 120   |
| Title                            | 136    | 4     | 9     | 34     | 36    |

Method: Python `tiktoken` (encodings `cl100k_base` and `o200k_base`) applied to the whole file
and to each chunk produced by splitting at second-level Markdown headings, as in this sketch:

```python
import re, sys, tiktoken
text = open(sys.argv[1], encoding="utf-8").read()
enc = tiktoken.get_encoding("cl100k_base")
for part in re.split(r"(?m)^(?=## )", text):
    print(part.splitlines()[0], len(enc.encode(part)))
```

Observations:

- The Package Catalog alone is about 23% of the file, and its content is derivable from
  `cargo metadata --no-deps --format-version 1` and `docs/packages.md`.
- The five largest sections (Package Catalog, Git Workflow, Key Configuration Files, Engineering
  Policies, Build/Test/Lint) account for about 60% of the tokens.
- Claude Code's own report (`/context all`) is the authoritative number for Claude; it is recorded
  in the next subsection.

### Claude Code `/context` Measurement

Reported by the maintainer on 2026-10-30 with `/context all`, run in the working session for this
issue on the model `claude-sonnet-5-5` (not a fresh session: it already held about 13k tokens of
messages). The measured `AGENTS.md` is the same unmodified file as in the table above:

| Item                         | Tokens (Claude Code) |
| ---------------------------- | -------------------- |
| `AGENTS.md`                  | 12,400               |
| Memory files (total)         | 12,500               |
| System prompt                | 2,800                |
| System tools                 | 21,600               |
| Skills (names/descriptions)  | 4,800                |
| Total context at that moment | 56,300 of 1,000,000  |

- Claude counts `AGENTS.md` at about 1.65 times the proxy tokenizers (12.4k against 7.5k). The
  proxies are therefore usable for relative before/after comparison, but they understate the real
  cost, and any target in tokens must be stated in Claude Code `/context` units.
- `AGENTS.md` is about 22% of the whole session context at that point, and the single largest
  item after system tools. It is loaded before any task starts, which matches the maintainer's
  report.
- The 56.3k total includes about 13k tokens of messages from this session; the fixed cost of an
  empty session is therefore about 43k, of which `AGENTS.md` is about 29%.

## Sources Consulted (T2)

### Primary

1. **Claude Code documentation, "How Claude remembers your project"**
   (<https://code.claude.com/docs/en/memory>). Vendor documentation for the tool that raised the
   concern. Findings used:
   - Instruction files are loaded at the start of every session and are treated as context, not
     enforced configuration. To block an action regardless of what the model decides, use a hook.
   - "Target under 200 lines per CLAUDE.md file. Longer files consume more context and reduce
     adherence."
   - Keep to facts held in every session: build commands, conventions, project layout, "always do
     X" rules. A multi-step procedure, or something that matters for one part of the codebase,
     belongs in a skill or a path-scoped rule.
   - `@path` imports "help you organize a long file but don't reduce its context cost", because
     imported files load together with the importing file. Splitting the file via imports saves
     nothing.
   - Block-level HTML comments are stripped before the content is injected into context, so they
     cost no tokens. This is Claude Code behavior; other tools may not strip them.
   - `AGENTS.md` is read directly when there is no `CLAUDE.md` or `CLAUDE.local.md` in the working
     directory or above it (Claude Code v2.1.277 or later). A subdirectory `AGENTS.md` loads on
     demand when Claude opens a file in that subdirectory. Adding a `CLAUDE.local.md` makes
     Claude stop reading `AGENTS.md` unless the Project instructions setting is changed.
   - `/doctor prompt-audit` audits instruction files, including `AGENTS.md`.
2. **Gloaguen et al., "Evaluating AGENTS.md: Are Repository-Level Context Files Helpful for
   Coding Agents?"** (arXiv:2602.11988, ETH Zurich, first submitted 2026-02-12, latest revision
   2026-09-29; <https://arxiv.org/abs/2602.11988>). The only controlled study found. Findings
   used, from the abstract: context files do not generally improve task success rates and
   increase inference cost by over 20% on average; this holds across LLMs and coding agents and
   for both LLM-generated and developer-committed files; agents follow the instructions well, but
   repository overviews are not helpful; context files are useful for specifying non-standard
   coding practices. The abstract page gave no per-type numbers; the full paper was not read
   for this note.
3. **AGENTS.md format site** (<https://agents.md>). Plain Markdown, no required fields; typical
   sections are project overview, build and test commands, code style, testing instructions,
   security considerations, and commit or PR guidelines. No size limit is given. For large
   projects it recommends nested `AGENTS.md` files; the closest file to the edited one takes
   precedence, and explicit user prompts override everything. It lists native support in
   Codex, Cursor, Copilot coding agent, Gemini CLI, Windsurf, Zed, Aider and others.

### Secondary (practitioner write-ups, not independently verified)

- AIHero, "A complete guide to AGENTS.md" (<https://aihero.dev/a-complete-guide-to-agents-md>):
  every token is loaded on every request, so keep the file minimal and use progressive disclosure
  (a short root file linking to separate docs, plus nested files); warns against `init`-generated
  boilerplate and stale file paths.
- Addy Osmani, "Stop Using /init for AGENTS.md" (<https://addyosmani.com/blog/agents-md/>):
  same argument against generated files.
- Several blog posts quote length targets (100 to 150 lines, under 500 lines for Cursor, a 32 KiB
  cap for Codex). These could not be traced to primary documentation in this pass; the 32 KiB
  and 500-line figures in particular are unverified and must be checked before being cited in
  any repository document.

### Contradicting evidence

A secondary write-up reports that, in a study of 124 Codex pull requests, having an `AGENTS.md`
lowered median runtime by about 28.6% and output tokens by about 16.6%. No primary source was
found, so it is treated as unverified. The two results are not necessarily in conflict: the ETH
study measures total cost and success on its benchmark, and the other reports runtime and output
tokens. Neither says a short, command-focused file is harmful.

## Field Checks Against the Docs

Checks run on 2026-11-03 with Claude Code 2.1.316, prompted by an outside comment on the issue.

- **Skill descriptions are not free, but repo skills are not auto-loaded here.** `/context all`
  lists only built-in and claude.ai skills; the repository skills live in `.github/skills/`, not
  `.claude/skills/`, so Claude Code does not load their descriptions. Moving text into a repo
  skill therefore costs no description tokens in Claude Code, but a skill is found only through a
  pointer in `AGENTS.md`, and that pointer costs tokens. Tools that do auto-load
  `.github/skills/` descriptions (for example GitHub Copilot) pay the description cost instead.
- **Nested `AGENTS.md` loads on demand here.** In this repository there is no `CLAUDE.md` or
  `CLAUDE.local.md` in the working directory or any parent. Reading
  `tests/lifecycle/signals.rs` with the Read tool injected `tests/AGENTS.md` as additional
  context. No hook is configured in the user or project settings, so this is not a local hook.
  This is a single observation on one version. The documented failure case is a `CLAUDE.md` or
  `CLAUDE.local.md` above the directory, which stops `AGENTS.md` from loading; the outside
  comment reports exactly that, for a repository with a root `CLAUDE.md`.
- **Completeness must be checked against the source, not against the moves.** A checklist seeded
  from the planned moves can only confirm those moves. The rule inventory
  ([`rule-inventory.md`](rule-inventory.md)) is built from the baseline file instead.

## Practices Adopted for This Issue

Each practice is tied to the evidence above and to this repository's constraints.

| #  | Practice                                                                                                  | Evidence          |
| -- | --------------------------------------------------------------------------------------------------------- | ----------------- |
| P1 | Keep inline only what every session needs: commands, hard rules, non-obvious conventions.                  | 1, 2              |
| P2 | Do not describe what an agent can derive from the code or a command (package lists, directory layout).     | 1, 2 (overviews)  |
| P3 | Move multi-step procedures and task-specific guidance to skills; link to them, do not copy them.           | 1                 |
| P4 | Do not split the file with `@` imports to save tokens; imports do not reduce the loaded size.              | 1                 |
| P5 | Keep high-consequence, always-applicable rules inline (GPG signing, linting gate, security reporting).     | 1, 2 (adherence)  |
| P6 | Keep rules that depend on enforcement in hooks or CI, not only in prose.                                   | 1                 |
| P7 | Use link-based progressive disclosure from a short root file to `docs/` and skills.                        | 3, AIHero         |
| P8 | Re-measure with the same method after the change and record the numbers.                                   | 2 (evaluate)      |

## Practices Considered and Not Adopted

| Practice                                              | Reason                                                                                                                                                                          |
| ----------------------------------------------------- | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| Adopt a fixed 200-line limit from the Claude docs     | It is a Claude Code recommendation for `CLAUDE.md`, not a cross-tool rule. Use it as a reference point for the target, which is still to be agreed with the maintainer (AC2). |
| Path-scoped `.claude/rules/` files                    | Claude Code specific. Policy 7 of `AGENTS.md` requires provider-specific adapters to stay optional and Git-tracked guidance to be the source of truth.                           |
| Nested `AGENTS.md` files in packages                  | Supported by the format and loaded on demand by Claude Code, so it saves tokens. Worth a follow-up evaluation per package, but it creates many new files and is out of this issue's scope. |
| Hiding content in HTML comments                       | Saves tokens only in Claude Code and hides content from other tools and from rendered views. Rejected as a general technique.                                                   |
| Add a `CLAUDE.md` that imports `AGENTS.md`            | Not needed: Claude Code reads `AGENTS.md` directly (T3), and adding any `CLAUDE.md` or `CLAUDE.local.md` changes which file is loaded.                                          |

## How the Major Tools Consume AGENTS.md (T3)

- **Claude Code:** reads `AGENTS.md` directly when no `CLAUDE.md` or `CLAUDE.local.md` exists in
  the working directory or any parent (v2.1.277 or later; some older or unusual sessions, such as
  Amazon Bedrock before v2.1.281, read `CLAUDE.md` only). This matches the maintainer's
  observation, and no adapter file is needed.
- **Other tools:** the agents.md site lists many tools that read `AGENTS.md` natively. This note
  did not verify each one's behavior, and none of them changes the plan.
- **Caveats to record in the repository, not to fix here:** a contributor's personal
  `CLAUDE.local.md` silently stops Claude Code from reading the repository `AGENTS.md`. The
  documented workaround is the `claude-md-and-agents-md` Project instructions setting.

Conclusion for T3: `AGENTS.md` stays the only root instruction file, and AC5 holds.

## Implications for This Repository

- The documented evidence favors a short file of commands, hard rules and non-obvious
  conventions, which is the opposite of the current mix of reference tables and procedures.
- The two highest-value candidates, by size and by being derivable, are the Package Catalog
  (about 23% of tokens) and the Key Configuration Files table.
- The Git Workflow section is the second largest. Much of it is procedure already owned by the
  `create-feature-branch`, `commit-changes`, `open-pull-request` and `merge-pull-request`
  skills.
- Hard rules that must stay inline because they are absolute and always applicable: signed
  commits and the GPG timeout rule, the linting gate, the pre-commit gate, never committing
  `storage/` or `target/`, and the security-reporting instruction.
- Rules enforced by hooks or CI (pre-commit, `cargo machete`, `cargo deny`) need only be
  mentioned once, with a pointer to the owning skill.

## Open Items

- Repeat the `/context` measurement after the change (M1) in a comparable fresh session, since
  that is the only Claude-native number.
- Verify the unverified length figures (Cursor, Codex) from primary documentation before citing
  them anywhere in repository documents.
- Read the full text of arXiv:2602.11988 for per-type results before using any number other than
  the abstract's.
- Agree the target reduction for AC2.
