---
schema-version: 1
doc-type: epic
status: planned
epic: null
github-issue: 2912
spec-path: docs/issues/open/2912-remove-source-locations-from-errors/EPIC.md
epic-owner: null
last-updated-utc: "2026-11-24 12:43"
semantic-links:
  skill-links:
    - create-issue
  related-artifacts:
    - .claude/skills/create-issue/SKILL.md
    - .claude/skills/handle-errors-in-code/SKILL.md
    - docs/issues/closed/2804-location-caller-track-caller-audit/ISSUE.md
    - docs/issues/closed/2804-location-caller-track-caller-audit/inventory.md
    - docs/issues/closed/2804-location-caller-track-caller-audit/options.md
    - docs/issues/open/384-strip-location-from-errors-sent-to-clients/ISSUE.md
---

<!-- skill-link: create-issue -->

# EPIC #2912 - Remove source locations from errors

## Goal

No error in the workspace records or prints a source location. Error types have no `location`
field, the workspace does not use `torrust-located-error`, and no text sent to an HTTP, UDP or REST
client, an event consumer or an operator holds a source file path. A repository check keeps it so.

This EPIC is independent: it has no parent EPIC, because it cuts across every package that defines
errors.

## Why This Is Needed

The feature was meant to point a developer to the origin of an error. The audit of issue #2804
(`docs/issues/closed/2804-location-caller-track-caller-audit/`) shows that it does not do that, and
that it has costs:

- **It is wrong by construction.** None of the 38 production `Location::caller()` sites of the audit (37 since #2850) is in a
  `#[track_caller]` function, so each reports the fixed line where it is written. `#[track_caller]`
  cannot fix any of them: they are reached through `str::parse`, `try_into`, closures, `async fn`
  or `async` blocks (`options.md`, fact 6; `experiments.md`, E3 to E8). Through `str::parse` the
  attribute makes the location a line inside the standard library.
- **It reaches clients.** 23 `Display` formats (22 since #2850) print the location and `LocatedError` prints its
  own. The text is in the HTTP tracker `failure reason`, the UDP error message, REST API error
  text, event payloads and logs (`inventory.md`, T2 table).
- **It was reported as a security concern** in 2023 (issue #384, still open).
- **It lets an attacker infer the running version.** Paths and line numbers change between
  releases. For a registry crate the path is inside the build machine's `~/.cargo/registry`.
- **It couples the user-facing interface to the implementation.** Moving a function changes a
  client-visible message.
- **It makes messages unstable for consumers**, who cannot match on text that changes with
  unrelated refactors.
- **It costs public API.** The field is part of 24 public enum variants (23 since #2850).

What the feature gives, a file and line for a developer, is available on the server from `tracing`
events, which record the file and line of the log call.

The four options the audit compared are in `options.md`. This EPIC implements the removal of the
field (its option 4), which keeps option 3's context-instead-of-location wording and also removes
the field. Keeping the field and stripping it from `Display` only (option 3 alone) was rejected by the
maintainer on 2026-11-17; the ADR of SI-1 records the decision.

## Maintainer Decisions

Recorded 2026-11-17 by the EPIC coordinator session:

1. **Remove the field (option 4).** Error types lose their `location` field, no non-test code calls
   `Location::caller()`, and the workspace drops `torrust-located-error`. Stripping the location from
   `Display` only (option 3 alone) was rejected: each of the 38 production `Location::caller()` sites of the audit (37 since #2850)
   records a fixed library line, so keeping the field keeps a misleading `Debug` and log value. The
   five `#[track_caller]` `From` conversions into `LocatedError` record a location that is correct
   for their call chain (`inventory.md`, class C), and they are removed as well, because that
   location is still an implementation path.
2. **#384 keeps its title.** It is linked as SI-7 and gets a comment that explains the new scope
   (removal instead of an option); it closes after the recheck.
3. **One sub-issue session at a time.** SI-1, then SI-2, SI-3, SI-4, SI-5 and SI-6 in that order,
   each started only after the maintainer's go. The parallel order in Delivery Strategy is what the
   work allows, not the plan.
4. **Model for sub-issue sessions:** Claude Opus 5.5 at default effort.

## Scope

### In Scope

- An ADR and the update of the `handle-errors-in-code` skill (SI-1).
- Removal of every `location` field, `Location::caller()` capture and `LocatedError` source from
  non-test code in `primitives`, `http-protocol`, `axum-http-server`, `http-core`, `tracker-core`,
  `configuration`, `axum-server`, `udp-server` and `tracker-client`, with reworded messages where a
  message loses its only context (SI-2 to SI-6).
- Removal of the `torrust-located-error` dependency from the workspace and a repository check that
  enforces the rule (SI-6).
- Client-side verification on a running tracker and the closing of #384 (SI-7).
- Drafts of the two upstream issues for the standalone crates, handed to the maintainer (SI-6).

### Out of Scope

- Changes inside `torrust-info-hash` and `torrust-located-error` (other repositories). Until
  `torrust-info-hash` changes, the workspace does not format its `ConversionError` into client text
  (SI-3).
- `#[track_caller]` in test helpers and assertion helpers, which stays allowed.
- A configuration option to strip locations: with the removal there is nothing to strip.
- The content of server-side logs.
- Publishing crates (#2583) and extracting `tracker-client` (#2594).
- Redesigning error types beyond what the removal needs.

## Subissues

Status values: `TODO`, `IN_PROGRESS`, `BLOCKED`, `DONE`.

SI-1 to SI-6 are issues #2913 to #2918, created by `josecelano-bot` on 2026-11-17. SI-7 is the
existing issue #384. SI-8 is issue #2996, created by `josecelano-bot` on 2026-11-22.

| Order | Issue | Local Spec | Status | Notes |
| --- | --- | --- | --- | --- |
| 1 | #2913 - SI-1 Record the decision that errors do not record source locations | `docs/issues/closed/2913-2912-si-1-adr-errors-do-not-record-source-locations/ISSUE.md` | DONE | First. Every code sub-issue waits for it |
| 2 | #2914 - SI-2 Remove source locations from the errors of `torrust-tracker-primitives` | `docs/issues/closed/2914-2912-si-2-primitives/ISSUE.md` | DONE | After SI-1. Closes finding F3 of the primitives test work (EPIC #1347). Must land before `primitives` 4.0.0 is published |
| 3 | #2915 - SI-3 Remove source locations from the HTTP tracker request errors | `docs/issues/closed/2915-2912-si-3-http-tracker-request-errors/ISSUE.md` | DONE | After SI-1. `http-protocol` and the `axum-http-server` extractors. Largest client-visible exposure |
| 4 | #2916 - SI-4 Remove source locations from the errors of `torrust-tracker-core` | `docs/issues/closed/2916-2912-si-4-tracker-core/ISSUE.md` | DONE | After SI-1; after SI-3 if not run in parallel, because both touch `http-core` tests. Largest diff |
| 5 | #2917 - SI-5 Remove source locations from the configuration, TLS and UDP server errors | `docs/issues/closed/2917-2912-si-5-startup-and-udp-server-errors/ISSUE.md` | DONE | After SI-1. Must land before `configuration` 4.0.0 is published |
| 6 | #2918 - SI-6 Drop `torrust-located-error` from the workspace and enforce the rule | `docs/issues/closed/2918-2912-si-6-drop-located-error-and-enforce-the-rule/ISSUE.md` | DONE | After SI-2 to SI-5 |
| 7 | #384 - Make Option to Strip Location from Errors sent to Clients | `docs/issues/open/384-strip-location-from-errors-sent-to-clients/ISSUE.md` | TODO | Existing issue. Its baseline reproduction can start at once; its recheck waits for SI-2 to SI-6 |
| 8 | #2996 - SI-8 Share the `assert_names_no_source_path` test helper | `docs/issues/open/2996-2912-si-8-share-assert-names-no-source-path/ISSUE.md` | TODO | Independent of SI-6; implement after SI-4 (PR #2991) and SI-5 (PR #3001), which add the last copies (copies 8 and 9 in `configuration` and `axum-server` stay, cross-linked). Reduces the nine copies to five definitions |

### External follow-ups (other repositories)

| Repository | Change | Owner | Status |
| --- | --- | --- | --- |
| `torrust/torrust-info-hash` | Remove the location from `ConversionError` (`info_hash.rs:273`, `:279` in 0.2.0), then update the dependency here | Maintainer opens the issue from the SI-6 draft | TODO |
| `torrust/torrust-located-error` | Decide the future of the crate once the tracker no longer uses it | Maintainer opens the issue from the SI-6 draft | TODO |

## Delivery Strategy

One pull request per sub-issue, each from its own session, merged into `develop`. Order: SI-1 first;
then SI-2, SI-3, SI-4 and SI-5, which touch different error types and may run in parallel (SI-3
before SI-4 if they run one after the other); then SI-6; then the recheck of SI-7.

The removal changes public enum variants. It is free of release cost only while the affected crates
are unpublished or already owe a major release (`options.md`, semver table; publish plan in
`docs/issues/open/2583-1669-first-publish-of-workspace-crates/ISSUE.md`): `primitives` and
`configuration` owe 4.0.0, and the other crates are at their first publish. The EPIC must therefore
finish SI-2 to SI-5 before the first publish of each crate, or record the extra major release.

For each subissue implementation in this EPIC, the default completion policy is:

1. Run automatic checks (`linter all`, relevant tests, pre-push checks when applicable).
2. Run manual verification scenarios and record evidence.
3. Re-review acceptance criteria after implementation and update verification evidence.
4. Complete an evidence-based implementation review. Create or update an
   issue-local retrospective for reusable lessons, material design changes, or
   meaningful deviations from the plan; otherwise record why one was unnecessary
   in the issue progress log.

### Phase 1: decision

- Outcome: the ADR and the skill update of SI-1 are merged; the baseline reproduction of SI-7 is
  recorded.
- Exit criteria: the ADR is accepted by the maintainer and the reviewers.

### Phase 2: removal

- Outcome: SI-2 to SI-5 merged.
- Exit criteria: no `location` field and no `LocatedError` source in the packages they cover; each
  sub-issue's protocol tests pass.

### Phase 3: enforcement and acceptance

- Outcome: SI-6 merged; the recheck of SI-7 done; #384 answered and closed; the upstream issue
  drafts handed over.
- Exit criteria: the check passes on `develop` with no migration entries in its allow-list, and
  the recheck evidence shows no source path in any client reply.

## Progress Tracking

### Workflow Checkpoints

- [x] Epic spec drafted in `docs/issues/drafts/` and moved to `docs/issues/open/`
- [x] Epic spec reviewed and approved by user/maintainer
- [x] GitHub epic issue created and issue number added to this spec
- [x] Subissues created and linked in this spec
- [ ] Subissue statuses kept up to date in the `Subissues` table
- [ ] For each implemented subissue: automatic checks completed and recorded
- [ ] For each implemented subissue: manual verification completed and recorded
- [ ] For each implemented subissue: acceptance criteria reviewed post-implementation
- [ ] For each implemented subissue: implementation completion review recorded
- [ ] Epic acceptance criteria reviewed and checked off
- [ ] Epic issue closed and spec moved from `docs/issues/open/` to `docs/issues/closed/`

### Progress Log

Append one line per meaningful update. When an AI agent did the work, name its model after the
agent, for example `GitHub Copilot (Claude Opus 5.5)`, so later reviews can relate outcomes to
the model that produced them.

- 2026-11-11 17:03 UTC - Claude Code (Fable 5.2) - Drafted the EPIC, six sub-issue drafts and the specification for #384 from the audit of issue #2804, after the maintainer proposed to remove the feature. Not approved; no GitHub issue created.
- 2026-11-17 16:45 UTC - Claude Code (Claude Opus 5.5) - EPIC coordinator recorded the maintainer's decisions (remove the field; keep #384's title; one session at a time; Opus 5.5 for sub-issue sessions).
- 2026-11-17 17:01 UTC - Claude Code (Claude Opus 5.5) - Maintainer approved the specification. Created EPIC #2912 and sub-issues #2913 to #2918 as `josecelano-bot`, commented on #384, and moved the specification and the six sub-issue specifications from `docs/issues/drafts/` to `docs/issues/open/`. GitHub sub-issue links wait for the maintainer (the bot account cannot add them).
- 2026-11-22 14:24 UTC - Claude Code (Claude Sonnet 5.5) - Added SI-8 (#2996, share the `assert_names_no_source_path` test helper, finding F5 of PR #2991) as row 8 of the `Subissues` table with its specification; the maintainer approved the specification on 2026-11-22.

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
- [ ] No non-test code in the workspace uses `Location::caller()`, `#[track_caller]` or `torrust-located-error`, enforced by the check of SI-6.
- [ ] No reply to an HTTP, UDP or REST client holds a source path, shown by the recheck of SI-7.
- [ ] The removal landed before the first publish of each affected crate, or the extra release is recorded.

### Acceptance Verification

| AC ID | Status (`TODO`/`DONE`) | Evidence |
| --- | --- | --- |
| AC1 | TODO | {issue/spec/PR links} |
| AC2 | TODO | {issue/spec/PR links} |
| AC3 | TODO | {issue/spec/PR links} |
| AC4 | TODO | {issue/spec/PR links} |
| AC5 | TODO | {issue/spec/PR links} |
| AC6 | TODO | {issue/spec/PR links} |
| AC7 | TODO | {issue/spec/PR links} |
| AC8 | TODO | {issue/spec/PR links} |
| AC9 | TODO | {issue/spec/PR links} |
| AC10 | TODO | {issue/spec/PR links} |
| AC11 | TODO | {issue/spec/PR links} |
| AC12 | TODO | {issue/spec/PR links} |

## Risks and Trade-offs

- **Losing the developer hint.** Mitigation: `tracing` events at the sites where the line was the
  only way to tell failures apart (SI-4), and messages that name what failed.
- **Breaking public types.** Mitigation: do it before the first publish; `primitives` and
  `configuration` inside their planned 4.0.0.
- **Reworded client messages.** Old text changed with every refactor, so no client could rely on
  it. Mitigation: each sub-issue lists old and new texts for approval.
- **The decision is reversed in review.** Mitigation: SI-1 goes first and no code starts before it.
- **A registry crate keeps printing a location.** Mitigation: the workspace stops formatting that
  source into client text (SI-3) and the upstream issue is handed over (SI-6).
- **Scope growth into a general error redesign.** Mitigation: the Out of Scope list; anything else
  is a follow-up.

## References

- Related issues: #2804 (audit that produced this EPIC), #384 (external report, sub-issue SI-7),
  #2583 (first publish of the workspace crates), #1347 (the EPIC whose primitives test work reported
  the finding)
- Related PRs: #2805 (audit specification), #2814 (inventory), #2821 (option analysis)
- Related ADRs: `docs/adrs/20261005145329_return_result_only_for_concretely_fallible_public_apis.md`;
  the ADR created by SI-1
