---
doc-type: manual-verification-evidence
issue-spec: docs/issues/drafts/2264-extend-strict-profiles-and-author-guidance/ISSUE.md
last-updated-utc: "2026-10-03 11:48"
---

# Manual Verification Evidence

## Purpose

Record real, human-oriented verification of the completed behavior. This is evidence from commands or interactions actually performed against the artifact; do not invent commands, output, logs, or results.

## Environment and Prerequisites

- Date and time (UTC): 2026-10-03 11:31 (attempt while drafting); 2026-10-03 11:44 (reproduction run)
- Artifact under test: the frontmatter guidance in `docs/skills/semantic-skill-link-convention.md` and the `frontmatter-validator` command, both as on `develop` at the base of the spec branch; the draft itself is at the commit "docs(issues): [#2264] name the spec-only branch in the row 3 draft" for the attempt, and the reproduction run used the branch head at the commit "docs(pr-reviews): audit PR #2425 Copilot findings", which leaves the guidance and the validator unchanged.
- Operating system / environment: the attempt while drafting ran in a Linux sandbox without a Rust toolchain (`cargo` and `rustc` are absent), which cannot build or run `frontmatter-validator`. The reproduction run used the loop's compute pod, a Linux container with the workspace toolchain: `cargo 1.101.0-nightly (f3865b2a4 2026-09-29)` and `rustc 1.101.0-nightly (21b707e3f 2026-09-30)`.
- Prerequisites and setup performed: two disposable specs written from the convention's field list (reproduced verbatim below); no repository file was changed to prepare them.

## Verification Processes

### B1 - Stale Convention Field List (Reproduction Before Review)

- Goal: show that an issue spec written from the "Required metadata fields for issue specs" list in `docs/skills/semantic-skill-link-convention.md` is rejected by the pre-commit frontmatter validator.
- Initial state: the convention's list omits `schema-version` and `epic`, offers an `open` status, and shows an unquoted `last-updated-utc`; the strict v1 issue profile in `contrib/dev-tools/checks/frontmatter-validator/src/profile.rs` requires `schema-version` and `epic`, rejects `open`, and requires a double-quoted timestamp.
- Status: `DONE`
- Reproduction outcome: **Reproduced** on 2026-10-03 11:44 UTC on the loop's compute pod: both runs exited `1` with exactly the predicted record. The attempt while drafting was **Infeasible**, because the drafting environment has no Rust toolchain; its field-by-field comparison and dispatch trace below predicted both records, and the recorded commands then ran unchanged on the pod.

#### Steps Performed

1. Wrote the first disposable spec, following the list and the convention's "Recommended shape" with each placeholder filled by a value the list offers, for placement at `docs/issues/drafts/reproduce-convention-issue-fields/ISSUE.md`:

   ```yaml
   ---
   doc-type: issue
   issue-type: task
   status: draft
   priority: p2
   github-issue: null
   spec-path: docs/issues/drafts/reproduce-convention-issue-fields/ISSUE.md
   branch: reproduce-convention-issue-fields
   related-pr: null
   last-updated-utc: 2026-10-03 11:35
   semantic-links:
     skill-links:
       - create-issue
     related-artifacts:
       - docs/skills/semantic-skill-link-convention.md
   ---
   ```

2. Wrote the second disposable spec, identical except for an added first line `schema-version: 1` and the `-v1` suffix in its folder, `spec-path`, and `branch`, for placement at `docs/issues/drafts/reproduce-convention-issue-fields-v1/ISSUE.md`.
3. Prepared the commands, to run from the repository root with the stable Rust toolchain after placing each spec at its path:

   ```text
   cargo run --package frontmatter-validator --bin frontmatter-validator -- docs/issues/drafts/reproduce-convention-issue-fields/ISSUE.md
   cargo run --package frontmatter-validator --bin frontmatter-validator -- docs/issues/drafts/reproduce-convention-issue-fields-v1/ISSUE.md
   ```

4. Attempted to run them in the drafting environment: not possible, because `cargo` is absent.
5. Compared the convention's list with the strict issue profile, field by field, and traced how the validator dispatches each spec.
6. On the loop's compute pod, from the repository root at the branch head, placed both specs at their paths, ran the two commands with the toolchain above, removed the specs, and confirmed that `git status --short` printed nothing.

#### Observed Result

Analysis recorded before the run, from inspecting the files at the base of the spec branch:

| Convention list entry | Strict v1 issue profile | Effect on a spec written from the list |
| --- | --- | --- |
| No `schema-version` | Required integer `1`; without it, `strict_document_type` returns no strict profile | The record is not strict; a primary spec under `docs/issues/drafts/` or `docs/issues/open/` gets a `legacy-shape` error |
| `doc-type: issue` | `issue` | Matches |
| `issue-type: <task\|bug\|feature\|enhancement>` | The same four values | Matches |
| `status: <draft\|open\|planned\|in-progress\|blocked\|in-review\|done>` | `STATUS_VALUES` has no `open` | `status: open` gets `invalid-allowed-value` |
| `priority: <p0\|p1\|p2\|p3>` | The same four values | Matches |
| No `epic` | Required, positive integer or `null` | `missing-required-field` for `epic` |
| `github-issue`, `spec-path`, `branch`, `related-pr` | Same names and types | Match |
| `last-updated-utc: YYYY-MM-DD HH:MM`, unquoted | Double-quoted UTC-minute string | An unquoted value gets `invalid-field-value` |

Predicted records, one per run:

```text
first spec:  {"kind":"diagnostic","path":"docs/issues/drafts/reproduce-convention-issue-fields/ISSUE.md","severity":"error","category":"legacy-shape",...}  exit 1
second spec: {"kind":"diagnostic","path":"docs/issues/drafts/reproduce-convention-issue-fields-v1/ISSUE.md","severity":"error","category":"missing-required-field","field_path":"epic",...}  exit 1
```

The library reports only the first structural failure per document, so the second run stops at the missing `epic` before the unquoted timestamp; the comparison table lists the remaining differences.

Observed records, 2026-10-03 11:44 UTC, on the loop's compute pod with the toolchain above:

```text
$ cargo run --package frontmatter-validator --bin frontmatter-validator -- docs/issues/drafts/reproduce-convention-issue-fields/ISSUE.md
{"kind":"diagnostic","path":"docs/issues/drafts/reproduce-convention-issue-fields/ISSUE.md","severity":"error","category":"legacy-shape","field_path":null,"message":"Draft and open issue specs must use the v1 frontmatter (`schema-version: 1`); copy the shape from `docs/templates/ISSUE.md` or `docs/templates/EPIC.md` and follow the migration checklist in `contrib/dev-tools/checks/frontmatter-validator/README.md`."}
exit=1
$ cargo run --package frontmatter-validator --bin frontmatter-validator -- docs/issues/drafts/reproduce-convention-issue-fields-v1/ISSUE.md
{"kind":"diagnostic","path":"docs/issues/drafts/reproduce-convention-issue-fields-v1/ISSUE.md","severity":"error","category":"missing-required-field","field_path":"epic","message":"Strict frontmatter requires `epic`."}
exit=1
```

The relayed transcript of the run shortened the first record's `message` after `docs/templates/EPIC.md`; the `message` above is the validator's own string at the branch head (`contrib/dev-tools/checks/frontmatter-validator/src/repository.rs`, function `legacy_shape`). Every other byte of both records, and both exit codes, are as relayed.

#### Conclusion

Reproduced: a spec written from the convention's list is rejected as `legacy-shape`, and the same spec with `schema-version: 1` added is still rejected for the missing `epic`, exactly as predicted. The implementation plan turns this reproduction into the maintained regression test and reruns it like-for-like after the fix.

## Failures and Follow-up

- The attempt while drafting was blocked by the missing toolchain; the recorded commands then ran on the loop's compute pod on 2026-10-03 11:44 UTC and reproduced the defect. No follow-up remains open for B1.
