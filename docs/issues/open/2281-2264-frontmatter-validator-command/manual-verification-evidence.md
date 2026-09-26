---
doc-type: manual-verification-evidence
issue-spec: docs/issues/open/2281-2264-frontmatter-validator-command/ISSUE.md
last-updated-utc: "2026-09-26 13:25"
---

# Manual Verification Evidence

## Purpose

Record real, human-oriented verification of the completed behavior. This is
evidence from commands or interactions actually performed against the artifact;
do not invent commands, output, logs, or results.

## Environment and Prerequisites

- Date and time (UTC): 2026-09-25 08:30-08:40
- Artifact under test: the `frontmatter-validator` crate on branch
  `2281-frontmatter-validator-command`. The broken state is the tree after
  `feat(frontmatter): [#2281] add severity and field path to diagnostics`; the fixed state adds
  `fix(frontmatter): [#2281] reject issue references truncated by unquoted YAML comments`.
- Operating system / environment: Linux; stable Rust toolchain for `cargo test`; Python 3 with
  PyYAML for the independent YAML parse.
- Prerequisites and setup performed: none beyond a repository checkout.

The `V` sections were run on 2026-09-26 13:00-13:25 UTC against the tree at
`docs(issues): [#2281] record T7 completion`, with the stable Rust toolchain and git 2.53.0.
`cargo run --offline` built the command. Failing scenarios ran in a disposable detached
`git worktree` under `.tmp/`, which has its own index; the main repository's index and working tree
stayed clean. Output lines are quoted as printed, except that the absolute binary path is shown as
`frontmatter-validator`.

## Verification Processes

Sections `V1`-`V7` record the issue's manual scenarios `M1`-`M7`. Bug-fix evidence uses `B`
sections.

### V1 - Focused Validation Passes (M1)

- Goal: validate this issue's spec and its folder with explicit paths.
- Status: `DONE`

```text
$ frontmatter-validator docs/issues/open/2281-2264-frontmatter-validator-command/ISSUE.md
exit=0 stdout_bytes=0 stderr_records=0

$ frontmatter-validator docs/issues/open/2281-2264-frontmatter-validator-command
exit=0 stdout_bytes=0 stderr_records=0
```

Conclusion: met. Clean runs are silent on both channels.

### V2 - Focused Validation Fails (M2)

- Goal: corrupt one field in a disposable copy and validate it.
- Status: `DONE`

```text
$ sed 's/^github-issue: 2281$/github-issue: "2281"/' .../ISSUE.md > .tmp/m2-corrupted-ISSUE.md
$ frontmatter-validator .tmp/m2-corrupted-ISSUE.md
exit=1 stdout_bytes=0 stderr_records=1
{"kind":"diagnostic","path":".tmp/m2-corrupted-ISSUE.md","severity":"error","category":"wrong-scalar-type","field_path":"github-issue","message":"`github-issue`: invalid type: string \"2281\", expected u64"}
```

Conclusion: met. One NDJSON error names the repository-relative path, category, and field path.

### V3 - Staged Mode Uses Index Content (M3)

- Goal: show that `--staged` validates the index, not the working copy, in both directions.
- Status: `DONE`

```text
# broken content staged, working copy restored to the valid version
index: github-issue: "2281"  working tree: github-issue: 2281
$ frontmatter-validator --staged
exit=1 stdout_bytes=0
{"kind":"diagnostic","path":"docs/issues/open/2281-2264-frontmatter-validator-command/ISSUE.md","severity":"error","category":"wrong-scalar-type","field_path":"github-issue",...}

# a valid change staged, working copy then broken
staged files: docs/issues/open/2281-2264-frontmatter-validator-command/ISSUE.md
index: github-issue: 2281  working tree: github-issue: "2281"
$ frontmatter-validator --staged
exit=0 stdout_bytes=0
```

The first reverse-direction attempt reset the index before running, so nothing was staged and the
pass proved nothing. It was redone with a real staged change, as shown.

Conclusion: met.

### V4 - Whole-Tree Mode (M4)

- Goal: run `--all` on the implementation branch and record the counts.
- Status: `DONE`

```text
$ frontmatter-validator --all
exit=1 stdout_bytes=0
     44 error   legacy-shape
      2 warning wrong-scalar-type
      1 warning invalid-reference-syntax
      1 error   invalid-allowed-value
```

The 44 `legacy-shape` errors are:

- 42 legacy issue specs;
- the unassigned draft EPIC;
- one draft whose envelope error had masked its legacy shape.

The one other error is the accepted AC10 exception: the completed #2324 spec, still in `open/`
with `status: open`. The warnings are advisory history in closed specs #2280, #2295, and #2308.

Conclusion: met, with the recorded #2324 exception.

### V5 - Pre-Commit Step (M5)

- Goal: run the real `pre-commit.sh` with staged invalid and then fixed content, for a v1 spec
  and for a legacy open spec.
- Status: `DONE`
- Setup: the disposable worktree shared the main `target/` through `CARGO_TARGET_DIR` and wrote
  logs under `.tmp/` through `TORRUST_GIT_HOOKS_LOG_DIR`.

```text
# M5a: staged invalid v1 spec (quoted github-issue)
hook exit=1
[Step 3/9] Checking staged Markdown frontmatter ... FAIL (2s)
{"kind":"diagnostic",...,"category":"wrong-scalar-type","field_path":"github-issue",...}

# M5a: fixed (only a valid timestamp change staged)
hook exit=0
[Step 3/9] Checking staged Markdown frontmatter ... PASS (0s)
SUCCESS: All pre-commit checks passed! (44s)

# M5b: staged text edit to legacy open spec #2179
hook exit=1
[Step 3/9] Checking staged Markdown frontmatter ... FAIL (1s)
{"kind":"diagnostic","path":"docs/issues/open/2179-fix-docker-e2e-package-flag/ISSUE.md","severity":"error","category":"legacy-shape","field_path":null,...}

# M5b: same edit plus a v1 migration (schema-version, quoted branch and timestamp)
hook exit=0
[Step 3/9] Checking staged Markdown frontmatter ... PASS (0s)
SUCCESS: All pre-commit checks passed! (27s)
```

The first M5b attempt appended an empty line, which made two consecutive blank lines. The
frontmatter step passed after migration, but `linter all` failed on markdownlint MD012 for that
throwaway edit. The case was redone with a text edit, as shown. #2179 is also closed on GitHub
while its spec is still in `open/`: another archive candidate, recorded in the issue progress log.

Conclusion: met. The named step fails and then passes in both cases.

### V6 - Invalid Invocation and Help (M6)

- Status: `DONE`

```text
$ frontmatter-validator
exit=2 stdout_bytes=0 stderr_records=1
{"kind":"usage_error","message":"error: the following required arguments were not provided:\n  <PATHS|--staged|--all>\n\nUsage: frontmatter-validator <PATHS|--staged|--all>\n\nFor more information, try '--help'.","exit_code":2}

$ frontmatter-validator --staged --all
exit=2 stdout_bytes=0 stderr_records=1
{"kind":"usage_error","message":"error: the argument '--staged' cannot be used with '--all'...","exit_code":2}

$ frontmatter-validator does/not/exist.md
exit=2 stdout_bytes=0 stderr_records=1
{"kind":"usage_error","message":"path `does/not/exist.md` does not exist","exit_code":2}

$ frontmatter-validator --version
exit=2 stdout_bytes=0 stderr_records=1
{"kind":"usage_error","message":"error: unexpected argument '--version' found...","exit_code":2}

$ frontmatter-validator --help
exit=0 stdout_bytes=0 stderr_records=1
{"kind":"help","message":"Validate the frontmatter of Markdown files against the repository's v1 contract\n\nUsage: frontmatter-validator <PATHS|--staged|--all>..."}
```

Conclusion: met. Each invocation writes exactly one D9 record and nothing on stdout.

### V7 - Offline (M7)

- Goal: repeat M1 and M4 without network access.
- Status: `DONE`

The commands ran inside `unshare -rn`, a user namespace with no network interfaces, with
`cargo run --offline`:

```text
network: unavailable
M7/M1 exit=0 stdout_bytes=0 stderr_records=0
M7/M4 exit=1 stdout_bytes=0
```

The M4 counts matched V4 exactly.

Conclusion: met. Nothing needed the network.

### B1 - Unquoted `issue #<n>` References Are Accepted as the Path `issue`

- Goal: reproduce `review-finding:pr-2337-f1` against the #2266 library, prove a maintained
  regression test fails for it, then recheck like-for-like after the fix.
- Initial state: the accepted fixtures and the
  `it_should_accept_all_provisional_related_artifact_forms` unit test write `- issue #<n>`
  without quotes.
- Status: `DONE`

#### Hypothesis

YAML treats `#` after whitespace as a comment, so `- issue #2264` parses as the plain string
`issue`. `issue` satisfies the repository-relative path alternative of the related-artifact
syntax, so the strict profile accepts it, and no test exercises the `issue #<n>` form.

#### Steps Performed

1. Reproduced the parse of the accepted fixtures with an independent YAML parser (PyYAML), reading
   `semantic-links.related-artifacts` from each fixture's frontmatter.
2. Selected the regression-test boundary as a unit test in `profile::tests`. The decision lives in
   the `RelatedArtifact` value type, which strict-profile validation exercises directly. Added
   `it_should_reject_an_unquoted_issue_reference_that_yaml_truncates_to_issue` before changing
   production code and ran it:
   `cargo test --package frontmatter-validator --lib unquoted_issue_reference` (stable Rust
   toolchain).
3. Fixed `is_related_artifact` and its schema pattern to reject the bare `issue` value with an
   actionable message. Quoted the references in both accepted fixtures and in the accept-all-forms
   unit test, then regenerated `docs/schemas/frontmatter-v1.schema.json`.
4. Re-ran the full crate tests. Then unquoted one fixture reference in the working tree to confirm
   the accepted-fixture test now fails for the old shape, and restored the fixture.
5. Repeated step 1 on the fixed fixtures.

#### Observed Result

Step 1, before the fix:

```text
issue ['issue', 'docs/templates/ISSUE.md', 'review-finding:pr-2230-f1']
epic ['issue', 'docs/templates/EPIC.md']
```

Step 2, red regression test:

```text
profile::tests::it_should_reject_an_unquoted_issue_reference_that_yaml_truncates_to_issue --- FAILED
called `Result::unwrap_err()` on an `Ok` value: Issue(Issue { ... semantic_links:
StrictSemanticLinks { skill_links: None, related_artifacts: Some([RelatedArtifact("issue")]) } })
test result: FAILED. 0 passed; 1 failed; 0 ignored; 0 measured; 47 filtered out
```

Step 4, green, then the unquoted fixture mutation:

```text
test result: ok. 48 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out
profile::tests::it_should_classify_the_accepted_issue_fixture_as_a_strict_issue_profile --- FAILED
test result: FAILED. 0 passed; 1 failed; 0 ignored; 0 measured; 47 filtered out
```

Step 5, like-for-like recheck after the fix:

```text
issue ['issue #2264', 'docs/templates/ISSUE.md', 'review-finding:pr-2230-f1']
epic ['issue #2003', 'docs/templates/EPIC.md']
```

The regenerated schema pattern was checked with Python `re` against the Rust predicate's boundary
cases. `issue` and `issue #0` are rejected. `issue #2264`, `issues/x.md`, `issue.md`,
`docs/issue`, and `Cargo.toml` are accepted.

#### Conclusion

The reproduced symptom is gone. Accepted fixtures now exercise the `issue #<n>` form, the
truncated form is rejected with `invalid-reference-syntax` on
`semantic-links.related-artifacts`, and both the new regression test and the accepted-fixture
test fail for the old shape.

## Failures and Follow-up

None.
