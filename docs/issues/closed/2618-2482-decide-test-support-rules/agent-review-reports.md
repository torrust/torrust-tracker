---
semantic-links:
  related-artifacts:
    - .github/agents/task-reviewer.agent.md
    - docs/agents/orchestration.md
    - docs/issues/closed/2618-2482-decide-test-support-rules/ISSUE.md
---

# Agent Review Reports - Decide how packages share test support code

> Append one completed independent-review entry at a time. Do not modify, reorder, or remove
> earlier entries. A correction is a new entry that names the earlier conclusion.

## Reports

### 2026-10-21 19:34 UTC - Task Reviewer (first pre-PR review)

- Invocation scope: Acceptance criteria AC1 to AC6 of the spec and every changed file of branch
  `2618-2482-decide-test-support-rules` against `torrust/develop`: the evidence file, the ADR and its
  index row, DEC-18, the seven migration drafts, the EPIC and inventory updates, and the two links
  added to the 2026-10-06 coupling report.
- Inputs: The spec, `git diff torrust/develop...HEAD`, the real manifests, sources, `deny.toml`,
  `Containerfile` and workflows, and GitHub issue states.
- Evidence: The reviewer re-ran `cargo +stable package --workspace --no-verify` (32 packaged), ran
  `linter markdown` and `linter cspell`, and a link and fragment check over the 15 changed Markdown
  files (0 broken). It checked the nine forced edges, the consumers of each environment, the
  `MockTorrentMetricsStore` users, the `deny.toml` wrapper entries, and the member counts against the
  repository.
- Findings:
  - Major: the container release binary is built by
    `cargo nextest archive --tests --all-features ... --release` (`Containerfile`, "Build Archive
    (release)"), which contradicts ADR rule A.5 and criterion 1 once `testing` features exist; no
    changed file mentioned it.
  - Minor: the ADR described the V3 cycle as "two publishable crates that dev-depend on each
    other"; the evidence cycle has a normal dependency in one direction.
  - Minor: the evidence did not show, for a crate with a self dev-dependency, that a production
    build leaves the feature off.
  - Minor: AC4 and EPIC task D4 claimed every inventory item has a destination and a draft, though
    items kept by the ADR have no draft.
  - Minor: the ADR attributed a phrase of the modules' documentation to DEC-13.
  - Minor: "no result contradicts the EPIC" needed a qualifier about the inventory's pattern D
    correction.
  - Minor: the EPIC said rows 4 to 7 run "as their gates clear" while the drafts fix an order.
- Verdict: REVIEW FAILED
- Follow-up actions:
  - Ask the maintainer how to handle the container build, and fix the ADR, evidence, EPIC and
    spec (done, see the second entry).

### 2026-10-22 17:22 UTC - Task Reviewer (re-review after the fixes)

- Invocation scope: The seven findings of the first entry, and new defects introduced by the commits
  that addressed them (among them "docs(adrs): [#2618] build shipped artifacts on their own with
  explicit features"). Invoked right after the commit "docs(issues):
  [#2618] record the review findings and the container decision"; it returned before 17:27 UTC.
- Inputs: The same branch against `torrust/develop`, the spec, the ADR, DEC-18, row 9's draft,
  the EPIC table and order paragraph, the inventory destination table.
- Evidence: `linter markdown`, `linter lychee` and `linter cspell` passed. It checked the
  `Containerfile` stages, the root `default-members` and the absence of a root `[features]` table.
  It could not re-run the V4c probe (the sandbox denied writing to `/tmp`) and relied on the
  recorded output.
- Findings:
  - Six of the seven earlier findings fixed; the "every inventory item has a draft" finding was only
    partly fixed (EPIC Phase 1 outcome, task D4 and the Subissues intro still said it).
  - Minor: the spec still said "rows 2 to 8" in D4, the rejected-alternatives table and the risks.
  - Minor: the EPIC sequencing table still counted eight subissues and said rows 6 and 7 wait on
    #1488.
  - Minor: the EPIC row 3 note said "after row 8" and omitted row 9.
  - Minor: the spec's AC1 evidence cell was unqualified and omitted V4c.
- Verdict: REVIEW FAILED
- Follow-up actions:
  - Fix the stale text (done in the commits "docs(issues): [#2482] count nine subissues and say
    which inventory items have drafts" and "docs(issues): [#2618] count rows 2 to 9 and qualify the
    evidence claim in the spec"). These were one-line documentation edits; the implementing agent
    verified them with `grep` and `linter all` and did not request a third review.
