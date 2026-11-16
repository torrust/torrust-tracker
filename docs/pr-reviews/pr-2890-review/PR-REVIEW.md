---
semantic-links:
  skill-links:
    - process-pr-review
  related-artifacts:
    - .claude/skills/process-pr-review/SKILL.md
---

<!-- skill-link: process-pr-review -->

# PR #2890 Review Audit

Source: pull-request reviews and inline review threads for https://github.com/torrust/torrust-tracker/pull/2890.

## Ownership

The PR author owns this tracked audit record. Reviewers, including repository review agents,
deliver findings through GitHub and have no repository-artifact obligation.

- Post-merge workflow approval: N/A

## Status Values

- Relationship: `ORIGINAL`, `RE_RAISE_OF:<FindingId>`
- Disposition: `FIXED`, `NO_ACTION`, `SUPERSEDED`, `FOLLOW_UP`
- Thread state: `OPEN`, `RESOLVED`, `NON_RESOLVABLE`, `SUPERSEDED`. `OPEN` applies prospectively
  to audits created or updated for approved follow-up work; historical audits remain valid without
  bulk migration.
- Severity: `Blocker`, `Major`, `Minor`, `Nit`, `Suggestion`; append `(inferred)` when derived
  from free prose.
- Author class: `Copilot`, `Human`, `Unknown`
- Category: `link-integrity`, `formatting`, `metadata`, `testing`, `correctness`,
  `documentation`, `maintainability`, `security`, `other`
- An outdated thread whose concern was fixed is `FIXED`/`RESOLVED`, even when GitHub marks the
  original thread outdated after the push. For in-PR feedback, use `NO_ACTION`/`SUPERSEDED` only
  for a duplicate, superseded, or no-change concern. A post-merge `NO_ACTION` requires maintainer
  approval to decline the follow-up work.
- When the current identity cannot resolve a thread (`viewerCanResolve` is `false`), keep the
  disposition established from the current tree and record `Thread state=OPEN`. The reviewer or PR
  author resolves it; permission says nothing about whether the concern was fixed.

## Findings

F1 to F7 are in da2ce7's review 5924864275 (`CHANGES_REQUESTED`, on the head `docs(issues): [#2846] record the SI-27 no-go and close its spec`), in that source order, and keep his IDs.

Copilot's review 5924775596 has no finding: its overview reports no open issues.

F8 is in da2ce7's review 5925179048 (`APPROVED`, on the head `docs(pr-reviews): [#2846] audit the first review of PR #2890`) and keeps his ID; the F8 fix push dismissed that approval.

| Finding ID | Review finding reference | Author class | Severity | Category | Relationship | Disposition | Thread state |
| ---------- | ------------------------ | ------------ | -------- | -------- | ------------ | ----------- | ------------ |
| F1 | review-finding:pr-2890-f1 | Human | Major | correctness | ORIGINAL | FIXED | RESOLVED |
| F2 | review-finding:pr-2890-f2 | Human | Minor | metadata | ORIGINAL | FIXED | RESOLVED |
| F3 | review-finding:pr-2890-f3 | Human | Minor | documentation | ORIGINAL | FIXED | RESOLVED |
| F4 | review-finding:pr-2890-f4 | Human | Nit | correctness | ORIGINAL | FIXED | RESOLVED |
| F5 | review-finding:pr-2890-f5 | Human | Nit | documentation | ORIGINAL | FIXED | RESOLVED |
| F6 | review-finding:pr-2890-f6 | Human | Nit | documentation | ORIGINAL | FIXED | RESOLVED |
| F7 | review-finding:pr-2890-f7 | Human | Nit | maintainability | ORIGINAL | NO_ACTION | SUPERSEDED |
| F8 | review-finding:pr-2890-f8 | Human | Nit | correctness | ORIGINAL | FIXED | RESOLVED |

## Finding Details

### F1 - State which servers hold a registration form and which hold the Registar

- PR number: 2890
- Source review ID: 5924864275
- Reviewer finding ID: F1
- Source URL: https://github.com/torrust/torrust-tracker/pull/2890#discussion_r4589472825
- Concern: The ADR said every server holds only a single-use `ServiceRegistrationForm` and can never remove anything, and scoped both alternatives to four server packages. The health check API server takes the whole `Registar` in `start_with_cancellation`, registers itself through `give_form()` and returns its `service_binding` for a caller that deregisters it; its job holds the same `Registar`. The same sentence was in the spec progress log and the PR description.
- Solution: The ADR names the HTTP tracker, REST API and UDP tracker as the form-holding servers, describes the health check API server as the registry reader that holds the `Registar` and already returns its binding, and scopes the capability reason and both alternatives to the three form-holding packages. The spec progress log gets an appended correction entry; the original T1 entry is kept. The PR description is corrected in the same round.
- Current-tree verification: `packages/axum-health-check-api-server/src/server.rs` takes `registar: Registar<RuntimeServiceMetadata>` in `start_with_cancellation`, calls `registar.give_form()` and documents `CancellationRunning::service_binding` for `Registar::deregister`; `src/bootstrap/jobs/health_check_api.rs` passes the `Registar`. The ADR sections Description, Agreement (reasons) and Alternatives Considered now say three form-holding servers.
- Resolution reference: docs(adrs): [#2846] scope the registration capability to the three form-holding servers
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2890#discussion_r4589591586

### F2 - Use the done status for the archived spec

- PR number: 2890
- Source review ID: 5924864275
- Reviewer finding ID: F2
- Source URL: https://github.com/torrust/torrust-tracker/pull/2890#discussion_r4589472837
- Concern: `status: closed` is not an allowed frontmatter value; `cleanup-completed-issues` and the frontmatter validator require `done` in `docs/issues/closed/`.
- Solution: The spec frontmatter says `status: done`; the not-planned outcome stays in the body.
- Current-tree verification: The spec frontmatter line 5 reads `status: done`.
- Resolution reference: docs(issues): [#2846] use the done status for the archived SI-27 spec
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2890#discussion_r4589592069

### F3 - List the new ADR in the spec Architectural Decisions section

- PR number: 2890
- Source review ID: 5924864275
- Reviewer finding ID: F3
- Source URL: https://github.com/torrust/torrust-tracker/pull/2890#discussion_r4589472844
- Concern: The section still said no ADR was expected and that a root-supervisor outcome would go into the SI-21 ADR.
- Solution: The section keeps the original plan and adds the ADR created after the no-go and the SI-21 ADR amendment, and states that no version bump is needed because no package changes.
- Current-tree verification: The Architectural Decisions section of the spec lists `docs/adrs/20261116172628_keep_registar_registrations_for_the_process_lifetime.md`.
- Resolution reference: docs(issues): [#2846] list the registration lifetime ADR in the SI-27 spec
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2890#discussion_r4589592439

### F4 - Say that only services with a health check are probed

- PR number: 2890
- Source review ID: 5924864275
- Reviewer finding ID: F4
- Source URL: https://github.com/torrust/torrust-tracker/pull/2890#discussion_r4589472853
- Concern: The ADR said the health check opens a connection to each registered service; registrations without a check are skipped, and the health check API registers itself with none.
- Solution: Both statements now say "each registered service that has a health check", and the Description names the health check API as the exception.
- Current-tree verification: `packages/axum-health-check-api-server/src/handlers.rs` `probe_services` uses `filter_map` over `spawn_check()`; the ADR Description and Agreement use the narrowed wording.
- Resolution reference: docs(adrs): [#2846] say that only services with a health check are probed
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2890#discussion_r4589592927

### F5 - Mark the publish gate as superseded by the no-go

- PR number: 2890
- Source review ID: 5924864275
- Reviewer finding ID: F5
- Source URL: https://github.com/torrust/torrust-tracker/pull/2890#discussion_r4589472860
- Concern: The EPIC position line still said SI-27 must land before the first publish (#2583).
- Solution: The line says SI-27 was to land before #2583 and that the no-go superseded it. The same statement in the Dependencies section gets the same note (same mistake, swept in the same commit).
- Current-tree verification: The EPIC position block and the Dependencies bullet of the spec both state that SI-27 no longer gates #2583.
- Resolution reference: docs(issues): [#2846] mark the SI-27 publish gate as superseded by the no-go
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2890#discussion_r4589593352

### F6 - Add an Affected Code section to the ADR

- PR number: 2890
- Source review ID: 5924864275
- Reviewer finding ID: F6
- Source URL: https://github.com/torrust/torrust-tracker/pull/2890#discussion_r4589472870
- Concern: `create-adr` Step 3.5 asks for an "Affected Code" section; the code links were only in the frontmatter and the References, and the PR description cited Step 3.5 as satisfied by a field comment.
- Solution: The ADR has an "Affected Code" section linking the six files that implement or rely on the decision. The `src/container.rs` field comment stays; the reviewer accepted either form. The PR description is corrected.
- Current-tree verification: The "Affected Code" section of the ADR; `linter lychee` passes.
- Resolution reference: docs(adrs): [#2846] add an affected code section to the registration lifetime ADR
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2890#discussion_r4589593852

### F7 - Keep the two reviewed commits as they are

- PR number: 2890
- Source review ID: 5924864275
- Reviewer finding ID: F7
- Source URL: https://github.com/torrust/torrust-tracker/pull/2890#discussion_r4589472874
- Concern: In the first commit's own tree the spec is still under `open/`, so the ADR's spec link resolves only from the second commit on; the reviewer suggested moving the spec in the first commit, and called it cosmetic since only the head is gated.
- Solution: No change. Fixing it rewrites the two commits this review cites by id, and the repository keeps intermediate commits; the head, which is the gated tree, resolves every link.
- Current-tree verification: `linter lychee` passes on the head.
- Resolution reference: https://github.com/torrust/torrust-tracker/pull/2890#discussion_r4589594292
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2890#discussion_r4589594292

### F8 - Narrow where the job manager cancels running components

- PR number: 2890
- Source review ID: 5925179048
- Reviewer finding ID: F8
- Source URL: https://github.com/torrust/torrust-tracker/pull/2890#discussion_r4589716344
- Concern: The Affected Code section called `request_shutdown` and `mark_finished` "the only places that cancel component tokens"; `reserve` also issues an already-cancelled token after shutdown, and each server cancels its own token when its registration fails at startup.
- Solution: The Affected Code entry says these are where the job manager cancels the running components once shutdown is requested. The same over-broad claim in the ADR Description was narrowed in the same commit, the spec progress log has an appended correction, and the PR description says "a running component".
- Current-tree verification: `src/bootstrap/jobs/manager.rs` `reserve` cancels a new token when `self.cancelled`; the ADR Description and Affected Code use the narrowed wording.
- Resolution reference: docs(adrs): [#2846] narrow where the job manager cancels running components
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2890#discussion_r4589765055

## Processing Log

- 2026-11-16 17:53 UTC - Review 5924775596 (Copilot, `COMMENTED`) submitted on the pre-rebase head: no finding; no reply needed.
- 2026-11-16 17:59 UTC - Review 5924864275 (da2ce7, `CHANGES_REQUESTED`) submitted on the rebased head: one Major, two Minor and four Nit findings, recorded here as F1 to F7.
- 2026-11-16 18:07 UTC - Started audit. F1 to F6 fixed in six signed commits (committed 18:02 to 18:06 UTC); F7 declined.
- 2026-11-16 18:10 UTC - Pushed the six fix commits (push 18:10:07 UTC) and corrected the PR description for F1 and F6. Replied to the seven threads (18:10:24 to 18:10:40 UTC).
- 2026-11-16 18:11 UTC - Resolved the seven threads (18:11 UTC); a GraphQL refresh shows no unresolved thread.
- 2026-11-16 18:22 UTC - Review 5925179048 (da2ce7, `APPROVED`) submitted on the audit head: one Nit, recorded here as F8.
- 2026-11-16 18:26 UTC - Pushed the F8 fix (committed 18:24 UTC, push 18:26:43 UTC), which dismissed review 5925179048, and corrected the PR description. Replied to the F8 thread (18:26:44 UTC) and resolved it (18:26:53 UTC); a GraphQL refresh shows no unresolved thread.

## Completion Rules

- Re-derive the reply claim against the current tree before replying or resolving a thread.
- Reply on every resolvable thread before resolving it.
- For an outdated thread whose concern was fixed, record `Disposition=FIXED` and
  `Thread state=RESOLVED`, even if GitHub marks the thread outdated after the push. For a
  duplicate, superseded, or no-change in-PR thread, reply exactly
  `Superseded by <FindingId>: <reason>.`, record `Disposition=NO_ACTION` and
  `Thread state=SUPERSEDED`, then resolve it. A post-merge `NO_ACTION` requires maintainer
  approval to decline the follow-up work. When the current identity cannot resolve a thread
  (`viewerCanResolve` is `false`), still reply, keep the disposition established from the current
  tree, and record `Thread state=OPEN`; the reviewer or PR author resolves it.
- A consolidated PR conversation response may cover multiple review rounds only when it names
  every review ID and every finding ID with its disposition and resolution reference. Record its
  durable URL in each related row.
- Cite a fix by its unique Conventional Commit subject or durable reply URL, never by a branch SHA
  that can change after a rebase.
- Refresh review threads using GraphQL and confirm that no unresolved actionable thread remains.
