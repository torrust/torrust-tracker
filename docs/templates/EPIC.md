---
doc-type: epic
status: draft
github-issue: null
spec-path: docs/issues/drafts/{short-description}/EPIC.md
epic-owner: null
last-updated-utc: YYYY-MM-DD HH:MM
semantic-links:
  skill-links:
    - create-issue
  related-artifacts:
    - .github/skills/dev/planning/create-issue/SKILL.md
---

<!-- skill-link: create-issue -->

# EPIC #[To be assigned] - {Title}

## Goal

Describe the high-level outcome this EPIC should deliver.

## Why This Is Needed

Describe the current pain, risk, or missed opportunity.

## Scope

### In Scope

- Item 1
- Item 2

### Out of Scope

- Item 1
- Item 2

## Subissues

Status values: `TODO`, `IN_PROGRESS`, `BLOCKED`, `DONE`.

| Order | Issue                                | Local Spec                                  | Status | Notes                  |
| ----- | ------------------------------------ | ------------------------------------------- | ------ | ---------------------- |
| 1     | #[To be assigned] - {Subissue title} | `docs/issues/open/{number}-{slug}/ISSUE.md` | TODO   | {Dependencies/remarks} |
| 2     | #[To be assigned] - {Subissue title} | `docs/issues/open/{number}-{slug}/ISSUE.md` | TODO   | {Dependencies/remarks} |

## Delivery Strategy

Describe rollout phases, dependency order, and merge strategy.

For each subissue implementation in this EPIC, the default completion policy is:

1. Run automatic checks (`linter all`, relevant tests, pre-push checks when applicable).
2. Run manual verification scenarios and record evidence.
3. Re-review acceptance criteria after implementation and update verification evidence.
4. Complete an evidence-based implementation review. Create or update an
   issue-local retrospective for reusable lessons, material design changes, or
   meaningful deviations from the plan; otherwise record why one was unnecessary
   in the issue progress log.

### Phase 1

- Outcome
- Exit criteria

### Phase 2

- Outcome
- Exit criteria

## Progress Tracking

### Workflow Checkpoints

- [ ] Epic spec drafted in `docs/issues/drafts/`
- [ ] Epic spec reviewed and approved by user/maintainer
- [ ] GitHub epic issue created and issue number added to this spec
- [ ] Subissues created and linked in this spec
- [ ] Subissue statuses kept up to date in the `Subissues` table
- [ ] For each implemented subissue: automatic checks completed and recorded
- [ ] For each implemented subissue: manual verification completed and recorded
- [ ] For each implemented subissue: acceptance criteria reviewed post-implementation
- [ ] For each implemented subissue: implementation completion review recorded
- [ ] Epic acceptance criteria reviewed and checked off
- [ ] Epic issue closed and spec moved from `docs/issues/open/` to `docs/issues/closed/`

### Progress Log

Append one line per meaningful update.

- YYYY-MM-DD HH:MM UTC - {Role/Agent} - {Update summary} - {Links to evidence}

## Acceptance Criteria

- [ ] All required subissues are created and linked.
- [ ] Implementation order is explicit and justified.
- [ ] Dependencies and blockers are documented and current.
- [ ] Epic status reflects actual state of linked subissues.
- [ ] Every completed subissue includes automated verification evidence.
- [ ] Every completed subissue includes manual verification evidence.
- [ ] Every completed subissue includes post-implementation acceptance criteria review.
- [ ] Every completed subissue includes an implementation completion review.
- [ ] Documentation and governance updates are included when required.

### Acceptance Verification

| AC ID | Status (`TODO`/`DONE`) | Evidence              |
| ----- | ---------------------- | --------------------- |
| AC1   | TODO                   | {issue/spec/PR links} |
| AC2   | TODO                   | {issue/spec/PR links} |

## Risks and Trade-offs

- Risk 1 and mitigation
- Risk 2 and mitigation

## References

- Related issues: #{number}
- Related PRs: #{number}
- Related ADRs: `docs/adrs/...`
