---
doc-type: manual-verification-evidence
issue-spec: docs/issues/open/2281-2264-frontmatter-validator-command/ISSUE.md
last-updated-utc: "2026-09-25 08:40"
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

## Verification Processes

Sections `V1`-`V7` are reserved for the issue's manual scenarios `M1`-`M7`. Bug-fix evidence uses
`B` sections.

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
