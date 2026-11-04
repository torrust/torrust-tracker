---
pr-number: 2711
pr-url: https://github.com/torrust/torrust-tracker/pull/2711
last-updated-utc: "2026-11-04 12:32"
---

# PR #2711 Review Audit

Source: pull-request reviews and inline review threads for https://github.com/torrust/torrust-tracker/pull/2711.

## Ownership

The PR author owns this tracked audit record. Reviewers, including repository review agents,
deliver findings through GitHub and have no repository-artifact obligation.

- Post-merge workflow approval: N/A

## Status Values

- Relationship: `ORIGINAL`, `RE_RAISE_OF:<FindingId>`
- Disposition: `FIXED`, `NO_ACTION`, `SUPERSEDED`, `FOLLOW_UP`
- Thread state: `OPEN`, `RESOLVED`, `NON_RESOLVABLE`, `SUPERSEDED`
- Severity: `Blocker`, `Major`, `Minor`, `Nit`, `Suggestion`; append `(inferred)` when derived
  from free prose.
- Author class: `Copilot`, `Human`, `Unknown`
- Category: `link-integrity`, `formatting`, `metadata`, `testing`, `correctness`,
  `documentation`, `maintainability`, `security`, `other`

## Findings

| Finding ID | Review finding reference | Author class | Severity | Category | Relationship | Disposition | Thread state |
| ---------- | ------------------------ | ------------ | -------- | -------- | ------------ | ----------- | ------------ |
| F1 | Copilot review 2026-11-04T12:15:55Z, inline comment 4467311382 | Copilot | Major (inferred) | correctness | ORIGINAL | NO_ACTION | SUPERSEDED |
| F2 | Copilot review 2026-11-04T12:15:55Z, inline comment 4467311434 | Copilot | Minor (inferred) | maintainability | ORIGINAL | FIXED | RESOLVED |

Round 1: one Copilot review with two inline threads (F1, F2). No review existed when this audit started; the review arrived at 12:15:55Z, 37 minutes after the Copilot request and on the head of that time.

## Finding Details

### F1 - `Future` said to be missing from the imports of the production-style example

- PR number: 2711
- Source review ID: 5768068187
- Reviewer finding ID: F1
- Source URL: https://github.com/torrust/torrust-tracker/pull/2711#discussion_r4467311382
- Concern: `impl Future<Output = &'static str>` in `install_shutdown_signal_handlers` is used without importing `Future`, which the reviewer expected to fail to compile.
- Solution: Not a defect, no change. The workspace edition is 2024, whose prelude includes `Future`; the existing `udp_only_public_tracker.rs` uses the same signature.
- Current-tree verification: `edition = "2024"` is in the root `Cargo.toml`; `cargo build -p torrust-tracker-udp-server --examples`, `cargo clippy -p torrust-tracker-udp-server --examples -- -D warnings` and `cargo +nightly check --tests --benches --examples --workspace --all-targets --all-features` pass on the current tree (evidence V6).
- Resolution reference: https://github.com/torrust/torrust-tracker/pull/2711#discussion_r4467495123
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2711#discussion_r4467495123

### F2 - Fixed temporary database file name in the production-style example

- PR number: 2711
- Source review ID: 5768068187
- Reviewer finding ID: F2
- Source URL: https://github.com/torrust/torrust-tracker/pull/2711#discussion_r4467311434
- Concern: The example creates its SQLite database at a fixed name in the shared temporary directory, which collides across concurrent runs and leaves stale state after a crash.
- Solution: The finding was right: the file name now includes the process id. The older `udp_only_public_tracker.rs` has the same fixed name; it is unchanged code outside this PR's scope and was left as is.
- Current-tree verification: `packages/udp-server/examples/udp_only_public_tracker_production_style.rs` builds the path with `std::process::id()`; the example was run, answered a UDP connect request, stopped on SIGTERM and left no file in the temporary directory.
- Resolution reference: `fix(udp-server): [#2699] use a per-process database file in the production-style example`
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2711#discussion_r4467495464

## Processing Log

- 2026-11-04 11:38 UTC - Started audit. The PR was opened at 2026-11-04T11:38:16Z. No reviews, comments, or threads existed yet; Copilot and da2ce7 are requested after this commit is pushed, so the push does not dismiss a review, and the next entry records the requests.

## Completion Rules

- Re-derive the reply claim against the current tree before replying or resolving a thread.
- Reply on every resolvable thread before resolving it.
- For an outdated thread whose concern was fixed, record `Disposition=FIXED` and
  `Thread state=RESOLVED`, even if GitHub marks the thread outdated after the push. For a
  duplicate, superseded, or no-change in-PR thread, reply exactly
  `Superseded by <FindingId>: <reason>.`, record `Disposition=NO_ACTION` and
  `Thread state=SUPERSEDED`, then resolve it. A post-merge `NO_ACTION` requires maintainer
  approval to decline the follow-up work.
- A consolidated PR conversation response may cover multiple review rounds only when it names
  every review ID and every finding ID with its disposition and resolution reference. Record its
  durable URL in each related row.
- Cite a fix by its unique Conventional Commit subject or durable reply URL, never by a branch SHA
  that can change after a rebase.
- Refresh review threads using GraphQL and confirm that no unresolved actionable thread remains.
- 2026-11-04 12:26 UTC - The `Test (Docker) (release)` job failed on the first push (head at that time: the audit-start commit): `cargo chef cook` rejected the third-party recipe because `torrust-cargo-chef` 0.1.78 does not prune the `testing` feature entry that names the now-optional workspace dependency. This is a CI failure, not a review finding. Fixed outside this repository (https://github.com/torrust/cargo-chef/pull/1, merged and published as 0.1.79) and in this PR by bumping the pin in the `Containerfile`; the decision and options are in the spec. Copilot's review of 12:15:55Z, which carries two inline findings, had already arrived when this entry was written; it is recorded as round 1 in the next entry. Copilot and da2ce7 were requested at 11:38 and 11:40 UTC and are re-requested after the next push.
- 2026-11-04 12:32 UTC - Round 1: Copilot reviewed at 12:15:55Z (COMMENTED, with the "didn't start before the timeout" warning) and left two inline threads. Verified F1 and F2 against the current tree: F1 is not a defect (edition 2024 prelude), so its thread got the `Superseded by F1: ...` reply and was resolved with `NO_ACTION`; F2 was right and was fixed in its own commit. Both fixes and the pin bump were pushed at 12:31 UTC; replies to both threads were posted and both threads resolved at 12:31 UTC. Copilot is re-requested once because of the timeout warning and da2ce7 is re-requested after the push of this audit; that is recorded in the next entry.
