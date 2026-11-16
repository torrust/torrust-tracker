---
doc-type: manual-verification-evidence
issue-spec: docs/issues/open/2866-2003-move-skills-to-claude-skills/ISSUE.md
last-updated-utc: 2026-11-16 08:29
---

<!-- cspell:ignore Autocompact dataviz endswith findall isfile normpath relpath startswith -->

# Manual Verification Evidence

## Purpose

Record real, human-oriented verification of the completed behavior. This is
evidence from commands or interactions actually performed against the artifact;
do not invent commands, output, logs, or results.

## Environment and Prerequisites

- Date and time (UTC): 2026-11-15 20:48 (V0)
- Artifact under test: V0: `develop` at `abd7a10ff` (`Merge torrust/torrust-tracker#2863`), before
  any change of this issue.
- Operating system / environment: Linux 7.0.0-43-generic; Claude Code 2.1.318
  (`claude --version`); model reported by `/context`: `claude-sonnet-5-5`, 1M context window.
- Prerequisites and setup performed: a fresh local clone of `develop` in a scratch folder with no
  `.claude/` folder (`ls .claude` failed with "No such file or directory");
  `find . -name SKILL.md -not -path './.git/*' | wc -l` printed `53`.

## Verification Processes

### V0 - Reproduce the defect before T3 (M0)

- Goal: show, in the real repository, that Claude Code does not discover the repository's skills.
- Initial state: fresh clone at `abd7a10ff`, detached `HEAD`, clean tree.
- Status: `DONE` (outcome: **Reproduced**)

#### Steps Performed

The two commands ran one after the other in one shell, at the clone's root. The times are the
output files' birth and last-modification times.

1. 20:48:11.17 to 20:48:13.86 UTC: `claude -p "/context" > m0-context.md`
2. 20:48:14.77 to 20:48:26.22 UTC: `claude -p "/create-issue" > m0-create-issue.txt`

#### Observed Result

Step 1, the category table, the Memory Files table and the whole Skills table of the output,
verbatim; the long scratch path of the clone in the Memory Files row is shortened to `<clone>`:

```text
### Estimated usage by category

| Category | Tokens | Percentage |
|----------|--------|------------|
| System prompt | 2.6k | 0.3% |
| System tools | 18.4k | 1.8% |
| MCP tools | 559 | 0.1% |
| MCP server instructions | 1.6k | 0.2% |
| MCP tools (deferred) | 13.1k | 1.3% |
| System tools (deferred) | 22.7k | 2.3% |
| Memory files | 5k | 0.5% |
| Skills | 4.6k | 0.5% |
| Messages | 10 | 0.0% |
| Free space | 934.2k | 93.4% |
| Autocompact buffer | 33k | 3.3% |

### Memory Files

| Type | Path | Tokens |
|------|------|--------|
| Project | <clone>/AGENTS.md | 5k |

### Skills

| Skill | Source | Tokens |
|-------|--------|--------|
| dataviz | Built-in | ~480 |
| artifact-design | Built-in | ~70 |
| artifact-diagramming | Built-in | ~70 |
| artifact-capabilities | Built-in | ~250 |
| update-config | Built-in | ~240 |
| keybindings-help | Built-in | ~80 |
| code-review | Built-in | ~280 |
| simplify | Built-in | ~60 |
| fewer-permission-prompts | Built-in | ~60 |
| loop | Built-in | ~120 |
| schedule | Built-in | ~130 |
| claude-api | Built-in | ~360 |
| workflow-authoring | Built-in | ~80 |
| run | Built-in | ~120 |
| plugin-authoring | Built-in | ~90 |
| init | Built-in | ~20 |
| security-review | Built-in | ~30 |
| docs | claude.ai sync | ~340 |
| docx | claude.ai sync | ~320 |
| google-workspace | claude.ai sync | ~330 |
| import-memory | claude.ai sync | ~60 |
| morning | claude.ai sync | ~120 |
| pdf | claude.ai sync | ~150 |
| pptx | claude.ai sync | ~330 |
| skill-creator | claude.ai sync | ~120 |
| xlsx | claude.ai sync | ~320 |
```

The Skills table has 26 rows: 17 with source `Built-in` and 9 with source `claude.ai sync`, and
none with source `Project` (`sed -n '/### Skills/,$p' m0-context.md | grep -c '| Project |'`
printed `0`). The only `Project` row in the whole output is the memory file `AGENTS.md`
(`grep -c 'Project' m0-context.md` printed `1`). The MCP Tools table between the category table
and the Memory Files table is left out: it lists connector tools only. The
4.6k-token Skills total equals the empty-repository baseline in the spec's pre-spec checks.

Step 2, the first two sentences of the reply:

```text
The repository has a `create-issue` skill (`.github/skills/dev/planning/create-issue/SKILL.md`). Your `/create-issue` command didn't run in this session, and the skill needs a description of the issue to draft.
```

#### Conclusion

Reproduced. No repository skill is discovered, and `/create-issue` does not run as a command. The
model found the skill file by reading the repository instead, which is step 2 of the spec's
"bug in plain terms".

### T2 - The placement rule fails on the tree before the move (red)

- Goal: show that the regression test of the spec's Regression Test Strategy fails for the defect.
- Initial state: branch `2866-move-skills-to-claude-skills` at `4342dd654` (`Merge
  torrust/torrust-tracker#2869`), with the T2 change in the working tree and no skill moved.
- Status: `DONE` (outcome: **Red**, as expected)

#### Steps Performed

1. 2026-11-16 08:03 UTC, stable Rust toolchain:
   `cargo run -q -p frontmatter-validator --bin frontmatter-validator -- --skill-layout 2> t2-red.ndjson`

#### Observed Result

The command exited `1` and wrote 53 records, all of kind `diagnostic`, severity `error` and
category `misplaced-skill`, one for each `SKILL.md` under `.github/skills/`; no record names
another path. The first record, verbatim:

```json
{"kind":"diagnostic","path":".github/skills/add-new-skill/SKILL.md","severity":"error","category":"misplaced-skill","field_path":null,"message":"A skill must be at `.claude/skills/<name>/SKILL.md`, where Claude Code discovers it."}
```

#### Mutations

Each mutation was made in the working tree, compiled, run with
`cargo test -q -p frontmatter-validator --lib skills`, and restored:

| Mutation in `src/skills.rs`                                         | Failing test case                                   |
| ------------------------------------------------------------------- | --------------------------------------------------- |
| Accept a skill folder that contains `/` (one folder too deep)       | `..._would_not_discover::case_2_one_folder_too_deep` |
| Skip paths under `.github/skills/`                                  | `..._would_not_discover::case_1_under_the_copilot_folder` |

After the restore, the 9 `skills` tests passed.

#### Test-Design Review

- `it_should_report_a_skill_that_claude_code_would_not_discover` and
  `it_should_accept_a_path_that_is_a_discoverable_skill_or_no_skill`: the path is the whole
  initial state and the one causal difference, named by each `case`. The Act is the production
  call `misplaced_skill(path)`; the expected result is written in the test (the
  `misplaced-skill` category, or no finding). Written with temporary Arrange, Act and Assert
  comments, the Arrange was empty and the other two restated the single line under them, so the
  comments were removed. No fixture or helper.
- `it_should_check_the_skill_layout_of_the_index_only_with_skill_layout` (`tests/cli.rs`): the
  binary decides which files the rule sees, so this test owns only that. The one causal
  difference is whether a misplaced skill is in the index; both skills are misplaced, so the
  library's placement decision is not what the test observes. The first draft also staged a
  correctly placed skill; that tested the library's rule a second time, so it was removed. The
  repository fixture holds only incidental mechanics (a disposable git repository).
- Unit-test level: the placement decision is pure and observable from `misplaced_skill`, so it is
  a unit test; the index selection needs a real git index, so it is a command-boundary test.

### T3 - The move and the reference sweep

- Goal: record the commands that moved the skills and classified and rewrote the references, so a
  reviewer can reproduce the sweep (spec, Classification rule for references).
- Initial state: branch `2866-move-skills-to-claude-skills` after the T2 commit.
- Status: `DONE`

#### Steps Performed

1. Built the name and area map from the 53 `SKILL.md` paths, then moved each skill folder:

   ```bash
   git ls-files .github/skills | grep 'SKILL.md$' | while read f; do d=$(dirname $f); n=$(basename $d); rel=${d#.github/skills/}; echo -e "$n\t$(dirname $rel)\t$d"; done | sort > .tmp/skill-map.tsv
   mkdir -p .claude/skills
   while IFS=$'\t' read n area d; do git mv "$d" ".claude/skills/$n"; done < .tmp/skill-map.tsv
   ```

   `git ls-files .github/skills | wc -l` printed `0`, `git ls-files .claude/skills | wc -l`
   printed `71`, and every staged change was a rename.
2. Ran the rewrite script below from the repository root (`python3 -I rewrite_skill_paths.py`).
   It printed 121 link rewrites and 227 literal rewrites in 171 files, and two links it could not
   map because they named an area folder, not a skill.
3. Reviewed the result and corrected it by hand:
   - restored the files the literal rewrite must not change: this issue's own spec and evidence
     (they describe the old layout), the dated report
     `docs/issues/open/2264-2003-refactor-semantic-link-conventions/external-link-check-residual-failures-2026-09-18.md`,
     the verbatim `docs/issues/drafts/2003-mine-ai-agent-memories/github-copilot-memory-snapshot.md`,
     and the placement test case in `src/skills.rs`, which must keep an old path;
   - the two folder links: `AGENTS.md` now links `.claude/skills/`, and `docs/testing.md` links
     `../.claude/skills/` until the T5 index exists;
   - prose that names the folder in general: `AGENTS.md` (four places),
     `.github/agents/README.md`, `setup-dev-environment`, `docs/agents/orchestration.md`, the
     #1347 sub-issue guidelines, the #2758 EPIC, #2767, the #2003 EPIC `related-artifacts`, and
     the validator README;
   - `frontmatter-validator`: `skill-links` now resolve against `.claude/skills/<name>/SKILL.md`
     only, through the placement rule's `skill_name`, with test cases for a skill one folder too
     deep and one under the old folder;
   - `validate-skill-links.sh` walks four folders up to the root instead of six.
   - five legacy specs whose paths changed failed the commit gate with `legacy-shape`, so they
     were migrated to v1 in the same commit, as the validator README requires:
     `docs/issues/drafts/increase-main-app-integration-test-coverage/ISSUE.md`,
     `docs/issues/drafts/optimize-event-publication-without-consumers/ISSUE.md`, and the open
     specs of #1768, #1774 and #1843. The migration added `schema-version: 1` and `epic: null`,
     quoted `last-updated-utc` (`00:00` where no time was recorded), replaced `status: open` with
     `planned` (#1843), gave the first draft a `{issue-number}-...` branch instead of `null`, and
     dropped its `related-artifacts` entry `tests/stats.rs`, deleted in `8151e920c` (2026-07-29).
4. Checked the result (stable Rust toolchain): `frontmatter-validator --skill-layout` exited `0`
   with no record; `linter lychee` exited `0`; `--all` reported only the 15 legacy-shape errors it
   reported before; the tests of `frontmatter-validator`, `agent-review-report-contract`,
   `github-review-threads` and `wait-for-pr-review`, and
   `python3 -m unittest discover -s .claude/skills/process-pr-review/scripts`, passed;
   `bash .claude/skills/run-tracker-locally/scripts/validate-skill-links.sh` printed
   `Skill links validation passed`.

#### Observed Result

`linter lychee` (`torrust-linting` 0.2.0) passes `lychee` the globs `**/*.md` and
`.github/**/*.md`; the first skips hidden folders. So the skills were link-checked under
`.github/skills/`, and after the move they are not: a broken link appended to
`.claude/skills/fix-bug/SKILL.md` was not reported, while one appended to
`.github/workflows/AGENTS.md` was. Closing that gap needs a change in `torrust-linting`; it is
recorded under Failures and Follow-up. Meanwhile a separate check resolved every relative
Markdown link outside code fences in the tracked Markdown under `.claude/skills/` and `.github/`:
115 links, none broken.

Historical records keep their plain-text paths and frontmatter; their Markdown links were
rewritten, because `lychee` checks them (only `docs/issues/closed/` is excluded). The literal
rewrite skipped `docs/issues/closed/`, `docs/pr-reviews/pr-*`, `docs/copilot-pr-reviews/pr-*`,
the closed refactor plans, `docs/discussions/`, the runner reports, and the
`.github/agents/*.agent.md` profiles (out of scope).

#### The Rewrite Script

<details>
<summary><code>rewrite_skill_paths.py</code> (one-off, not kept in the tree)</summary>

```python
"""Rewrite references to the moved skills (issue #2866, T3).

Run from the repository root after `git mv` of every skill folder is staged.

1. Markdown link targets: in every tracked Markdown file the link checker checks (all but
   docs/issues/closed/ and docs/external-snapshots/), each relative link is resolved against the
   file's OLD location, the target is mapped to its NEW location, and the link is recomputed from
   the file's NEW location. This fixes depth changes inside moved skills and links into skills.
2. Literal paths: in live files only, `.github/skills/<old skill dir>` becomes
   `.claude/skills/<name>`.

Prints every change as `file<TAB>kind<TAB>old<TAB>new` on stdout; unresolved targets under the old
folder go to stderr.
"""

import os
import re
import subprocess
import sys

OLD_ROOT = ".github/skills"
LINK_EXCLUDED = ("docs/issues/closed/", "docs/external-snapshots/")
HISTORICAL = (
    "docs/issues/closed/",
    "docs/pr-reviews/pr-",
    "docs/copilot-pr-reviews/pr-",
    "docs/refactor-plans/closed/2",
    "docs/refactor-plans/closed/agent-docs-refactor-plan/",
    "docs/discussions/",
    "docs/self-hosted-runner-reports/2",
    "docs/external-snapshots/",
)
OUT_OF_SCOPE = re.compile(r"^\.github/agents/.*\.agent\.md$")


def git(*args):
    return subprocess.run(["git", *args], check=True, capture_output=True, text=True).stdout


def renames():
    files = {}
    for line in git("diff", "--cached", "-M", "--name-status").splitlines():
        parts = line.split("\t")
        if parts[0].startswith("R") and parts[1].startswith(OLD_ROOT + "/"):
            files[parts[1]] = parts[2]
    return files


FILES = renames()
DIRS = {}
for old, new in FILES.items():
    if old.endswith("/SKILL.md"):
        DIRS[os.path.dirname(old)] = os.path.dirname(new)
NEW_TO_OLD = {new: old for old, new in FILES.items()}


def new_of(old_path):
    """The new repository path of an old path, or the path itself when it did not move."""
    if old_path in FILES:
        return FILES[old_path]
    for old_dir, new_dir in DIRS.items():
        if old_path == old_dir:
            return new_dir
        if old_path.startswith(old_dir + "/"):
            return new_dir + old_path[len(old_dir):]
    if old_path == OLD_ROOT or old_path.startswith(OLD_ROOT + "/"):
        return None
    return old_path


LINK = re.compile(r"(\]\()(<?)([^)\s>]+)(>?)((?:\s+\"[^\"]*\")?\))|^(\[[^\]]+\]:\s+)(\S+)", re.M)


def rewrite_target(target, old_file, new_file, report):
    if re.match(r"^[a-z][a-z0-9+.-]*:", target, re.I) or target.startswith(("#", "/")):
        return target
    path, sep, fragment = target.partition("#")
    if not path:
        return target
    old_target = os.path.normpath(os.path.join(os.path.dirname(old_file), path))
    new_target = new_of(old_target)
    if new_target is None:
        report.append((new_file, target))
        return target
    if new_target == old_target and old_file == new_file:
        return target
    relative = os.path.relpath(new_target, os.path.dirname(new_file) or ".")
    if path.endswith("/") and not relative.endswith("/"):
        relative += "/"
    if os.path.normpath(os.path.join(os.path.dirname(new_file), path)) == new_target:
        return target  # still correct, keep the author's spelling
    return relative + sep + fragment


def main():
    tracked = git("ls-files").splitlines()
    literal = sorted(DIRS.items(), key=lambda item: -len(item[0]))
    unresolved = []
    for new_file in tracked:
        if not os.path.isfile(new_file):
            continue
        old_file = NEW_TO_OLD.get(new_file, new_file)
        try:
            text = open(new_file, encoding="utf-8").read()
        except UnicodeDecodeError:
            continue
        original = text
        if new_file.endswith(".md") and not new_file.startswith(LINK_EXCLUDED):
            def replace(match):
                if match.group(1):
                    target = match.group(3)
                    new = rewrite_target(target, old_file, new_file, unresolved)
                    if new != target:
                        print(f"{new_file}\tlink\t{target}\t{new}")
                    return match.group(1) + match.group(2) + new + match.group(4) + match.group(5)
                target = match.group(7)
                new = rewrite_target(target, old_file, new_file, unresolved)
                if new != target:
                    print(f"{new_file}\tlink\t{target}\t{new}")
                return match.group(6) + new
            text = LINK.sub(replace, text)
        live = not new_file.startswith(HISTORICAL) and not OUT_OF_SCOPE.match(new_file)
        if live:
            for old_dir, new_dir in literal:
                if old_dir in text:
                    count = len(re.findall(re.escape(old_dir) + r"(?![\w-])", text))
                    text = re.sub(re.escape(old_dir) + r"(?![\w-])", new_dir, text)
                    if count:
                        print(f"{new_file}\tliteral\t{old_dir}\t{new_dir}\t{count}")
        if text != original:
            open(new_file, "w", encoding="utf-8").write(text)
    for file, target in unresolved:
        print(f"UNRESOLVED\t{file}\t{target}", file=sys.stderr)


main()
```

</details>

### T6 - Validator rule increments and their test-design reviews

Each increment was run with `cargo test -p frontmatter-validator` and
`cargo clippy -p frontmatter-validator --all-targets -- -D warnings` (stable Rust toolchain), then
mutated, reviewed and committed before the next started.

#### Increment 1 - The stub inventory

- Rules: an inventoried old `SKILL.md` path is exempt from the placement rule; an inventoried path
  with no stub is `missing-redirect-stub`; any other file under `.github/skills/` except the
  inventory is `stray-redirect-file` (a `SKILL.md` there stays `misplaced-skill`). The binary
  reads the inventory from the index.
- After the increment, `frontmatter-validator --skill-layout` on the branch exited `0` with no
  record: the 53 stubs are inventoried.
- Mutations, each restored afterwards:

  | Mutation                                         | Failing tests                                                                                     |
  | ------------------------------------------------ | ------------------------------------------------------------------------------------------------- |
  | No exemption for inventoried paths               | `..._accept_an_inventoried_redirect_stub...`, `..._no_redirect_stub`, `..._does_not_list`          |
  | Never report a missing stub                      | `..._whose_redirect_stub_was_deleted`                                                              |
  | Never report a stray file                        | `..._that_is_no_redirect_stub`                                                                     |
  | The binary reads the inventory from the working tree | `it_should_read_the_stub_inventory_from_the_index_with_skill_layout` (`tests/cli.rs`)          |

  A first version of the third mutation left `stray_redirect_file` unused, which the workspace
  lints reject; it ran no test, so it was replaced by one that compiles.
- Test-design review: each unit test varies one thing against the same accepted state (one stub,
  inventoried): the stub is deleted, a skill the inventory does not list is added, or a non-stub
  file is added. The Act is `layout_findings` itself; the expected `(path, category)` pairs are
  written in each test. Temporary Arrange, Act and Assert comments added nothing the variable
  names and the blank lines did not already say, so they were removed. The helpers `inventory`
  and `layout` hold only incidental mechanics: rendering the inventory table, and reducing
  findings to their path and category. The command-boundary test owns only where the inventory
  is read from; its one causal difference is the working-tree inventory that differs from the
  staged one.

#### Increment 2 - The stub contents

- Rules: a stub whose inventory row does not say `removed` has no frontmatter and exactly one
  link, which resolves to `.claude/skills/<current name>/SKILL.md`, and that file exists;
  otherwise `invalid-redirect-stub`. A stub of a removed skill is not checked. The binary reads
  the stubs from the index too.
- After the increment, `frontmatter-validator --skill-layout` on the branch exited `0` with no
  record.
- Mutations, each restored afterwards:

  | Mutation                                   | Failing tests                                                  |
  | ------------------------------------------ | -------------------------------------------------------------- |
  | Accept frontmatter                         | `..._does_not_lead_to_its_skill::case_1_frontmatter`           |
  | Accept any number of links                 | `::case_2_no_link`, `::case_3_two_links`                       |
  | Do not check that the skill exists         | `..._whose_skill_was_deleted`                                  |
  | Do not check where the link points         | `::case_4_link_to_another_skill`                               |
  | Check a stub of a removed skill            | `..._of_a_removed_skill_without_a_link`                        |

  Three first attempts left a variable or function unused, which the workspace lints reject;
  they ran no test and were replaced by mutations that compile.
- Test-design review: the increment-1 tests used two helpers that rebuilt the whole snapshot in
  each test. Once stub texts mattered, that hid the one difference under repeated setup, so the
  fixture became `Snapshot::accepted()` (one moved skill, its stub, its inventory row), and each
  test changes it once with `with` or `without`: the causal difference is the only line of the
  test's Arrange. The renamed and removed cases change three things because the rename or
  removal is those three edits together (the skill folder, the stub, the inventory row); the
  test names say so. The expected findings are written in each test with `only(path,
  category)`. The fixture owns only incidental mechanics: the stub's wording and the inventory
  table's layout.

#### Increment 3 - The skill name

- Rule: the frontmatter `name` of `.claude/skills/<folder>/SKILL.md` equals `<folder>`; a
  different, missing, or unreadable `name` is `skill-name-mismatch`. Claude Code lists a skill by
  this name, and a stub's link and the index rows rely on it matching the folder.
- After the increment, `frontmatter-validator --skill-layout` on the branch exited `0`: all 53
  names match their folders.
- Mutations, each restored afterwards: accepting a missing `name` failed `::case_2_no_name` and
  `::case_3_no_frontmatter`; accepting a different `name` failed `::case_1_another_name`.
- Test-design review: the accepted snapshot's skill now has a real frontmatter, written by the
  `skill(name)` helper, so the earlier tests stay accepted for the reason they claim. The new
  test replaces only the skill's text; its three cases name the one difference each. The
  command-boundary fixture gained a `name` for the same reason; what it observes did not change.

#### Increment 4 - The index

- Rule: the table rows of `.claude/skills/README.md` link every skill folder exactly once, as
  `<name>/SKILL.md`, and link no missing skill; otherwise `skill-index-mismatch`, reported on
  the index. A missing index is a finding only when a skill exists. Links outside table rows do
  not count: the first run on the branch reported `add-new-skill` twice, because the index's
  introduction links it too.
- The rules now read a file's text only when the file is in the snapshot, so the binary no longer
  guards that itself.
- After the increment, `frontmatter-validator --skill-layout` on the branch exited `0` with no
  record.
- Mutations, each restored afterwards:

  | Mutation                               | Failing tests                                                                         |
  | -------------------------------------- | ------------------------------------------------------------------------------------- |
  | Never report an unlisted skill         | `..._exactly_once::case_1_unlisted_skill`                                             |
  | Never report a repeated row            | `..._exactly_once::case_2_listed_twice`                                               |
  | Never report a listed missing skill    | `::case_3_listed_but_missing`, `..._index_row_of_a_deleted_skill`                     |
  | Count links outside table rows         | `it_should_count_only_the_index_table_rows`                                           |
  | Read the text of a file not in the snapshot | `it_should_report_a_missing_index`                                               |

- Test-design review: the accepted snapshot gained the index that lists its skill, so every
  earlier test stays accepted for its own reason. Deleting a skill now yields two findings, its
  stub and its index row; the test was renamed to say so, and it is the behavior scenario M5
  expects. The rename and removal tests update the index too, because a rename or removal
  includes that edit. The index test's cases each change one thing: an extra skill, a repeated
  row, or a row for a missing skill. The no-skill test calls `layout_findings` directly with no
  fixture, because the accepted snapshot has a skill by design.

#### Wiring

`--skill-layout` runs as step 4 of `contrib/dev-tools/git/hooks/pre-commit.sh`, after the staged
frontmatter check, and as the step `Check Agent Skill Layout` of the Docs Lint workflow, which
runs on every push and pull request, documentation-only ones included. The
`run-pre-commit-checks` skill lists the new step. Green run on the branch, 2026-11-16 08:24 UTC, as the hook step of the wiring commit,
stable Rust toolchain: `cargo run -q -p frontmatter-validator --bin frontmatter-validator --
--skill-layout` exited `0` and wrote no record (the red run is under T2).

## Failures and Follow-up

- `linter lychee` does not check `.claude/**/*.md` (T3): `torrust-linting` passes only
  `**/*.md` and `.github/**/*.md`. Follow-up: add `.claude/**/*.md` there, then confirm a broken
  link in a skill fails `linter lychee`.
