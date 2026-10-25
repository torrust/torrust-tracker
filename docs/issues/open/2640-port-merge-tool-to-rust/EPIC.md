---
schema-version: 1
doc-type: epic
status: planned
epic: null
github-issue: 2640
spec-path: docs/issues/open/2640-port-merge-tool-to-rust/EPIC.md
epic-owner: josecelano
last-updated-utc: "2026-10-24 20:31"
semantic-links:
  skill-links:
    - create-issue
    - merge-pull-request
    - add-workspace-member
    - handle-secrets
  related-artifacts:
    - .github/skills/dev/planning/create-issue/SKILL.md
    - .github/skills/dev/git-workflow/merge-pull-request/SKILL.md
    - .github/skills/dev/rust-code-quality/handle-secrets/SKILL.md
    - contrib/dev-tools/git/github-merge.py
    - contrib/dev-tools/git/github-merge-COPYING
    - contrib/dev-tools/git/merge-pull-request.sh
    - contrib/dev-tools/git/README-github-merge.md
    - contrib/dev-tools/git/tests/test-github-merge-symlinks.py
    - contrib/dev-tools/git/tests/test-merge-pull-request.sh
    - docs/adrs/20260519000000_define_global_cli_output_contract.md
    - docs/github-rulesets.md
    - docs/issues/closed/2525-ci-gate-required-check/ISSUE.md
    - docs/issues/open/1843-migrate-git-hooks-scripts-from-bash-to-rust/ISSUE.md
    - docs/issues/closed/2022-vendor-and-document-maintainer-merge-workflow/ISSUE.md
    - docs/issues/closed/2175-merge-tool-symlink-exceptions/ISSUE.md
    - docs/issues/closed/2507-stop-requiring-pull-request-rebases/ISSUE.md
---

<!-- skill-link: create-issue -->

# EPIC #2640 - Port the maintainer merge tool to Rust

## Goal

Replace the vendored Python merge tool with a Rust tool that the project owns, first without changing what
it does, and then improve it: show the CI state of the head commit before the maintainer signs, and harden the
ACK rules. Every improvement, security ones included, is done after the migration.

## Why This Is Needed

The maintainer merges every pull request with `./contrib/dev-tools/git/merge-pull-request.sh`, a wrapper
that checks local preconditions and then runs `contrib/dev-tools/git/github-merge.py`. The Python tool was
vendored from Bitcoin Core under the MIT license in issue #2022 and has been changed here since: the
symbolic-link declaration (#2175), and the stale recorded-base guard, the refusal of a pull request that
targets another branch, and timeouts (#2507). It is about 820 lines. The maintainer plans to own it in Rust.

The tool builds a merge commit locally, shows it, lets the maintainer inspect it, and signs and pushes it
after two typed confirmations. Today it never looks at CI. The decision of 2026-10-22 in issue #2525 keeps CI
checks advisory: no status check is required, and the maintainer's signed ACK that names the head commit,
together with this merge flow, is the gate. A maintainer who merges a pull request whose checks are failing,
pending, or never reported gets no signal, and the tool counts an ACK from any commenter, not only from a
maintainer. The improvements of this EPIC close those two gaps, after the port.

### What the tool does today

Read from the source on 2026-10-23. The case list of the first subissue is what settles each point, so treat
this as the starting hypothesis, not as the specification.

- It reads its settings from `git config`: `githubmerge.repository` and `user.signingkey` are mandatory;
  `githubmerge.host`, `githubmerge.branch`, `githubmerge.merge-author-email`, `githubmerge.testcmd`,
  `githubmerge.pushmirrors`, and `user.ghtoken` are optional. `GIT`, `GH`, and `SHELL` can be overridden by
  environment variable.
- It reads the pull request, its comments, and its reviews over HTTP with Python's `urllib`, from a
  hardcoded `https://api.github.com/repos/<repository>/...`, sending `user.ghtoken` when it is set and none
  when it is not, and following the `Link` header for pages. It runs `gh api -X PATCH` only to refresh a stale
  recorded base. The HTTP reads have no timeout; the `gh` call has one of 60 seconds. Warnings of a failed
  read are printed on standard output.
- It fetches `refs/pull/<n>/*` and the target branch, refuses a pull request whose recorded base is stale
  (refreshing it first), builds an unsigned merge commit with a fixed message format, and refuses a merge that
  is not clean, that is already merged, or that introduces a symbolic link the declaration does not admit.
- It records the SHA-512 of the merged tree, lets the maintainer inspect it (a configured test command, or an
  interactive shell), and refuses to continue if the tree changed. Only when a test command is configured does
  it compare its merge with GitHub's merge, unless GitHub's merge is based on an older tip; with the
  interactive shell it does not compare.
- It collects ACKs from the comments and reviews of the pull request (any commenter; a line with `ACK` and
  the first six characters of the head hash, not quoted and not indented), and writes them and the tree hash
  into the merge commit message.
- It asks for `s` to sign (or `x` to reject), signs with `git commit --gpg-sign`, then asks for `push` (or
  `x`) before pushing. Its exit codes include 1, 3, 4, 5, 6, and 8.
- It deletes its temporary branches whether it succeeds or not.
- It loops forever when its input ends, so the wrapper refuses to start without an interactive terminal.

### How the code is laid out

A static analysis of the file on 2026-10-23 (function sizes, calls between functions, and whether a body does
input or output). It is shallow: a function that only calls another function that does input or output looks
pure to it, so the table below is grouped by hand.

| Cluster                | Functions                                                                                                     | Lines | Outside world        |
| ---------------------- | ------------------------------------------------------------------------------------------------------------- | ----- | -------------------- |
| Symbolic-link policy   | `check_symlinks`, `get_symlink_entries`, `read_symlink_declaration`, five declaration helpers, `quote_tree_value`, `symlink_declaration_path_error` | ~250  | git                  |
| ACKs                   | `get_acks_from_comments`, `make_acks_message`                                                                  | 25    | none                 |
| GitHub reads           | `get_response`, `retrieve_json` (pagination), `sanitize_ghdata`, `retrieve_pr_*`, `sanitize`                    | ~80   | HTTP                 |
| Recorded base          | `refresh_pr_base`, `require_current_recorded_base`                                                             | ~50   | `gh`                 |
| Tree hash              | `tree_sha512sum`                                                                                               | 48    | git                  |
| Settings and arguments | `git_config_get`, `parse_arguments`                                                                            | ~36   | git config           |
| Prompts and printing   | `ask_prompt`, `print_merge_details`                                                                            | 30    | standard input, output |
| Orchestration          | `main`                                                                                                         | 224   | everything           |

`main` is 27 percent of the file and holds the sequencing, the cleanup, the exit codes, and the push. The
existing symbolic-link suite (`test-github-merge-symlinks.py`, 51 tests, 7.5 seconds on 2026-10-23) passes
today, builds real git repositories with a bare upstream, and injects the GitHub data by replacing Python
functions in-process. The wrapper test (`test-merge-pull-request.sh`) has 13 scenarios for the wrapper's
preconditions.
Nothing tests the rest of the flow end to end.

## Scope

### In Scope

- A Rust binary, in a workspace crate, that does what the Python tool does, as identically as possible: the
  same interactive confirmations, the same signing, the same exit codes, and every setting, option, and
  message of the Python tool, including those that this repository does not use, because other maintainers may
  use them. It is written to stay close to the structure of the Python code.
- A list of cases, written as data, that settles what "the same" means, and a campaign that runs each case
  against both tools and compares them.
- Automating those cases, then refactoring the Rust code for testability, with a fast suite that uses test
  doubles.
- Moving the wrapper, the `merge-pull-request` skill, and the README to the Rust tool, with the Python tool
  kept as a fallback until the refactor is done and the EPIC owner decides that the Rust tool is ready to
  replace it, and then removed, keeping the MIT notice.
- Then, as separate subissues, after the migration: showing the head's CI state before the sign-off and
  requiring a distinct confirmation when it is not all green; hardening the ACK rules; the end-of-input and
  request timeout fixes; and the output-contract and exit-code decision. They are listed in "Improvements After
  the Migration".

### Out of Scope

- Required status checks, a bot or cron job that merges, and any change to a ruleset.
- Changing what the tool merges during the port: the merge commit format, the symbolic-link policy, and the
  stale-base guard stay as they are.
- Porting the git hooks (issue #1843), although the two ports may share structure.

## Strategy Decision

There are two separate problems: moving the code to Rust, and making the code good. The decision below is
about the first. Improvements to the second are deliberately kept out of the port, so that "the same" can be
checked.

### The three strategies

**Strategy 1: harness first.** Build a suite that runs against both tools, make it pass on the Python tool,
then write the Rust tool until it passes.

- Pros: the strongest evidence that the port is faithful, and the suite stays as a regression net.
- Cons: a large upfront cost before any Rust exists; the suite needs the real GitHub API (a fake cannot
  reproduce the recorded base, the asynchronous merge ref, or ruleset rejections), so it needs a token and
  cannot run on every pull request; and it can take many attempts at the end for the tool to pass it.
- Fits: a high-risk tool, or one with external users.

**Strategy 2: strangler slices.** Move the code in slices, with Python calling a hidden subcommand of the
Rust binary for each migrated slice, until no Python is left.

- Pros: the tool always works, each step is small and checked at function level, the existing 51-test suite
  keeps running against every step, and the Rust code can be refactored as it arrives. `cargo-mutants` is
  practical on small slices, and a shadow mode can run both implementations in real merges and log
  mismatches.
- Cons: the production Python is edited at every step; the wrapper must build the Rust binary before it runs
  the Python tool, with unmeasured build time and failure modes; the bridge and the shims are temporary code
  that must be removed; the behaviors at the process boundary (warnings printed on standard output, `None`
  versus an exception) must survive each hop; about seven pull requests of reviews; and `main`, 27 percent of
  the file and the riskiest part, is still one last step that needs the end-to-end harness of strategy 1.
- Fits: a tool that must never be down.

**Strategy 3: faithful port, then verify.** Regenerate the same code in Rust, keeping it as close to the
Python code as possible; run a full list of cases by hand against both tools in real repositories and compare
the output and the final state; then automate the cases; then refactor, add tests with test doubles, and
replace the old tool.

- Pros: the simplest and fastest way to a working Rust tool; the Python tool stays untouched as the oracle; no
  bridge, no build requirement for Python; the port is judged on the same cases that later become automatic
  tests.
- Cons: the parity evidence is manual, and a manual campaign is expensive to repeat after a fix; a port can
  make a guard silently vacuous, which nothing shows until the guard should have fired; the automation can be
  deferred and never happen; and a faithful transliteration is untidy Rust until the refactor.
- Fits: a small internal tool with a cheap rollback.

### Decision

The maintainer chose strategy 3 on 2026-10-23, after reasoning that the script is small, that it is an
internal tool used by maintainers, and that errors can be fixed immediately. The reasoning holds for crashes
and refusals, which are harmless, and for a wrong merge, which the maintainer's inspection and two
confirmations guard and which can be reverted by a new commit but not erased (the rulesets forbid a force push
and require signed commits). It holds less for a guard that silently stops guarding, which is the failure that
this EPIC treats as the main risk. Refinements recorded with the decision:

1. The list of cases is written as data from the start, even though it is run by hand first, so that the
   campaign can be repeated after a fix and the cases become automatic tests without being written again.
2. Every guard of the tool has a negative case that must refuse: the symbolic-link policy, the stale
   recorded base, the pull request that targets another branch, a merge that is not clean or already merged,
   a changed tree hash, a failing test command, and a merge that differs from GitHub's.
3. "The same" is defined before the campaign: normalized standard output, standard error, exit code, and the
   final git state are equal. The normalization removes what differs between two repositories: commit ids,
   dates, pull request numbers, and repository names. The accepted differences are decided up front: the
   `argparse` usage text and its abbreviation of options, and what happens when GitHub returns an unexpected
   shape (a Python traceback has no meaningful equivalent).
4. The port does not improve anything, security included. The end-of-input loop, the missing request timeout,
   and the weaknesses of the ACK rules are kept, written down in "Improvements After the Migration", and fixed
   afterwards in their own subissues.
5. Python stays available through the cutover and the refactor: the wrapper selects the tool, the default flips when parity is
   shown and the cases are automatic, and Python is removed only after the refactor, when the EPIC owner
   decides that the Rust tool is ready to replace it.
6. The cases are measured for completeness: the branches of the Python tool that no case executes are
   measured, with `coverage.py` in a throwaway virtual environment that is not added to the repository, and
   each is covered by a case or recorded as untested with a reason.

Each of the other two strategies is kept here as the way to go back if the campaign shows that this one is not
enough: if many cases differ, or a guard regression is found late, the case list already is the executable
specification that strategy 1 needs, and the rest of strategy 2 can be applied to the parts that remain.

## Subissues

Status values: `TODO`, `IN_PROGRESS`, `BLOCKED`, `DONE`.

| Order | Issue                                                                 | Local Spec                                  | Status | Notes                                                                                           |
| ----- | --------------------------------------------------------------------- | ------------------------------------------- | ------ | ----------------------------------------------------------------------------------------------- |
| 1     | #[To be assigned] - Executable case list and its completeness measure | `docs/issues/open/{number}-{slug}/ISSUE.md` | TODO   | Cases as data, with a negative case per guard; coverage of the Python tool by the cases         |
| 2     | #[To be assigned] - Faithful Rust port of the merge tool              | `docs/issues/open/{number}-{slug}/ISSUE.md` | TODO   | Can start in parallel with 1; crate, workspace member, license notice; no improvements          |
| 3     | #[To be assigned] - Manual parity campaign on two real repositories   | `docs/issues/open/{number}-{slug}/ISSUE.md` | TODO   | Needs 1 and 2; fixtures repositories in the test organization, created with explicit go-ahead   |
| 4     | #[To be assigned] - Automate the cases                                | `docs/issues/open/{number}-{slug}/ISSUE.md` | TODO   | After 3; one suite that runs either tool; opt-in for the cases that need the real GitHub API    |
| 5     | #[To be assigned] - Cutover with a fallback to the Python tool        | `docs/issues/open/{number}-{slug}/ISSUE.md` | TODO   | After 3 and 4; wrapper flag, documentation, skill; Python stays until after 6; it is removed when the EPIC owner decides the Rust tool is ready |
| 6     | #[To be assigned] - Refactor for testability, fast suite              | `docs/issues/open/{number}-{slug}/ISSUE.md` | TODO   | After 5 (decided 2026-10-24); test doubles for GitHub, mutation checks with `cargo-mutants`      |
| 7     | #[To be assigned] - Show the CI state before the sign-off             | `docs/issues/open/{number}-{slug}/ISSUE.md` | TODO   | After 6; verify the check-run semantics against real responses first                            |
| 8     | #[To be assigned] - Harden the ACK rules                              | `docs/issues/open/{number}-{slug}/ISSUE.md` | TODO   | After 6; the candidates are in "Improvements After the Migration"                               |
| 9     | #[To be assigned] - End-of-input and request-timeout fixes            | `docs/issues/open/{number}-{slug}/ISSUE.md` | TODO   | After 6; both are in "Improvements After the Migration"                                         |
| 10    | #[To be assigned] - Output-contract classification and exit codes     | `docs/issues/open/{number}-{slug}/ISSUE.md` | TODO   | A decision, possibly an ADR; the tool is interactive and fits neither class of the contract     |

Subissue specifications are drafted one at a time, after this EPIC is approved and created.

## Delivery Strategy

Each of the first subissues is its own pull request or several, so that increments merge small. The tool is
never in a half-migrated state: the Python tool is the production tool until the cutover.

For each subissue implementation in this EPIC, the default completion policy is:

1. Run automatic checks (`linter all`, relevant tests, pre-push checks when applicable).
2. Run manual verification scenarios and record evidence.
3. Re-review acceptance criteria after implementation and update verification evidence.
4. Complete an evidence-based implementation review. Create or update an
   issue-local retrospective for reusable lessons, material design changes, or
   meaningful deviations from the plan; otherwise record why one was unnecessary
   in the issue progress log.

### Phase 1: the cases

- Outcome: the case list as data, run against the Python tool with a measure of what it never executes.
- Exit criteria: every guard has a negative case; each unexecuted branch is covered or recorded as untested.

### Phase 2: the port and the campaign

- Outcome: the Rust tool, and the same cases run against both tools in two real repositories.
- Exit criteria: every case gives the same normalized transcript and final state, or the difference is
  recorded and accepted by the maintainer. The recorded sessions are kept as evidence.

### Phase 3: automation and cutover

- Outcome: the cases run automatically against either tool, and the Rust tool is the default of the wrapper
  with Python as a fallback.
- Exit criteria: the Rust tool is the default. The Python tool is removed later, after the refactor of phase 4,
  when the EPIC owner decides that the Rust tool is ready to replace it. The signing and
  the push are never run against a real remote by an automatic test; for real they stay with the maintainer.

### Phase 4: refactor and improve

- Outcome: the Rust code is testable with test doubles, and the improvements are delivered one by one.
- Exit criteria: each improvement has its tests, and `cargo-mutants` shows no surviving mutant in the logic that
  it changes, or each survivor is explained. Removing the Python tool is a separate action of the EPIC owner,
  taken after the refactor and tracked by AC4; it gets its own subissue when the owner decides.

### How the manual campaign runs

- **Two repositories.** For each case, the same scenario is created in two real repositories in the
  maintainer's test organization (`josecelano-test`), one for each tool, with the same commits, pull request,
  comments, and reviews. Both are public, so the tool's unauthenticated reads work, with a limit of 60 requests
  an hour, and the token is used where the case needs it. The maintainer creates them in the test organization;
  this specification does not. Suggested names: `merge-tool-fixtures-python` and `merge-tool-fixtures-rust`,
  with identical content, so each tool has its own repository. The wrapper refuses any repository other than
  `torrust/torrust-tracker`, so the campaign runs each tool directly, with the arguments that the wrapper
  passes (`--symlinks .symlinks.json <pull request> develop`), and `githubmerge.repository` set in a clone of
  the matching fixtures repository.
- **Recording.** Each run is recorded with its prompts and answers and normalized as above; the comparison is a
  diff of the two normalized transcripts and of the final git state, not a reading.
- **Setup as commands.** The commands that create each scenario are recorded in the evidence, so that the
  setup can be automated later.
- **What the campaign cannot show.** A timeout, a 5xx response, and malformed JSON cannot be produced on the
  real API; a refused connection might be reproduced with a plain proxy that refuses `CONNECT`, to be tried.
  The cases that need them are listed as not compared, and tested on the Rust side only.
- **Secrets.** The token comes from an environment variable and is never printed (the `handle-secrets` skill
  applies).
- **Signing and pushing.** Whether a throwaway key can be generated and used by a test, and whether git's
  `url.<base>.pushInsteadOf` gives a fetch from GitHub and a push to a local repository, is not verified and is
  settled in subissue 3 or 4. Without it, push cases use a fresh pull request per run.

## Improvements After the Migration

None of these is part of the port. The port keeps every one of the behaviors below as it is, so that "the same"
can be checked, and each is decided and done in its own subissue after the migration. The candidates were
found by reading the Python source on 2026-10-23; none was tested by running the tool, and each is a candidate
for the maintainer to decide, not a decision.

| Candidate                                          | What the source does today                                                                                                                                  | Subissue |
| -------------------------------------------------- | ----------------------------------------------------------------------------------------------------------------------------------------------------------- | -------- |
| ACK matches an abbreviated hash                    | `get_acks_from_comments` matches the first six characters of the head hash, 24 bits, on the reasoning that an accidental collision within a pull request is unlikely. A collision made on purpose is a different matter. The ACKs of both reviewers in the eight most recent merge commits on `develop` name the full hash | 8        |
| ACK matches the text `ACK` anywhere in a line      | A line counts when it contains `ACK` and the abbreviation and is not quoted or indented, so a line that says `NACK` or `do not ACK` followed by the hash would be listed as an ACK | 8        |
| ACK counts from any commenter                      | The author of a matching comment or review is not checked; only the login is recorded. A maintainer-only rule would use the `role_name` of the collaborator permission endpoint, because its `permission` field reports the Maintain role as `write` | 8        |
| A missing ACK does not stop the sign-off           | With no ACK the tool prints a red warning and writes "Top commit has no ACKs." in the message, then still asks to sign. Whether to require a distinct confirmation, as for a red check, is a decision | 8        |
| The last matching comment of a user replaces earlier ones | The ACKs are kept in a dictionary by login, so a later comment of the same user with the hash replaces an earlier one                                 | 8        |
| HTTP reads have no timeout                         | `get_response` opens the URL through `urllib` without a timeout, so a stalled connection hangs the tool; the `gh` call has one of 60 seconds                                  | 9        |
| End of input loops forever                         | `ask_prompt` reads a line and the callers loop until a valid answer, so closed input never ends; the wrapper refuses to start without a terminal              | 9        |
| The CI state of the head is not shown              | The tool never reads check runs or statuses                                                                                                                 | 7        |
| Exit codes and the output contract                 | The tool exits with 1, 3, 4, 5, 6, and 8, and the README and the `merge-pull-request` skill document code 4; the global contract has 0, 1, and 2, and fits neither of its classes | 10       |

The accepted differences of the port (the argument parser's usage text and abbreviation of options, and
unexpected GitHub shapes) are a different list: they are what a port in another language cannot keep.

## Progress Tracking

### Workflow Checkpoints

- [x] Epic spec drafted in `docs/issues/drafts/`
- [x] Epic spec reviewed and approved by user/maintainer
- [x] GitHub epic issue created and issue number added to this spec
- [ ] Subissues created and linked in this spec
- [ ] Subissue statuses kept up to date in the `Subissues` table
- [ ] For each implemented subissue: automatic checks completed and recorded
- [ ] For each implemented subissue: manual verification completed and recorded
- [ ] For each implemented subissue: acceptance criteria reviewed post-implementation
- [ ] For each implemented subissue: implementation completion review recorded
- [ ] Epic acceptance criteria reviewed and checked off
- [ ] Epic issue closed and spec moved from `docs/issues/open/` to `docs/issues/closed/`

### Progress Log

- 2026-10-23 17:24 UTC - GitHub Copilot (Claude Sonnet 5.5) - Rewrote the earlier single-issue draft as this EPIC, after the maintainer compared three strategies and chose a faithful port followed by a manual parity campaign (strategy 3), and asked that all three be recorded with their pros and cons. Facts about the tool were read from `github-merge.py`, its tests, and a static analysis of it on 2026-10-23; the existing symbolic-link suite was run and passed (51 tests). Not verified: that `pushInsteadOf` splits fetch and push as expected, that a throwaway signing key can be generated in a test, that a refused-`CONNECT` proxy reproduces a network failure, and the argument-parser behaviors named in the accepted differences. Nothing was created on GitHub or in the test organization.
- 2026-10-23 17:37 UTC - GitHub Copilot (Claude Sonnet 5.5) - Recorded the maintainer's answers: this section is the record of the strategy decision (no ADR); Python stays the fallback until the EPIC owner decides the Rust tool is ready; the maintainer creates the two fixtures repositories. Added suggested repository names, the note that the wrapper refuses any repository other than `torrust/torrust-tracker` so the campaign runs each tool directly, and a table of the tool's `git config` settings (read from the source and from the wrapper's call; the settings set in this clone were listed by name only, with no values printed).
- 2026-10-23 17:46 UTC - GitHub Copilot (Claude Sonnet 5.5) - Recorded the maintainer's two corrections. The port keeps every setting and option of the Python tool, including those this repository does not use, because other maintainers may use them, so the question of which settings are used was removed and the table is now an inventory the case list must cover. Every improvement, security ones included, is written down in a new section "Improvements After the Migration" and done only after the port; the port keeps the current behavior. The section lists nine candidates read from the source (among them the six-character hash match, the match on `ACK` as a substring, which would list a `NACK` line, ACKs from any commenter, and a missing ACK not stopping the sign-off); none was tested by running the tool. Subissue 8 was renamed to hardening the ACK rules.
- 2026-10-24 20:31 UTC - GitHub Copilot - Took the draft over from the session that wrote it, as part of closing #2525. The maintainer decided the one open question: the cutover comes before the refactor, and Python is removed only after the refactor, not at the cutover. The decision is recorded in "Decisions Recorded Before Creation", and the cutover subissue, phase 3, AC4, and the scope were adjusted to match. The maintainer approved the draft for creation.

## Acceptance Criteria

- [ ] All required subissues are created and linked.
- [ ] Implementation order is explicit and justified.
- [ ] Dependencies and blockers are documented and current.
- [ ] Epic status reflects actual state of linked subissues.
- [ ] Every completed subissue includes automated verification evidence.
- [ ] Every completed subissue includes manual verification evidence.
- [ ] Every completed subissue includes post-implementation acceptance criteria review.
- [ ] Every completed subissue includes an implementation completion review.
- [ ] Documentation and governance updates are included when required.
- [ ] AC1: Every guard of the tool has a negative case, and every branch of the Python tool that no case executes is covered or recorded as untested with a reason.
- [ ] AC2: Every case gave the same normalized transcript and final git state on both tools, or its difference is recorded and accepted by the maintainer.
- [ ] AC3: The cases run automatically against either tool, and the cases that need the real GitHub API are opt-in, with their output recorded as evidence.
- [ ] AC4: The Rust tool is the default of the wrapper, the documentation, the skill, and the README describe it, the MIT notice is kept, and the Python tool is removed after the refactor, once the EPIC owner decides that the Rust tool is ready to replace it.
- [ ] AC5: Each improvement is delivered in its own subissue with tests, and none was part of the port.

### Acceptance Verification

| AC ID | Status (`TODO`/`DONE`) | Evidence |
| ----- | ---------------------- | -------- |
| AC1   | TODO                   |          |
| AC2   | TODO                   |          |
| AC3   | TODO                   |          |
| AC4   | TODO                   |          |
| AC5   | TODO                   |          |

## Risks and Trade-offs

- **The tool signs and pushes to `develop`.** A wrong merge can be reverted but not erased. Mitigation: the
  Python tool stays as the fallback until after the refactor, the maintainer inspects every merge and confirms
  twice, and the port starts from the cases, not from the code.
- **A guard silently stops guarding.** The symbolic-link policy, the stale-base refusal, and the
  wrong-target refusal were added after real incidents, and nothing shows a vacuous port until the day they
  should fire. Mitigation: a negative case for every guard, in the case list.
- **Manual evidence is expensive to repeat.** After a fix, a manual campaign is not rerun. Mitigation: cases
  as data, recorded sessions, and setup as commands, so the same cases become automatic in subissue 4, before
  the cutover.
- **The automation never happens.** Mitigation: it is a subissue before the cutover, and the cutover depends
  on it.
- **A faithful port is untidy Rust.** Global state and early exits translate badly. Mitigation: that is
  accepted for the port, and fixed under tests in subissue 6; the port itself changes nothing else.
- **Real GitHub is slow, flaky, and needs credentials.** The cases that use it cannot run on pull requests
  from forks. Mitigation: they are opt-in, with their output recorded; the fast suite of subissue 6 takes
  over in CI. The recorded base is refreshed by GitHub on its own schedule, so that case records what
  happened and may need a retry.
- **Fixtures decay.** GitHub can change what its API returns, and the fixtures repositories are long-lived.
  Mitigation: a check of their expected state runs before the cases.
- **License.** The Python tool is MIT licensed from Bitcoin Core. A port of it is a derived work, and the MIT
  license requires keeping the copyright and permission notice. This is not legal advice; the maintainer
  decides how the notice is kept, and the cutover does not delete `github-merge-COPYING`.
- **Exit codes and the output contract.** The tool is interactive and fits neither class of the global
  contract, and its exit codes (1, 3, 4, 5, 6, 8) differ from the contract's. The port keeps them; subissue 10
  decides.
- **Reading CI state.** GitHub's check API has behaviors that are easy to get wrong: a later run of a job
  supersedes an earlier one, a skipped job leaves a skipped run, and a workflow skipped by a path filter
  reports nothing (evidence V9 and V10 of issue #2525). Subissue 7 verifies them against real responses.
- **A warning can be ignored.** The distinct phrase makes merging past a red check deliberate; it does not
  prevent it. This is the intent: the maintainer decides.

## Decisions Recorded Before Creation

1. Cutover before the refactor (decided by the maintainer on 2026-10-24). The cutover comes first, with Python
   as a fallback, and the refactor runs under the automatic cases of subissue 4. Python is removed only after
   the refactor, so a regression introduced by the refactor can still be bypassed.

### The tool's `git config` settings

The port keeps all of them, and `--repo-from` too, whether or not this repository or this maintainer uses them,
because other maintainers may. This is the inventory the port must cover, read from the source and from the
wrapper's call on 2026-10-23.

| Setting                          | What it does                                                                                                 | Note                                                      |
| -------------------------------- | ------------------------------------------------------------------------------------------------------------ | --------------------------------------------------------- |
| `githubmerge.repository`         | The repository to merge into; mandatory                                                                      | Pinned to `torrust/torrust-tracker` by the wrapper        |
| `user.signingkey`                | The key that signs the merge; mandatory                                                                       | Required by the wrapper                                   |
| `githubmerge.host`               | Where to fetch and push: `git@github.com` over SSH by default, HTTP(S) URLs when it starts with `http`       | The case list covers both forms                           |
| `user.ghtoken`                   | A token for the HTTP reads of the pull request                                                                | The case list covers set and not set                      |
| `githubmerge.testcmd`            | A command to run instead of an interactive shell; also turns on the comparison with GitHub's merge            | The case list covers set and not set                      |
| `githubmerge.merge-author-email` | The email on the signed merge commit                                                                          | The case list covers set and not set                      |
| `githubmerge.branch`             | The default target branch                                                                                     | The wrapper passes `develop` explicitly                   |
| `githubmerge.pushmirrors`        | Extra remotes to push to                                                                                      | Read only when the target branch is `master`              |

The `--repo-from` option is accepted only for the `master` branch. Because the campaign runs the tools
directly, the cases can exercise the settings and options that the wrapper never uses.

Decided by the maintainer on 2026-10-23:

- Strategy 3, and this EPIC section is the record of the decision; there is no separate ADR.
- The port keeps every setting, option, message, and exit code of the Python tool, including those that this
  repository does not use. Every improvement, security ones included, is written down in "Improvements After
  the Migration" and done only after it.
- The Python tool stays as the fallback until the EPIC owner decides that the Rust tool is ready to replace
  it; there is no fixed grace period.
- The maintainer creates the two fixtures repositories in the test organization, with the names suggested in
  the campaign section or their own.
- The port keeps an HTTP client for the GitHub reads, so the token and rate-limit behavior is unchanged, and
  using `gh` for every call is a later change.
- Real GitHub, not a fake, for the cases that need it; the Rust suite runs either tool through a command
  variable; coverage of the Python tool is measured with `coverage.py` in a throwaway virtual environment that
  is not added to the repository.

## References

- Related issues: #2525 (the decision that keeps checks advisory), #2022 (vendoring), #2175 (symbolic links),
  #2507 (stale base), #1843 (the hooks port)
- Related documents: [`docs/github-rulesets.md`](../../../github-rulesets.md),
  [`merge-pull-request` skill](../../../../.github/skills/dev/git-workflow/merge-pull-request/SKILL.md),
  [global CLI output contract](../../../adrs/20260519000000_define_global_cli_output_contract.md)
