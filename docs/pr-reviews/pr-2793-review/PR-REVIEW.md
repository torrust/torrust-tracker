---
semantic-links:
  skill-links:
    - process-pr-review
  related-artifacts:
    - .github/skills/dev/pr-reviews/process-pr-review/SKILL.md
---

<!-- skill-link: process-pr-review -->

# PR #2793 Review Audit

Source: pull-request reviews and inline review threads for https://github.com/torrust/torrust-tracker/pull/2793.

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

| Finding ID | Review finding reference | Author class | Severity | Category | Relationship | Disposition | Thread state |
| ---------- | ------------------------ | ------------ | -------- | -------- | ------------ | ----------- | ------------ |
| F1 | review-finding:pr-2793-f1 | Copilot | Minor | maintainability | ORIGINAL | FIXED | RESOLVED |
| F2 | review-finding:pr-2793-f2 | Copilot | Minor | documentation | ORIGINAL | FIXED | RESOLVED |
| F3 | review-finding:pr-2793-f3 | Copilot | Minor | maintainability | ORIGINAL | FIXED | RESOLVED |
| F4 | review-finding:pr-2793-f4 | Human | Major | documentation | ORIGINAL | FIXED | RESOLVED |
| F5 | review-finding:pr-2793-f5 | Human | Major | documentation | ORIGINAL | FIXED | RESOLVED |
| F6 | review-finding:pr-2793-f6 | Human | Minor | maintainability | RE_RAISE_OF:F1 | FIXED | RESOLVED |
| F7 | review-finding:pr-2793-f7 | Human | Minor | documentation | RE_RAISE_OF:F2 | FIXED | RESOLVED |
| F8 | review-finding:pr-2793-f8 | Human | Minor | documentation | ORIGINAL | FIXED | RESOLVED |
| F9 | review-finding:pr-2793-f9 | Human | Minor | testing | ORIGINAL | FIXED | RESOLVED |
| F10 | review-finding:pr-2793-f10 | Human | Nit | metadata | ORIGINAL | FIXED | RESOLVED |
| F11 | review-finding:pr-2793-f11 | Human | Nit | maintainability | ORIGINAL | NO_ACTION | OPEN |
| F12 | review-finding:pr-2793-f12 | Human | Suggestion | documentation | ORIGINAL | FIXED | RESOLVED |

## Finding Details

### F1 - Remove the impl-wide allowance

- PR number: 2793
- Source review ID: 5839493923
- Reviewer finding ID: R1
- Source URL: https://github.com/torrust/torrust-tracker/pull/2793#discussion_r4523127511
- Concern: An impl-wide allow(deprecated) on HttpServer<Stopped> also covered the token-aware start methods and hid them from -D deprecated.
- Solution: Removed the impl-wide allowance; the two legacy start methods keep scoped allowances.
- Current-tree verification: Read the impl header: it carries no allowance; the workspace check `RUSTFLAGS="-D deprecated" cargo check --workspace --lib --bins` exits 0.
- Resolution reference: fix: [#2751] deprecate the running-state aliases and Running::new, add the signal-ownership note and narrow an allowance
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2793#discussion_r4523371287

### F2 - Give the UDP form in the guide

- PR number: 2793
- Source review ID: 5839493923
- Reviewer finding ID: F4
- Source URL: https://github.com/torrust/torrust-tracker/pull/2793#discussion_r4523127620
- Concern: The guide told UDP users to join a nonexistent shutdown_controller.
- Solution: Gave the UDP form (single task) and scoped the controller step to HTTP, REST API and health-check.
- Current-tree verification: Read the guide's Stop-by-cancelling bullet and the UDP table rows.
- Resolution reference: docs(shutdown): [#2751] give the UDP form in the migration guide and link the guide from the README
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2793#discussion_r4523374628

### F3 - Deprecate the running-state aliases

- PR number: 2793
- Source review ID: 5839493923
- Reviewer finding ID: F1
- Source URL: https://github.com/torrust/torrust-tracker/pull/2793#discussion_r4523127672
- Concern: The running-state aliases were not deprecated with their types.
- Solution: Deprecated RunningHttpServer, RunningApiServer and RunningUdpServer with the replacement and removal note.
- Current-tree verification: `cargo check` of a scratch consumer printed the deprecation for each alias (evidence M2).
- Resolution reference: fix: [#2751] deprecate the running-state aliases and Running::new, add the signal-ownership note and narrow an allowance
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2793#discussion_r4523375309

### F4 - Complete the verification, acceptance and completion sections

- PR number: 2793
- Source review ID: 5839541059
- Reviewer finding ID: F1
- Source URL: https://github.com/torrust/torrust-tracker/pull/2793#discussion_r4523161783
- Concern: The spec was marked done while its verification, acceptance and completion sections still said unverified, and M2 had not been run.
- Solution: Ran M2 from scratch consumers, recorded the output, added compile_fail doctests as a guard, and updated M2 to M4, the acceptance rows, the frontmatter and the completion review.
- Current-tree verification: Read the spec sections and the evidence file; `cargo test --doc` runs the compile_fail examples, and removing graceful_shutdown's attribute made its example fail.
- Resolution reference: docs(issues): [#2751] complete the SI-18 verification, acceptance and completion sections
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2793#discussion_r4523372365

### F5 - Say that executables own OS-signal subscriptions

- PR number: 2793
- Source review ID: 5839541059
- Reviewer finding ID: F2
- Source URL: https://github.com/torrust/torrust-tracker/pull/2793#discussion_r4523161792
- Concern: No deprecation note said that executables, not libraries, own OS-signal subscriptions (Requirement 1).
- Solution: Appended the clause and a pointer to the migration guide to every note.
- Current-tree verification: `cargo check` of a scratch consumer printed the clause in each note (evidence M2).
- Resolution reference: fix: [#2751] deprecate the running-state aliases and Running::new, add the signal-ownership note and narrow an allowance
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2793#discussion_r4523373950

### F6 - Remove the impl-wide allowance (re-raise)

- PR number: 2793
- Source review ID: 5839541059
- Reviewer finding ID: F3
- Source URL: https://github.com/torrust/torrust-tracker/pull/2793#discussion_r4523161798
- Concern: The impl-wide allowance silenced the token-aware start methods.
- Solution: Removed it.
- Current-tree verification: Read the impl header; it carries no allowance.
- Resolution reference: fix: [#2751] deprecate the running-state aliases and Running::new, add the signal-ownership note and narrow an allowance
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2793#discussion_r4523374319

### F7 - Scope the controller sentence to non-UDP servers

- PR number: 2793
- Source review ID: 5839541059
- Reviewer finding ID: F4
- Source URL: https://github.com/torrust/torrust-tracker/pull/2793#discussion_r4523161802
- Concern: "join task and shutdown_controller" is wrong for udp-server.
- Solution: Scoped the sentence and gave the UDP form; the UDP stop note says await the returned task.
- Current-tree verification: Read the guide and the UDP stop note.
- Resolution reference: docs(shutdown): [#2751] give the UDP form in the migration guide and link the guide from the README
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2793#discussion_r4523372010

### F8 - Deprecate Running::new

- PR number: 2793
- Source review ID: 5839541059
- Reviewer finding ID: F5
- Source URL: https://github.com/torrust/torrust-tracker/pull/2793#discussion_r4523161808
- Concern: The REST API Running::new was listed as deprecated but carried no attribute.
- Solution: Annotated it.
- Current-tree verification: `cargo check` of a scratch consumer printed the deprecation for Running::new (evidence M2).
- Resolution reference: fix: [#2751] deprecate the running-state aliases and Running::new, add the signal-ownership note and narrow an allowance
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2793#discussion_r4523371608

### F9 - Name the test by condition and mark Arrange, Act and Assert

- PR number: 2793
- Source review ID: 5839541059
- Reviewer finding ID: F6
- Source URL: https://github.com/torrust/torrust-tracker/pull/2793#discussion_r4523161813
- Concern: The new test had no Arrange/Act/Assert, no condition in its name and no recorded design review.
- Solution: Renamed it to it_should_stop_the_legacy_server_when_a_normal_halt_is_sent, marked the sections and recorded the prose design in the evidence file.
- Current-tree verification: Read the test and the evidence file's Behavior section.
- Resolution reference: test(health-check-api): [#2751] name the legacy start test by condition and mark Arrange, Act and Assert
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2793#discussion_r4523372717

### F10 - Date the progress-log entry

- PR number: 2793
- Source review ID: 5839541059
- Reviewer finding ID: F7
- Source URL: https://github.com/torrust/torrust-tracker/pull/2793#discussion_r4523161814
- Concern: The progress-log entry had no date.
- Solution: Dated it 2026-11-09 19:33 UTC.
- Current-tree verification: Read the spec's progress log.
- Resolution reference: docs(issues): [#2751] complete the SI-18 verification, acceptance and completion sections
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2793#discussion_r4523373147

### F11 - Commit type of the test commit

- PR number: 2793
- Source review ID: 5839541059
- Reviewer finding ID: F8
- Source URL: https://github.com/torrust/torrust-tracker/pull/2793#discussion_r4523161817
- Concern: A commit typed docs(shutdown) added a Rust test.
- Solution: Not changed: splitting it would rewrite pushed history and the bot's push wrapper has no force option; later test changes use test-typed commits. The thread is left open for the maintainer's reviewer.
- Current-tree verification: N/A
- Resolution reference: https://github.com/torrust/torrust-tracker/pull/2793#discussion_r4523373565
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2793#discussion_r4523373565

### F12 - Link the guide from the README text

- PR number: 2793
- Source review ID: 5839541059
- Reviewer finding ID: F9
- Source URL: https://github.com/torrust/torrust-tracker/pull/2793#discussion_r4523161825
- Concern: The migration guide was linked only from the README frontmatter.
- Solution: Linked it from the README Status text.
- Current-tree verification: Read the README Status section.
- Resolution reference: docs(shutdown): [#2751] give the UDP form in the migration guide and link the guide from the README
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2793#discussion_r4523374962

## Processing Log

- 2026-11-10 09:21 UTC - Copilot review 5839493923 submitted with 3 inline findings (audited as F1 to F3); the reviewer's IDs were R1, F4 and F1.
- 2026-11-10 09:25 UTC - Review 5839541059 by `da2ce7` submitted with 9 inline findings (audited as F4 to F12; F6 and F7 re-raise F1 and F2).
- 2026-11-10 09:30 UTC - Started on both reviews. The fix commits were made between 09:35 and 09:44 UTC; the history split asked for by F11 was not pushed, because it needs a force push.
- 2026-11-10 09:46 UTC - Pushed the fix commits; the pull-request head matched the local head before any reply.
- 2026-11-10 09:48 UTC - Replied on all 12 threads (the forge's `created_at` runs from 09:48:01 to 09:48:28 UTC) and resolved 11 of them, each returning `isResolved: true`. The F11 thread stays open for the maintainer's reviewer.

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
