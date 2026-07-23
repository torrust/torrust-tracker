# Maintainer Pull-Request Merge Tool

`merge-pull-request.sh` is the repository-local entry point for maintainers who construct a
local GitHub pull-request merge commit. It fixes the repository to
`torrust/torrust-tracker` and the target branch to `develop`, then invokes the vendored
`github-merge.py` tool.

Run the non-destructive preflight before a real merge attempt:

```sh
./contrib/dev-tools/git/merge-pull-request.sh --dry-run <pull-request-number>
```

For the interactive workflow, credentials, signing prerequisites, hook behavior, validation,
and recovery steps, follow the canonical
[`merge-pull-request` skill](../../../.github/skills/dev/git-workflow/merge-pull-request/SKILL.md).
The tool is intentionally not a replacement for maintainer review or explicit approval to sign
and push.

## Provenance and License

`github-merge.py` is a byte-identical vendor copy of the reviewed planning snapshot from issue
\#2022, SHA-256 `e390eb014131f3183a2cba642134974a6b09b19a65322d17dd7c81cf4ffbaad2`.
It originates from the Bitcoin Core developers (copyright 2016-2017) and retains its source
header. Its MIT license is in [`github-merge-COPYING`](github-merge-COPYING).

Local changes to the vendored algorithm require a documented security, portability, or
correctness reason and a new provenance hash. This integration deliberately confines
repository-specific behavior to `merge-pull-request.sh` so the vendor copy remains auditable.

## Deterministic Coverage Boundary

Run `bash contrib/dev-tools/git/tests/test-merge-pull-request.sh` to test the wrapper's local,
non-destructive contract: argument validation, clean-tree protection, fixed repository
configuration, target-branch selection, signing-key presence, and `--dry-run` behavior. The
test replaces Python with a local stub to verify delegation without contacting GitHub.

The vendored tool's GitHub API, credentials, interactive shell, GPG pinentry, actual merge, and
push paths are intentionally outside deterministic automated coverage. They require external
services or explicit maintainer approval; use the manual scenarios in the merge skill.

## Future Automation

This is an interim, versioned maintainer workflow related to EPIC \#2003. It does not select the
EPIC's final automation architecture. A later approved decision may migrate it to Rust or
replace it with another approved architecture.
