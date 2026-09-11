# Verification Evidence - Explicit Symlink Exceptions for the Vendored Maintainer Merge Tool

Evidence for the automatic checks and the manual verification scenarios of [issue #2175](ISSUE.md), recorded from runs against branch `2175-merge-tool-symlink-exceptions` at commit `116a020c`.

## Environment

- Date: 2026-09-09
- Host: project build server (Ubuntu), all runs at idle scheduling priority
- Python: 3.12.3
- Git: 2.43.0
- Branch and commit under test: `2175-merge-tool-symlink-exceptions` @ `116a020c`

## Automatic Checks

| Check | Command | Exit | Wall |
| ----- | ------- | ---- | ---- |
| Symbolic-link suite | `python3 contrib/dev-tools/git/tests/test-github-merge-symlinks.py` | 0 (18 tests, OK) | 6.6 s |
| Wrapper suite | `bash contrib/dev-tools/git/tests/test-merge-pull-request.sh` | 0 (all passed) | 1.2 s |
| Linters | `linter all` | 0 (all linters passed) | 18.1 s |
| Pre-commit hook | `TORRUST_GIT_HOOKS_LOG_DIR=.tmp ./contrib/dev-tools/git/hooks/pre-commit.sh` | 0 (6 of 6 steps PASS) | 42.9 s |
| Pre-push hook | `TORRUST_GIT_HOOKS_LOG_DIR=.tmp ./contrib/dev-tools/git/hooks/pre-push.sh` | 0 (4 of 4 steps PASS) | 185.7 s |

Both suites leave the working tree clean: `git status --porcelain` returns nothing after running them in sequence, which the merge workflow itself depends on because it refuses to start on a dirty tree.

## Manual Verification Scenarios

Each scenario builds a fixture repository together with the bare upstream that publishes one pull request to it. The upstream holds the target branch and the pull-request refs, and a URL rewrite sends the tool's fetch there, so the tool's own fetch, merge, and check code runs with no network and no GitHub API. The signing prompt is answered with `x`, so a run that passes the check exits `1` at that prompt and a run the check refuses exits `4` before the tree hash is computed. Every scenario except M8 passes `--symlinks .symlinks.json`, directly or through `merge-pull-request.sh`.

Commit identifiers below are fixture commits, recreated on each run; the transcripts are the record.

### M1 - Declared link is accepted (DONE)

The merged result commits both the link and a declaration whose target matches.

```text
---- pull request head commit: 471d8d6c4b81f3d63362fe6214750223d80453e0
---- github-merge.py --symlinks .symlinks.json 2175 develop
Accepted symlink: '.dockerignore' -> '.containerignore': Docker reads only .dockerignore, while Podman and Buildah prefer .containerignore.
...
Type 's' to sign off on the above merge, or 'x' to reject and exit.
Not signing off on merge, exiting.
echo $? -> 1
```

The accepted link is printed with its path, target, and reason; the run continues past the check, computes the tree hash, shows the merge details, and stops only at the signing prompt.

### M2 - Undeclared link is refused (DONE)

The M1 fixture with the entry removed from the committed declaration, that removal itself committed into the merged result.

```text
---- pull request head commit: e75a62b46733847f15720dfbb5ca35f6eb32a4e9
ERROR: File '.dockerignore' was a symlink in commit 471d8d6c4b81f3d63362fe6214750223d80453e0
ERROR: File '.dockerignore' was a symlink in commit e75a62b46733847f15720dfbb5ca35f6eb32a4e9
ERROR: File '.dockerignore' was a symlink in commit b5b9d2622b0d0592f35a58dfea4814bcf739108b
echo $? -> 4
```

Three refusals, one per checked commit that carries the link: the commit that added it, the commit that dropped its entry, and the local merge commit `b5b9d262`.

### M3 - Escaping targets are refused when declared (DONE)

An absolute target and a target containing a `..` segment, each committed alongside a declaration that names exactly that target.

```text
M3a  ln -s /etc/hostname .dockerignore, declared as "/etc/hostname"
ERROR: File '.dockerignore' was a symlink in commit 58c4a40ec3b68ada51edf85870e767ec9d98b9a8
ERROR: File '.dockerignore' was a symlink in commit 217d6f807732d836741ecf23daf5d8d897467b15
echo $? -> 4

M3b  ln -s ../outside-the-repository .dockerignore, declared as "../outside-the-repository"
ERROR: File '.dockerignore' was a symlink in commit f7bc555e901a70c009d83925fadab8f42e7b9435
ERROR: File '.dockerignore' was a symlink in commit e4e658093d848d6059758c9e1b8ed7b55807bf6b
echo $? -> 4
```

The declaration matches the link byte for byte in both runs and still grants nothing, which is the point: the target form decides, not the file.

### M4 - Missing declaration file changes nothing (DONE)

Run through `merge-pull-request.sh`, which passes `--symlinks .symlinks.json` unconditionally, against a merged result that carries no such file.

```text
---- merged result carries no .symlinks.json:
.containerignore
.dockerignore
README.md
---- ./contrib/dev-tools/git/merge-pull-request.sh 2175
ERROR: File '.dockerignore' was a symlink in commit 5659bb78e20a62a9790dfb484b2cc560bb083d03
ERROR: File '.dockerignore' was a symlink in commit dddc710beeec4cb9e360181d7ff47259eaddfddf
echo $? -> 4
```

The absent path produces today's refusal with exit code `4` and no error of its own: nothing reports a missing file, a failed read, or an unusable argument, which is what distinguishes "no exceptions" from "a tool error".

### M5 - Out-of-tree declaration exempts nothing (DONE)

```text
M5a  a valid declaration written into the working directory and left uncommitted
---- uncommitted declaration present in the working directory:
?? .symlinks.json
ERROR: File '.dockerignore' was a symlink in commit 5c02ae4f86f17ba1165a0fcb0ba084bf0011abe8
ERROR: File '.dockerignore' was a symlink in commit b941a491751beced31bc0cfaa3fd383d1cd4dcad
echo $? -> 4

M5b  a declaration committed on the base branch and deleted by the pull request
---- declaration on the base branch, absent from the merged result:
.symlinks.json
README.md
ERROR: File '.dockerignore' was a symlink in commit 4d73bf788b9b6a682e4fa5dc5a31d1e050b932e7
ERROR: File '.dockerignore' was a symlink in commit 39f1c867754a9ef799c13d0c136ff2071b55905a
echo $? -> 4
```

Only the merged result is consulted: neither the maintainer's working directory nor the base branch can change the verdict.

### M6 - Intermediate-commit link is refused (DONE)

A link added in one commit of the pull request and deleted in a later one, with no declaration anywhere.

```text
---- intermediate commit carrying the link: 143e1e712fe90b01f8b7aba3fd2cee0a4725c3eb
---- pull request head commit (carries no link): 341ebdf0279613d0e76b87e0517b212a6392e040
ERROR: File '.dockerignore' was a symlink in commit 143e1e712fe90b01f8b7aba3fd2cee0a4725c3eb
echo $? -> 4
```

Exactly one refusal, naming the intermediate commit. The tip and the merge commit carry no link and are not named, so a tip-only check would have passed this merge.

### M7 - Removal pull request needs the retained entry (DONE)

A base branch that already carries a declared link, and a pull request with one ordinary commit that still carries it followed by a commit deleting it.

```text
M7a  the same pull request also drops the manifest entry
---- pre-deletion commit: 35e14b54ea73c6ac07c9340c08ac611eda96b1ff
ERROR: File '.dockerignore' was a symlink in commit 35e14b54ea73c6ac07c9340c08ac611eda96b1ff
echo $? -> 4

M7b  the manifest entry is retained in the final tree
Accepted symlink: '.dockerignore' -> '.containerignore': Docker reads only .dockerignore, while Podman and Buildah prefer .containerignore.
WARNING: Declared symlink '.dockerignore' is not a symbolic link in the merged result; the entry in '.symlinks.json' is stale and can be dropped once no commit a merge introduces still carries the link.
...
Not signing off on merge, exiting.
echo $? -> 1
```

The first run refuses and names the pre-deletion commit; the second passes, admits that commit's link, and reports the retained entry as stale rather than refusing it.

### M8 - Omitted argument disables the mechanism (DONE)

The M1 fixture, whose merged result commits both the link and a declaration that matches it, invoked with no `--symlinks` argument.

```text
---- the merged result does carry the declaration:
{
  "namespace": "com.torrust.repository.symlinks",
  ...
      "path": ".dockerignore",
      "target": ".containerignore",
---- github-merge.py 2175 develop   (no --symlinks argument)
ERROR: File '.dockerignore' was a symlink in commit c5951c7fe0c6287d7722308ab1061bbe37cabfec
ERROR: File '.dockerignore' was a symlink in commit 4aa547600c2db725e9ff9f3814b2272f5a54d8a9
echo $? -> 4
```

The declaration is present in the merged result and is not read: the run refuses the link it declares, and prints neither accepted-link nor stale-entry output. The tool has no default path to find it by.

## Acceptance Criteria Evidence

| AC ID | Status | Evidence |
| ----- | ------ | -------- |
| AC1 | DONE | `contrib/dev-tools/git/README-github-merge.md`, section "Declared Symbolic Links": the format with field meanings, and the rule list including the target forms that are never accepted. |
| AC2 | DONE | M5a, M5b, and M8 above; `it_should_ignore_a_declaration_that_only_exists_in_the_working_directory` and `it_should_ignore_a_declaration_the_merged_result_does_not_carry`. The tool reads the declaration only with `git show <merge commit>:<path>`. |
| AC3 | DONE | M6 (intermediate commit named, tip carries nothing) and M7a (pre-deletion commit named); `it_should_not_walk_history_the_merge_does_not_introduce` shows a link removed from the base branch before the pull request branched is never named. |
| AC4 | DONE | M1 (byte-exact match admitted), M2 and M3 (no match, no admission); `it_should_reject_an_absolute_declaration_path` and `it_should_reject_a_declaration_path_that_escapes_the_repository` show the argument itself is validated, and M8 shows it has no default. |
| AC5 | DONE | M1 and M7b print `Accepted symlink: '<path>' -> '<target>': <reason>` before the signing prompt. The report is flushed explicitly, so it precedes the prompt even when standard output is not a terminal. |
| AC6 | DONE | M2, M3a, M3b, M5a, M5b, M6, M7a, M8: every refusal uses `ERROR: File '<path>' was a symlink in commit <hash>` and exits `4`. |
| AC7 | DONE | M7b: the retained entry is reported as stale and the merge passes the check. |
| AC8 | DONE | M8 (declaration present, argument omitted, nothing exempted, no accepted or stale output) and M4 (argument given, no file at that path, nothing exempted, no tool error). |
| AC9 | DONE | `contrib/dev-tools/git/merge-pull-request.sh` passes `--symlinks "${SYMLINK_DECLARATION}"` in its final `exec` with no filesystem check; the wrapper suite asserts the full delegated argument list and asserts the path is passed with no such file in the working tree. M4 is the same delegation observed end to end. |
| AC10 | DONE | `contrib/dev-tools/git/README-github-merge.md`, section "Provenance and License": one statement naming the origin, the license file, the original SHA-256, and the introducing commit, with no re-hashing or byte-identity requirement. |
