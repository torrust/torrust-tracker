---
doc-type: issue
issue-type: feature
status: open
priority: p2
epic: null
github-issue: 2175
spec-path: docs/issues/open/2175-merge-tool-symlink-exceptions/ISSUE.md
branch: "2175-merge-tool-symlink-exceptions"
related-pr: null
last-updated-utc: 2026-09-09 08:20
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

# Issue #2175 - Explicit Symlink Exceptions for the Vendored Maintainer Merge Tool

## Goal

Allow the vendored maintainer merge tool to accept symbolic links that a repository has declared explicitly, in an auditable root file that names each accepted link, its exact target, and the reason it exists, read from the merged tree the tool has just produced rather than from the maintainer's working directory, and applied to the links in every commit the merge brings in rather than to the final tree alone, while every undeclared or mismatched link keeps failing exactly as it does today.

## Background

`contrib/dev-tools/git/github-merge.py` refuses to continue when the merged tree contains any symbolic link. `get_symlink_files()` (lines 151-157) runs `git ls-tree --full-tree -r HEAD` and collects every entry whose mode satisfies `(mode & 0o170000) == 0o120000`. Its caller (lines 393-397) prints `ERROR: File '<path>' was a symlink` once per collected entry and calls `sys.exit(4)` when the list is not empty. The check runs after the unsigned merge commit is created and before `tree_sha512sum()` computes the tree hash that the sign-off step later compares against, so a repository containing a symbolic link cannot reach signing at all.

The check itself is sound. A symbolic link in a merged tree is a way to make a reviewed path resolve somewhere else, and rejecting the whole class removes that from the merge path without asking the maintainer to judge each case. What the check lacks is any way to admit a link that a repository has chosen deliberately: the refusal is unconditional, so on such a repository every merge stops, and the only workaround available today is deleting the link.

The first repository this blocks is the sibling `torrust/torrust-index`. Its tree carries exactly one symbolic link, `.dockerignore`, whose literal content is `.containerignore`. The link was introduced deliberately in commit `01e7c701c9d8cb60` (2026-04-24) as part of its ADR-T-009 container-infrastructure work, because Docker reads only `.dockerignore` while Podman and Buildah prefer `.containerignore`; one link lets both toolchains share a single ignore list instead of two files that drift apart. That repository sets `githubmerge.repository` to `torrust/torrust-index` and runs the same tool; a merge run against its pull request 884 created the local merge commit and then stopped with `ERROR: File '.dockerignore' was a symlink`, leaving the tool's temporary `pull/884/local-merge` branch behind.

Where the exception is read from is part of the design rather than a detail of it. The tool has already created the local merge commit by the time the check runs: it checks out `pull/<n>/local-merge` (lines 366-368), changes into the repository root (lines 372-373), creates the unsigned merge commit with `git merge --commit --no-ff` (line 383), and confirms that `HEAD`'s subject is that merge (lines 388-391) before reaching `get_symlink_files()` at line 393, which lists the links with `git ls-tree --full-tree -r HEAD`. The merged tree is therefore available as a commit at exactly the point the decision is made, so the declaration can be read out of it with `git show HEAD:<path>`, or the equivalent `git cat-file` against the merge commit, and the links and the declaration that admits them come from one and the same tree.

Reading the declaration from anywhere else would move the decision out of the merge result. A file taken from the maintainer's working directory would let the maintainer's environment, rather than the reviewed merge outcome, decide what is admitted, and two maintainers merging the same pull request could reach different verdicts. Reading it from the merged tree instead makes the exception a reviewed, versioned part of the history the merge produces: whoever later inspects the merge commit sees both the link and the declaration that admitted it. It also closes the case where a pull request introduces a symbolic link that a stale declaration, or one present only on the base branch, would silently admit; a link added by the pull request is only ever exempt if the merged result also carries a declaration covering it, which means the declaration went through review too.

Checking only the final tree would also be too narrow, because the merge publishes history, not just a tip. Every commit a merge brings in is a checkout target from then on: a link that appears in an intermediate commit and disappears again before the tip passes a tip-only check while still resolving somewhere on every checkout of the commit that carries it, and on every archive, bisect, or review of that commit. The range the merge introduces is already known to the tool at the check site. Both endpoints are ordinary branches by then: a single `git fetch` creates `pull/<n>/head` from `refs/pull/<n>/*` and `pull/<n>/base` from the target branch (lines 349-350), the head branch is verified to exist immediately afterwards (lines 355-357), and the tool itself already enumerates `pull/<n>/base..pull/<n>/head` with `git log --no-merges --topo-order` to build the merge-commit message body (line 380). Listing each of those commits' links with `git ls-tree --full-tree -r <commit>` costs one more walk over refs the tool has already fetched.

The manifest that judges them is the one in the final merged tree, and only that one. A declaration that changed part-way through the range would let earlier commits be admitted by a statement the merge result no longer stands behind, which is the same defect as reading the file from outside the tree, moved inside history. Making the final manifest the single authority over the whole range keeps one reviewed declaration answerable for everything the merge admits, and leaves the maintainer one file to read before signing rather than one per commit.

The tracker's own tree contains no symbolic links today: `git ls-files -s | awk '$1=="120000"'` returns nothing. This change therefore alters no tracker merge until the tracker itself declares a link, and it stays a no-op for any repository that ships no declaration file, as well as for any invocation that does not name one. The tracker is where the tool, the `merge-pull-request.sh` wrapper, the wrapper test suite, and the provenance record live, so the tool-side fix belongs here and is mirrored into other repositories that adopt the same workflow.

## Scope

### In Scope

- Define a repository-level declaration file, which Torrust repositories place at `.symlinks.json` in the repository root, that lists each accepted symbolic link with its exact target and the reason it exists. The name is a repository convention supplied by the caller, not a path the tool knows.
- Read that declaration out of the merged tree, from the local merge commit the tool has just created, so the links and the file that admits them come from the same tree.
- Check the links in every commit the merge introduces, the pull request's own commits in `pull/<n>/base..pull/<n>/head` plus the local merge commit, against that single final declaration, and report the carrying commit when one refuses.
- Define the matching and rejection rules the merge tool applies to that file, including the target forms that are never accepted.
- Add an optional `--symlinks <path>` argument to the vendored merge tool that names a repository-relative path inside the merged tree and exempts exactly the matching links from the refusal. The argument carries no default inside the tool: a run without it performs no declaration processing at all and keeps today's unconditional refusal.
- Print every accepted link, with its path, target, and reason, into the merge output so the maintainer sees what was admitted before signing.
- Teach `contrib/dev-tools/git/merge-pull-request.sh` to pass `--symlinks .symlinks.json` unconditionally, so maintainers never type the argument and the repository's chosen declaration path is stated in exactly one visible place. The wrapper still reads nothing, parses nothing, and applies no policy of its own.
- Extend `contrib/dev-tools/git/tests/test-merge-pull-request.sh` to cover the wrapper's delegation of the tree path.
- Rewrite the provenance section of `contrib/dev-tools/git/README-github-merge.md` to the single provenance statement described in AD1, dropping the local-change policy, the new-hash requirement, and the byte-identity wording.
- Note the new behavior in the `merge-pull-request` skill.

### Out of Scope

- Linter enforcement of `.symlinks.json`: no schema check, no drift check between the file and the tree, and no `linter all` step that reads it. The merge tool is the only consumer in this issue.
- Reading the declaration from any source other than the merged tree. The working directory, the base branch, the pull-request branch, an environment variable, and a path outside the repository are all excluded by design, not left open for a later option.
- Changing the check's semantics for links that are not declared. An undeclared link, a declared link whose target does not match, and a target of a form the rules reject all keep failing with the current message and exit code.
- Re-checking history that the merge does not introduce. Commits already reachable from the base branch are not part of the merge and are not walked; this issue adds no retroactive check over existing history.
- Any symbolic-link policy for the tracker's own tree. This issue does not add a link to this repository or decide whether one should ever be added.
- Fixing the known upstream issues the README records. AD1 makes them ordinarily fixable in the tool rather than only recordable, but they are separate defects with their own review; this issue only re-checks how the README describes them.
- Mirroring the change into `torrust/torrust-index` or any other repository. That is separate work in those repositories.

## Architectural Decisions

- Related ADRs: None known in `docs/adrs/`.
- ADRs to create: None. AD1 and AD2 are resolved below; each settles how one repository file is maintained or invoked rather than a project architecture or design pattern.

### AD1 - The merge tool is maintained here, not tracked against upstream (RESOLVED)

The refusal lives inside `github-merge.py`, before the tree hash is taken, and no wrapper argument or environment setting reaches it, so this feature cannot be confined to `merge-pull-request.sh` the way the current README asks repository-specific behavior to be. The decision is therefore about what the vendored file is: a mirror that must stay byte-identical to Bitcoin Core, or an ordinary repository file that happens to have come from there. It is the latter, and this issue implements it that way.

The vendored copy becomes a normal tool in this repository, modified under ordinary review like any other file. `README-github-merge.md` keeps exactly one provenance statement: the file arrived from the Bitcoin Core developers' `github-merge.py` (MIT, retained in `github-merge-COPYING`), with SHA-256 `e390eb014131f3183a2cba642134974a6b09b19a65322d17dd7c81cf4ffbaad2`, in commit `833a4160e5753d54cde47bcc4bed25df7a04c0f6`, "feat(git): vendor maintainer merge workflow" (2026-07-23). That statement is fixed and does not change again. What goes away is the machinery around it: no re-hashing after each change, no divergence log, no byte-identity requirement, no re-vendoring plan.

The reason is what a provenance record is for. A reader of a vendored file has one question, where this came from and what it looked like when it arrived, and a single dated statement naming the origin, the license, the original hash, and the introducing commit answers it completely and permanently. Byte-identity plus per-change re-hashing answers a different question, whether the copy still matches upstream, and that is only worth its cost if the copy is meant to track upstream. This one is not. Its consumers are Torrust repositories, whose needs Bitcoin Core does not share, and this issue is the first of those needs. Keeping a tracking discipline for a file nobody intends to re-sync would tax every future change while guaranteeing nothing a reader benefits from; git history already records what changed after arrival, more precisely than a hash in a README.

Offering the feature to Bitcoin Core first was considered and declined. It would preserve a byte-identity that nothing depends on once the copy is no longer tracked, and its schedule is not ours: upstream may reasonably decline a feature its own repository does not need, review may take arbitrarily long, and `torrust/torrust-index` stays blocked throughout. Nothing prevents contributing the idea upstream later on its own merits; it is simply not a precondition for fixing a tool this project maintains.

### AD2 - The declaration path has no default inside the tool (RESOLVED)

Two behaviors were open for a run that passes no `--symlinks` argument. Either the tool falls back to a built-in default path and processes `.symlinks.json` whenever the merged tree happens to carry one, or the argument is the only thing that ever names a declaration and its absence means no declaration processing at all. This specification takes the second: the tool carries no default path, and a run without `--symlinks` reads no file, reports no accepted or stale entries, and refuses every symbolic link in every checked commit exactly as the tool does today, even when the merged tree contains `.symlinks.json`. The repository's own choice of path is stated once, in `merge-pull-request.sh`, which passes `--symlinks .symlinks.json` unconditionally.

Three properties decide it. First, the vendored tool keeps its current behavior for any invocation that does not ask for the new one, so a reader comparing this copy against the file it came from sees a change that only ever adds behavior behind an explicit flag; the diff stays small and reviewable, and the single provenance statement AD1 preserves stays honest about how far the copy has moved. Second, an exemption from a security refusal should be visible in the invocation that grants it. A built-in default would make the difference between "every link refused" and "these links admitted" depend on whether a file happens to be present in the tree, which is precisely the invisible condition an exception mechanism should not have; with no default, the argument on the command line is the whole answer, and a maintainer reading the wrapper or the merge output can see it. Third, the wrapper is already the maintainer-facing entry point and already exists to hold repository-specific wiring, so stating the path there introduces no new moving part and keeps the policy in one place instead of two.

The cost is that a repository adopting the tool must add the argument to its own wrapper rather than inheriting the behavior by naming its file correctly. That is the intended trade: adopting an exception mechanism is a decision a repository should have to write down, and the wrapper line is where it is written.

## Design and Ownership Review

This work adds no child process, no asynchronous I/O, no network readiness step, no new resource with a drop path, and no reusable test fixture beyond the temporary repositories the existing wrapper test suite already builds, so the deadline and lifetime concerns this section normally covers do not arise. The responsibility split is the part worth fixing before implementation:

- `.symlinks.json` carries repository policy only. It states which links a repository accepts and why; it grants no capability by itself and is inert in a repository whose tooling does not read it.
- `github-merge.py` owns enforcement, and it owns reading. It reads both the links and the declaration out of the merged tree at `HEAD`, the declaration from the tree path `--symlinks` names, decides which links are exempt, prints what it admitted, and keeps its current refusal and exit code for everything else. It never reads the declaration from the filesystem, never writes it, and never infers an exemption that the file does not state. It also never chooses a declaration path: without `--symlinks` it reads no declaration at all, and its behavior is the behavior it has today.
- `merge-pull-request.sh` owns the repository-specific wiring only, and under AD2 it is the only place the tracker's declaration path is stated: it passes the repository-relative tree path the tracker uses. It does not check whether that file exists, because existence is a property of the merged tree that the wrapper cannot observe before the merge commit is created; it reads nothing, performs no parsing, and applies no policy of its own beyond naming the path.
- The direction of matching is from the trees to the declaration, never the reverse. The tool walks the links actually present in the commits it checks and asks whether each one is declared; a declaration entry can only ever remove a refusal for a link that exists, and can never introduce one.
- The checked range and the authoritative declaration are deliberately asymmetric. The range is every commit the merge introduces, `pull/<n>/base..pull/<n>/head` plus the local merge commit; the declaration is read from the final merged tree alone, and no intermediate commit's copy of the declaration is ever consulted. One reviewed statement therefore answers for the whole range.
- Both inputs come from commits the merge produced or introduced, never from the filesystem. The declaration is read from `HEAD` of the local merge at the argument's path, so no state outside the merge result can change the verdict.

## Implementation Plan

Status values: `TODO`, `IN_PROGRESS`, `BLOCKED`, `DONE`.

| ID  | Status | Task                                          | Notes / Expected Output                                                                                                                                                                                                                                       |
| --- | ------ | --------------------------------------------- | --------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| T1  | TODO   | Define the declaration format and its rules   | The `.symlinks.json` shape, field meanings, shared namespace, and the accept/reject rules below are written into `README-github-merge.md`.                                                                                                                     |
| T2  | TODO   | Add `--symlinks` and the range check          | Optional argument next to `--repo-from` in `parse_arguments()` (lines 260-280), taking a repository-relative tree path with no default value, so its absence is distinguishable from any path. When it is absent, the caller (lines 393-397) skips declaration processing entirely and keeps the current refusal for every checked commit. When it is present, the caller reads the declaration once from the merge commit with `git show HEAD:<path>`, then iterates `pull/<n>/base..pull/<n>/head` plus the merge commit, listing each commit's links, exempting matches, reporting refusals with the carrying commit, and printing each accepted path, target, and reason. The tool is edited directly under AD1. |
| T3  | TODO   | Pass the tree path from the wrapper           | `merge-pull-request.sh` inserts `--symlinks .symlinks.json` before the positional arguments of its final `exec python3` call (line 136), unconditionally and without a filesystem check, because the path names a location in the merged tree. Under AD2 this line is the only statement of the tracker's declaration path; the tool supplies no default behind it. |
| T4  | TODO   | Cover the behavior with tests                 | Wrapper delegation of the tree path; a declared link merges past the check; an undeclared link, a declared link with a different target, a target containing `..`, and an absolute target each refuse with exit code `4`; a declaration committed in the merged result while the tool runs without `--symlinks` exempts nothing and produces no accepted or stale-entry output; a declaration present only in the working directory, or only on the base branch and not in the merged result, exempts nothing; a link added in an intermediate commit and removed before the tip is refused and names that commit; a link declared with one target mid-range and another at the tip is refused; a removal pull request that keeps the manifest entry passes with a stale-entry report; symbolic links already on the base branch are not re-checked. |
| T5  | TODO   | Rewrite the provenance record                 | `README-github-merge.md` carries the single provenance statement from AD1; the local-change policy, the new-hash requirement, and the byte-identical wording are removed; the known-upstream-issues section is re-checked, since those items can now be fixed in the tool directly in a follow-up rather than only recorded. |
| T6  | TODO   | Note the behavior in the merge skill          | `.github/skills/dev/git-workflow/merge-pull-request/SKILL.md` explains that the declaration is read from the merged result rather than the working copy, that every commit the merge introduces is checked against it, what the accepted-link output looks like, that an undeclared link is still a hard stop, that the wrapper's `--symlinks` argument is what enables the mechanism at all, and that removing a declared link takes two changes. |

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

- The checked commits are every commit the merge introduces: the pull request's own commits, the range `pull/<n>/base..pull/<n>/head`, plus the local merge commit itself. Each is listed with `git ls-tree --full-tree -r <commit>` under the same mode mask the current check uses.
- Commits already reachable from the base branch are not part of what the merge introduces and are not walked. This rule adds no retroactive check over existing history.
- The declaration is read from the final merged tree only, out of the local merge commit the tool has just created, with `git show HEAD:<path>` or the equivalent `git cat-file` against that commit, where `<path>` is the value of `--symlinks`. It is never read from the working directory, the index, the base branch, or an intermediate commit in the range, and never from any path outside the repository.
- `--symlinks <path>` names a repository-relative path inside that final tree. The argument has no default value: the tool never selects a declaration path of its own, and `.symlinks.json` is a repository convention supplied by the caller, not a fallback inside the tool. It is a tree path, not a filesystem path, so an absolute path or a path escaping the repository is not a usable argument.
- A declaration that exists only in the maintainer's working tree, only on the base branch, or only in an intermediate commit without reaching the merged result exempts nothing. Only what the merge produced counts.
- `path` is repository-relative and must name a symbolic link in at least one checked commit.
- `target` must equal the link's literal content byte for byte in every checked commit that carries that link. A target that resolves to the same file by another spelling does not match.
- A target that is absolute, or that contains a `..` segment, is never accepted, whatever the declaration file says. This is a property of the target itself, not of the declaration, so no file can grant it.
- A symbolic link found in any checked commit but absent from the final declaration produces `ERROR: File '<path>' was a symlink in commit <hash>` and exit code `4`. The message carries the carrying commit in every case, including the merge commit, so the maintainer can go straight to it.
- A symbolic link found in a checked commit and declared with a different target is treated as undeclared and produces the same message and exit code. Any divergence between the final declaration and the links in any checked commit refuses exactly as an undeclared link does today.
- A declaration entry that matches no symbolic link in any checked commit exempts nothing, because matching runs from the commits to the file. Such a stale entry is reported in the merge output so it can be removed; it does not refuse.
- Removing a declared link therefore takes two changes whenever the pull request introduces any commit that still carries the link, which is the ordinary case. That pull request must keep the manifest entry in the final merged tree, because those pre-deletion commits are judged against the final manifest; the entry is then stale, which the preceding rule reports rather than refuses, and a later change drops it once no checked commit carries the link. This is a consequence of the rule rather than an oversight, and it is the price of one manifest answering for the whole range.
- A declaration file absent from the final merged tree at the path `--symlinks` names is not itself an error and grants no exceptions: the tool refuses every symbolic link in every checked commit exactly as it does today.
- A run that passes no `--symlinks` argument performs no declaration processing at all. The tool reads no file, admits nothing, reports no accepted entries and no stale entries, and refuses every symbolic link in every checked commit exactly as it does today, whatever the merged tree contains. A `.symlinks.json` committed in that tree changes nothing on such a run, because the tool has no default path to find it by. The reasoning is recorded in AD2.

## Commit Points

| Task | Coherent change set                                                                         | Commit policy                                                                                        |
| ---- | ------------------------------------------------------------------------------------------- | ---------------------------------------------------------------------------------------------------- |
| T1   | Declaration format and rules documented in the vendoring README.                            | Commit after review of the format and rules, before any tool change.                                 |
| T2   | The `--symlinks` argument, tree-sourced reading, the range walk, the exemption logic, and the accepted-link output. | Commit after focused validation.                                              |
| T3   | Wrapper delegation of the declaration tree path.                                            | Commit separately from the tool change; it is independently reviewable and independently revertible. |
| T4   | Test increments for delegation, acceptance, each rejection case, and the range behaviors.   | Commit each reviewed test increment before starting the next behavior area.                          |
| T5   | Provenance rewrite and known-issues re-check in the vendoring README.                       | Commit with, or immediately after, the tool change whose maintenance model it states.                |
| T6   | Skill note describing the workflow-visible behavior.                                        | Commit after the behavior it documents is merged or staged in the same branch.                       |

Record a justified no-change decision in the task's evidence without creating an empty commit. Use a Conventional Commit message with the narrow affected scope, and sign every commit with GPG.

## Progress Tracking

### Workflow Checkpoints

- [x] Folder-style spec drafted in `docs/issues/drafts/merge-tool-symlink-exceptions/ISSUE.md`
- [x] Spec reviewed and approved by user/maintainer
- [x] AD1 resolved and recorded in this specification
- [x] GitHub issue #2175 created and issue number added to this spec
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
- 2026-09-08 17:08 UTC - Spec author - Widened the check from the final tree to every commit the merge introduces, judged against the final manifest alone, after verifying that the tool already fetches both range endpoints (lines 349-357) and already enumerates `pull/<n>/base..pull/<n>/head` for the merge message (line 380); recorded the two-step link-removal consequence in the rules, criteria, scenarios, and risks - This spec
- 2026-09-08 17:49 UTC - Spec author - Specification approved and opened as GitHub issue #2175; moved the folder from `docs/issues/drafts/` to `docs/issues/open/2175-merge-tool-symlink-exceptions/`, updated the frontmatter, title, and checkpoints, and re-verified that every relative link still resolves from the new location - https://github.com/torrust/torrust-tracker/issues/2175
- 2026-09-08 19:24 UTC - Spec author - Parameterized the manifest path throughout after PR #2176 review: the declaration is read from `HEAD:<path>`, the value `--symlinks` names, defaulting to `.symlinks.json`, so the rules no longer contradict the overridable argument - https://github.com/torrust/torrust-tracker/pull/2176#discussion_r3961420915
- 2026-09-09 08:20 UTC - Spec author - Removed the tool-side default path after PR #2176 review found it contradicting the omission rule: `--symlinks` now has no default, an omitted argument disables declaration processing entirely, and `merge-pull-request.sh` is the single place the tracker's `.symlinks.json` path is stated; recorded the reasoning as AD2 and propagated it through scope, ownership, plan, rules, criteria, and scenarios - https://github.com/torrust/torrust-tracker/pull/2176#pullrequestreview-5151505040

## Acceptance Criteria

- [ ] AC1: `.symlinks.json` has a documented shape and a documented rule set, including the target forms that are never accepted.
- [ ] AC2: When `--symlinks` is given, the tool reads the declaration from the final merged tree, out of the local merge commit it has just created, at the tree path the argument names, and from no other source. A declaration present only in the working directory, only on the base branch, or only in an intermediate commit without reaching the merged result exempts nothing.
- [ ] AC3: The tool checks the links in every commit the merge introduces, `pull/<n>/base..pull/<n>/head` plus the local merge commit, against that single final declaration, and does not walk commits already reachable from the base branch.
- [ ] AC4: `github-merge.py` accepts `--symlinks <path>` as a repository-relative path inside the final merged tree, with no default value, and exempts exactly the declared links whose target matches the link content byte for byte in every checked commit that carries them.
- [ ] AC5: Every accepted link is printed with its path, target, and reason before the maintainer is asked to sign.
- [ ] AC6: An undeclared link, a link declared with a different target, a declared absolute target, and a declared target containing a `..` segment each refuse with `ERROR: File '<path>' was a symlink in commit <hash>` and exit code `4`, naming the commit that carries the link.
- [ ] AC7: A pull request that deletes a declared link and keeps its manifest entry in the final merged tree passes the check, and the retained entry is reported as stale rather than refused.
- [ ] AC8: Declaration processing happens only when `--symlinks` is given. Run without the argument, the tool reads no declaration and behaves exactly as it does today for every checked commit, even when the final merged tree carries a `.symlinks.json` that would have declared the link; and run with the argument while the final merged tree carries no file at that path, it exempts nothing and refuses every checked commit's links exactly as it does today.
- [ ] AC9: `merge-pull-request.sh` passes `--symlinks .symlinks.json` unconditionally and performs no filesystem check for that file, and it is the only place the tracker's declaration path is stated.
- [ ] AC10: `README-github-merge.md` carries the single provenance statement, naming the origin, the license, the original SHA-256, and the introducing commit, and no longer requires re-hashing or byte-identity.
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
| M2  | Undeclared link is refused              | Remove the entry from the fixture's committed `.symlinks.json`, commit that change into the merged result, and repeat the run.                                        | `ERROR: File '<path>' was a symlink in commit <hash>` and exit code `4`.                                                    | TODO   | {captured output and `echo $?`}       |
| M3  | Escaping targets are refused when declared | In the fixture, point the link at an absolute path, then at a path containing a `..` segment, committing a matching declaration alongside the link in each case.       | Both runs refuse with the same message and exit code `4`, showing that the declaration cannot grant an escaping target.     | TODO   | {captured output for both variants}   |
| M4  | Missing declaration file changes nothing   | Commit the fixture without `.symlinks.json` in the merged result and run the merge through `merge-pull-request.sh`, which passes `--symlinks .symlinks.json` unconditionally.  | The run reproduces today's refusal for the link that is present, with exit code `4`; the absent path is reported as no exceptions rather than as a tool error. | TODO   | {captured output and `echo $?`}       |
| M5  | Out-of-tree declaration exempts nothing | Write a valid `.symlinks.json` into the fixture's working directory without committing it, and separately commit one on the base branch only so the merged result does not carry it; run the merge in both cases. | Both runs refuse with exit code `4`, showing that only the final merged tree is consulted.                                  | TODO   | {captured output for both variants}   |
| M6  | Intermediate-commit link is refused     | In the fixture's pull-request branch, add a symbolic link in one commit and delete it in a later commit so the tip carries no link, with no manifest entry; run the merge.  | The run refuses with exit code `4` and names the intermediate commit that carries the link, not the tip.                    | TODO   | {captured output and the named hash}  |
| M7  | Removal pull request needs the retained entry | From a fixture whose base already carries a declared link, open a pull request with one ordinary commit that still carries the link followed by a commit deleting it; run the merge once with the manifest entry dropped in the same pull request, then once with the entry retained in the final tree. | The first run refuses and names the pre-deletion commit; the second passes and reports the retained entry as stale.         | TODO   | {captured output for both runs}       |
| M8  | Omitted argument disables the mechanism | Use the M1 fixture, whose merged result commits both the symbolic link and a `.symlinks.json` declaring it with the matching target, and invoke `github-merge.py` directly with no `--symlinks` argument.  | The run refuses the declared link with exit code `4` and prints no accepted-link and no stale-entry output, showing that the tool reads no declaration unless the argument names one. | TODO   | {captured output and `echo $?`}       |

Notes:

- Every scenario except M8 invokes the tool with `--symlinks .symlinks.json`, directly or through `merge-pull-request.sh`. M8 is the scenario that omits the argument, and it is the only one that does.
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
| AC9   | TODO                   | {test/log/PR link} |
| AC10  | TODO                   | {test/log/PR link} |

## Risks and Trade-offs

- **The exception could weaken a security check.** Mitigation is in the rule set rather than in review discipline: the mechanism is opt-in per invocation, so a run that does not name a declaration admits nothing and is the tool as it is today; an exception is explicit, so nothing is admitted that the repository has not written down; it is tree-sourced, so the exception is part of the reviewed merge result and no state in the maintainer's environment can change the verdict; it is history-wide, so a link cannot slip through by existing only in a commit that is not the tip; it is byte-exact, so a link that changes where it points stops matching and is refused again; it is path-restricted, because absolute targets and targets containing `..` are refused whatever the file says, which keeps the exception inside the repository; and it is printed, so the maintainer sees every admitted link before signing rather than after. The residual risk is a repository declaring a link it should not have; that risk already exists for every other file the repository commits, and the declaration makes it visible in review instead of invisible.
- **The tool is now this project's to maintain.** Under AD1 the file is modified here under ordinary review, so the effort of understanding and fixing it falls to this project rather than to upstream. That is the deliberate result, not a side effect: the alternative was leaving a needed fix to a schedule nobody here controls. The residual risk is narrower, that a later reader loses sight of where the file came from and mistakes it for original work; mitigation is the permanent provenance statement naming the origin, the license, the original hash, and the introducing commit, kept next to the retained upstream source header and `github-merge-COPYING`.
- **The tracker and the sibling copies can drift.** Once other repositories adopt the tool, each holds its own copy of the argument and its own declaration file, and a fix applied in one is easy to forget in the others. Mitigation is to keep the tracker's copy the reference, state that in the vendoring README, and mirror deliberately rather than by editing each copy independently.
- **Removing a link costs an extra change.** Because the final manifest judges every commit the merge introduces, a pull request that deletes a declared link has to keep the entry, and a later change removes it. The cost is one extra change per removed link, on an action that is rare and deliberate; what it buys is that one reviewed declaration answers for the whole merged range, with no commit judged by a statement the final result has withdrawn. Mitigation is to name the two-step sequence in the rules, in the merge skill, and in the stale-entry report itself, so a maintainer meets it as documented behavior rather than as a surprise mid-merge.
- **A declaration file can go stale.** A link can be removed while its entry remains, and the two-step removal above makes that state routine rather than exceptional. Reading from the merged tree keeps the declaration from lagging the trees it is compared against, since it comes from the same merge the range belongs to, so the remaining staleness is exactly the retained entry the removal sequence expects. Mitigation is the stale-entry report in the merge output, which distinguishes a deliberate retained entry from a forgotten one by making both visible; enforcing the file against the tree in the linter is deliberately left out of this issue and can follow once the format has settled.

## Implementation Completion Review

After implementation, compare the result with this specification. Record invalidated assumptions, material design changes, unexpected validation findings, and reusable lessons.

- Retrospective: `Not yet assessed`
- If needed, create `implementation-retrospective.md` from the repository template at `docs/templates/IMPLEMENTATION-RETROSPECTIVE.md` in this issue specification's directory.
- If no retrospective is needed, add a concise progress-log entry explaining why the work had no material discovery.
- When an independent reviewer receives this folder-style specification, it records its result in `agent-review-reports.md` using `docs/templates/AGENT-REVIEW-REPORTS.md`.

## References

- GitHub issue: #2175
- Related issues: #2022 vendored and documented the maintainer merge workflow that this issue extends; #2003 is the automation umbrella EPIC that the workflow reports to. Neither is a parent of this issue.
- Related PRs: none yet.
- Related ADRs: `docs/adrs/` holds none for this area. The sibling repository's container-infrastructure decision, ADR-T-009 (`adr/009-container-infrastructure-refactor.md` in `torrust/torrust-index`), is the reason its tree carries the first link this issue has to admit.
- Provenance record: [`contrib/dev-tools/git/README-github-merge.md`](../../../../contrib/dev-tools/git/README-github-merge.md)
- Merge workflow: [`merge-pull-request` skill](../../../../.github/skills/dev/git-workflow/merge-pull-request/SKILL.md)
- Vendoring specification: [issue #2022 specification](../../closed/2022-vendor-and-document-maintainer-merge-workflow/ISSUE.md)
- Automation umbrella: [EPIC #2003](../../open/2003-overhaul-guardrails-and-automation/EPIC.md)
- Blocked merge in the sibling repository: <https://github.com/torrust/torrust-index/pull/884>
