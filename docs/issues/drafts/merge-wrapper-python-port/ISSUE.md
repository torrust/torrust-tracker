---
doc-type: issue
issue-type: task
status: draft
priority: p2
epic: null
github-issue: null
spec-path: docs/issues/drafts/merge-wrapper-python-port/ISSUE.md
branch: "{issue-number}-merge-wrapper-python-port"
related-pr: null
last-updated-utc: 2026-09-10 13:22
semantic-links:
  skill-links:
    - create-issue
  related-artifacts:
    - contrib/dev-tools/git/merge-pull-request.sh
    - contrib/dev-tools/git/github-merge.py
    - contrib/dev-tools/git/tests/test-merge-pull-request.sh
    - contrib/dev-tools/git/tests/test-github-merge-symlinks.py
    - contrib/dev-tools/git/README-github-merge.md
    - .github/skills/dev/git-workflow/merge-pull-request/SKILL.md
    - docs/issues/open/2175-merge-tool-symlink-exceptions/ISSUE.md
    - docs/issues/closed/2022-vendor-and-document-maintainer-merge-workflow/ISSUE.md
---

<!-- skill-link: create-issue -->

# Issue #[To be assigned] - Port the Maintainer Merge Wrapper to a Repository-Neutral Python Program

## Goal

Replace `contrib/dev-tools/git/merge-pull-request.sh` with a Python program beside the vendored engine that performs the same preflight and the same delegation, reads every repository-specific value from Git configuration instead of holding it as a constant, and therefore runs unchanged in any repository that has configured the merge workflow.

## Background

The vendored engine `contrib/dev-tools/git/github-merge.py` bakes in no repository identity. It reads `githubmerge.repository` (line 543), `githubmerge.host` (line 544), `githubmerge.branch` (line 545), `githubmerge.merge-author-email` (line 546), `githubmerge.testcmd` (line 547), `user.ghtoken` (line 548) and `user.signingkey` (line 549) from Git configuration, and its own help text (lines 513-522) documents them as the configuration surface. Its one Torrust-named constant, `SYMLINK_DECLARATION_NAMESPACE` (line 240), names the declaration format rather than a repository, which the comment above it and the format documentation both state; every repository adopting the workflow carries the same value. The engine is configuration-driven all the way down.

The wrapper is not. `merge-pull-request.sh` holds `EXPECTED_REPOSITORY="torrust/torrust-tracker"` (line 11), `TARGET_BRANCH="develop"` (line 12) and `SYMLINK_DECLARATION=".symlinks.json"` (line 13), and `require_repository_configuration` (lines 38-50) exits unless `githubmerge.repository` is exactly equal to the first of them. The same string is asserted in the wrapper's own test suite (`contrib/dev-tools/git/tests/test-merge-pull-request.sh` lines 44, 65, 109, 134, 271), in the sibling engine suite's fixture (`contrib/dev-tools/git/tests/test-github-merge-symlinks.py` line 30), in `contrib/dev-tools/git/README-github-merge.md` (lines 4-6) and in `.github/skills/dev/git-workflow/merge-pull-request/SKILL.md` (lines 21-22, 63, 70).

The consequence is that the preflight is unavailable to every repository except this one. `torrust/torrust-index` is the concrete case: it has `githubmerge.repository` configured for its own repository and it carries a symbolic link in its tree, `.dockerignore`, which is the link that made the declaration mechanism necessary in the first place, but it ships no copy of `contrib/dev-tools/git`, so a maintainer there invokes this repository's engine directly. Invoking the engine directly loses everything the wrapper adds: the argument shape check, the clean-tree check, the current-branch check, the repository-local signing-key message, the readable-engine check, the terminal guard that keeps an interactive tool from looping forever at end of input, the non-destructive `--dry-run`, and the `--symlinks` argument without which the engine performs no declaration processing at all and refuses every symbolic link it meets.

A wrapper whose engine is configuration-driven should not be the component that acquires a repository identity. The wrapper's job is preflight, and a preflight can validate the shape of a configured value without knowing which value is correct: that `githubmerge.repository` is set and names a repository as `<owner>/<repo>`, that a signing key is configured, that the working tree is clean, that the checked-out branch is the branch the merge will target. Those checks are exactly as strong for a sibling repository as they are here, and the only thing lost by dropping the constants is the wrapper's ability to refuse a repository that is not this one, which was never a safety property.

The remaining reason the wrapper is a shell script is historical: #2022 vendored the engine and added a small shell entry point around it. That entry point has since grown a terminal guard, a repository-local signing message and a declaration argument, and its test suite has grown a pseudo-terminal harness and a stub interpreter on `PATH` to observe delegation. It is a program now, written in the language whose argument handling, string comparison and process spawning this repository has had to review most carefully. Writing it in the language the engine and its own test suite already use removes the second language from the workflow and lets the port reuse the conventions the engine suite established.

## Scope

### In Scope

- Add `contrib/dev-tools/git/merge-pull-request.py`, a repository-neutral Python program that performs the preflight the shell wrapper performs today and then delegates to `github-merge.py`.
- Read the upstream repository from `githubmerge.repository`, require it, and validate it against the `<owner>/<repo>` shape rather than against a value. Report the key and the expected shape; never report an expected repository.
- Read the target branch from `githubmerge.branch`, defaulting to `develop`, and pass it to the engine explicitly as the shell wrapper does today.
- Read the symbolic-link declaration tree path from `githubmerge.symlinks`, defaulting to `.symlinks.json`, validate its shape with the same rule the engine applies to `--symlinks`, and pass it unconditionally.
- Preserve every message a test asserts today, except where this specification names a change and gives the reason.
- Add `contrib/dev-tools/git/tests/test-merge-pull-request.py`, replacing the bash suite, written with `unittest` in the conventions of `test-github-merge-symlinks.py`, and supplying its pseudo-terminal cases from the standard library rather than from `script(1)`.
- Add repository-neutral coverage the bash suite could not express: a fixture configured for a repository other than this one that passes preflight, and a fixture configured for a target branch other than `develop`.
- Remove `merge-pull-request.sh` and `test-merge-pull-request.sh` once the port carries every behaviour.
- Update `.github/skills/dev/git-workflow/merge-pull-request/SKILL.md`, `contrib/dev-tools/git/README-github-merge.md` and `AGENTS.md` to the Python entry point and to configuration-key wording.
- State the Python interpreter version the workflow requires, and check it in the program.
- Record the static-analysis coverage the port loses and what replaces it.

### Out of Scope

- Any change to the engine's merge semantics. `github-merge.py` is not edited by this issue: no change to fetching, merging, the symbolic-link check, the tree hash, signing, pushing, or any message it prints.
- Any new runtime dependency. The program and its tests use the Python standard library and the `git` the workflow already requires.
- Any behaviour that differs between repositories other than through configuration. The program contains no repository name, no branch name outside a documented default, and no conditional keyed on which repository it is running in.
- Vendoring or mirroring the port into `torrust/torrust-index` or any other repository. Adopting it there is separate work in those repositories; this issue only makes it adoptable.
- Migrating the workflow to Rust. #2003 may later select a different automation architecture; this issue keeps the repository-specific integration narrow, which is what that decision asked for.
- Adding a Python linter to the `linter` binary. That binary is built from `torrust/torrust-linting` and cannot be changed from here; this issue records the gap and refers it.
- Enforcing a declaration file against the tree, or any other new policy. The port moves existing behaviour; it adds no check the shell wrapper did not perform.

## Architectural Decisions

- Related ADRs: none in `docs/adrs/` covers the maintainer merge workflow.
- ADRs to create: none expected. The decisions below are local to one program and its documentation, and none of them selects an architecture for the repository.

### AD1 - The entry point keeps its name and gains the engine's extension

The program is `merge-pull-request.py`, beside `github-merge.py`. The name is what the skill, the vendoring README, `AGENTS.md` and every maintainer's habit already say, so keeping it means the only thing that changes at the command line is the extension. The extension is kept rather than dropped because the sibling engine carries one, and because it tells a reader which interpreter runs the file without opening it. The alternative, an extensionless `merge-pull-request`, was rejected for that reason: it would be the only executable in the directory whose language is invisible.

### AD2 - The repository is validated by shape, never by value

`githubmerge.repository` is required and must match `<owner>/<repo>`: exactly one separator, both parts non-empty, an owner limited to letters, digits and hyphens, a repository name limited to letters, digits, hyphens, underscores and dots, and neither part equal to a relative-path name. That is the rule GitHub itself enforces on those names, so a value that passes here is a value the engine can build a remote from, and a value that fails is a typing mistake rather than a policy disagreement.

Validating the shape rather than a value is the point of the change, but it is worth stating what is given up, because a reviewer will ask. Today the wrapper refuses when `githubmerge.repository` names some other repository, which reads like a guard against merging the wrong project. It is not one. The value is repository-local configuration the maintainer set themselves; the engine builds its fetch remote from that same value; and the wrapper's current-branch and clean-tree checks run against the working tree in front of the maintainer either way. A maintainer who has configured a different repository in this checkout has made a mistake the engine shows them in its merge details before anything is signed, and the wrapper refusing it here only means the same mistake in a sibling repository is unreachable by the preflight at all.

### AD3 - The target branch has a documented default and a documented key

The branch comes from `githubmerge.branch` and defaults to `develop`. A default is needed because the wrapper passes the branch to the engine explicitly and must keep doing so: the engine's own resolution order is the command-line argument, then `githubmerge.branch`, then the pull request's base branch as GitHub reports it, then `master` (line 584). Leaving the branch unset would let a merge target whatever base branch a pull request happens to name, which is the behaviour the wrapper exists to prevent.

`develop` is the convention of the repositories that share this tool, and it is not a hidden assumption: the wrapper checks that the checked-out branch equals the resolved target before doing anything else, so a repository whose development branch is named differently fails on its first run with a message naming `githubmerge.branch`. The alternative of requiring the key with no default was rejected because it invalidates every existing configuration on the day the port lands, in exchange for turning a loud first-run failure into a loud first-run failure.

### AD4 - The declaration path is configuration, and the mechanism has no off switch

The declaration tree path comes from `githubmerge.symlinks` and defaults to `.symlinks.json`. The key lives in the `githubmerge` namespace because that is where the engine reads every other repository fact, and it is named for the engine argument it feeds, so a maintainer who has set `githubmerge.repository` finds this one in the same place under a name they already know.

The value is validated as a tree path with the rule the engine applies to `--symlinks` (`symlink_declaration_path_error`, line 208): non-empty, not absolute, and containing no `..` segment. The wrapper duplicates that rule deliberately rather than deferring to the engine, because `--dry-run` must be able to report a misconfigured declaration path without invoking the engine at all, and a preflight that cannot check the configuration it forwards is not a preflight. The duplication is bounded by a test that asserts the wrapper and the engine reject the same values.

An empty configured value is a configuration error, not a way to switch the mechanism off. The wrapper always passes `--symlinks`, because the argument is what enables the check at all and the refusal it enables is a safety property: a repository that wants no exception simply commits no declaration file, which the engine already treats as granting nothing rather than as an error. Providing an off switch would let a repository lift a refusal by editing configuration instead of by committing a reviewed declaration, which is the property #2175 was designed to remove.

### AD5 - Delegation replaces the process, under the interpreter that was checked

The program delegates with `os.execv` on `sys.executable`, replacing itself the way the shell wrapper's `exec` does (line 142). Replacement is not an implementation detail here: the engine is interactive, reads its prompts from the terminal, and starts an interactive shell for testing when `githubmerge.testcmd` is unset. Leaving a parent process between the terminal and that program would insert something to mediate signals and terminal ownership for no benefit, and it would break the property that the engine's exit code is the command's exit code.

The interpreter is `sys.executable` rather than the first `python3` on `PATH`, so the engine runs under the interpreter that has just executed the wrapper's preflight rather than under a different one that happens to be earlier on the path. This is why the separate interpreter-on-`PATH` check is dropped; AD6 states what replaces it.

### AD6 - The interpreter requirement moves from a check to a documented floor

`require_python` (lines 79-84) exists because a bash program can run where Python does not. A Python program cannot: if it is running, an interpreter is present, and the check would be tautological. What that check never covered is an interpreter that is present but too old, which is the failure that actually reaches a maintainer, and which today produces a traceback from whichever file first uses a newer construct.

The port therefore states a minimum interpreter version in the documentation and checks it in the program, before any other work, with a message naming the version found and the version required. The floor is set from what the tree already requires and verified during implementation rather than assumed here; the observable lower bounds today are the engine's formatted string literals (line 218) and the engine suite's use of `subprocess.run` with captured, decoded output (lines 197-202). No document in the repository states a required interpreter version at all today, which is itself worth fixing while the workflow's entry point becomes a Python program.

The cost of dropping the check is that a missing interpreter is now reported by the shebang rather than by the program, as it is for every other Python file in the tree. The compensation is that the requirement is documented in the skill's prerequisites, where a maintainer setting up the workflow reads it, instead of being discoverable only by running the wrapper on a machine that lacks it.

### AD7 - The pseudo-terminal comes from the standard library

The terminal guard is tested by giving the program a pseudo-terminal on standard input from `pty.openpty()`, not by driving it through `script(1)`.

The bash suite's `require_pseudo_terminal_support` (lines 10-19) exists because `script` is a command name shared by implementations that reject the `--command` and `--return` flags the suite passes; a review during #2173 found the suite failing mid-run with a usage error, and the probe was added so it would fail by name instead. The probe is a workaround for depending on a tool outside the language the test is written in. `pty` is in the standard library, so the port's suite depends on nothing the interpreter does not already ship, and the probe disappears with the dependency.

The trade-off is that `pty.openpty()` is available on POSIX platforms only. The workflow already is: the engine selects terminal escapes on `os.name == 'posix'` (line 41), and its shell-out for interactive testing assumes a POSIX shell.

## Design and Ownership Review

The program spawns child processes, `git` and then the engine, and its tests allocate pseudo-terminal file descriptors, so these concerns apply.

- Interface and responsibility: the program's public surface is its command line. It owns argument validation, configuration reading, environment preconditions, and the construction of the engine's argument list. It owns no merge logic, reads no declaration file, and parses nothing the engine parses.
- Child-process ownership in normal operation: every `git` invocation goes through one helper that calls `subprocess.run` with an argument list, captures output, and returns a value; no invocation goes through a shell, and no value read from configuration is interpolated into a command string. The final delegation transfers ownership of the process image to the engine, so beyond that point there is no child to own.
- Failure and drop paths: a `git` invocation that exits non-zero is a preflight failure with a message naming what was being read, never a traceback. A `git` binary that is absent is reported in the same shape. Because delegation replaces the process, no path leaves the program holding a running child while it exits.
- Deadlines: the program awaits nothing over a network and holds no readiness operation, so no absolute deadline applies to it. Its tests do hold one: a case that supplies a pseudo-terminal must read from the primary side while the child runs and must bound that wait, so a child that unexpectedly blocks fails the case instead of hanging the suite. That matters more than usual here, because the behaviour under test is a guard against a program that never terminates.
- Design-review checkpoint: after the first vertical slice, meaning argument handling plus one configuration read plus one preflight failure with its test, the shape is reviewed before the remaining behaviours are ported.

## Implementation Plan

Status values: `TODO`, `IN_PROGRESS`, `BLOCKED`, `DONE`.

| ID  | Status | Task | Notes / Expected Output |
| --- | ------ | ---- | ----------------------- |
| T1  | TODO   | Establish the program skeleton and its first slice | `merge-pull-request.py` with the command line, the interpreter-version floor, the `git` helper and one configuration read, plus the test module that exercises them. Reviewed before the remaining behaviours are ported. |
| T2  | TODO   | Port argument handling | `--dry-run`, `-h`/`--help`, and a pull-request argument accepted only in the shape the shell wrapper accepts, with the current message text and exit code `2`. |
| T3  | TODO   | Port the environment preconditions | Working-tree presence, clean tree, checked-out branch equal to the resolved target, signing key configured, with the current message text and exit code `1`. |
| T4  | TODO   | Port the configuration reads and their shape rules | `githubmerge.repository`, `githubmerge.branch` and `githubmerge.symlinks` with the defaults and rules of AD2, AD3 and AD4, and messages naming the key and the expected shape. |
| T5  | TODO   | Port `--dry-run` and delegation | The dry-run report, and `os.execv` on `sys.executable` with the engine, `--symlinks <path>`, the pull-request number and the resolved branch. |
| T6  | TODO   | Complete the test module | Every behaviour in the mapping table below, the pseudo-terminal cases from `pty`, and the repository-neutral cases the bash suite could not express. |
| T7  | TODO   | Retire the shell wrapper and its suite | `merge-pull-request.sh` and `test-merge-pull-request.sh` are deleted once T6 is green. |
| T8  | TODO   | Update the documentation and the skill | Skill, vendoring README and `AGENTS.md` name the Python entry point, describe the configuration keys and their defaults, state the interpreter requirement, and state the static-analysis gap. |

### Behaviour mapping

Every behaviour of `merge-pull-request.sh` and every case of `test-merge-pull-request.sh` appears once. The bash suite runs thirteen cases (lines 346-358) behind one harness precondition (lines 10-19); all fourteen entries are mapped, and the cases are numbered C1 to C13 in the order that list runs them. "Preserved" means the observable result, message text included, is unchanged.

| ID | Behaviour, at its source | Covering bash case | In the port |
| -- | ------------------------ | ------------------ | ----------- |
| B1 | `--dry-run` recognised as the leading option (lines 100-104) | C1, C2, C3, C4, C5, C6, C10, C13 | Preserved, and accepted in any position, which argparse gives for free and which no case asserts against. |
| B2 | `-h`/`--help` prints usage and exits `0` (lines 105-108, 18-29) | none | Preserved, with the text on standard output rather than standard error, which is argparse's convention for a help page a user asked for. Error paths keep writing to standard error. |
| B3 | Exactly one argument, matching `^[1-9][0-9]*$`, else `ERROR: PULL_REQUEST must be a positive integer.`, usage, exit `2` (lines 111-115) | C13 | Preserved with a custom argparse type carrying that exact sentence. A plain integer type is not used: it would accept `0`, `-3`, `007` and surrounding blanks, all of which the shell wrapper rejects. |
| B4 | Must run inside a Git working tree, else exit `1` (lines 119-122) | none | Preserved. |
| B5 | Clean working tree via `git status --porcelain`, else exit `1` (lines 31-36) | C2 | Preserved, including that nothing in the working tree is touched. |
| B6 | `githubmerge.repository` must be set, else a message naming the key and the command that sets it, exit `1` (lines 38-49) | C4 | Preserved, with `<owner>/<repo>` in the suggested command instead of this repository's name, matching the engine's own message (line 552). |
| B7 | `githubmerge.repository` must equal `torrust/torrust-tracker`, else a message naming the configured value, exit `1` (lines 42-47) | C3 | Replaced by the shape rule of AD2. A well-formed value for any repository passes; a malformed value is refused with a message naming the key, the value found and the expected shape. C3 becomes a malformed-value case, and a new case asserts that a well-formed value for another repository passes preflight. |
| B8 | Checked-out branch must equal the target, else a message naming both, exit `1` (lines 52-60) | none | Preserved, with the target resolved per AD3 and the message naming `githubmerge.branch`. The `detached HEAD` rendering for an empty current branch is preserved. |
| B9 | `user.signingkey` must be non-empty, else the repository-local message, exit `1` (lines 62-70) | C5, C6 | Preserved verbatim, including that an empty value is treated as unset. This is the message #2173 added to correct the engine's global-scope advice, and the vendoring README points a reader at it (line 88). |
| B10 | `--dry-run` reports `Dry-run preflight passed for <repository> PR <number> targeting <branch>.` on standard output and exits `0` before any engine, interpreter or terminal check (lines 129-132) | C1, C10 | Preserved, with the repository and branch now the resolved values. The ordering is preserved because it is what makes the dry run usable without a terminal and without the engine present. |
| B11 | Engine file must exist and be readable, else a message naming its path, exit `1` (lines 72-77) | C11 | Preserved, including that the path reported is the resolved absolute path. |
| B12 | An interpreter must be on `PATH`, else exit `1` (lines 79-84) | C12 | Dropped, per AD5 and AD6. C12 is replaced by an interpreter-version case: too old is refused with a message naming the version found and the version required. |
| B13 | Standard input must be a terminal, else the loops-forever message, exit `1` (lines 86-95) | C9 | Preserved verbatim, including that it runs after the dry-run exit so `--dry-run` stays usable without a terminal, and that no interpreter is spawned when it fails. |
| B14 | Delegation with `exec`, so the engine's exit code is the command's (line 142) | C7 | Preserved with `os.execv`, per AD5. |
| B15 | `--symlinks <path>` passed unconditionally with no filesystem check, because the path names a location in a tree that does not exist yet (lines 138-142) | C8 | Preserved, with the path resolved per AD4. The case's assertion that the working tree is neither read nor written at that path is preserved. |
| B16 | Engine located relative to the program's own directory, not the working directory (lines 14-16) | implied by C11 | Preserved, resolved from the program's own file location. |
| B17 | `set -euo pipefail`: an unexpected failure aborts rather than continuing (line 9) | none | Preserved by construction: an unhandled failure is an exception, and every expected failure is an explicit exit. |
| B18 | Harness precondition: `script(1)` supports the flags the suite passes (lines 10-19) | harness | Dropped with its dependency, per AD7. |

Two behaviours are dropped, B12 and B18, and both because the port removes the reason they existed rather than because the port cannot express them. B7 is the only behaviour deliberately changed, and it is the change this issue exists for.

### Exit codes

The program documents and uses three, matching the shell wrapper and the conventions already in the tree.

| Code | Meaning |
| ---- | ------- |
| `0` | Preflight passed under `--dry-run`, or a help page was requested. |
| `1` | A precondition or a configuration check failed. |
| `2` | The command line was not usable. This is also argparse's own code for a rejected command line, so the two paths agree without special handling. |
| other | Produced by the engine after delegation. The engine's codes are unchanged by this issue; `4` for a refused symbolic link and `1` for an unsigned exit are the ones the sibling suite asserts (lines 33-35). |

## Commit Points

| Task | Coherent change set | Commit policy |
| ---- | ------------------- | ------------- |
| T1 | Program skeleton, interpreter floor, `git` helper, first configuration read, and their tests. | Commit after the design-review checkpoint on the first vertical slice. |
| T2 | Argument handling and its tests. | Commit after focused validation. |
| T3 | Environment preconditions and their tests. | Commit after focused validation. |
| T4 | Configuration reads, shape rules, defaults, and their tests. | Commit after focused validation; this is the commit that makes the program repository-neutral and it should be reviewable alone. |
| T5 | Dry-run report and delegation, with their tests. | Commit after focused validation. |
| T6 | Remaining test increments, one behaviour area per commit. | Commit each reviewed increment before starting the next area, and stop for maintainer review after the final increment. |
| T7 | Deletion of `merge-pull-request.sh` and `test-merge-pull-request.sh`. | Commit alone, so the removal is independently reviewable and independently revertible. |
| T8 | Skill, vendoring README and `AGENTS.md` updates. | Commit after the behaviour they describe is in the branch. |

The whole sequence belongs to one pull request rather than to a port followed by a separate retirement. Two entry points for one workflow is an ambiguity that has to be documented for as long as it lasts: the skill, the README and the coverage boundary would each have to say which one is authoritative, and a maintainer following the older instruction would keep using the wrapper that refuses sibling repositories. The port's own suite is the evidence that every behaviour survived, so the deletion is best reviewed in the diff that carries that evidence. Keeping T7 as its own commit preserves the ability to revert the removal without reverting the port.

Record a justified no-change decision in the task's evidence without creating an empty commit. Use a Conventional Commit message with the narrow affected scope.

## Progress Tracking

### Workflow Checkpoints

- [ ] Folder-style spec drafted in `docs/issues/drafts/merge-wrapper-python-port/ISSUE.md`
- [ ] Spec reviewed and approved by user/maintainer
- [ ] GitHub issue created and issue number added to this spec
- [ ] Spec-only PR merged into `develop` before implementation
- [ ] Implementation completed
- [ ] Automatic verification completed (`linter all`, relevant tests, and any pre-push checks)
- [ ] Manual verification scenarios executed and recorded in issue-local `manual-verification-evidence.md`
- [ ] Acceptance criteria reviewed after implementation and updated with evidence
- [ ] Evidence-based implementation completion review recorded: issue-local retrospective created for material discoveries, or progress log states why none was needed
- [ ] Reviewer validated acceptance criteria and updated checkboxes
- [ ] Independent reviewer reports recorded in issue-local `agent-review-reports.md` when reviewers received this folder-style specification
- [ ] Committer verified spec progress is up to date before commit
- [ ] Issue closed and spec moved from `docs/issues/open/` to `docs/issues/closed/`

### Progress Log

Append one line per meaningful update.

- 2026-09-10 13:22 UTC - Spec author - Draft written against `develop` at `89d45145`; behaviour mapping built from `merge-pull-request.sh` and the thirteen cases of `test-merge-pull-request.sh` - this document

## Acceptance Criteria

- [ ] AC1: `contrib/dev-tools/git/merge-pull-request.py` contains no repository name, and no branch name other than the documented default of `githubmerge.branch`. Searching the file for this repository's name returns nothing.
- [ ] AC2: The program requires `githubmerge.repository` and accepts any value matching `<owner>/<repo>`. A fixture configured for a repository other than this one reaches the same preflight result as one configured for this one.
- [ ] AC3: A malformed `githubmerge.repository`, an unset one, a declaration path that is absolute or contains a `..` segment, and an empty declaration path each fail with exit code `1` and a message naming the configuration key and the expected shape, and naming no expected repository.
- [ ] AC4: The target branch resolves from `githubmerge.branch`, defaults to `develop`, is checked against the checked-out branch, and is passed to the engine explicitly. A fixture configured for another branch passes preflight on that branch and is refused on `develop`.
- [ ] AC5: The declaration path resolves from `githubmerge.symlinks`, defaults to `.symlinks.json`, is passed unconditionally as `--symlinks`, and is never looked up in the working tree. The wrapper rejects exactly the paths the engine rejects for that argument.
- [ ] AC6: Every behaviour marked "Preserved" in the mapping table produces the same observable result as the shell wrapper does at `89d45145`, message text included.
- [ ] AC7: Delegation replaces the process, so the engine's exit code is the command's exit code, and the engine runs under the interpreter that executed the preflight.
- [ ] AC8: `python3 contrib/dev-tools/git/tests/test-merge-pull-request.py` covers every mapped behaviour, supplies its terminal cases from the standard library, and requires no tool outside Python and `git`.
- [ ] AC9: `merge-pull-request.sh` and `test-merge-pull-request.sh` are removed, and no file in the repository refers to either.
- [ ] AC10: The skill, the vendoring README and `AGENTS.md` name the Python entry point, document `githubmerge.branch` and `githubmerge.symlinks` with their defaults, and state the interpreter requirement.
- [ ] AC11: The vendoring README's provenance statement continues to name `github-merge.py` alone, and the new program carries no upstream copyright header.
- [ ] AC12: The static-analysis coverage the port loses is stated in the vendoring README's coverage boundary, with the referred follow-up named.
- [ ] `linter all` exits with code `0`
- [ ] Relevant tests pass
- [ ] Manual verification scenarios are executed and documented in issue-local `manual-verification-evidence.md`
- [ ] Acceptance criteria are re-reviewed after implementation and reflect actual behavior
- [ ] Documentation is updated when behavior/workflow changes

## Verification Plan

Define verification before implementation starts and execute it before closing the issue.

### Automatic Checks

- `python3 contrib/dev-tools/git/tests/test-merge-pull-request.py`
- `python3 contrib/dev-tools/git/tests/test-github-merge-symlinks.py`, unchanged by this issue and run to show it stays green
- `linter all`
- Pre-push checks (when applicable)

### Static Analysis Coverage

`linter` offers `markdown`, `yaml`, `toml`, `cspell`, `clippy`, `rustfmt` and `shellcheck`, and the pre-commit hook runs `linter all` alongside the dictionary formatter and `hadolint` (`contrib/dev-tools/git/hooks/pre-commit.sh` lines 52-56). Nothing in that set analyses Python. `merge-pull-request.sh` and `test-merge-pull-request.sh` are checked by `shellcheck` today; after the port they are gone and their replacements are checked by nothing, while `github-merge.py` and the engine suite were already unchecked. The port therefore removes the only static analysis the wrapper had, and it is worth saying plainly rather than discovering later.

The `linter` binary is built from `torrust/torrust-linting` and cannot gain a subcommand from this repository, so this issue does not close the gap. It requires instead that the gap be stated in the vendoring README's coverage boundary and referred to that repository as a request for a Python subcommand, so the Python files here and their equivalents in sibling repositories are covered in one place. A repository-local check script wired into the pre-commit hook, in the shape `lint-containerfile.sh` already uses for `hadolint`, is the fallback if that request is declined; it is the fallback rather than the proposal because it would add a tool installation to every contributor's pre-commit run for files only maintainers execute, and it would leave every sibling repository to repeat the wiring.

### Manual Verification Scenarios

Status values: `TODO`, `IN_PROGRESS`, `DONE`, `FAILED`, `BLOCKED`.

| ID | Scenario | Human-oriented command/steps | Expected Result | Status | Evidence |
| -- | -------- | ---------------------------- | --------------- | ------ | -------- |
| M1 | Dry run in this repository | On a clean `develop`, run `./contrib/dev-tools/git/merge-pull-request.py --dry-run <pull-request-number>` with neither `githubmerge.branch` nor `githubmerge.symlinks` configured. | The dry-run line names this repository, the pull request and `develop`, and the command exits `0`. | TODO | `manual-verification-evidence.md` section V1 |
| M2 | Dry run in a sibling repository | In a clone of `torrust/torrust-index` with `githubmerge.repository` and `user.signingkey` set, on its development branch, run this repository's program with `--dry-run`. | The dry-run line names that repository and its branch, and the command exits `0`. This is the case the shell wrapper refuses. | TODO | `manual-verification-evidence.md` section V2 |
| M3 | Misconfigured repository | Set `githubmerge.repository` to a value with no separator and run `--dry-run`. | Exit `1`, with a message naming the key and the `<owner>/<repo>` shape and naming no expected repository. | TODO | `manual-verification-evidence.md` section V3 |
| M4 | Wrong branch checked out | On a branch other than the resolved target, run `--dry-run`. | Exit `1`, with a message naming the checked-out branch, the target, and `githubmerge.branch`. | TODO | `manual-verification-evidence.md` section V4 |
| M5 | Terminal guard in a real shell | In an interactive terminal session, run the program without `--dry-run` and with standard input redirected from `/dev/null`. | Exit `1` with the loops-forever message, and no engine process started. | TODO | `manual-verification-evidence.md` section V5 |
| M6 | Real merge inspection and rejection | With maintainer authorisation, run the program without `--dry-run` against a disposable pull request in a terminal, inspect the constructed merge, and answer the signing prompt with a refusal. | The engine runs exactly as it does under the shell wrapper today, the declaration argument is in effect, and the refusal leaves the repository on its target branch with the temporary branches cleaned up. | TODO | `manual-verification-evidence.md` section V6 |
| M7 | Declared symbolic link still admitted | With maintainer authorisation, repeat M6 against a pull request whose merged result declares a symbolic link at the configured path. | The accepted link is printed with its path, target and reason before the signing prompt, showing the declaration argument survived the port. | TODO | `manual-verification-evidence.md` section V7 |

Notes:

- Manual verification is mandatory even when automated tests pass. It is a real human-oriented use of the workflow, not a simulated result and not merely running automated tests.
- Create `manual-verification-evidence.md` from `docs/templates/MANUAL-VERIFICATION-EVIDENCE.md` when executing these scenarios. Record actual prerequisites, actions, commands, program output, and outcomes there.
- M6 and M7 require explicit maintainer authorisation and are the only scenarios that contact GitHub. Neither is signed or pushed as part of this verification.
- If a scenario fails, record the failure and diagnosis in the progress log before proceeding.

### Disposable Verification Scripts

None are proposed. Every behaviour this issue moves is deterministic and belongs in the maintained Python suite, which is where the equivalent bash coverage already lives; the scenarios that cannot be automated need GitHub, credentials, a terminal and maintainer judgment, and the manual scenarios above cover them. Should one become necessary during implementation, record its issue-local path, what it verifies and its removal owner, and note that it is Python rather than Rust because it exercises a Python program's command line directly.

### Acceptance Verification

| AC ID | Status (`TODO`/`DONE`) | Evidence |
| ----- | ---------------------- | -------- |
| AC1 | TODO | {test/log/PR link} |
| AC2 | TODO | {test/log/PR link} |
| AC3 | TODO | {test/log/PR link} |
| AC4 | TODO | {test/log/PR link} |
| AC5 | TODO | {test/log/PR link} |
| AC6 | TODO | {test/log/PR link} |
| AC7 | TODO | {test/log/PR link} |
| AC8 | TODO | {test/log/PR link} |
| AC9 | TODO | {test/log/PR link} |
| AC10 | TODO | {test/log/PR link} |
| AC11 | TODO | {test/log/PR link} |
| AC12 | TODO | {test/log/PR link} |

## Risks and Trade-offs

- **The repository check is relaxed.** Dropping the equality check removes a refusal a reader may have taken for a safety net. AD2 argues it was not one: the value is local configuration the maintainer set, the engine builds its remote from the same value, and the merge details are displayed before anything is signed. What remains is what always did the work, the current-branch and clean-tree checks against the working tree in front of the maintainer, and those are unchanged. The residual risk is a maintainer with a mistyped but well-formed repository configured; the engine's fetch fails against a repository that does not exist, and its merge details name the repository when it does.
- **A default branch is a convention encoded in a program.** `develop` is right for the repositories that share this tool and wrong for one that uses another name, and AD3 accepts that in exchange for not invalidating existing configurations. The mitigation is that the mistake cannot be silent: the current-branch check refuses on the first run and its message names the key to set. The residual risk is a repository whose development branch is named `develop` but which wants merges targeting something else; that repository sets the key, as any repository may.
- **Two programs will state the declaration-path rule.** AD4 duplicates the engine's path rule in the wrapper so `--dry-run` can report a misconfiguration without the engine. Duplicated rules drift. The mitigation is a test that feeds the same values to both and asserts they agree, so drift fails the suite instead of surfacing during a merge.
- **The port loses static analysis.** Stated in full under Static Analysis Coverage, with the referral and the fallback. The residual risk is that the referral is declined and the fallback is not taken up, leaving this Python checked only by review and by its own tests; the tests are the stronger of the two and they grow in this issue rather than shrink.
- **A message the maintenance model depends on could be lost in translation.** The repository-local signing message exists because the engine's own advice names the global scope, and the vendoring README points a reader at the wrapper's message for the correct one. A port that paraphrases it breaks that reference silently. The mitigation is AC6 and a test asserting the sentence verbatim, as the bash suite does today.
- **#2175 stated that the declaration path is named in exactly one place.** That acceptance criterion described a shell constant. After this port the path is named once as a documented default in the program, overridable by `githubmerge.symlinks`. The substance survives, one reviewed statement per repository, but the wording in the vendoring README (lines 22, 28) and in the skill (line 130) describes a line of a file that will no longer exist. Those sentences are rewritten under T8 rather than left to be read as an unmet criterion.
- **Sibling repositories can still drift.** Making the program adoptable does not make it adopted, and a repository that copies it holds its own copy. The mitigation is the one the vendoring README already states: this repository's copy is the reference, and changes are mirrored deliberately. The port narrows what has to be mirrored, because a copy now differs from this one in nothing at all.

## Implementation Completion Review

After implementation, compare the result with this specification. Record invalidated assumptions, material design changes, unexpected validation findings, and reusable lessons.

- Retrospective: `Not yet assessed`
- If needed, create `implementation-retrospective.md` from the repository template at `docs/templates/IMPLEMENTATION-RETROSPECTIVE.md` in this issue specification's directory.
- If no retrospective is needed, add a concise progress-log entry explaining why the work had no material discovery.
- When an independent reviewer receives this folder-style specification, it records its result in `agent-review-reports.md` using `docs/templates/AGENT-REVIEW-REPORTS.md`.

## References

- Related issues: #2022 vendored the engine and added the shell entry point this issue replaces; #2175 added the symbolic-link declaration the port must keep passing; #2003 is the automation umbrella EPIC the workflow reports to. None of them is a parent of this issue.
- Related PRs: #2173 added the repository-local signing message, the terminal guard and the `script(1)` flag probe; #2184 merged #2175.
- Related ADRs: `docs/adrs/` holds none for this area.
- Provenance record: [`contrib/dev-tools/git/README-github-merge.md`](../../../../contrib/dev-tools/git/README-github-merge.md)
- Workflow documentation: [`merge-pull-request` skill](../../../../.github/skills/dev/git-workflow/merge-pull-request/SKILL.md)
