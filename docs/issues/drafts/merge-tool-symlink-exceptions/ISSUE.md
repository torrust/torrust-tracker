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
last-updated-utc: 2026-09-08 16:08
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

Allow the vendored maintainer merge tool to accept symbolic links that a repository has declared explicitly, in an auditable root file that names each accepted link, its exact target, and the reason it exists, while every undeclared or mismatched link keeps failing exactly as it does today.

## Background

`contrib/dev-tools/git/github-merge.py` refuses to continue when the merged tree contains any symbolic link. `get_symlink_files()` (lines 151-157) runs `git ls-tree --full-tree -r HEAD` and collects every entry whose mode satisfies `(mode & 0o170000) == 0o120000`. Its caller (lines 393-397) prints `ERROR: File '<path>' was a symlink` once per collected entry and calls `sys.exit(4)` when the list is not empty. The check runs after the unsigned merge commit is created and before `tree_sha512sum()` computes the tree hash that the sign-off step later compares against, so a repository containing a symbolic link cannot reach signing at all.

The check itself is sound. A symbolic link in a merged tree is a way to make a reviewed path resolve somewhere else, and rejecting the whole class removes that from the merge path without asking the maintainer to judge each case. What the check lacks is any way to admit a link that a repository has chosen deliberately: the refusal is unconditional, so on such a repository every merge stops, and the only workaround available today is deleting the link.

The first repository this blocks is the sibling `torrust/torrust-index`. Its tree carries exactly one symbolic link, `.dockerignore`, whose literal content is `.containerignore`. The link was introduced deliberately in commit `01e7c701c9d8cb60` (2026-04-24) as part of its ADR-T-009 container-infrastructure work, because Docker reads only `.dockerignore` while Podman and Buildah prefer `.containerignore`; one link lets both toolchains share a single ignore list instead of two files that drift apart. That repository sets `githubmerge.repository` to `torrust/torrust-index` and runs the same tool; a merge run against its pull request 884 created the local merge commit and then stopped with `ERROR: File '.dockerignore' was a symlink`, leaving the tool's temporary `pull/884/local-merge` branch behind.

The tracker's own tree contains no symbolic links today: `git ls-files -s | awk '$1=="120000"'` returns nothing. This change therefore alters no tracker merge until the tracker itself declares a link, and it stays a no-op for any repository that ships no declaration file. The tracker is where the vendored copy, the `merge-pull-request.sh` wrapper, the wrapper test suite, and the vendoring policy live, so the tool-side fix belongs here and is mirrored into other repositories that adopt the same workflow.

## Scope

### In Scope

- Define a repository-level declaration file, `.symlinks.json` at the repository root, that lists each accepted symbolic link with its exact target and the reason it exists.
- Define the matching and rejection rules the merge tool applies to that file, including the target forms that are never accepted.
- Add an optional `--symlinks <json-file>` argument to the vendored merge tool that loads a declaration file and exempts exactly the matching links from the refusal.
- Print every accepted link, with its path, target, and reason, into the merge output so the maintainer sees what was admitted before signing.
- Teach `contrib/dev-tools/git/merge-pull-request.sh` to pass `--symlinks .symlinks.json` automatically when that file exists, so maintainers never type the argument.
- Extend `contrib/dev-tools/git/tests/test-merge-pull-request.sh` to cover the wrapper's delegation with and without the declaration file present.
- Record the vendor divergence, or the upstream contribution, in `contrib/dev-tools/git/README-github-merge.md` and update its provenance hash if the vendored copy changes.
- Note the new behavior in the `merge-pull-request` skill.

### Out of Scope

- Linter enforcement of `.symlinks.json`: no schema check, no drift check between the file and the tree, and no `linter all` step that reads it. The merge tool is the only consumer in this issue.
- Changing the check's semantics for links that are not declared. An undeclared link, a declared link whose target does not match, and a target of a form the rules reject all keep failing with the current message and exit code.
- Any symbolic-link policy for the tracker's own tree. This issue does not add a link to this repository or decide whether one should ever be added.
- Changing any other vendored behavior, including the known upstream issues already recorded in the README.
- Mirroring the change into `torrust/torrust-index` or any other repository. That is separate work in those repositories.

## Architectural Decisions

- Related ADRs: None known in `docs/adrs/`.
- ADRs to create: a vendored-tool divergence record, if and only if decision AD1 is resolved toward a local modification.

### AD1 - How the change reaches the vendored copy (OPEN)

`github-merge.py` is currently a byte-identical vendor copy of the reviewed snapshot from issue #2022, recorded in `README-github-merge.md` with SHA-256 `e390eb014131f3183a2cba642134974a6b09b19a65322d17dd7c81cf4ffbaad2`. That README states that local changes to the vendored algorithm require a documented security, portability, or correctness reason and a new provenance hash, and that repository-specific behavior is deliberately confined to `merge-pull-request.sh` so the vendor copy stays auditable. This feature cannot be confined to the wrapper: the refusal lives inside the tool, before the tree hash is taken, and no wrapper argument or environment setting reaches it. So the change has to enter the vendored copy one of two ways, and this specification does not choose between them.

**Option (a) - documented local modification.** Patch the vendored copy, record the divergence in the README's provenance section, and publish a new provenance hash under the existing local-change policy. A correctness and portability reason exists: the tool cannot merge a repository whose tree legitimately contains a link that both container toolchains need, and the alternative is deleting a file the repository requires. The cost is that the vendored copy stops being byte-identical to upstream, every future re-vendor has to re-apply the patch or consciously drop it, and the README has to carry the divergence for as long as it lasts.

**Option (b) - offer it upstream first.** Propose the argument and the declaration format to Bitcoin Core, and vendor the result back once it is merged there. The benefit is that byte-identity is preserved and there is nothing to re-apply at the next re-vendor. The cost is that the outcome is not ours to schedule: Bitcoin Core may decline a feature its own repository does not need, review may take arbitrarily long, and `torrust/torrust-index` stays blocked in the meantime.

The two options are not exclusive. Option (a) can ship now to unblock the sibling repository, and option (b) can be attempted afterwards; if upstream accepts an equivalent feature, the local patch is dropped at the next re-vendor and the README returns to recording a byte-identical copy. Resolve AD1 before starting T2; T1 does not depend on it.

## Design and Ownership Review

This work adds no child process, no asynchronous I/O, no network readiness step, no new resource with a drop path, and no reusable test fixture beyond the temporary repositories the existing wrapper test suite already builds, so the deadline and lifetime concerns this section normally covers do not arise. The responsibility split is the part worth fixing before implementation:

- `.symlinks.json` carries repository policy only. It states which links a repository accepts and why; it grants no capability by itself and is inert in a repository whose tooling does not read it.
- `github-merge.py` owns enforcement. It reads the merged tree, decides which links are exempt, prints what it admitted, and keeps its current refusal and exit code for everything else. It never writes the declaration file and never infers an exemption that the file does not state.
- `merge-pull-request.sh` owns the repository-specific wiring: it decides whether a declaration file exists and passes its path. It performs no parsing and applies no policy of its own.
- The direction of matching is from the tree to the declaration, never the reverse. The tool walks the links actually present in the merged tree and asks whether each one is declared; a declaration entry can only ever remove a refusal for a link that exists, and can never introduce one.

## Implementation Plan

Status values: `TODO`, `IN_PROGRESS`, `BLOCKED`, `DONE`.

| ID  | Status | Task                                          | Notes / Expected Output                                                                                                                                                                                                                                       |
| --- | ------ | --------------------------------------------- | --------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| T1  | TODO   | Define the declaration format and its rules   | The `.symlinks.json` shape, field meanings, and the accept/reject rules below are written into `README-github-merge.md`. Independent of AD1.                                                                                                                   |
| T2  | TODO   | Add `--symlinks` to the vendored tool         | Optional argument next to `--repo-from` in `parse_arguments()` (lines 260-280); `get_symlink_files()` and its caller (lines 393-397) exempt matching links and print each accepted path, target, and reason. Blocked until AD1 is resolved.                     |
| T3  | TODO   | Pass the file automatically from the wrapper  | `merge-pull-request.sh` inserts `--symlinks .symlinks.json` before the positional arguments of its final `exec python3` call (line 136) when the file exists at the repository top level, and passes nothing otherwise.                                          |
| T4  | TODO   | Cover the behavior with tests                 | Wrapper delegation with and without the file; a declared link merges past the check; an undeclared link, a declared link with a different target, a target containing `..`, and an absolute target each keep the existing message and exit code `4`.            |
| T5  | TODO   | Update the vendoring record                   | `README-github-merge.md` provenance section records the outcome of AD1, with a new SHA-256 if the vendored copy changed; the known-issues section is reviewed so it still describes the copy as shipped.                                                        |
| T6  | TODO   | Note the behavior in the merge skill          | `.github/skills/dev/git-workflow/merge-pull-request/SKILL.md` explains when the wrapper passes the declaration file, what the accepted-link output looks like, and that an undeclared link is still a hard stop.                                                 |

### Declaration format

```json
{
  "namespace": "com.torrust.tracker.symlinks",
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

The `namespace` is per repository, so a declaration file cannot be moved between repositories unnoticed; the tracker uses `com.torrust.tracker.symlinks`. The `version` is the declaration-format version, not the repository version.

### Rules

- `path` is repository-relative and must name a symbolic link in the merged tree.
- `target` must equal the link's literal content byte for byte. A target that resolves to the same file by another spelling does not match.
- A target that is absolute, or that contains a `..` segment, is never accepted, whatever the declaration file says. This is a property of the target itself, not of the declaration, so no file can grant it.
- A symbolic link present in the merged tree but absent from the declaration file still produces `ERROR: File '<path>' was a symlink` and exit code `4`.
- A symbolic link present in the merged tree and declared with a different target is treated as undeclared and produces the same message and exit code.
- A declaration entry whose path is not a symbolic link in the merged tree exempts nothing, because matching runs from the tree to the file. Such a stale entry is reported in the merge output so it can be removed.
- A missing declaration file means no exceptions: the tool behaves exactly as it does today. So does running the tool without `--symlinks`.

## Commit Points

| Task | Coherent change set                                                                         | Commit policy                                                                                        |
| ---- | ------------------------------------------------------------------------------------------- | ---------------------------------------------------------------------------------------------------- |
| T1   | Declaration format and rules documented in the vendoring README.                            | Commit after review of the format and rules, before any tool change.                                 |
| T2   | The `--symlinks` argument, the exemption logic, and the accepted-link output.                | Commit after focused validation and after AD1 is resolved and recorded.                              |
| T3   | Wrapper delegation of the declaration file.                                                 | Commit separately from the tool change; it is independently reviewable and independently revertible. |
| T4   | Test increments for delegation, acceptance, and each rejection case.                        | Commit each reviewed test increment before starting the next behavior area.                          |
| T5   | Provenance and known-issues update, including any new SHA-256.                              | Commit with, or immediately after, the change whose provenance it records.                           |
| T6   | Skill note describing the workflow-visible behavior.                                        | Commit after the behavior it documents is merged or staged in the same branch.                       |

Record a justified no-change decision in the task's evidence without creating an empty commit. Use a Conventional Commit message with the narrow affected scope, and sign every commit with GPG.

## Progress Tracking

### Workflow Checkpoints

- [ ] Folder-style spec drafted in `docs/issues/drafts/merge-tool-symlink-exceptions/ISSUE.md`
- [ ] Spec reviewed and approved by user/maintainer
- [ ] AD1 resolved and recorded in this specification
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

## Acceptance Criteria

- [ ] AC1: `.symlinks.json` has a documented shape and a documented rule set, including the target forms that are never accepted.
- [ ] AC2: `github-merge.py` accepts `--symlinks <json-file>` and exempts exactly the declared links whose target matches the link content byte for byte.
- [ ] AC3: Every accepted link is printed with its path, target, and reason before the maintainer is asked to sign.
- [ ] AC4: An undeclared link, a link declared with a different target, a declared absolute target, and a declared target containing a `..` segment each keep the message `ERROR: File '<path>' was a symlink` and exit code `4`.
- [ ] AC5: With no declaration file, and with no `--symlinks` argument, the tool behaves exactly as it does today.
- [ ] AC6: `merge-pull-request.sh` passes `--symlinks .symlinks.json` when that file exists and passes nothing when it does not.
- [ ] AC7: `README-github-merge.md` records the outcome of AD1: either a documented local modification with a new provenance hash, or an upstream contribution that preserved byte-identity.
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
| M1  | Declared link is accepted               | Build a fixture repository containing a symbolic link and a `.symlinks.json` declaring it with the matching target; run the merge tool with `--symlinks .symlinks.json`. | The tool prints the accepted path, target, and reason, and continues past the check to compute the tree hash.               | TODO   | {captured merge output}               |
| M2  | Undeclared link is refused              | Remove the entry from the fixture's `.symlinks.json` and repeat the run.                                                                                              | `ERROR: File '<path>' was a symlink` and exit code `4`.                                                                     | TODO   | {captured output and `echo $?`}       |
| M3  | Escaping targets are refused when declared | In the fixture, point the link at an absolute path, then at a path containing a `..` segment, declaring each in `.symlinks.json` with the exact matching target.       | Both runs refuse with the same message and exit code `4`, showing that the declaration cannot grant an escaping target.     | TODO   | {captured output for both variants}   |
| M4  | Missing declaration file changes nothing   | Delete `.symlinks.json` from the fixture and run through `merge-pull-request.sh`, and separately run the tool with no `--symlinks` argument.                           | The wrapper passes no argument, and both runs reproduce today's refusal for the link that is present.                       | TODO   | {captured output for both runs}       |

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

## Risks and Trade-offs

- **The exception could weaken a security check.** Mitigation is in the rule set rather than in review discipline: an exception is explicit, so nothing is admitted that the repository has not written down; it is byte-exact, so a link that changes where it points stops matching and is refused again; it is path-restricted, because absolute targets and targets containing `..` are refused whatever the file says, which keeps the exception inside the repository; and it is printed, so the maintainer sees every admitted link before signing rather than after. The residual risk is a repository declaring a link it should not have; that risk already exists for every other file the repository commits, and the declaration makes it visible in review instead of invisible.
- **A local modification has to be carried.** If AD1 resolves to option (a), the vendored copy is no longer byte-identical, every future re-vendor has to re-apply or consciously drop the patch, and the README has to keep the divergence current. Mitigation is to keep the patch as small and as separable as the feature allows, to record it in the provenance section rather than only in commit history, and to reconsider option (b) at each re-vendor.
- **The tracker and the sibling copies can drift.** Once other repositories adopt the tool, each holds its own copy of the argument and its own declaration file, and a fix applied in one is easy to forget in the others. Mitigation is to keep the tracker's copy the reference, state that in the vendoring README, and mirror deliberately rather than by editing each copy independently.
- **A declaration file can go stale.** A link can be removed while its entry remains. Mitigation is the stale-entry report in the merge output; enforcing the file against the tree in the linter is deliberately left out of this issue and can follow once the format has settled.

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
