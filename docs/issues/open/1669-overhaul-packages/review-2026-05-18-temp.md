# EPIC Review Notes (Temp)

Date: 2026-05-18
Scope: [EPIC.md](EPIC.md) and SI drafts linked from the Active Subissues table.
Review focus: consistency, dependency integrity, sequencing risk.

## Working Mode

Address one finding at a time, in order.

1. Pick the next open finding from the queue.
2. Apply only the minimal doc edits needed to close that finding.
3. Re-check cross-references impacted by that edit.
4. Mark the finding as done with a short note.

## Findings Queue

### F1 (High) - SI-14 prerequisite cannot be satisfied by SI-08 as written

Status: DONE

Problem:

- SI-14 requires publish completion before extraction starts: [docs/issues/drafts/1669-14-extract-torrust-metrics-to-standalone-repo.md](../../drafts/1669-14-extract-torrust-metrics-to-standalone-repo.md#L42), [docs/issues/drafts/1669-14-extract-torrust-metrics-to-standalone-repo.md](../../drafts/1669-14-extract-torrust-metrics-to-standalone-repo.md#L47), [docs/issues/drafts/1669-14-extract-torrust-metrics-to-standalone-repo.md](../../drafts/1669-14-extract-torrust-metrics-to-standalone-repo.md#L94).
- SI-08 explicitly says publishing is out of scope and has no publish task: [docs/issues/drafts/1669-08-rename-torrust-tracker-metrics-to-torrust-metrics.md](../../drafts/1669-08-rename-torrust-tracker-metrics-to-torrust-metrics.md#L65), [docs/issues/drafts/1669-08-rename-torrust-tracker-metrics-to-torrust-metrics.md](../../drafts/1669-08-rename-torrust-tracker-metrics-to-torrust-metrics.md#L70), [docs/issues/drafts/1669-08-rename-torrust-tracker-metrics-to-torrust-metrics.md](../../drafts/1669-08-rename-torrust-tracker-metrics-to-torrust-metrics.md#L77).

Why this matters:

- SI-14 remains structurally blocked even if SI-08 is completed.

Proposed minimal fix:

- Choose one policy and align both specs:
  - Option A: keep SI-08 as rename-only; in SI-14 change prerequisite wording to rename completion only and move publish responsibility into SI-14.
  - Option B: add publish task + acceptance criteria to SI-08, keep SI-14 as currently written.

Recommendation:

- Option A, to keep rename and extraction concerns separate.

Resolution (2026-05-18):

- SI-08 remains rename-only (no publish step). Publishing is deferred as long as
  possible per project policy (Refactor → Publish → Extract).
- SI-14 updated: prerequisite changed to "SI-08 complete (rename done)"; new task T1b
  added within SI-14 to publish `torrust-metrics` on crates.io before extraction begins.
- Workflow checkpoint added in SI-14 for the publish step.

### F2 (High) - EPIC shows extracted state that conflicts with SI statuses

Status: DONE

Problem:

- EPIC lists already extracted packages: [docs/issues/open/1669-overhaul-packages/EPIC.md](EPIC.md#L154), [docs/issues/open/1669-overhaul-packages/EPIC.md](EPIC.md#L158), [docs/issues/open/1669-overhaul-packages/EPIC.md](EPIC.md#L159).
- Same EPIC marks SI-12 and SI-15 as TODO/blocked: [docs/issues/open/1669-overhaul-packages/EPIC.md](EPIC.md#L215), [docs/issues/open/1669-overhaul-packages/EPIC.md](EPIC.md#L218).

Why this matters:

- Current state reporting becomes ambiguous.

Proposed minimal fix:

- Either rename section title to planned/future extracted state, or update SI rows/checklists to reflect completion if truly done.

Recommendation:

- Retitle section to clearly indicate target state, unless extraction is already merged.

Resolution (2026-05-18):

- Both packages (`torrust-bencode`, `torrust-tracker-client`) are still in the workspace;
  extraction is not done.
- EPIC section renamed from "Extracted from workspace" to "Planned for extraction from
  workspace" with a clarifying note pointing to SI-12 and SI-15.

### F3 (Medium) - Baseline is described as established while SI-01 is still TODO

Status: DONE

Problem:

- EPIC first-cycle says baseline established: [docs/issues/open/1669-overhaul-packages/EPIC.md](EPIC.md#L260), [docs/issues/open/1669-overhaul-packages/EPIC.md](EPIC.md#L262).
- SI-01 remains TODO in EPIC and SI-01 acceptance/checkpoints are unchecked: [docs/issues/open/1669-overhaul-packages/EPIC.md](EPIC.md#L204), [docs/issues/open/1669-overhaul-packages/EPIC.md](EPIC.md#L224), [docs/issues/drafts/1669-01-establish-baseline-analysis.md](../../drafts/1669-01-establish-baseline-analysis.md#L141).

Why this matters:

- Phase reporting can drift from issue status.

Proposed minimal fix:

- Mark SI-01 appropriately (if done), or reword EPIC first-cycle outcome to pending/in progress.

Action taken (2026-05-18):

- SI-01 scope extended with new task T8: research `packages/configuration` splitting into
  per-service sub-packages, with two prototype scenarios (UDP-only and HTTP+REST-API) and
  a trade-off table. Outcome either opens a new subissue or records a decision.
- SI-01 remains open. EPIC first-cycle outcome text still reads "Baseline established" while
  SI-01 is in progress — the EPIC wording fix is deferred until SI-01 is actually complete.
- Changes committed in ff152f4c.

### F4 (Medium) - Package count mismatch (26 vs 27)

Status: OPEN

Problem:

- Coupling report says 27 workspace packages: [docs/issues/open/1669-overhaul-packages/workspace-coupling-report.md](workspace-coupling-report.md#L5).
- EPIC and SI-01 repeatedly refer to 26: [docs/issues/open/1669-overhaul-packages/EPIC.md](EPIC.md#L56), [docs/issues/drafts/1669-01-establish-baseline-analysis.md](../../drafts/1669-01-establish-baseline-analysis.md#L45), [docs/issues/drafts/1669-01-establish-baseline-analysis.md](../../drafts/1669-01-establish-baseline-analysis.md#L109).

Why this matters:

- Acceptance criteria and coverage checks can be off-by-one.

Proposed minimal fix:

- Update EPIC and SI-01 to a single source-of-truth count and timestamp, or phrase counts as point-in-time with explicit date and include/exclude rules.

Resolution (2026-05-18):

- The 27th package is the root `torrust-tracker` crate (the main binary, `src/`). It was
  excluded from the original 26-package count because EPIC/SI-01 were drafted before the
  coupling report was generated.
- All occurrences of "26 packages" updated to "27 packages" in EPIC.md (lines 46, 56, 100)
  and SI-01 (Background, T2 notes, T4 notes, acceptance criteria ×2, MV1).
- MV1 verification criterion reworded to: "27 packages total: 5 leaves + 22 with deps."

### F5 (Medium) - SI-02 prerequisite points at SI-09 T12 (doc update) instead of technical completion

Status: DONE

Problem:

- SI-02 prerequisite requires SI-09 T12: [docs/issues/drafts/1669-02-move-duration-since-unix-epoch-to-torrust-clock.md](../../drafts/1669-02-move-duration-since-unix-epoch-to-torrust-clock.md#L68), [docs/issues/drafts/1669-02-move-duration-since-unix-epoch-to-torrust-clock.md](../../drafts/1669-02-move-duration-since-unix-epoch-to-torrust-clock.md#L105).
- SI-09 T12 is EPIC table/doc update, not rename mechanics: [docs/issues/drafts/1669-09-rename-torrust-tracker-clock-to-torrust-clock.md](../../drafts/1669-09-rename-torrust-tracker-clock-to-torrust-clock.md#L119).

Why this matters:

- Introduces avoidable scheduling blockage.

Proposed minimal fix:

- Change SI-02 prerequisite to SI-09 technical completion criteria (crate rename and dependency/use-path migration), not T12.

Resolution (2026-05-18):

- SI-09 T12 is the EPIC table update only; the actual blocker for SI-02 is T1–T4 (crate
  rename, Cargo dep key updates, use-path migration).
- SI-02 prerequisite paragraph rewritten to reference SI-09 T1–T4 explicitly and note that
  T12 is not a blocker.
- SI-02 T1 task description updated from "T12 of clock rename spec" to "SI-09 T1–T4
  complete (crate renamed, dep keys updated, use-paths migrated workspace-wide)".

### F6 (Low) - SI-03 related-artifacts points to non-matching rename spec path

Status: DONE

Problem:

- SI-03 related-artifacts references a rename spec path that does not match current naming: [docs/issues/drafts/1669-03-move-default-timeout-from-configuration-to-clock.md](../../drafts/1669-03-move-default-timeout-from-configuration-to-clock.md#L19).
- Existing rename draft is: [docs/issues/drafts/1669-09-rename-torrust-tracker-clock-to-torrust-clock.md](../../drafts/1669-09-rename-torrust-tracker-clock-to-torrust-clock.md).

Why this matters:

- Weakens traceability and tooling reliability.

Proposed minimal fix:

- Replace artifact path with the canonical SI-09 file path.

Resolution (2026-05-18):

- SI-03 `related-artifacts` entry corrected from
  `docs/issues/drafts/rename-torrust-tracker-clock-to-torrust-clock.md` to
  `docs/issues/drafts/1669-09-rename-torrust-tracker-clock-to-torrust-clock.md`.

## Progress Log

- 2026-05-18: Initial review logged with six findings, severity-ranked.
- 2026-05-18: F1 resolved and committed (f83350ee). F2 resolved and committed (6effa6a0).
- 2026-05-18: F3 partially resolved (T8 added to SI-01) and committed (ff152f4c). EPIC wording deferred to SI-01 completion.
- 2026-05-18: F4 resolved and committed (313e0d46). Package count updated 26→27; 27th is root `torrust-tracker` crate.
- 2026-05-18: F5 resolved — SI-02 prerequisite rewritten to reference SI-09 T1–T4 (technical completion), not T12.
- 2026-05-18: F6 resolved — SI-03 related-artifacts path corrected to `1669-09-rename-torrust-tracker-clock-to-torrust-clock.md`.
