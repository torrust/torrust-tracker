---
spec-path: docs/issues/closed/2347-2003-triage-post-merge-review-findings/triage.md
last-updated-utc: "2026-09-28 17:41"
semantic-links:
  related-artifacts:
    - docs/issues/closed/2347-2003-triage-post-merge-review-findings/ISSUE.md
    - .github/skills/dev/pr-reviews/process-pr-review/SKILL.md
---

# Issue #2347 - T2 Triage of Post-Merge Review Findings

<!-- cspell:ignore misattributes -->

Triage of the 32 findings against `develop` at `478516cf`, done on 2026-09-27 on branch
`2347-2003-triage-post-merge-review-findings`. Every disposition below is a **proposal** for T3;
none is approved until the maintainer's approval record exists on #2347.

The approved dispositions and routes differ from some of these proposals; see the spec's
[Disposition Decisions (T3)](ISSUE.md#disposition-decisions-t3).

## Thread State at Triage

`github-review-threads fetch` then `show` for each PR, filtered to unresolved threads: 7, 8, 8, 7,
and 2 threads, each with exactly two comments (the finding and the T1 tracking reply). No reviewer
activity since T1.

Each review body labels its findings post-merge. The submitted review IDs match the spec: #2290
`5284003304`, #2293 `5283816543`, #2300 `5284011916`, #2313 `5293099957`, #2320 `5302919075`.

## Audit ID Mapping

| PR | Audit IDs | Reviewer IDs | Reason |
| -- | --------- | ------------ | ------ |
| #2290 | F1-F7 | loop F1-F7 | New audit. |
| #2293 | F1-F8 | loop F1-F8 | New audit. |
| #2300 | F5-F12 | loop F5-F12 | Existing audit ends at F4. |
| #2313 | F4-F10 | loop F4-F10 | Existing audit ends at F3. |
| #2320 | F28-F29 | F24-F25 | Existing audit ends at F27; the reviewer IDs collide. |

## Findings

Legend: **Live** is the current-tree result. The proposed disposition is `FIXED` (a documentation
fix in this task), `NO_ACTION` (declined or not live), or `FOLLOW_UP` (owned by another issue).

### PR #2290

| Audit ID | Reviewer ID | Severity | Category | Live | Proposed |
| -------- | ----------- | -------- | -------- | ---- | -------- |
| F1 | loop F1 | Minor | link-integrity | yes | FIXED |
| F2 | loop F2 | Minor | correctness | yes | FIXED |
| F3 | loop F3 | Minor | documentation | yes | FIXED |
| F4 | loop F4 | Minor | correctness | yes | FOLLOW_UP (new issue FU-A) |
| F5 | loop F5 | Minor | other | yes | FIXED (audit created in T4) |
| F6 | loop F6 | Nit | documentation | yes | FIXED |
| F7 | loop F7 | Suggestion | documentation | yes (PR body) | NO_ACTION |

- **F1.** `git cat-file -t 7e6f5425` and `git cat-file -t d75276ff` both print
  `fatal: Not a valid object name`. The retrospective's Evidence list reads
  `` Commits `7e6f5425` and `d75276ff` ``, and the agent-review-reports Inputs line names the same
  pair. GitHub still serves both objects (`gh api repos/torrust/torrust-tracker/commits/<sha>`),
  with subjects `docs(quality): define Clippy exception decisions` and
  `refactor(udp-protocol): remove Clippy baseline`; both subjects are on `develop`.
  Proposal: cite the two subjects in the retrospective, and append a correction entry to
  `agent-review-reports.md`, which is append-only.
- **F2.** M1's first command, re-run as recorded, prints `packages/udp-protocol/src/lib.rs` and
  `15:    clippy::empty_enums,` instead of `No #2261-owned Clippy controls found.`. The second
  command prints lines 9, 10, and 16. The M1 Result still says `No owned suppression remains in
  the final UDP protocol source`. Proposal: append a dated correction to M1 recording the re-run
  output and that A159 is retained.
- **F3.** The spec prose reads `Arrange creates each supported action at`. The test loop is
  `for action in 0i32..4`, and action 3 reaches `_ => Err(RequestParseError::unsendable_text("Invalid action"))`.
  Proposal: reword the sentence to say "each action code 0-3, including the unsupported code 3".
- **F4.** AC2 is checked. A159 is a crate-level `#![allow(clippy::empty_enums, reason = ...)]`
  in `lib.rs`. The framework says `Broad crate-level suppression and statements that only say the
  warning is intentional are insufficient.` The allowance is still needed: removing it in the
  working tree and running `cargo +nightly clippy -p torrust-tracker-udp-protocol --all-targets
  --all-features -- -D warnings` (Rust `1.100.0-nightly` 2026-09-23) fails with 18 unique
  `enum with no variants` diagnostics: `common.rs` 11, `announce.rs` 5, `connect.rs` 1, and
  `scrape.rs` 1. `lib.rs` was restored afterwards and the tree was clean. Proposal: `FOLLOW_UP`.
  Either narrow the allowance to the four modules (a Rust change), or add a generated-code
  carve-out to the framework (which needs maintainer review). Both are outside this task's scope.
- **F5.** `docs/pr-reviews/pr-2290-review/` is absent. The three Copilot threads are resolved,
  each with only Copilot's comment. Proposal: T4 creates the audit. **Open decision Q1:** backfill
  the three Copilot findings as rows, or not.
- **F6.** The revalidation names `common.rs`, `connect.rs`, and `scrape.rs`. The nightly Clippy
  run above shows that `announce.rs` emits 5 of the 18 unique diagnostics. Proposal: append a
  correction naming all four modules and their counts.
- **F7.** The PR body says `Remove all twelve nonnumeric UDP protocol crate-level Clippy
  allowances owned by #2261.` The in-tree records correctly say `Eleven`. Proposal: `NO_ACTION`.
  The merged PR description is delivery history, and an edit to it has no admissible `FIXED`
  resolution reference at `develop` (see #2313 F6). The final reply records the correct count
  on the thread.

### PR #2293

| Audit ID | Reviewer ID | Severity | Category | Live | Proposed |
| -------- | ----------- | -------- | -------- | ---- | -------- |
| F1 | loop F1 | Minor | correctness | yes | FOLLOW_UP (new issue FU-B) |
| F2 | loop F2 | Minor | documentation | yes | FIXED |
| F3 | loop F3 | Minor | formatting | yes | FIXED |
| F4 | loop F4 | Minor | link-integrity | yes | FIXED |
| F5 | loop F5 | Minor | testing | yes | FOLLOW_UP (new issue FU-B) |
| F6 | loop F6 | Suggestion | security | yes | FOLLOW_UP (new issue FU-C) |
| F7 | loop F7 | Suggestion | metadata | yes | FIXED |
| F8 | loop F8 | Nit | metadata | no | NO_ACTION |

- **F1.** In `generate_coverage_pr.yaml`, job `Package Coverage Regression` runs under
  `if: ${{ always() }}`, has no `continue-on-error`, and writes
  `printf '%s' "$DISCOVERY" > "$GITHUB_WORKSPACE/package-coverage-discovery.json"`.
  `continue-on-error: true` appears only on the matrix job. Proposal: `FOLLOW_UP`, because the fix
  changes the workflow or the Rust tool.
- **F2.** `docs/testing.md` reads ``The `Generate Coverage Report (PR)` workflow``. The workflow is
  `name: Generate Coverage Reports (PR)`. Proposal: fix the name.
- **F3.** The plan's `related-artifacts` entries are indented 4, 3, 4, 6, and 6 spaces
  (`cat -A`). Python `yaml.safe_load` on that frontmatter raises
  `ParserError while parsing a block mapping`. Proposal: re-indent all five entries to 4 spaces.
- **F4.** `88822c5c` and `aa026584` are dangling objects in this local clone, not on any branch
  (`git branch -a --contains` is empty), and GitHub returns `No commit found for SHA` for both.
  Their subjects are `chore(ci): lock coverage discovery dependencies` (no merged commit has this
  subject) and `feat(ci): add package coverage regression report` (merged). The evidence file
  still contains 10 `package-coverage-regression` mentions, and the disclaimer reads `Reproduce
  them by checking out the recorded commits`. Proposal: append a correction saying the two SHAs
  are pre-rebase and unreachable, naming their subjects and the merged equivalents, and pointing
  to `package-coverage-check` for current commands.
- **F5.** M2 and M3 are `Status: TODO`, plan step 5 is `[ ]`, and every #2222 acceptance criterion
  is unchecked in an issue folder already under `closed/`. The reviewer's open risk (the source
  prefix match) has since been exercised on a hosted runner. Run
  <https://github.com/torrust/torrust-tracker/actions/runs/36305549957> (a `pull_request` event
  for PR #2351) produced comparison artifacts with non-zero counts, for example
  `torrust-tracker-udp-server` base `5250/5378` and head `5524/5651`. Proposal: `FOLLOW_UP`,
  together with F1, in one issue that reconciles #2222's hosted evidence and acceptance and
  handles a failed or empty discovery. That is larger than a documentation touch-up.
- **F6.** `permissions:` does not appear anywhere in `generate_coverage_pr.yaml`, while
  `.github/workflows/AGENTS.md` says `Use least-privilege permissions and preserve existing trust
  boundaries.` Proposal: `FOLLOW_UP` in its own issue, because it is a workflow behaviour change.
  Alternative: `NO_ACTION`, since fork PRs already get a read-only token.
- **F7.** `git grep 'skill-link: implement-workflow'` returns nothing, and the skill has no
  `Skill Links` section. Proposal: add the marker to `generate_coverage_pr.yaml` (comment only)
  and to the `.github/workflows/AGENTS.md` frontmatter, and add a `Skill Links` section to the
  skill.
- **F8.** Every `last-updated-utc` in the #2222 folder now reads `2026-09-22 16:40`, set by
  `chore(issues): archive closed issue 2222 spec`. Proposal: `NO_ACTION`, not live.

### PR #2300

| Audit ID | Reviewer ID | Severity | Category | Live | Proposed |
| -------- | ----------- | -------- | -------- | ---- | -------- |
| F5 | loop F5 | Minor | correctness | yes | FIXED |
| F6 | loop F6 | Minor | correctness | yes | FIXED |
| F7 | loop F7 | Minor | testing | yes | FIXED |
| F8 | loop F8 | Minor | correctness | no | NO_ACTION |
| F9 | loop F9 | Minor | link-integrity | yes | FIXED |
| F10 | loop F10 | Suggestion | maintainability | yes | FOLLOW_UP (#2278 order 8) |
| F11 | loop F11 | Suggestion | documentation | yes | FIXED |
| F12 | loop F12 | Minor | maintainability | yes (history) | NO_ACTION |

- **F5.** The Processing Log still holds `15:13 UTC - Committed fixes for F1, F3, and F4` and
  `15:15 UTC - Pushed the fix, replied to F1-F4, resolved all four threads`. Proposal: append a
  correction entry naming both stamps and the event times, re-derived from GitHub in T4.
- **F6.** The #2295 spec still has ``[x] AC6: No file under `docs/pr-reviews/` changes.``, although
  PR #2300 added `docs/pr-reviews/pr-2300-review/PR-REVIEW.md`. Proposal: append a progress-log
  correction to the #2295 spec: AC6 holds only as "no existing audit record changes".
- **F7.** V2 records `167:tracking row plus one matching detail entry carrying the remaining
  narrative and`. The same `grep -nF` now prints `185:` for that line. Proposal: append a dated
  V2 re-run at `develop` with the matched text.
- **F8.** Not live. The `## Status Values` section of 10 records
  (`pr-2320`, `pr-2334`, `pr-2335`, `pr-2337`, `pr-2339`, `pr-2344`, `pr-2346`, `pr-2348`, `pr-2350`,
  and `pr-2353`) is byte-identical to the template, including the `OPEN` sentence. The check
  extracted the section from each of the 119 records and compared it with `diff -q`. Proposal:
  `NO_ACTION`. The template content concern in #2313 F7 is a separate finding.
- **F9.** The agent-review-reports Inputs line cites `` commits `1682aff5`, `b33e2807`, and
  `27a57e1b` ``. None of them is reachable. Their subjects,
  `docs(pr-reviews): single-source the audit field roster`,
  `docs(pr-reviews): mark audit template section ownership`, and
  `docs(issues): record issue 2295 verification evidence`, each match exactly one merged commit.
  Proposal: append a correction entry naming the subjects.
- **F10.** Owner: EPIC #2278 order 8 ("Extend the audit validator to the adopted invariants",
  register F58: "the validator enforces the skeleton"). #2349 excludes "Changing what the pins
  require, other than their granularity". Proposal: `FOLLOW_UP`, with a pointer reply and this
  finding's reference added to the order 8 row in the #2278 EPIC.
- **F11.** The EPIC's 2026-09-22 06:59 entry no longer carries the dropped clause. Proposal:
  append an EPIC progress-log entry naming the removed clause. The clause is not restored,
  because it was a stale forward-looking statement.
- **F12.** `docs(pr-reviews): clarify audit roster optional fields` carries the F1, F3, and F4
  fixes. Proposal: `NO_ACTION`, because merged history is immutable. The F12 detail entry
  records which fixes that commit carries.

### PR #2313

| Audit ID | Reviewer ID | Severity | Category | Live | Proposed |
| -------- | ----------- | -------- | -------- | ---- | -------- |
| F4 | loop F4 | Minor | correctness | yes | FOLLOW_UP (new #2278 subissue FU-D) |
| F5 | loop F5 | Minor | correctness | yes | FOLLOW_UP (FU-D) |
| F6 | loop F6 | Minor | correctness | yes | FOLLOW_UP (FU-D) |
| F7 | loop F7 | Minor | maintainability | yes | FOLLOW_UP (FU-D) |
| F8 | loop F8 | Minor | testing | yes | FIXED |
| F9 | loop F9 | Suggestion | documentation | yes | FOLLOW_UP (FU-D) |
| F10 | loop F10 | Nit | metadata | no | NO_ACTION |

Neither PR #2335 nor PR #2344 settled F4-F7 or F9. They are skill and template contract rules,
so they belong to EPIC #2278. Order 5 (the author self-audit gate) is a different topic. The
proposal is one new #2278 subissue, FU-D, that reconciles these residual rules. It should
coordinate with #2349, because F5 touches a pinned placeholder.

- **F4.** The template has both ``An outdated thread whose concern was fixed is
  `FIXED`/`RESOLVED` `` and ``For in-PR feedback, use `NO_ACTION`/`SUPERSEDED` only for a duplicate,
  superseded, or no-change concern.``. Nothing orders these two rules for a duplicate whose
  concern was fixed.
- **F5.** The template still has `- Resolution reference: <UNIQUE_COMMIT_SUBJECT_OR_REPLY_URL>`,
  the FIXED-only rule ``Use a unique Conventional Commit subject as the `Resolution reference` for
  `FIXED`.``, and `- Cite a fix by its unique Conventional Commit subject or durable reply URL`.
  `agent-review-report-contract/src/main.rs` pins the placeholder.
- **F6.** The skill says `` `FIXED` resolution references are unique Conventional Commit
  subjects ``, and no rule covers a fix that is not a repository change. This also limits #2290 F7.
- **F7.** The template's copied-verbatim `## Status Values` still ends with the policy bullet
  ``A post-merge `NO_ACTION` requires maintainer approval to decline the follow-up work.``. Records
  from `pr-2320` onward copy it; `pr-2313` does not. The historical record is not rewritten.
- **F8.** V1's command, re-run as recorded, matches the skill only once:
  `68:   Every re-raise has its own tracking row and detail entry; never collapse it`. The fixed
  outdated-thread rule, `If a code or documentation change fixed`, is not matched. Proposal:
  append a correction to V1 with a command that matches both rules and its output.
- **F9.** The skill checklist and step 8 cover only responses to multiple review rounds
  (`Consolidated responses that cover multiple review rounds ...`). No rule covers a single round
  with several findings.
- **F10.** The #2308 evidence stamp now reads `last-updated-utc: "2026-09-23 11:00"`. Proposal:
  `NO_ACTION`, not live.

### PR #2320

| Audit ID | Reviewer ID | Severity | Category | Live | Proposed |
| -------- | ----------- | -------- | -------- | ---- | -------- |
| F28 | F24 | Nit | documentation | yes | FIXED |
| F29 | F25 | Nit | documentation | yes | FIXED |

- **F28.** `git grep -nE 'round-4 body|Branch ids: F2, F9|once, the 20:12 push' --
  review-retrospective.md` matches four lines (161, 429, 433, and 434). Record F24 says `matches
  lines 429, 433 and 434`. Proposal: append a Processing Log correction naming the extra match
  (line 161, cost item 9). The historical row is not rewritten.
- **F29.** The retrospective says `Head ids are quoted from the review headers and are pre-rebase
  ids`. `git merge-base --is-ancestor <id> develop` holds for `305ddce1`, `7e426ba2`, `c84a224a`,
  `a1386c2f`, and `69fc9030`, and fails for `d8c82bec` and `ae66bb69`. Proposal: correct the note
  in place to the reviewer's wording, and record the edit in the Processing Log.

## Summary of Proposals

- `FIXED`: 17. #2290 F1, F2, F3, F5, F6; #2293 F2, F3, F4, F7; #2300 F5, F6, F7, F9, F11;
  #2313 F8; #2320 F28, F29.
- `NO_ACTION`: 5. #2290 F7; #2293 F8; #2300 F8, F12; #2313 F10.
- `FOLLOW_UP`: 10. #2290 F4 (FU-A); #2293 F1 and F5 (FU-B), F6 (FU-C); #2300 F10 (#2278 order 8);
  #2313 F4, F5, F6, F7, F9 (FU-D).

Proposed follow-up issues:

- FU-A: resolve the crate-level scope of UDP protocol allowance A159 (parent #2003).
- FU-B: make the package coverage report robust to a failed discovery job, and reconcile #2222's
  hosted evidence and acceptance (parent #1347 or #2003).
- FU-C: least-privilege `permissions:` for the PR coverage workflow (parent #2003).
- FU-D: reconcile the residual audit-contract rules from the PR #2313 post-merge review
  (parent #2278).

## Recurring Causes Observed

Candidates for the implementation retrospective:

- Branch SHAs cited in evidence and agent-review reports, which rebases make unreachable:
  #2290 F1, #2293 F4, and #2300 F9. The `AGENT-REVIEW-REPORTS.md` template has no rule against it.
- Evidence and acceptance claims not re-run after a later review round or commit: #2290 F2,
  #2300 F6, F7, and #2313 F8.
- Resolved review threads whose fix never landed: #2293 F2, F3, and F4.
