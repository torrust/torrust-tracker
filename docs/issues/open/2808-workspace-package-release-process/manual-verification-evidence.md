---
doc-type: manual-verification-evidence
issue-spec: docs/issues/open/2808-workspace-package-release-process/ISSUE.md
last-updated-utc: "2026-11-11 11:16"
---

# Manual Verification Evidence

## Environment and Prerequisites

- Date (UTC): 2026-11-11
- Artifact under test: the implementation PR of #2808 (commits `docs(release): [#2808] split the release
  process ...`, `docs(release): [#2808] define the workspace-package and standalone-crate release process`,
  `docs(skills): [#2808] add release-workspace-package ...`).
- Environment: Linux, stable Rust toolchain of the workspace, no `cargo login` token used (dry runs only).
- Nothing was published, tagged, or pushed upstream.

## Verification Processes

### V1 - Workspace-package path (dry run)

- Goal: reach `cargo publish -p <crate> --dry-run` using only the skill and the document.
- Initial state: the document's preconditions read for `torrust-tracker-primitives` (explicit version `3.1.0`).
- Status: `DONE`

#### Steps Performed

1. Followed the skill (variant table, agent checklist) to the document's "Workspace Package" path and its Preconditions.
2. Ran `cargo publish -p torrust-tracker-primitives --dry-run --allow-dirty` (`--allow-dirty`
   because the documentation edits were still uncommitted; a release runs from a clean merged commit
   and needs no such flag).

#### Observed Result

```text
   Compiling torrust-tracker-primitives v3.1.0 (.../target/package/torrust-tracker-primitives-3.1.0)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 3.61s
   Uploading torrust-tracker-primitives v3.1.0 (.../packages/primitives)
warning: aborting upload due to dry run
```

#### Conclusion

Every step up to the dry run was findable. The dry run succeeded.

### V2 - Standalone-crate path (dry run)

- Goal: reach `cargo publish --dry-run` for `torrust-server-lib` from the document alone.
- Initial state: `torrust-server-lib` on `main`, clean tree (`git status -sb` showed `## main...origin/main`).
- Status: `DONE`

#### Steps Performed

1. Followed the "Standalone-Repository Crate" section.
2. Ran `cargo publish --dry-run` in the `torrust-server-lib` checkout, with the build output in a scratch target folder.

#### Observed Result

```text
   Compiling torrust-server-lib v0.6.0 (.../torrust-server-lib-0.6.0)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 4.98s
   Uploading torrust-server-lib v0.6.0 (.../torrust-server-lib)
warning: aborting upload due to dry run
```

#### Conclusion

The standalone variant is complete on its own. The duplicate checklist left in `torrust-server-lib`'s
`docs/release_process.md` is covered by the follow-up issue text in
`torrust-server-lib-followup-issue.md`.

### V3 - Duplicated instructions

- Status: `DONE`
- Steps: `grep -c "git push\|cargo publish\|git tag" .github/skills/dev/git-workflow/release-workspace-package/SKILL.md`
  printed `0`. `grep -in "cargo publish\|releases/pkg\|release process" README.md packages/*/README.md` printed nothing.
- Conclusion: only pointers found; no README describes releasing, so none was changed (as the spec decided).

### V4 - Old links after the split

- Status: `DONE`
- Steps: `grep -n "^## Publishing a Workspace Package" docs/release_process.md` found the redirect heading, so
  `#publishing-a-workspace-package` still resolves; `linter markdown` exited 0 (it checks the links of
  `AGENTS.md`, `docs/index.md`, `docs/AGENTS.md` and the new documents); both workflow comments name the new documents.
- Conclusion: no dead link.

## Failures and Follow-up

None.
