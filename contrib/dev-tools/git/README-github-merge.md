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

## Known Upstream Issues

These are upstream (Bitcoin Core) behaviors of the vendored copy, recorded here rather than
patched: the copy stays byte-identical and its provenance hash is unchanged, as the local-change
policy above requires.

- **A failed comment fetch raises instead of exiting cleanly.** Line 441 adds the results of
  `retrieve_pr_comments(...)` and `retrieve_pr_reviews(...)` before the `if comments is None` check
  on line 442, so an unavailable GitHub API makes the addition raise a `TypeError` and the intended
  "Could not fetch PR comments and reviews" exit is never reached. The `finally` block still deletes
  the temporary branches; the maintainer sees a Python error report instead of the diagnostic.
- **The prompts never end at end of input.** `ask_prompt` (line 144) reads a reply with
  `stdin.readline()` (line 147), and the sign loop (lines 457-470) and push loop (lines 484-492)
  leave only on `s`, `x`, or `push`. When standard input is not a terminal every read returns
  immediately at end of input and those loops reprint their prompt forever. `merge-pull-request.sh`
  mitigates this by refusing to start the tool unless standard input is a terminal.
- **The missing-key message names the global configuration scope.** Line 297 prints
  `git config --global user.signingkey <key>`. Signing configuration for this repository is
  repository-local, so the wrapper's message is the authoritative one.

## Deterministic Coverage Boundary

Run `bash contrib/dev-tools/git/tests/test-merge-pull-request.sh` to test the wrapper's local,
non-destructive contract: argument validation, clean-tree protection, fixed repository
configuration, target-branch selection, signing-key presence, interactive-terminal refusal, and
`--dry-run` behavior. The test replaces Python with a local stub to verify delegation without
contacting GitHub.

The vendored tool's GitHub API, credentials, interactive shell, GPG pinentry, actual merge, and
push paths are intentionally outside deterministic automated coverage. They require external
services or explicit maintainer approval; use the manual scenarios in the merge skill.

## Future Automation

This is an interim, versioned maintainer workflow related to EPIC \#2003. It does not select the
EPIC's final automation architecture. A later approved decision may migrate it to Rust or
replace it with another approved architecture.
