---
doc-type: issue
issue-type: feature
status: draft
priority: p2
epic: null
github-issue: null
spec-path: docs/issues/drafts/merge-tool-symlink-exceptions/ISSUE.md
branch: "merge-tool-symlink-exceptions-spec"
related-pr: null
last-updated-utc: 2026-09-08 16:54
semantic-links:
  skill-links:
    - create-issue
  related-artifacts:
    - contrib/dev-tools/git/github-merge.py
    - contrib/dev-tools/git/merge-pull-request.sh
    - contrib/dev-tools/git/README-github-merge.md
    - contrib/dev-tools/git/tests/test-merge-pull-request.sh
    - .github/skills/dev/git-workflow/merge-pull-request/SKILL.md
    - docs/issues/closed/2022-vendor-and-document-maintainer-merge-workflow/ISSUE.md
    - docs/issues/open/2003-overhaul-guardrails-and-automation/EPIC.md
---

<!-- skill-link: create-issue -->

# Issue #[To be assigned] - Explicit Symlink Exceptions for the Vendored Maintainer Merge Tool

## Goal

Allow the vendored maintainer merge tool to accept symbolic links that a repository has declared explicitly, in an auditable root file that names each accepted link, its exact target, and the reason it exists, read from the merged tree the tool has just produced rather than from the maintainer's working directory, while every undeclared or mismatched link keeps failing exactly as it does today.

## Background

`contrib/dev-tools/git/github-merge.py` refuses to continue when the merged tree contains any symbolic link. `get_symlink_files()` (lines 151-157) runs `git ls-tree --full-tree -r HEAD` and collects every entry whose mode satisfies `(mode & 0o170000) == 0o120000`. Its caller (lines 393-397) prints `ERROR: File '<path>' was a symlink` once per collected entry and calls `sys.exit(4)` when the list is not empty. The check runs after the unsigned merge commit is created and before `tree_sha512sum()` computes the tree hash that the sign-off step later compares against, so a repository containing a symbolic link cannot reach signing at all.

The check itself is sound. A symbolic link in a merged tree is a way to make a reviewed path resolve somewhere else, and rejecting the whole class removes that from the merge path without asking the maintainer to judge each case. What the check lacks is any way to admit a link that a repository has chosen deliberately: the refusal is unconditional, so on such a repository every merge stops, and the only workaround available today is deleting the link.

The first repository this blocks is the sibling `torrust/torrust-index`. Its tree carries exactly one symbolic link, `.dockerignore`, whose literal content is `.containerignore`. The link was introduced deliberately in commit `01e7c701c9d8cb60` (2026-04-24) as part of its ADR-T-009 container-infrastructure work, because Docker reads only `.dockerignore` while Podman and Buildah prefer `.containerignore`; one link lets both toolchains share a single ignore list instead of two files that drift apart. That repository sets `githubmerge.repository` to `torrust/torrust-index` and runs the same tool; a merge run against its pull request 884 created the local merge commit and then stopped with `ERROR: File '.dockerignore' was a symlink`, leaving the tool's temporary `pull/884/local-merge` branch behind.

Where the exception is read from is part of the design rather than a detail of it. The tool has already created the local merge commit by the time the check runs: it checks out `pull/<n>/local-merge` (lines 366-368), changes into the repository root (lines 372-373), creates the unsigned merge commit with `git merge --commit --no-ff` (line 383), and confirms that `HEAD`'s subject is that merge (lines 388-391) before reaching `get_symlink_files()` at line 393, which lists the links with `git ls-tree --full-tree -r HEAD`. The merged tree is therefore available as a commit at exactly the point the decision is made, so the declaration can be read out of it with `git show HEAD:.symlinks.json`, or the equivalent `git cat-file` against the merge commit, and the links and the declaration that admits them come from one and the same tree.

Reading the declaration from anywhere else would move the decision out of the merge result. A file taken from the maintainer's working directory would let the maintainer's environment, rather than the reviewed merge outcome, decide what is admitted, and two maintainers merging the same pull request could reach different verdicts. Reading it from the merged tree instead makes the exception a reviewed, versioned part of the history the merge produces: whoever later inspects the merge commit sees both the link and the declaration that admitted it. It also closes the case where a pull request introduces a symbolic link that a stale declaration, or one present only on the base branch, would silently admit; a link added by the pull request is only ever exempt if the merged result also carries a declaration covering it, which means the declaration went through review too.

The tracker's own tree contains no symbolic links today: `git ls-files -s | awk '$1=="120000"'` returns nothing. This change therefore alters no tracker merge until the tracker itself declares a link, and it stays a no-op for any repository that ships no declaration file. The tracker is where the tool, the `merge-pull-request.sh` wrapper, the wrapper test suite, and the provenance record live, so the tool-side fix belongs here and is mirrored into other repositories that adopt the same workflow.

## Scope

### In Scope

- Define a repository-level declaration file, `.symlinks.json` at the repository root, that lists each accepted symbolic link with its exact target and the reason it exists.
- Read that declaration out of the merged tree, from the local merge commit the tool has just created, so the links and the file that admits them come from the same tree.
- Define the matching and rejection rules the merge tool applies to that file, including the target forms that are never accepted.
- Add an optional `--symlinks <path>` argument to the vendored merge tool that names a repository-relative path inside the merged tree, defaulting to `.symlinks.json`, and exempts exactly the matching links from the refusal.
- Print every accepted link, with its path, target, and reason, into the merge output so the maintainer sees what was admitted before signing.
- Teach `contrib/dev-tools/git/merge-pull-request.sh` to pass `--symlinks .symlinks.json` unconditionally, so maintainers never type the argument and the wrapper makes no decision of its own.
- Extend `contrib/dev-tools/git/tests/test-merge-pull-request.sh` to cover the wrapper's delegation of the tree path.
- Rewrite the provenance section of `contrib/dev-tools/git/README-github-merge.md` to the single provenance statement described in AD1, dropping the local-change policy, the new-hash requirement, and the byte-identity wording.
- Note the new behavior in the `merge-pull-request` skill.

### Out of Scope

- Linter enforcement of `.symlinks.json`: no schema check, no drift check between the file and the tree, and no `linter all` step that reads it. The merge tool is the only consumer in this issue.
- Reading the declaration from any source other than the merged tree. The working directory, the base branch, the pull-request branch, an environment variable, and a path outside the repository are all excluded by design, not left open for a later option.
- Changing the check's semantics for links that are not declared. An undeclared link, a declared link whose target does not match, and a target of a form the rules reject all keep failing with the current message and exit code.
- Any symbolic-link policy for the tracker's own tree. This issue does not add a link to this repository or decide whether one should ever be added.
- Fixing the known upstream issues the README records. AD1 makes them ordinarily fixable in the tool rather than only recordable, but they are separate defects with their own review; this issue only re-checks how the README describes them.
- Mirroring the change into `torrust/torrust-index` or any other repository. That is separate work in those repositories.

## Architectural Decisions

- Related ADRs: None known in `docs/adrs/`.
- ADRs to create: None. AD1 is resolved below, and it settles how one repository file is maintained rather than a project architecture or design pattern.

### AD1 - The merge tool is maintained here, not tracked against upstream (RESOLVED)

The refusal lives inside `github-merge.py`, before the tree hash is taken, and no wrapper argument or environment setting reaches it, so this feature cannot be confined to `merge-pull-request.sh` the way the current README asks repository-specific behavior to be. The decision is therefore about what the vendored file is: a mirror that must stay byte-identical to Bitcoin Core, or an ordinary repository file that happens to have come from there. It is the latter, and this issue implements it that way.

The vendored copy becomes a normal tool in this repository, modified under ordinary review like any other file. `README-github-merge.md` keeps exactly one provenance statement: the file arrived from the Bitcoin Core developers' `github-merge.py` (MIT, retained in `github-merge-COPYING`), with SHA-256 `e390eb014131f3183a2cba642134974a6b09b19a65322d17dd7c81cf4ffbaad2`, in commit `833a4160e5753d54cde47bcc4bed25df7a04c0f6`, "feat(git): vendor maintainer merge workflow" (2026-07-23). That statement is fixed and does not change again. What goes away is the machinery around it: no re-hashing after each change, no divergence log, no byte-identity requirement, no re-vendoring plan.

The reason is what a provenance record is for. A reader of a vendored file has one question, where this came from and what it looked like when it arrived, and a single dated statement naming the origin, the license, the original hash, and the introducing commit answers it completely and permanently. Byte-identity plus per-change re-hashing answers a different question, whether the copy still matches upstream, and that is only worth its cost if the copy is meant to track upstream. This one is not. Its consumers are Torrust repositories, whose needs Bitcoin Core does not share, and this issue is the first of those needs. Keeping a tracking discipline for a file nobody intends to re-sync would tax every future change while guaranteeing nothing a reader benefits from; git history already records what changed after arrival, more precisely than a hash in a README.

Offering the feature to Bitcoin Core first was considered and declined. It would preserve a byte-identity that nothing depends on once the copy is no longer tracked, and its schedule is not ours: upstream may reasonably decline a feature its own repository does not need, review may take arbitrarily long, and `torrust/torrust-index` stays blocked throughout. Nothing prevents contributing the idea upstream later on its own merits; it is simply not a precondition for fixing a tool this project maintains.

## Design and Ownership Review

This work adds no child process, no asynchronous I/O, no network readiness step, no new resource with a drop path, and no reusable test fixture beyond the temporary repositories the existing wrapper test suite already builds, so the deadline and lifetime concerns this section normally covers do not arise. The responsibility split is the part worth fixing before implementation:

- `.symlinks.json` carries repository policy only. It states which links a repository accepts and why; it grants no capability by itself and is inert in a repository whose tooling does not read it.
- `github-merge.py` owns enforcement, and it owns reading. It reads both the links and the declaration out of the merged tree at `HEAD`, decides which links are exempt, prints what it admitted, and keeps its current refusal and exit code for everything else. It never reads the declaration from the filesystem, never writes it, and never infers an exemption that the file does not state.
- `merge-pull-request.sh` owns the repository-specific wiring only: it passes the repository-relative tree path the tracker uses. It does not check whether that file exists, because existence is a property of the merged tree that the wrapper cannot observe before the merge commit is created; it performs no parsing and applies no policy of its own.
- The direction of matching is from the tree to the declaration, never the reverse. The tool walks the links actually present in the merged tree and asks whether each one is declared; a declaration entry can only ever remove a refusal for a link that exists, and can never introduce one.
- Both inputs come from one commit. Because the links and the declaration are read from the same `HEAD`, there is no window in which they can disagree, and no state outside the merge result can change the verdict.

## Implementation Plan

Status values: `TODO`, `IN_PROGRESS`, `BLOCKED`, `DONE`.

| ID  | Status | Task                                          | Notes / Expected Output                                                                                                                                                                                                                                       |
| --- | ------ | --------------------------------------------- | --------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| T1  | TODO   | Define the declaration format and its rules   | The `.symlinks.json` shape, field meanings, shared namespace, and the accept/reject rules below are written into `README-github-merge.md`.                                                                                                                     |
| T2  | TODO   | Add `--symlinks` to the vendored tool         | Optional argument next to `--repo-from` in `parse_arguments()` (lines 260-280), taking a repository-relative tree path that defaults to `.symlinks.json`; the caller (lines 393-397) reads it from the merge commit with `git show HEAD:<path>`, exempts matching links, and prints each accepted path, target, and reason. The tool is edited directly under AD1.                                                        |
| T3  | TODO   | Pass the tree path from the wrapper           | `merge-pull-request.sh` inserts `--symlinks .symlinks.json` before the positional arguments of its final `exec python3` call (line 136), unconditionally and without a filesystem check, because the path names a location in the merged tree.                    |
| T4  | TODO   | Cover the behavior with tests                 | Wrapper delegation of the tree path; a declared link merges past the check; an undeclared link, a declared link with a different target, a target containing `..`, and an absolute target each keep the existing message and exit code `4`; a declaration present only in the working directory, or only on the base branch and not in the merged result, exempts nothing; a declaration in the merged tree whose values disagree with that tree's links refuses. |
| T5  | TODO   | Rewrite the provenance record                 | `README-github-merge.md` carries the single provenance statement from AD1; the local-change policy, the new-hash requirement, and the byte-identical wording are removed; the known-upstream-issues section is re-checked, since those items can now be fixed in the tool directly in a follow-up rather than only recorded. |
| T6  | TODO   | Note the behavior in the merge skill          | `.github/skills/dev/git-workflow/merge-pull-request/SKILL.md` explains that the declaration is read from the merged result rather than the working copy, what the accepted-link output looks like, and that an undeclared link is still a hard stop.             |

### Declaration format

```json
{
  "namespace": "com.torrust.repository.symlinks",
  "version": [1, 0, 0],
  "symlinks": [
    {
      "path": ".dockerignore",
      "target": ".containerignore",
      "reason": "Docker reads only .dockerignore, while Podman and Buildah prefer .containerignore. One link keeps a single ignore list for both toolchains."
    }
  ]
}
```

The `namespace` identifies the declaration format, not the repository that carries it, so every Torrust repository uses the same value, `com.torrust.repository.symlinks`. Binding a declaration to its repository is already done by tree sourcing: the file is read out of that repository's own merged tree, so it cannot be borrowed from elsewhere no matter what string it contains. A per-repository namespace would add no guarantee on top of that and would oblige every consumer to teach the tool a string of its own. The `version` is the declaration-format version, not the repository version.

### Rules

- The declaration is read from the merged tree, out of the local merge commit the tool has just created, with `git show HEAD:.symlinks.json` or the equivalent `git cat-file` against that commit. It is never read from the working directory, the index, the base branch, or any path outside the repository.
- `--symlinks <path>` names a repository-relative path inside that tree, defaulting to `.symlinks.json`. It is a tree path, not a filesystem path, so an absolute path or a path escaping the repository is not a usable argument.
- A declaration that exists only in the maintainer's working tree, only on the base branch, or only on the pull-request branch without reaching the merged result exempts nothing. Only what the merge produced counts.
- `path` is repository-relative and must name a symbolic link in the merged tree.
- `target` must equal the link's literal content in the merged tree byte for byte. A target that resolves to the same file by another spelling does not match.
- A target that is absolute, or that contains a `..` segment, is never accepted, whatever the declaration file says. This is a property of the target itself, not of the declaration, so no file can grant it.
- A symbolic link present in the merged tree but absent from the declaration file still produces `ERROR: File '<path>' was a symlink` and exit code `4`.
- A symbolic link present in the merged tree and declared with a different target is treated as undeclared and produces the same message and exit code. Any divergence between the declaration in the merged tree and the links in the merged tree refuses exactly as today.
- A declaration entry whose path is not a symbolic link in the merged tree exempts nothing, because matching runs from the tree to the file. Such a stale entry is reported in the merge output so it can be removed.
- A declaration file absent from the merged tree means no exceptions: the tool behaves exactly as it does today. So does running the tool without `--symlinks`.

## Commit Points

| Task | Coherent change set                                                                         | Commit policy                                                                                        |
| ---- | ------------------------------------------------------------------------------------------- | ---------------------------------------------------------------------------------------------------- |
| T1   | Declaration format and rules documented in the vendoring README.                            | Commit after review of the format and rules, before any tool change.                                 |
| T2   | The `--symlinks` argument, tree-sourced reading, the exemption logic, and the accepted-link output. | Commit after focused validation.                                                              |
| T3   | Wrapper delegation of the declaration tree path.                                            | Commit separately from the tool change; it is independently reviewable and independently revertible. |
| T4   | Test increments for delegation, acceptance, and each rejection case.                        | Commit each reviewed test increment before starting the next behavior area.                          |
| T5   | Provenance rewrite and known-issues re-check in the vendoring README.                       | Commit with, or immediately after, the tool change whose maintenance model it states.                |
| T6   | Skill note describing the workflow-visible behavior.                                        | Commit after the behavior it documents is merged or staged in the same branch.                       |

Record a justified no-change decision in the task's evidence without creating an empty commit. Use a Conventional Commit message with the narrow affected scope, and sign every commit with GPG.

## Progress Tracking

### Workflow Checkpoints

- [ ] Folder-style spec drafted in `docs/issues/drafts/merge-tool-symlink-exceptions/ISSUE.md`
- [ ] Spec reviewed and approved by user/maintainer
- [x] AD1 resolved and recorded in this specification
- [ ] GitHub issue created and issue number added to this spec
- [ ] (Optional, recommended for complex issues) Spec-only PR merged into `develop` before implementation
- [ ] Implementation completed
- [ ] Automatic verification completed (`linter all`, wrapper test suite, and any pre-push checks)
- [ ] Manual verification scenarios executed and recorded (status + evidence)
- [ ] Acceptance criteria reviewed after implementation and updated with evidence
- [ ] Evidence-based implementation completion review recorded: issue-local retrospective created for material discoveries, or progress log states why none was needed
- [ ] Reviewer validated acceptance criteria and updated checkboxes
- [ ] Independent reviewer reports recorded in issue-local `agent-review-reports.md` when reviewers received this folder-style specification
- [ ] Committer verified spec progress is up to date before commit
- [ ] Issue closed and spec moved from `docs/issues/open/` to `docs/issues/closed/`

### Progress Log

- 2026-09-08 16:08 UTC - Spec author - Drafted this unnumbered specification after verifying the refusal path in the vendored tool, the vendoring policy in the README, the wrapper's invocation, and the absence of symbolic links in the tracker tree - This spec
- 2026-09-08 16:20 UTC - Spec author - Amended the design to read the declaration from the merged tree rather than the filesystem, after verifying that the local merge commit already exists at the check site (lines 366-397), and propagated the consequence through scope, ownership, plan, rules, criteria, scenarios, and risks - This spec
- 2026-09-08 16:54 UTC - Spec author - Shared one declaration namespace across Torrust repositories, since tree sourcing already binds a declaration to its repository, and resolved AD1 toward maintaining the tool here under a single fixed provenance statement, after confirming that the file introduced in `833a4160e` still hashes to the recorded SHA-256 and has not been modified since - This spec

## Acceptance Criteria

- [ ] AC1: `.symlinks.json` has a documented shape and a documented rule set, including the target forms that are never accepted.
- [ ] AC2: The tool reads the declaration from the merged tree, out of the local merge commit it has just created, and from no other source. A declaration present only in the working directory, only on the base branch, or only on the pull-request branch without reaching the merged result exempts nothing.
- [ ] AC3: `github-merge.py` accepts `--symlinks <path>` as a repository-relative path inside the merged tree, defaulting to `.symlinks.json`, and exempts exactly the declared links whose target matches that tree's link content byte for byte.
- [ ] AC4: Every accepted link is printed with its path, target, and reason before the maintainer is asked to sign.
- [ ] AC5: An undeclared link, a link declared with a different target, a declared absolute target, and a declared target containing a `..` segment each keep the message `ERROR: File '<path>' was a symlink` and exit code `4`.
- [ ] AC6: With no declaration file in the merged tree, and with no `--symlinks` argument, the tool behaves exactly as it does today.
- [ ] AC7: `merge-pull-request.sh` passes `--symlinks .symlinks.json` unconditionally and performs no filesystem check for that file.
- [ ] AC8: `README-github-merge.md` carries the single provenance statement, naming the origin, the license, the original SHA-256, and the introducing commit, and no longer requires re-hashing or byte-identity.
- [ ] `linter all` exits with code `0`
- [ ] Relevant tests pass
- [ ] Manual verification scenarios are executed and documented (status + evidence)
- [ ] Acceptance criteria are re-reviewed after implementation and reflect actual behavior
- [ ] Documentation is updated when behavior/workflow changes

## Verification Plan

Define verification before implementation starts and execute it before closing the issue.

### Automatic Checks

- `bash contrib/dev-tools/git/tests/test-merge-pull-request.sh`
- `linter all`
- Pre-push checks (when applicable)

### Manual Verification Scenarios

Status values: `TODO`, `IN_PROGRESS`, `DONE`, `FAILED`, `BLOCKED`.

| ID  | Scenario                                | Command/Steps                                                                                                                                                        | Expected Result                                                                                                            | Status | Evidence                              |
| --- | --------------------------------------- | -------------------------------------------------------------------------------------------------------------------------------------------------------------------- | -------------------------------------------------------------------------------------------------------------------------- | ------ | ------------------------------------- |
| M1  | Declared link is accepted               | Build a fixture repository whose merged result commits both a symbolic link and a `.symlinks.json` declaring it with the matching target; run the merge tool with `--symlinks .symlinks.json`. | The tool prints the accepted path, target, and reason, and continues past the check to compute the tree hash.               | TODO   | {captured merge output}               |
| M2  | Undeclared link is refused              | Remove the entry from the fixture's committed `.symlinks.json`, commit that change into the merged result, and repeat the run.                                        | `ERROR: File '<path>' was a symlink` and exit code `4`.                                                                     | TODO   | {captured output and `echo $?`}       |
| M3  | Escaping targets are refused when declared | In the fixture, point the link at an absolute path, then at a path containing a `..` segment, committing a matching declaration alongside the link in each case.       | Both runs refuse with the same message and exit code `4`, showing that the declaration cannot grant an escaping target.     | TODO   | {captured output for both variants}   |
| M4  | Missing declaration file changes nothing   | Commit the fixture without `.symlinks.json` in the merged result and run through `merge-pull-request.sh`, then separately run the tool with no `--symlinks` argument.  | Both runs reproduce today's refusal for the link that is present.                                                          | TODO   | {captured output for both runs}       |
| M5  | Out-of-tree declaration exempts nothing | Write a valid `.symlinks.json` into the fixture's working directory without committing it, and separately commit one on the base branch only so the merged result does not carry it; run the merge in both cases. | Both runs refuse with `ERROR: File '<path>' was a symlink` and exit code `4`, showing that only the merged tree is consulted. | TODO   | {captured output for both variants}   |

Notes:

- Manual verification is mandatory even when automated tests pass.
- If a scenario fails, record the failure and diagnosis in the progress log before proceeding.

### Acceptance Verification

| AC ID | Status (`TODO`/`DONE`) | Evidence           |
| ----- | ---------------------- | ------------------ |
| AC1   | TODO                   | {test/log/PR link} |
| AC2   | TODO                   | {test/log/PR link} |
| AC3   | TODO                   | {test/log/PR link} |
| AC4   | TODO                   | {test/log/PR link} |
| AC5   | TODO                   | {test/log/PR link} |
| AC6   | TODO                   | {test/log/PR link} |
| AC7   | TODO                   | {test/log/PR link} |
| AC8   | TODO                   | {test/log/PR link} |

## Risks and Trade-offs

- **The exception could weaken a security check.** Mitigation is in the rule set rather than in review discipline: an exception is explicit, so nothing is admitted that the repository has not written down; it is tree-sourced, so the exception is part of the reviewed merge result and no state in the maintainer's environment can change the verdict; it is byte-exact, so a link that changes where it points stops matching and is refused again; it is path-restricted, because absolute targets and targets containing `..` are refused whatever the file says, which keeps the exception inside the repository; and it is printed, so the maintainer sees every admitted link before signing rather than after. The residual risk is a repository declaring a link it should not have; that risk already exists for every other file the repository commits, and the declaration makes it visible in review instead of invisible.
- **The tool is now this project's to maintain.** Under AD1 the file is modified here under ordinary review, so the effort of understanding and fixing it falls to this project rather than to upstream. That is the deliberate result, not a side effect: the alternative was leaving a needed fix to a schedule nobody here controls. The residual risk is narrower, that a later reader loses sight of where the file came from and mistakes it for original work; mitigation is the permanent provenance statement naming the origin, the license, the original hash, and the introducing commit, kept next to the retained upstream source header and `github-merge-COPYING`.
- **The tracker and the sibling copies can drift.** Once other repositories adopt the tool, each holds its own copy of the argument and its own declaration file, and a fix applied in one is easy to forget in the others. Mitigation is to keep the tracker's copy the reference, state that in the vendoring README, and mirror deliberately rather than by editing each copy independently.
- **A declaration file can go stale.** A link can be removed while its entry remains. Reading from the merged tree shrinks this risk rather than only mitigating it: the declaration cannot lag the tree it is compared against, because both come from the same commit, so the only staleness left is an entry that the merge result itself still carries after its link is gone. Mitigation for that remainder is the stale-entry report in the merge output; enforcing the file against the tree in the linter is deliberately left out of this issue and can follow once the format has settled.

## Implementation Completion Review

After implementation, compare the result with this specification. Record invalidated assumptions, material design changes, unexpected validation findings, and reusable lessons.

- Retrospective: `Not yet assessed`
- If needed, create `implementation-retrospective.md` from the repository template at `docs/templates/IMPLEMENTATION-RETROSPECTIVE.md` in this issue specification's directory.
- If no retrospective is needed, add a concise progress-log entry explaining why the work had no material discovery.
- When an independent reviewer receives this folder-style specification, it records its result in `agent-review-reports.md` using `docs/templates/AGENT-REVIEW-REPORTS.md`.

## References

- Related issues: #2022 vendored and documented the maintainer merge workflow that this issue extends; #2003 is the automation umbrella EPIC that the workflow reports to. Neither is a parent of this issue.
- Related PRs: none yet.
- Related ADRs: `docs/adrs/` holds none for this area. The sibling repository's container-infrastructure decision, ADR-T-009 (`adr/009-container-infrastructure-refactor.md` in `torrust/torrust-index`), is the reason its tree carries the first link this issue has to admit.
- Vendoring policy and provenance: [`contrib/dev-tools/git/README-github-merge.md`](../../../../contrib/dev-tools/git/README-github-merge.md)
- Merge workflow: [`merge-pull-request` skill](../../../../.github/skills/dev/git-workflow/merge-pull-request/SKILL.md)
- Vendoring specification: [issue #2022 specification](../../closed/2022-vendor-and-document-maintainer-merge-workflow/ISSUE.md)
- Automation umbrella: [EPIC #2003](../../open/2003-overhaul-guardrails-and-automation/EPIC.md)
- Blocked merge in the sibling repository: <https://github.com/torrust/torrust-index/pull/884>
