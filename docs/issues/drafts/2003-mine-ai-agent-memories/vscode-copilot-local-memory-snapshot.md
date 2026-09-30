# Snapshot: VS Code GitHub Copilot Chat Local Memory

<!-- cspell:ignore behaviours -->

> **Status:** Immutable source snapshot. Do not edit the records below; record corrections and
> classifications in the analysis artifacts instead.
>
> **Issue contract:** [ISSUE.md](ISSUE.md)

## Provenance

- **Source:** The GitHub Copilot Chat memory tool in VS Code on the maintainer's workstation
  (josecelano). GitHub Copilot read every memory file with the tool's `view` command.
- **Captured:** 2026-09-30.
- **Workspace:** a local clone of `torrust/torrust-tracker` in the folder
  `torrust-tracker-agent-02`.
- **Scopes:**
  - **User memory** (`/memories/`): kept for every workspace and conversation of this user,
    including other repositories. Records `U01`-`U40`.
  - **Repository memory** (`/memories/repo/`): kept per workspace folder. Other local clones,
    such as other `torrust-tracker-agent-*` folders, can hold different repository memories.
    They are not captured here. Records `R01`-`R10`.
  - **Session memory** (`/memories/session/`): empty at capture time. It is deleted when a
    conversation ends, so it is out of scope.
- **Records:** 50. Each Markdown bullet in a memory file is one record. File headings are not
  records.
- **Not captured:** other contributors' local memories, other clones' repository memories, and
  memories of other agent products.

## Conversion Notes

- The record text is unchanged except for the text redacted in the privacy review below. The
  only escaping is `\|` for pipes inside table cells.
- The `File` column is the memory file path relative to its scope root.

## Privacy Review

Reviewed on 2026-09-30 before publication. No credentials, tokens, keys, email addresses, IP
addresses, private hostnames, or local filesystem paths were found. The only hostnames are the
public Torrust demo trackers. The maintainer's own name and GitHub handle already appear publicly
in commit history, so they are kept.

Redacted records:

- `U11`: removed a contributor's real first name; the public GitHub handle is kept.
- `U19`: removed the record text. It described how the maintainer's agent sessions may run
  commands on remote servers. That is operational security detail, not repository guidance. The
  record stays in the table so that IDs and counts still match the source.

## User Memory

| ID  | File                  | Memory |
| --- | --------------------- | ------ |
| U01 | `git-authorship.md`   | When reorganizing or porting another contributor's implementation, prefer preserving the original implementation author with git commit --author; keep new review/docs-only commits under Jose Celano. |
| U02 | `git-authorship.md`   | For mixed substantial work from both authors, consider Co-authored-by trailers rather than replacing authorship blindly. |
| U03 | `git-history.md`      | Never propose squashing/fixup/rewriting branch history. Intermediate commits, including mistakes and their corrections, are kept deliberately as training material. Rebasing onto develop is fine (and preferred by the user whenever develop has moved, even without conflicts); it is not a history rewrite in this sense. |
| U04 | `git-history.md`      | Minor skill/doc/process improvements learned during a task ride along in the same PR as separate `docs(...)`/`chore(...)` commits. Do not propose a separate PR for them. |
| U05 | `git-history.md`      | Before every push to a PR branch: `git fetch` + behind-check as a SEPARATE step; if behind, stop and rebase (`git rebase -S <upstream>/develop`, then `--force-with-lease`). Never chain the check and the push in one command (pushed a stale branch once on PR #2335). READ the count before pushing: on PR #2363 the check printed 2 and I pushed anyway (2026-09-28). |
| U06 | `git-history.md`      | Both are now in AGENTS.md "Commit history and PR scope" (torrust-tracker). |
| U07 | `git-history.md`      | Limit work in progress: finish and merge open PRs before starting new issues or PRs. Merge PRs one at a time; rebase the next one onto the latest develop just before its merge. Ask before opening a new PR while others are open (2026-09-27). |
| U08 | `git-history.md`      | Copilot review severities live in `<picture>` badge markup (`alt="Medium severity"`); stripping HTML with sed hides them. Grep the raw body for `alt=".* severity"` before recording severity (missed on PR #2363, 2026-09-28). |
| U09 | `git-history.md`      | Timestamps in logs/comments: run `date -u` immediately before writing each stamp; never estimate (wrote future stamps twice on #2347, 2026-09-28). |
| U10 | `git-history.md`      | Scope: the WIP rule covers only PRs from the current session. Do not process, rebase, or review other open PRs (other sessions/agents) unless the user asks; at most list them as out of scope (2026-09-27). |
| U11 | `git-history.md`      | Spec-only means stop after the spec PR merges: never propose implementation next, esp. under an EPIC another contributor owns (#2003 → da2ce7/[name redacted]). #2375, 2026-09-29. |
| U12 | `git-history.md`      | Hand-offs: when an issue/EPIC is handed to another contributor, state explicitly who does the remaining steps (close-out, closure, archive). Before starting any post-merge step, check for open PRs on the same issue (`gh pr list --search "#<issue>"`). 2026-09-28: #2368 duplicated da2ce7's #2367 T7 close-out, opened 3 min apart. |
| U13 | `git-signing.md`      | Never pipe `git commit -S` through a filter like `grep -E "ERROR\|..."` (case-sensitive; hides the cause). A commit that did not land may be a pre-commit hook failure (read `.tmp/pre-commit-*.log`, e.g. cspell) or a GPG failure; diagnose first, and ask before retrying a GPG failure (2026-09-26: an unnoticed failure was retried automatically). |
| U14 | `git-signing.md`      | The user intends to enter their own GPG passphrase at the terminal. If a signed commit times out because they were away, ask whether to retry the identical signed command so they can sign it; never retry automatically or handle the passphrase in chat. |
| U15 | `github-tooling.md`   | 2026-09-04: GitHub MCP server write tools misbehaved (PR-create failed; `create_repository` made a stray private repo `josecelano/x` twice). Likely a bug in that server version, not a permanent rule. Until re-verified on a newer version, prefer `gh` CLI with `--body-file` for GitHub writes in torrust repos; read-only MCP tools are fine. |
| U16 | `github-tooling.md`   | Re-test MCP writes on a throwaway target before trusting them again; remove this note if they work. |
| U17 | `github-tooling.md`   | For fork-based PRs: `gh pr create --repo torrust/torrust-tracker --base develop --head josecelano:<branch>`. |
| U18 | `secrets-handling.md` | Never ask for or handle secrets (passwords, tokens, passphrases). Give the user the exact command with a placeholder (e.g. `<TOKEN>`); they substitute and run it themselves. |
| U19 | `secrets-handling.md` | [Redacted before publication: personal operational note about remote server administration.] |
| U20 | `shell-quoting.md`    | When using `gh pr create` from shell, never include unescaped backticks in `--body`; shell command substitution can corrupt PR text. |
| U21 | `shell-quoting.md`    | Prefer `--body-file` with a heredoc-written temp file for multiline PR descriptions. |
| U22 | `shell-quoting.md`    | Use a NEW, PR-specific body filename and create it in a separate step BEFORE `gh pr create`; never chain them in parallel. On #2382 the body-file create failed ("file exists") and `gh` used a stale spec-PR body (`Related to` instead of `Closes`); fixed with `gh pr edit --body-file`. |
| U23 | `shell-quoting.md`    | In shell `rg` commands, avoid backticks inside double-quoted patterns; use single quotes or escape them to prevent command substitution. |
| U24 | `shell-quoting.md`    | Never gate a commit with `linter all \| tail -1 && git commit`: the pipe returns `tail`'s exit code, so a failing gate still commits. Capture the exit code (`cmd > log 2>&1; rc=$?`) or use `grep -E "FAIL\|SUCCESS"` only for display, never in an `&&` chain. |
| U25 | `tooling-language.md` | Rust is the target for all developer tooling; existing Python tools (e.g. `github-merge.py`, `validate-audit-record.py`) are prototypes slated for migration to Rust. |
| U26 | `tooling-language.md` | Python is acceptable for prototyping a tool, but say so explicitly in the doc/skill that references it and keep the prototype's behaviour as the reference for the Rust port. |
| U27 | `tooling-language.md` | Don't default to Python out of habit; when logic is non-trivial, propose Rust and let the user pick. |
| U28 | `tracker-testing.md`  | Prefer external UDP/HTTP verification target: torrust-tracker-demo endpoints (`udp://udp1.torrust-tracker-demo.com:6969/announce`, `https://http1.torrust-tracker-demo.com:443/announce`). |
| U29 | `tracker-testing.md`  | Legacy torrust-demo tracker (`tracker.torrust-demo.com`) may be overloaded; if it times out, retry against torrust-tracker-demo before assuming code issues. |
| U30 | `tracker-testing.md`  | For deterministic verification, local tracker run is acceptable/preferred when public tracker load is high. |
| U31 | `tracker-testing.md`  | Use the unified `tracker_client` binary: `cargo run -p torrust-tracker-client --bin tracker_client -- udp announce <url> <hash>` and `... -- http announce <url> <hash>`. The old `udp_tracker_client` and `http_tracker_client` binaries are deprecated. |
| U32 | `tracker-testing.md`  | Arrange sections (maintainer preference): ask "what is the ONE difference in initial state that makes the Act behave differently?" and make it visible: inline value, readable builder chain (when the chain itself names the choice), or a scenario fixture named for the state (e.g. `ServerStartWithDuplicateRegistration`) when several coordinated steps produce it. Builders are NOT discouraged — pick the tool that states the condition most directly. Keep Act + typed assertions visible. |
| U33 | `tracker-testing.md`  | Test bootstraps intentionally do NOT reuse production container factories: construct only the services the test needs (fewer deps, explicit coupling, speed). Do not propose "just use the production factory". |
| U34 | `tracker-testing.md`  | Before writing tests for a module, ask "what does THIS module decide?" and inventory behaviours (own vs collaborator-owned) first; maintainer wants the list, then a unit/collaboration split, then only bug-scoped tests implemented and the rest noted in the owning modules with an EPIC ref (#1347 for swarm-coordination-registry coverage). |
| U35 | `tracker-testing.md`  | A test whose outcome depends on a collaborator's semantics is a collaboration test: move it up (e.g. `statistics::tests` in the parent mod), don't leave it in the module's unit tests. If the module's decision has no observable seam, extract a pure fn and unit-test that. |
| U36 | `tracker-testing.md`  | Prose-first = the test BODY reads as prose via state-named fixtures and one-level action names; a `// Scenario:` comment over boilerplate is not enough. Drop assertions another test already owns. |
| U37 | `tracker-testing.md`  | Pedantic clippy: `assert_eq!` on f64 trips `float_cmp` (return counts as usize); `unused_self` on a fixture method hints it belongs elsewhere. cspell: reword ordinary English rather than growing project-words.txt. |
| U38 | `tracker-testing.md`  | #1347 package-coverage subissues: EVERY Rust file in the package gets its own file test plan (review/refactor existing tests vs write-unit-test skill → add unit tests → integration only if loopback is clearer → record per-file coverage). T1 inventory "no change" rows are hypotheses that order the work, NOT terminal decisions; never skip a file or jump to T5+ because the inventory said "no change". Plans are created lazily right before starting a file; the ISSUE.md ledger (PENDING/IN_PROGRESS/DONE) tracks all files. Gates: maintainer approves plan, refactor result, completed file; one commit per file. |
| U39 | `tracker-testing.md`  | Mutation proof: if the test being proven is uncommitted in the SAME file, never `git checkout --` it (wipes the new test); revert the mutation by hand. |
| U40 | `tracker-testing.md`  | Bug-fix regression tests: prove them by mutating the fix in the working tree (never stage), confirm failure, `git checkout --` restore; try nearest variants (captured-at-startup vs cached-after-first-use). Stale-state bugs need two iterations of the same instance in one test. Rules live in write-unit-test SKILL.md "Prove a Regression Test Guards the Bug". |

## Repository Memory

| ID  | File                          | Memory |
| --- | ----------------------------- | ------ |
| R01 | `torrust-tracker-workflow.md` | Base branch for new issues/features: `develop` |
| R02 | `torrust-tracker-workflow.md` | Remotes: `torrust` (upstream), `josecelano` (fork) |
| R03 | `torrust-tracker-workflow.md` | Use conventional commits (dependabot style for deps: `chore(deps): bump X from N to M`) |
| R04 | `torrust-tracker-workflow.md` | After PR merge: pull `torrust/main` (or `torrust/develop`), force-delete local branch if squash-merged |
| R05 | `torrust-tracker-workflow.md` | If `git commit -S` times out awaiting the GPG passphrase, ask the user whether they want to commit manually or want the agent to retry while they enter the passphrase; never bypass signing. |
| R06 | `torrust-tracker-workflow.md` | Issue cleanup workflow: always enumerate both `docs/issues/open/*/` directories and standalone `docs/issues/open/*.md` specs before checking closed status and archiving. |
| R07 | `torrust-tracker-workflow.md` | 2026-09-08: Newly enabled Lychee reports existing repository-wide local-link failures. For current unrelated work, record Lychee as the sole expected pre-commit failure; do not suppress/fix it ad hoc. Address it in a dedicated maintenance task. |
| R08 | `torrust-tracker-workflow.md` | Git-spawning dev tools run from hooks: never strip `GIT_INDEX_FILE` (hooks get a temp index for `commit -a`/`--only`); only strip location vars (`GIT_DIR`, `GIT_WORK_TREE`, ...). Test via a real `.git/hooks/pre-commit` + `git commit -a`/`--only`, not by running `--staged` after `git add`. |
| R09 | `torrust-tracker-workflow.md` | Regression tests: assert the record kind/category, not just the path; error records often carry the same path (PR #2357 F9). |
| R10 | `torrust-tracker-workflow.md` | After each rebase, re-run `frontmatter-validator --all`: upstream archive PRs change specs and counts. |
