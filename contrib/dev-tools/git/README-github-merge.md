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

## Declared Symbolic Links

The merge tool refuses a merge that introduces a symbolic link, because a link is a way to make a reviewed path resolve somewhere else. A repository that carries a link on purpose declares it in a JSON file it commits, and the tool exempts exactly the declared links. The tool has no notion of that file's name or location: it reads whatever tree path the `--symlinks` argument names, and no name is special to it. The mechanism is opt-in per invocation: the tool holds no declaration path of its own, so a run that passes no `--symlinks` argument reads no declaration and refuses every link it finds, whatever the merged tree contains. `merge-pull-request.sh` passes `--symlinks .symlinks.json` unconditionally, and that line is the only place this repository's declaration path is stated.

This repository's tree carries no symbolic link and therefore ships no declaration file. The mechanism stays inert here until a link is declared, and the missing file is not an error.

### Declaration format

The declaration is a JSON object. This repository keeps it at `.symlinks.json` in the repository root, which is a convention of this repository stated once in `merge-pull-request.sh`, not a path the tool knows; a repository adopting the workflow may keep it anywhere in its tree and name it in its own wrapper. The shape below is what the tool reads at whatever path it is given:

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

`namespace` identifies the declaration format rather than the repository that carries it, so every Torrust repository uses the same value, `com.torrust.repository.symlinks`. A declaration is bound to its repository by where it is read from, that repository's own merged tree, so no string inside the file adds a guarantee on top of that. `version` is the version of the declaration format, not of the repository.

Each entry in `symlinks` describes one accepted link: `path` is repository-relative, `target` is the link's literal content, and `reason` records why the link exists so a maintainer reading the merge output can judge it. All three fields are required strings, and no two entries may name the same path.

### Rules

- The checked commits are every commit the merge introduces: the pull request's own commits in `pull/<n>/base..pull/<n>/head`, plus the local merge commit the tool has just created. Each is listed with `git ls-tree --full-tree -r -z <commit>`, using the same mode mask the tip-only check used. `-z` asks for the paths themselves rather than the quoted rendering git prints by default, so a declaration names the path a commit carries.
- Commits already reachable from the base branch are not part of what the merge introduces and are not walked. Existing history is never re-checked.
- The declaration is read from the final merged tree alone, out of the local merge commit, at the tree path `--symlinks` names. It is never read from the working directory, the index, the base branch, or an intermediate commit in the range, so no state outside the merge result can change the verdict. A declaration that reaches only one of those places exempts nothing.
- No intermediate commit's own copy of the declaration is ever consulted. One reviewed statement answers for the whole range.
- `--symlinks <path>` names a repository-relative path inside the final merged tree. It is a tree path rather than a filesystem path, so an absolute value, or one containing a `..` segment, is rejected as a usage error before any merge work starts. The argument has no default value.
- Matching runs from the trees to the declaration, never the reverse: the tool walks the links actually present in the checked commits and asks whether each one is declared. An entry can only ever remove a refusal for a link that exists, and can never introduce one.
- `target` must equal the link's literal content byte for byte in every checked commit that carries that link. A target that resolves to the same file by another spelling does not match. The comparison is on bytes, never on a decoded rendering of them: the declaration's UTF-8 encoding is matched against the link's content as the tree carries it, so a link whose content is not valid UTF-8 has no declaration it can equal and is refused, and two links whose contents differ can never both match one entry.
- `path` is matched the same way, against the path the tree carries rather than against any rendering of it.
- A target that is absolute, or that contains a `..` segment, is never accepted, whatever the declaration says. This is a property of the target itself, so no file can grant it.
- A symbolic link found in a checked commit and not covered by the final declaration, including one declared with a different target, produces `ERROR: File '<path>' was a symlink in commit <hash>` and exit code `4`. The message names the carrying commit in every case, the local merge commit included.
- A declaration entry whose path is not a symbolic link in the merged result is reported as a stale entry in the merge output. It does not refuse, and it may still be the entry that admits the same link in an earlier commit of the range, which is why the report says the entry can be dropped later rather than that it is unused.
- A declaration file absent from the final merged tree at that path is not an error and grants no exception: every symbolic link in every checked commit refuses exactly as it did before this mechanism existed.
- A declaration file that cannot be read as a valid declaration exempts nothing either. The tool reports why and then refuses links as if no declaration were present, so a broken file can never widen what is accepted, and a tree without links still passes.
- A run that passes no `--symlinks` argument performs no declaration processing at all: no file is read, nothing is exempted, and neither accepted-link nor stale-entry output is printed.
- Every path, target, and reason the report prints was chosen by whoever wrote the commit, so each is escaped before it is printed and a value carrying a newline or a terminal escape cannot forge a line of the report. Ordinary paths and targets print exactly as they read, and the escaping is a rendering only: it never changes what the rules above match.

Every accepted link is printed with its path, target, and reason before the maintainer is asked to sign, so what the merge admitted is visible at the moment the decision is made rather than afterwards.

### Removing a declared link takes two changes

Because the final declaration judges every commit the merge introduces, a pull request that deletes a declared link must keep the entry in the final merged tree whenever any of its own commits still carries the link, which is the ordinary case. Those pre-deletion commits are judged against the final declaration, so dropping the entry in the same pull request refuses the merge. The retained entry is then stale, which the merge output reports rather than refuses, and a later change removes it once no checked commit carries the link. This is what one reviewed declaration answering for a whole range costs.

## Provenance and License

`github-merge.py` came from the Bitcoin Core developers' `github-merge.py` (copyright 2016-2017), whose MIT license is in [`github-merge-COPYING`](github-merge-COPYING) and whose source header the file retains. It arrived with SHA-256 `e390eb014131f3183a2cba642134974a6b09b19a65322d17dd7c81cf4ffbaad2` in commit `833a4160e5753d54cde47bcc4bed25df7a04c0f6`, "feat(git): vendor maintainer merge workflow" (2026-07-23). That is the whole provenance record: where the file came from, under what license, and what it looked like when it arrived. It is permanent and does not change again.

From then on the copy is an ordinary file of this repository, modified under ordinary review like any other, rather than a mirror kept identical to its origin. Its consumers are Torrust repositories whose needs Bitcoin Core does not share, so there is nothing to re-sync with, and re-hashing the file after each change would answer a question nobody asks: git history already records what changed after arrival, and does it more precisely than a hash in a README could.

Keep this repository's copy the reference for sibling repositories that adopt the same workflow, and mirror changes into them deliberately rather than editing each copy on its own.

## Known Upstream Issues

These defects arrived with the copy and are recorded here rather than fixed. Each is a separate defect with its own review, and none blocks the workflow today; under the maintenance model above they are ordinarily fixable here when one is taken up. Line numbers refer to the file as it currently stands.

- **A failed comment fetch raises instead of exiting cleanly.** Line 565 adds the results of `retrieve_pr_comments(...)` and `retrieve_pr_reviews(...)` before the `if comments is None` check on line 566, so an unavailable GitHub API makes the addition raise a `TypeError` and the intended "Could not fetch PR comments and reviews" exit is never reached. The `finally` block still deletes the temporary branches; the maintainer sees a Python error report instead of the diagnostic.
- **The prompts never end at end of input.** `ask_prompt` (line 144) reads a reply with `stdin.readline()` (line 147), and the sign loop (lines 581-593) and push loop (lines 608-616) leave only on `s`, `x`, or `push`. When standard input is not a terminal every read returns immediately at end of input and those loops reprint their prompt forever. `merge-pull-request.sh` mitigates this by refusing to start the tool unless standard input is a terminal.
- **The missing-key message names the global configuration scope.** Line 418 prints `git config --global user.signingkey <key>`. Signing configuration for this repository is repository-local, so the wrapper's message is the authoritative one.

## Deterministic Coverage Boundary

Run `bash contrib/dev-tools/git/tests/test-merge-pull-request.sh` to test the wrapper's local,
non-destructive contract: argument validation, clean-tree protection, fixed repository
configuration, target-branch selection, signing-key presence, interactive-terminal refusal,
delegation of the symbolic-link declaration path, and `--dry-run` behavior. The test replaces
Python with a local stub to verify delegation without contacting GitHub.

Run `python3 contrib/dev-tools/git/tests/test-github-merge-symlinks.py` to test the symbolic-link
check inside the tool: which links a declaration admits, which it refuses and in which commit,
where the declaration may come from, what an omitted `--symlinks` argument does, and how the
report renders tree content that carries control characters. Each case
builds a repository and the bare upstream that publishes one pull request to it, and answers the
signing prompt with a refusal, so the tool's real fetch, merge, and check paths run with no
network and no GitHub API.

The vendored tool's GitHub API, credentials, interactive shell, GPG pinentry, signing, and push
paths are intentionally outside deterministic automated coverage. They require external services
or explicit maintainer approval; use the manual scenarios in the merge skill.

## Future Automation

This is an interim, versioned maintainer workflow related to EPIC \#2003. It does not select the
EPIC's final automation architecture. A later approved decision may migrate it to Rust or
replace it with another approved architecture.
