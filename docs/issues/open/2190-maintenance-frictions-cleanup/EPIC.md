---
doc-type: epic
status: planned
github-issue: 2190
spec-path: docs/issues/open/2190-maintenance-frictions-cleanup/EPIC.md
branch: "2190-maintenance-frictions-cleanup-spec"
epic-owner: null
last-updated-utc: 2026-09-10 09:10
semantic-links:
  skill-links:
    - create-issue
    - link-subissue-to-parent-issue
  related-artifacts:
    - .github/skills/dev/planning/create-issue/SKILL.md
    - .github/skills/dev/github/link-subissue-to-parent-issue/SKILL.md
    - docs/templates/EPIC.md
    - docs/issues/open/README.md
    - docs/issues/drafts/README.md
    - docs/issues/open/2003-overhaul-guardrails-and-automation/EPIC.md
    - AGENTS.md
    - .github/workflows/container.yaml
    - contrib/dev-tools/checks/tests/test-format-project-words.sh
    - contrib/dev-tools/analysis/workspace-coupling/src/main.rs
---

<!-- skill-link: create-issue -->

# EPIC #2190 - Repository maintenance frictions clean-up

## Goal

Turn a verified inventory of small repository maintenance frictions into a set of domain-clustered, independently reviewable subissues, so each friction is fixed in its own small pull request and under the EPIC that already owns its domain. This EPIC is planning-only: it changes no code, configuration, or documentation outside its own specification and the subissue specifications it produces.

## Why This Is Needed

Routine work across this repository keeps running into the same set of small defects: a developer-tool test suite that has been red on `develop` since it was moved, three such suites that no orchestrator runs at all, sixteen issue specifications for closed issues still sitting in the open backlog, a workflow trigger that starts a job on branches it immediately refuses, and documentation that still points at binaries which have since moved into a package. None of these is large enough to have justified its own issue when it was found, and none of them blocks a feature, so each has survived several release cycles. Individually they are noise; collectively they are a standing tax on every contributor and agent who reads a stale path, re-diagnoses a red test, or scrolls past a failed workflow run that was never going to succeed.

The first version of this specification proposed fixing all of them in one pull request. That shape has two costs. A single pull request touching workflows, shell fixtures, Rust source, documentation, and sixteen directory moves is hard to review and hard to revert item by item, and the reviewing effort grows faster than the size of the diff. It also discards domain context: several of the frictions sit squarely inside the scope of an EPIC that already exists, and fixing them elsewhere hides them from the EPIC that will later have to reason about the same surface. Clustering by domain keeps each fix next to the work that shares its context, which is easier for a human reviewer to hold in mind and easier for an agent to work through with the parent EPIC as its context.

The inventory itself is still worth keeping. Each entry was verified against the tree rather than reported, and that evidence is what makes the resulting subissues cheap to write and cheap to accept. This EPIC preserves that evidence and spends it on subissue specifications instead of on one large change.

## Scope

### In Scope

- Keep the verified inventory as the durable record behind every subissue this EPIC produces, with each item's evidence held in the subissue specification that owns it.
- Assign every inventory item to the EPIC whose own stated scope covers it, and record that assignment.
- Produce one draft subissue specification per item or group of items sharing a domain, each sized for one focused pull request and each naming its parent EPIC.
- Create the GitHub subissues from the approved drafts, link them under their parent EPIC, and move each specification into `docs/issues/open/` under the naming convention.
- Hand the two developer-tool-check items to EPIC #2003, including the row each needs in its subissue table.
- Track completion of the subissues this EPIC owns until every inventory item is delivered or explicitly closed as won't-fix.

### Out of Scope

- Fixing any friction in this EPIC's own pull request. Every code, configuration, and documentation change belongs to a subissue and is reviewed there.
- Re-verifying the inventory. It was verified against `develop` at revision `f6b73e29` on 2026-09-09; a subissue re-checks its own item at implementation time, and a stale entry is corrected in that subissue rather than here.
- Automating the issue-specification archival flow. That is #1774; the archival subissue performs the current archival by hand and does not constrain the script's design.
- Triaging external-link check findings. That is #2185, whose specification is under review in #2186.
- Repairing the `Docker E2E` job in `.github/workflows/testing.yaml`. That is #2179; this EPIC only records which stale pull requests are blocked behind it.
- Migrating legacy single-file specifications to the folder-style layout. That is #2159.
- Designing the long-term check harness and sensor architecture. That is EPIC #2003; the two items handed to it add and repair invocations of checks that already exist and do not prejudge that design.
- Any behavioural change to the tracker itself. The one behavioural defect found while verifying the inventory is item `R1`, which becomes a standalone issue with its own acceptance criteria.
- Local Markdown link health. `linter lychee` exits 0 on `develop` with all local links passing in 0.137 s, and `.github/workflows/docs-lint.yaml:60` already enforces it; external links are wholly owned by #2185. The `.lycheeignore` file named in the original candidate list does not exist and, per #2150, will not: the configuration lives in `lychee.toml` and `.github/lychee-online.toml`.
- A second never-run merge-tool suite. No `test-github-merge-symlinks.py` exists on `develop`; `contrib/dev-tools/git/tests/` contains only `test-merge-pull-request.sh`. That Python suite belongs to the unmerged symlink-exceptions work for #2175 and arrives with it.

## Architectural Decisions

No architectural decision is expected from this EPIC. Every item is a mechanical correction to an existing decision's implementation, an action outside this repository, or — for `R1` — a behavioural defect whose own issue carries its design question. A subissue that discovers an architectural decision during implementation raises it under its own parent rather than here, because this EPIC holds no implementation.

- Related ADRs: `docs/adrs/20260519000000_define_global_cli_output_contract.md`, which the stale-references subissue extends in its related-artifacts list without touching the decision itself.
- ADRs to create: `None known`.

## Friction Inventory

Every item was verified against `develop` at revision `f6b73e29` on 2026-09-09; each was observed in the tree rather than reported. The evidence for an item — its files, line numbers, commands, and observed output — lives in the draft specification named in its row, which is the source of truth for it. `Owner` is the EPIC that carries the item to delivery.

| ID  | Area                          | Friction                                                                                                                                    | Draft specification                                                                 | Owner                 |
| --- | ----------------------------- | ------------------------------------------------------------------------------------------------------------------------------------------- | ------------------------------------------------------------------------------------ | --------------------- |
| F1  | Developer tooling             | Three developer-tool test suites exist and no orchestrator, local or in CI, runs any of them.                                                | `docs/issues/drafts/2003-run-developer-tool-test-suites-in-ci/ISSUE.md`             | EPIC #2003            |
| F2  | Issue lifecycle               | Sixteen specification directories under `docs/issues/open/` belong to issues GitHub reports as closed.                                      | `docs/issues/drafts/2190-archive-closed-issue-specifications/ISSUE.md`              | EPIC #2190            |
| F3  | Workflow triggers             | The container workflow starts on every package release push and then refuses itself on its own semver guard.                                | `docs/issues/drafts/2190-narrow-container-workflow-release-branch-trigger/ISSUE.md` | EPIC #2190            |
| F4  | Documentation drift           | Five live references still point at binaries that moved to `packages/e2e-tools/src/bin/`, and one ADR under-describes the binary landscape.  | `docs/issues/drafts/2190-correct-stale-documentation-references/ISSUE.md`           | EPIC #2190            |
| F5  | Test health                   | The project-dictionary formatter's test suite has been red on `develop` since it was moved beside its sensor, for two independent reasons.   | `docs/issues/drafts/2003-repair-project-dictionary-formatter-test-suite/ISSUE.md`   | EPIC #2003            |
| F6  | Code readability              | One line of the workspace-coupling tool cannot survive being quoted in a review discussion, so the artifact looks self-refuting.             | `docs/issues/drafts/2190-remove-unquotable-character-type-generic/ISSUE.md`         | EPIC #2190            |
| F7  | Dead comment                  | A sensor's note that it has no automated tests yet stops being true when F1 lands, so it is corrected in the same change.                    | `docs/issues/drafts/2003-run-developer-tool-test-suites-in-ci/ISSUE.md`             | EPIC #2003            |
| F8  | Documentation drift           | `project-words.txt` still carries a dictionary entry for a file that #2150 decided will never be created.                                    | `docs/issues/drafts/2190-correct-stale-documentation-references/ISSUE.md`           | EPIC #2190            |
| A1  | Repository administration     | The allowed-actions allowlist pins an exact patch version, so no bump of that action can start, and the failure is invisible in check views. | `docs/issues/drafts/2190-widen-workflow-actions-allowlist-entry/ISSUE.md`           | EPIC #2190            |
| A2  | Repository administration     | Two dependency update pull requests have been stalled since August behind the single check that #2179 fixes.                                 | `docs/issues/drafts/2190-resolve-stalled-dependency-update-pull-requests/ISSUE.md`  | EPIC #2190            |
| R1  | Configuration error reporting | A version-2 configuration file is rejected with an unknown-field error instead of the unsupported-version error written for that case.       | `docs/issues/drafts/configuration-schema-version-error-ordering/ISSUE.md`           | Standalone; no parent |

## Subissues

Status values: `TODO`, `IN_PROGRESS`, `BLOCKED`, `DONE`.

These are the subissues this EPIC owns. Titles and boundaries may be adjusted during maintainer review; no GitHub issues should be created from these drafts without approval. Issue numbers are assigned when the drafts are approved and the GitHub issues are created; until then the draft path is the specification.

| Order | Issue                                                             | Local Spec                                                                          | Status | Notes                                                                                                                                                                                                                                                              |
| ----- | ----------------------------------------------------------------- | ------------------------------------------------------------------------------------ | ------ | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------ |
| 1     | #[To be assigned] - Narrow the container workflow release trigger | `docs/issues/drafts/2190-narrow-container-workflow-release-branch-trigger/ISSUE.md` | TODO   | F3. One-line trigger change; independent of every other subissue. Not a #1840 item: that EPIC's goal is pull request wait time and its Out of Scope excludes workflows off the two critical paths, and a package release push is on neither.                         |
| 2     | #[To be assigned] - Correct stale documentation references        | `docs/issues/drafts/2190-correct-stale-documentation-references/ISSUE.md`           | TODO   | F4 and F8. Documentation and dictionary only; independently revertible. Not a #1669 item: that EPIC's documentation duty covers `docs/packages.md`, `packages/AGENTS.md`, the extracted-package tables, and the dependency diagram, and none of these files is one.  |
| 3     | #[To be assigned] - Archive closed issue specifications           | `docs/issues/drafts/2190-archive-closed-issue-specifications/ISSUE.md`              | TODO   | F2. Large diff of pure moves; re-verify issue state at merge time. Automating the flow is #1774, which is paused behind EPIC #2003's architecture decision; one manual pass implements no tool and must not inherit that block.                                      |
| 4     | #[To be assigned] - Remove the unquotable character-type generic  | `docs/issues/drafts/2190-remove-unquotable-character-type-generic/ISSUE.md`         | TODO   | F6. Signature change with a before/after output comparison. The tool appears in EPIC #2003's related artifacts, but that EPIC's scope is the architecture of automation, not the source hygiene of any one tool.                                                     |
| 5     | #[To be assigned] - Widen the workflow actions allowlist entry    | `docs/issues/drafts/2190-widen-workflow-actions-allowlist-entry/ISSUE.md`           | TODO   | A1. Repository administrator action; no file in this repository changes. An issue rather than a table row so that it has an owner, an acceptance criterion, and a visible state.                                                                                     |
| 6     | #[To be assigned] - Resolve the stalled dependency update PRs     | `docs/issues/drafts/2190-resolve-stalled-dependency-update-pull-requests/ISSUE.md`  | TODO   | A2. Maintainer action, blocked until #2179 merges; no file in this repository changes. Does not duplicate #2179's fix and does not touch either branch.                                                                                                             |

## Delivery Strategy

The EPIC delivers specifications, not fixes. Each item is assigned to the EPIC whose own stated scope covers it, and to this EPIC only when no existing EPIC's scope does. The order above reflects dependency and reviewer cost, not priority.

Three items leave this EPIC. F5 and the F1/F7 pair go to EPIC #2003 - Overhaul: Automation Tools and AI Agent Guardrails, whose In Scope undertakes to evaluate check placement across pre-commit, pre-push, and CI and which explicitly permits the interim project-dictionary formatter that these two repair and invoke; both also meet that EPIC's stated exception for low-risk, additive, independently verifiable work that selects no shared runner, cache, or enforcement platform, so neither waits on its architecture decision. F5 is delivered before F1 and F7, because the suite must be green before anything runs it in CI. R1 leaves without a parent: it is a behavioural change to a user-facing error path, needing its own acceptance criteria and its own regression test, and the configuration overhaul EPIC that would have owned it, #1978, is closed. Each of the three drafts names its own parent, and the adopting EPIC's subissue table gains its row when the GitHub issue is created, not in this EPIC's pull request.

For each subissue implementation in this EPIC, the default completion policy is:

1. Run automatic checks (`linter all`, relevant tests, pre-push checks when applicable).
2. Run manual verification scenarios and record evidence.
3. Re-review acceptance criteria after implementation and update verification evidence.
4. Complete an evidence-based implementation review. Create or update an issue-local retrospective for reusable lessons, material design changes, or meaningful deviations from the plan; otherwise record why one was unnecessary in the issue progress log.

### Phase 1

- Outcome: every inventory item is assigned to a parent, with the assignment argued from that parent's own stated scope, and a draft specification exists for every item.
- Exit criteria: the Friction Inventory and Subissues tables are complete and a maintainer has reviewed the assignment.

### Phase 2

- Outcome: the approved drafts become GitHub issues, attached to their parent EPIC through the GitHub sub-issues API as the `link-subissue-to-parent-issue` skill describes, with each specification moved from `docs/issues/drafts/` to `docs/issues/open/` under its assigned number.
- Exit criteria: every row in the Subissues table carries a real issue number, the three handed-away drafts carry theirs, EPIC #2003's subissue table carries the two rows it adopted, and each source artifact a subissue will change carries an `issue: #<number>` marker where the link is high-signal. No `issue-spec:` marker is added to a source artifact before then, because a draft path added now would have to be rewritten as soon as the issue exists.

### Phase 3

- Outcome: each subissue is implemented and merged in its own pull request, in the ordering recorded above.
- Exit criteria: every inventory item is delivered or explicitly closed as won't-fix with the reason recorded in this EPIC's progress log.

## Progress Tracking

### Workflow Checkpoints

- [x] Epic spec drafted and moved to `docs/issues/open/2190-maintenance-frictions-cleanup/`
- [x] Epic spec reviewed and approved by user/maintainer
- [x] GitHub epic issue [#2190](https://github.com/torrust/torrust-tracker/issues/2190) created and issue number added to this spec
- [x] Specification converted from a single implementation issue into a planning-only EPIC
- [ ] Parent assignment reviewed and approved by a maintainer
- [ ] Spec-only PR merged into `develop`
- [ ] GitHub issue #2190 converted to an EPIC issue: title, labels, and body updated to match this specification
- [ ] Subissues created from the approved drafts and attached to their parent EPIC through the GitHub sub-issues API
- [ ] Subissue specifications moved from `docs/issues/drafts/` to `docs/issues/open/`
- [ ] EPIC #2003's subissue table updated with the two adopted items
- [ ] Subissue statuses kept up to date in the `Subissues` table
- [ ] For each implemented subissue: automatic checks completed and recorded
- [ ] For each implemented subissue: manual verification completed and recorded
- [ ] For each implemented subissue: acceptance criteria reviewed post-implementation
- [ ] For each implemented subissue: implementation completion review recorded
- [ ] Epic acceptance criteria reviewed and checked off
- [ ] Epic issue closed and spec moved from `docs/issues/open/` to `docs/issues/closed/`

### Progress Log

Append one line per meaningful update.

- 2026-09-09 14:11 UTC - Specification author - Drafted from a re-verification of every candidate friction against `develop` at `f6b73e29`; eight in-scope items, two maintainer actions, one related-but-separate behavioural defect, two candidates that did not reproduce - evidence recorded inline in the specification
- 2026-09-09 15:39 UTC - Specification author - GitHub issue #2190 created from the reviewed draft; specification moved to `docs/issues/open/2190-maintenance-frictions-cleanup/ISSUE.md` - https://github.com/torrust/torrust-tracker/issues/2190
- 2026-09-10 08:43 UTC - Specification author - Restructured after the review on PR #2193 requesting an EPIC with domain-clustered subissues instead of one implementation issue: specification renamed to `EPIC.md` and made planning-only, the items clustered by domain, two clusters handed to EPIC #2003, and nine draft subissue specifications added under `docs/issues/drafts/` - https://github.com/torrust/torrust-tracker/pull/2193#pullrequestreview-5164674289
- 2026-09-10 09:10 UTC - Specification author - Aligned the specification with the shape of EPICs #1840 and #1347: the friction, maintainer-action, related-but-separate, cluster, per-parent-argument, and handoff sections collapsed into one compact Friction Inventory and the Subissues table, each item's evidence left in the subissue draft that owns it, the two candidates that did not reproduce recorded in Out of Scope, and the frontmatter reduced to the model key set - https://github.com/torrust/torrust-tracker/pull/2193

## Acceptance Criteria

- [ ] Every item in the Friction Inventory has exactly one named owner, and each item this EPIC owns appears in exactly one row of the Subissues table.
- [ ] Each owner is justified from that owner's own stated scope, and the drafts adopted by another EPIC set `epic:` to that EPIC and name it below their title.
- [ ] Each draft subissue specification is sized for one focused pull request and carries its own evidence, acceptance criteria, and verification plan.
- [ ] This EPIC's pull request changes no file outside `docs/issues/` and `docs/copilot-pr-reviews/`.
- [ ] Every subissue row and every handed-away draft carries a created GitHub issue number, linked as a subissue of its parent EPIC.
- [ ] Every inventory item is delivered by a merged subissue pull request, or closed as won't-fix with the reason recorded in the progress log.
- [ ] The two candidates that did not reproduce remain recorded, so the same ground is not re-covered.
- [ ] `linter all` exits with code `0`.
- [ ] Documentation and governance updates are included when required.

### Acceptance Verification

| AC ID | Status (`TODO`/`DONE`) | Evidence                                                     |
| ----- | ---------------------- | ------------------------------------------------------------- |
| AC1   | TODO                   | Friction Inventory and Subissues tables                      |
| AC2   | TODO                   | Subissue notes, Delivery Strategy, and the draft frontmatter |
| AC3   | TODO                   | The draft specifications listed in the Friction Inventory    |
| AC4   | TODO                   | {PR link}                                                    |
| AC5   | TODO                   | {issue links}                                                |
| AC6   | TODO                   | {PR links}                                                   |
| AC7   | TODO                   | The Out of Scope entries for the two candidates              |
| AC8   | TODO                   | The gate run recorded on this EPIC's spec-only pull request  |
| AC9   | TODO                   | This specification and the subissue specifications it lists  |

## Risks and Trade-offs

- Risk: splitting eight frictions into six subissues plus two adoptions costs more issue-management overhead than one pull request would have. Mitigation: the split is by domain rather than by item, so the count stays close to the number of reviewer contexts involved rather than to the number of defects, and two items are absorbed by an EPIC that already has the context.
- Risk: small subissues can stall before they are filed, leaving the inventory as documentation rather than work. Mitigation: the created issues and their delivery are acceptance conditions of this EPIC, which stays open until every item is delivered or closed as won't-fix.
- Risk: the inventory is a snapshot taken at `f6b73e29`, and an item can be fixed or changed by unrelated work before its subissue is implemented. Mitigation: each subissue re-checks its own evidence at implementation time and records a no-change outcome rather than forcing a fix.
- Risk: the archival list is the most perishable entry, because issues close while a pull request is open. Mitigation: its subissue re-verifies every remaining directory against GitHub at merge time rather than trusting the list drafted here.
- Risk: handing F5 and F1/F7 to EPIC #2003 places them behind that EPIC's review attention, which is directed at an unfinished architecture decision. Mitigation: both are filed under that EPIC's own exception for additive, independently verifiable work that may proceed before the decision, so neither waits on it.
- Risk: two subissues change no file in this repository, which is an unusual shape for an issue here. Mitigation: their specifications state the no-change outcome explicitly and record a justified no-change decision in the progress log rather than producing an empty commit.

## References

- GitHub EPIC: https://github.com/torrust/torrust-tracker/issues/2190
- Related issues: #1774 (archival automation), #2003 (guardrails and automation EPIC), #2150 (lychee link checker), #2159 (folder-style spec adoption), #2179 (Docker E2E package flag), #2185 (external-link triage)
- Related PRs: #2055, #2106, #2180, #2186, #2193
- Related ADRs: `docs/adrs/20260519000000_define_global_cli_output_contract.md` (the stale-references subissue extends its related-artifacts list; it does not touch the decision)
