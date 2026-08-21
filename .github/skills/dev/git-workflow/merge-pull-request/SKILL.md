---
name: merge-pull-request
description: Safely construct, inspect, validate, sign, and optionally push a maintainer GitHub pull-request merge using the repository-local vendored tool. Use when asked to merge a pull request or perform a maintainer merge workflow.
metadata:
  author: torrust
  version: "1.0"
---

# Merging a Pull Request

Use this workflow only when a maintainer has selected an already reviewed pull request for
merging. It constructs a local merge commit for inspection. It does not replace maintainer
judgment, review, branch protection, or explicit authorization.

The repository-local entry point is:

```sh
./contrib/dev-tools/git/merge-pull-request.sh <pull-request-number>
```

It wraps the vendored `github-merge.py` tool, fixes the target to
`torrust/torrust-tracker:develop`, and creates temporary branches named:

- `pull/<number>/base`
- `pull/<number>/head`
- `pull/<number>/merge`
- `pull/<number>/local-merge`

The full provenance, license, deterministic test boundary, and EPIC #2003 relationship are in
[`contrib/dev-tools/git/README-github-merge.md`](../../../../../contrib/dev-tools/git/README-github-merge.md).

## Mandatory Guardrails

- Verify the target is `develop` and the Git working tree is clean before starting. Preserve
  unrelated work with a commit or a named stash; never use `git reset --hard` to discard it.
- Run the repository-local wrapper, not a personal path outside this repository.
- Inspect the temporary merge and run the required validation before considering a signature.
- Never type `s` to sign or `push` to push unless an authorized maintainer has explicitly
  confirmed that action in the current request.
- If GPG reports a timeout while signing, stop the failed attempt. Do not bypass signing or use
  `--no-gpg-sign`; ask the maintainer whether they prefer to retry the signed commit manually or
  have the agent rerun the same command while they enter the passphrase directly in the terminal
  prompt. Do not retry until the maintainer chooses, and never request or handle the passphrase
  in chat.

## Prerequisites

1. Confirm the upstream remote and target branch:

   ```sh
   git remote -v
   git switch develop
   git fetch <upstream-remote>
   git pull --ff-only <upstream-remote> develop
   git status --short --branch
   ```

Replace `<upstream-remote>` with the contributor-local remote name that points to
`torrust/torrust-tracker`; do not assume it is named `torrust`.

1. Configure the required local Git values. Use a fine-grained GitHub token with access to the
   upstream repository only when unauthenticated API access is insufficient; do not expose it in
   chat, commits, or command output.

   ```sh
   git config githubmerge.repository torrust/torrust-tracker
   git config --global user.signingkey <gpg-key-id>
   git config user.ghtoken <github-token>
   ```

   `user.ghtoken` is optional. `githubmerge.host` defaults to `git@github.com`; SSH credentials
   must permit fetching the upstream repository and pushing only after authorization. The wrapper
   passes `develop` directly, so `githubmerge.branch` is not required. Optional settings supported
   by the vendor tool are `githubmerge.testcmd`,
   `githubmerge.merge-author-email`, and `githubmerge.pushmirrors` (the latter applies only to
   its historical `master` behavior and is not used by this `develop` wrapper).

1. Confirm the installed hooks and signing setup. Hooks are installed with
   `./contrib/dev-tools/git/install-git-hooks.sh`. A real signing attempt requires an available
   GPG agent and pinentry session.

## Preflight and Merge Inspection

First perform the deterministic, non-destructive preflight:

```sh
./contrib/dev-tools/git/merge-pull-request.sh --dry-run <pull-request-number>
```

It validates the argument, clean tree, `githubmerge.repository`, current `develop` branch, and
`user.signingkey` without contacting GitHub, creating branches, merging, signing, or pushing.

If it passes and an authorized maintainer wants an inspection attempt, run:

```sh
./contrib/dev-tools/git/merge-pull-request.sh <pull-request-number>
```

The vendor tool fetches the pull request and upstream base, checks out its temporary branches,
and creates an unsigned local merge with `git merge --commit --no-edit --no-ff --no-gpg-sign`.
Inspect the displayed commit graph, merge title, PR description, and `git diff HEAD~`. If no
`githubmerge.testcmd` is configured, it starts an interactive shell for testing; exit that shell
only after inspection is complete.

Before starting the real tool, run the repository quality gate on clean `develop`. This detects a
mutating hook action before it can run inside the temporary merge. A hook must leave the merge
tree unchanged; a hook that rewrites files is a failed precondition, not a change to include in
the merge.

```sh
TORRUST_GIT_HOOKS_LOG_DIR=.tmp ./contrib/dev-tools/git/hooks/pre-commit.sh --format=json
```

If it formats `project-words.txt`, review and commit that canonical change separately, then
repeat the gate from clean `develop`. After the temporary merge is constructed, run the gate
again and confirm `git diff --exit-code` succeeds before signing. Review any warning that the
local merge differs from GitHub's merge; continue only with explicit maintainer judgment. The
vendor tool then adds review ACKs and the `Tree-SHA512` value to the merge message.

## Hook Side Effects and Recovery

The temporary `git merge --commit` runs installed `pre-commit` hooks. The current hook invokes
`format-project-words.sh`, which may rewrite `project-words.txt` and intentionally abort with a
non-zero exit. A mutating hook action therefore blocks the temporary merge: the merge tree no
longer matches the expected canonical tree and must not be signed as-is.

When a merge attempt fails or is rejected, first inspect `git status --short`. The wrapper's
clean-tree check means pre-existing unrelated work was rejected before the attempt. The vendored
tool calls `git merge --abort` after a hook failure and restores its temporary checkout; do not
use a hard reset. Then return safely to the target and remove only the named temporary state:

```sh
git merge --abort 2>/dev/null || true
git switch develop
git branch -D pull/<number>/head pull/<number>/base pull/<number>/merge pull/<number>/local-merge 2>/dev/null || true
git status --short --branch
```

If a pull request causes the dictionary formatter to abort the temporary merge, ask the PR author
to commit the canonical dictionary formatting, or prepare an approved follow-up commit; do not
retry a non-canonical merge. The vendor tool also performs this branch cleanup in its `finally`
block, but verify it after every failure. If a failure happens after local `develop` was reset to
the signed merge, use `git reflog` to identify the pre-merge tip and ask an authorized maintainer
before changing it.

## Signing and Push Confirmation

After successful inspection and validation, the tool prompts for `s` or `x`. Enter `x` unless
the maintainer has explicitly approved signing this exact inspected merge. After a successful
signature, it resets local `develop` to the signed temporary merge and deletes the temporary
branches. It then prompts for `push` or `x`.

Enter `push` only after separate, explicit maintainer confirmation to publish the signed merge to
the displayed remote and branch. Entering `x` leaves the signed local commit unpushed; report its
commit ID and wait for maintainer direction. Never push directly as an autonomous agent.

## Verification Boundaries

Run the deterministic wrapper coverage before changing repository-specific behavior:

```sh
bash contrib/dev-tools/git/tests/test-merge-pull-request.sh
```

Manual verification remains required for an authorized disposable pull request: prerequisite
discovery, non-destructive inspection and rejection, hook-side-effect recovery in an isolated
checkout, and signed completion with an explicit push confirmation. The tests intentionally do
not exercise GitHub networking, credentials, interactive shells, GPG pinentry, real merges, or
pushes because they cannot be safely deterministic.

## Relationship to EPIC #2003

Issue #2022 makes the current workflow reproducible now. It does not choose the automation
architecture proposed for evaluation in EPIC #2003. A future approved decision may migrate this
workflow to Rust or replace it with another approved architecture; keep repository-specific
integration narrow and preserve vendor provenance until that decision is implemented.
