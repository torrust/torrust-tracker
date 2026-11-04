---
pr-number: 2711
pr-url: https://github.com/torrust/torrust-tracker/pull/2711
last-updated-utc: "2026-11-04 13:20"
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
| F3 | Copilot review 2026-11-04T12:52:28Z, inline comment 4467698875 | Copilot | Nit (inferred) | maintainability | ORIGINAL | FIXED | RESOLVED |
| F4 | Copilot review 2026-11-04T12:52:28Z, inline comment 4467698939 | Copilot | Nit (inferred) | formatting | ORIGINAL | NO_ACTION | SUPERSEDED |
| F5 | Copilot review 2026-11-04T12:52:28Z, inline comment 4467698998 | Copilot | Minor (inferred) | documentation | ORIGINAL | FIXED | RESOLVED |

Round 1: one Copilot review with two inline threads (F1, F2). Round 2: one Copilot review with three inline threads (F3, F4, F5). No review existed when this audit started; the review arrived at 12:15:55Z, 37 minutes after the Copilot request and on the head of that time.

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

### F3 - Fixed-size array in the listener helper of the production-style example

- PR number: 2711
- Source review ID: 5768592714
- Reviewer finding ID: F3
- Source URL: https://github.com/torrust/torrust-tracker/pull/2711#discussion_r4467698875
- Concern: `stop_event_listeners` takes `[JoinHandle<()>; 3]`, which couples the helper to the number of listeners.
- Solution: The finding was fair: the helper now takes `impl IntoIterator<Item = JoinHandle<()>>`.
- Current-tree verification: `packages/udp-server/examples/udp_only_public_tracker_production_style.rs` has the new signature; clippy `-D warnings` is clean and the example was run, answered a UDP connect request and stopped on SIGTERM.
- Resolution reference: `refactor(udp-server): [#2699] accept any number of listeners in the production-style example helper`
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2711#discussion_r4467786732

### F4 - `default = [  ]` spelling in the `udp-server` manifest

- PR number: 2711
- Source review ID: 5768592714
- Reviewer finding ID: F4
- Source URL: https://github.com/torrust/torrust-tracker/pull/2711#discussion_r4467698939
- Concern: `default = [  ]` was said to fight TOML formatters and to be better written `default = []`.
- Solution: Not a defect, no change. That spelling is what the repository's `taplo` formatting requires; `tracker-core` and `udp-protocol` use it too.
- Current-tree verification: with `default = []` in `packages/udp-server/Cargo.toml`, `taplo fmt --check` exited 1 ("the file is not properly formatted"); with the committed spelling it exited 0; the manifest was restored (`git diff --quiet` on it exits 0).
- Resolution reference: https://github.com/torrust/torrust-tracker/pull/2711#discussion_r4467787105
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2711#discussion_r4467787105

### F5 - Wording of the `testing` module note

- PR number: 2711
- Source review ID: 5768592714
- Reviewer finding ID: F5
- Source URL: https://github.com/torrust/torrust-tracker/pull/2711#discussion_r4467698998
- Concern: "Enable it from a `[dev-dependencies]` entry only" reads as a Cargo limit, and it contradicts the example headers, which say an end user may adopt the testing style.
- Solution: The finding had merit on the wording: the note now states the rule as a workspace rule enforced by the `Shipped Features` check and says that anyone else opts in knowingly. The claim that examples and benches need another wiring is not right: they use `[dev-dependencies]`, and target-specific dev-dependency tables are still dev-dependencies.
- Current-tree verification: `packages/udp-server/src/testing/mod.rs` carries the new note; `cargo doc -p torrust-tracker-udp-server --no-deps --features testing` shows no warning.
- Resolution reference: `docs(udp-server): [#2699] word the testing module note as a workspace rule, not a Cargo limit`
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2711#discussion_r4467787532

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
- 2026-11-04 13:00 UTC - Round 2: the re-request of 12:34:30Z (Copilot timeline event) produced a second Copilot review at 12:52:28Z on the head of 12:34 UTC (COMMENTED, again with the "didn't start before the timeout" warning) with three inline threads and its overview listing F1 and F2 as resolved. Verified F3, F4 and F5 against the current tree: F3 and F5 were fixed in their own commits, F4 is not a defect (taplo fails on `default = []`) and got the `Superseded by F4: ...` reply with `NO_ACTION`. The fixes were pushed at 13:00 UTC, the three replies were posted and the three threads resolved at 13:00 UTC. The one allowed Copilot re-request was used at 12:34 UTC, so Copilot is not requested again; da2ce7 is re-requested after the push of this audit and that is recorded in the next entry.
- 2026-11-04 13:20 UTC - The `Test (Docker) (release)` job passed on the head of 13:00 UTC (run 41834169245), so the container failure of the first push is resolved (evidence V10). da2ce7's review request of 11:40:46Z is still pending (`GET pulls/2711/requested_reviewers` lists him at 13:20 UTC) and he has not reviewed, so a push does not dismiss anything of his and no new request is needed; this audit and the spec updates are pushed once more as the last change, after which the maintainer decides whether to merge without waiting for him.
