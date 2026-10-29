---
semantic-links:
  skill-links:
    - process-pr-review
  related-artifacts:
    - .github/skills/dev/pr-reviews/process-pr-review/SKILL.md
    - docs/issues/open/2681-2482-gate-tracker-core-test-modules-and-mocks/ISSUE.md
---

<!-- skill-link: process-pr-review -->

# PR #2688 Review Audit

Source: pull-request reviews and inline review threads for <https://github.com/torrust/torrust-tracker/pull/2688>.

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

## Findings

Two reviews of `copilot-pull-request-reviewer` have arrived. Review 5686786536 (COMMENTED, 2026-10-28 17:17 UTC, on the head whose subject is `docs(issues): [#2681] record the pre-PR review, the completion review and the retrospective`) carries a "didn't start before the timeout" warning, so Copilot was requested again once; it lists three open issues, each with one inline thread (F1 to F3). The re-requested review 5687032971 (COMMENTED, 17:35 UTC, on the head whose subject is `docs(pr-reviews): [#2681] record the review of PR 2688`) has no open issue and one "previously missed" finding in its body, without an inline thread (F4). Copilot was requested through the `request_copilot_review` MCP tool. da2ce7 was requested through the REST `requested_reviewers` endpoint and has not reviewed. The maintainer asked on 2026-10-28 to address Copilot's review.

| Finding ID | Review finding reference | Author class | Severity | Category | Relationship | Disposition | Thread state |
| ---------- | ------------------------ | ------------ | -------- | -------- | ------------ | ----------- | ------------ |
| F1 | review-finding:pr-2688-f1 | Copilot | Suggestion (inferred) | maintainability | ORIGINAL | NO_ACTION | SUPERSEDED |
| F2 | review-finding:pr-2688-f2 | Copilot | Suggestion (inferred) | maintainability | ORIGINAL | NO_ACTION | SUPERSEDED |
| F3 | review-finding:pr-2688-f3 | Copilot | Suggestion (inferred) | maintainability | ORIGINAL | NO_ACTION | SUPERSEDED |
| F4 | review-finding:pr-2688-f4 | Copilot | Minor (inferred) | testing | ORIGINAL | FIXED | NON_RESOLVABLE |

## Finding Details

### F1 - Inner `#[cfg(test)]` and `pub mod tests` in `src/test_helpers.rs`

- PR number: 2688
- Source review ID: 5686786536
- Reviewer finding ID: N/A
- Source URL: <https://github.com/torrust/torrust-tracker/pull/2688#discussion_r4403778680>
- Concern: the module is already declared `#[cfg(test)] mod test_helpers;`, so the inner `#[cfg(test)]` is redundant, and `pub mod tests` reads like exported API. The reviewer suggests removing the inner attribute and using `mod tests`, or keeping `pub(crate)` with a targeted `#[allow(clippy::redundant_pub_crate)]` (or `pub(super)`).
- Solution: No change to the code. The first version of this paragraph, and of the reply, said that `mod tests` or `pub(super)` cannot work because 28 references need crate-wide visibility. That was wrong for `pub(super)` (finding F5 of this review, which also names the correction below). A private `mod tests` cannot work for either helper, because callers outside the `test_helpers` module use it. `pub(super)` is rejected by clippy's `redundant_pub_crate` for this root helper, because `super` of `tests` here is the crate root, so the lint reads it as `pub(crate)`. `pub(crate)` with an `allow` is the option the spec rejected in D5 (a lint suppression for no benefit; `pub` is clippy's own suggestion), and a comment above the module explains the `pub`. Privacy does not come from the `pub`: the parent module is a private `#[cfg(test)] mod`, so nothing here is reachable outside the crate (the default `cargo doc` lists no helper entry). The inner `#[cfg(test)]` stays because rule C of ADR 20261021190318 says the module is declared `#[cfg(test)] mod` and its content is `#[cfg(test)]`.
- Current-tree verification: `grep -rn "crate::test_helpers::tests" packages/tracker-core/src` finds 27 references of the root helper, and a separate `grep` finds 1 reference of the whitelist helper (28 in total, in 19 files); `cargo clippy -p torrust-tracker-core --all-targets -- -D warnings` passes with `pub mod tests` and fails with `pub(crate) mod tests` (the earlier prototype). A temporary edit of the root helper to `pub(super) mod tests` failed the same clippy lint (`pub(crate) module inside private module`); it was reverted.
- Resolution reference: https://github.com/torrust/torrust-tracker/pull/2688#discussion_r4403811220
- Follow-up PR URL: N/A
- Reply URL: <https://github.com/torrust/torrust-tracker/pull/2688#discussion_r4403811220>

### F2 - Same pattern in `src/whitelist/test_helpers.rs`

- PR number: 2688
- Source review ID: 5686786536
- Reviewer finding ID: N/A
- Source URL: <https://github.com/torrust/torrust-tracker/pull/2688#discussion_r4403778758>
- Concern: the same two observations for the whitelist helper module: the inner `#[cfg(test)]` is redundant and `pub mod tests` can mislead; the reviewer prefers `mod tests` or `pub(super)`/`pub(crate)` with an `allow`.
- Solution: No change to the code. The first version of this paragraph said the same 28 references need crate-wide visibility; that was wrong for this helper (finding F5 of this review). Only one reference uses the whitelist helper, `crate::whitelist::test_helpers::tests` in `whitelist/mod.rs:31`, and its `super` is `whitelist`, which contains that caller, so `pub(super) mod tests` would have been valid here. The `pub` form stays because D5 of the spec chose one form for both helpers (clippy's own suggestion, no lint suppression, and `pub(super)` is rejected for the root helper); a private `mod tests` cannot work, and `pub(crate)` needs a lint suppression (D5). Rule C keeps the content `#[cfg(test)]`. The comment above the module explains the `pub`.
- Current-tree verification: `grep -rn "whitelist::test_helpers::tests" packages/tracker-core/src` finds one reference, in `whitelist/mod.rs:31`; the module is private and `#[cfg(test)]` in `whitelist/mod.rs`, so it is unreachable outside the crate. A temporary edit of this helper to `pub(super) mod tests` passed `cargo clippy -p torrust-tracker-core --all-targets -- -D warnings` and compiled the tests; it was reverted.
- Resolution reference: https://github.com/torrust/torrust-tracker/pull/2688#discussion_r4403811653
- Follow-up PR URL: N/A
- Reply URL: <https://github.com/torrust/torrust-tracker/pull/2688#discussion_r4403811653>

### F3 - Pinning `version = "0.1.0"` on a path dev-dependency

- PR number: 2688
- Source review ID: 5686786536
- Reviewer finding ID: N/A
- Source URL: <https://github.com/torrust/torrust-tracker/pull/2688#discussion_r4403778841>
- Concern: optional: a `version` on the path-based dev-dependency of `torrust-tracker-core` in `http-core` creates churn when the workspace version is bumped; the reviewer suggests omitting it if the repository convention allows.
- Solution: No change. Every dependency on a workspace crate in this repository carries `version` and `path` together (the normal `torrust-tracker-core` entry two sections above in the same manifest, and the `torrust-tracker-test-helpers` dev-dependency next to it), so a bump already touches each consumer. D4 of the spec records the choice; only the self dev-dependency is path-only, as ADR rule A.3 requires.
- Current-tree verification: `grep -n torrust-tracker-core packages/http-core/Cargo.toml packages/udp-core/Cargo.toml` shows the normal and the dev entries with the same `version` and `path`; `cargo +stable package --workspace --no-verify` passes.
- Resolution reference: https://github.com/torrust/torrust-tracker/pull/2688#discussion_r4403811999
- Follow-up PR URL: N/A
- Reply URL: <https://github.com/torrust/torrust-tracker/pull/2688#discussion_r4403811999>

### F4 - Test the preserved shipped-profile binary, not a target path that is rebuilt

- PR number: 2688
- Source review ID: 5687032971
- Reviewer finding ID: N/A
- Source URL: <https://github.com/torrust/torrust-tracker/pull/2688#pullrequestreview-5687032971>
- Concern: in the evidence file, the `TORRUST_TRACKER_BINARY` run points into Cargo's target directory, which the preceding plain test run can rebuild, so the transcript does not establish that the inspected shipped-profile binary was the one tested. The reviewer asks to copy the binary to a separate path right after building it, use the copy as the override, and record the actual result.
- Solution: the finding was right. The plain run did rebuild `target/debug/torrust-tracker` (its checksum changed). V4 of the evidence now records a copy made right after the shipped-profile build, its checksum before and after, the `strings` check on the copy, and the override run against the copy (25 tests pass, copy unchanged). The review has no inline thread, so there is nothing to reply to or resolve.
- Current-tree verification: ran the sequence build, copy, `sha256sum`, plain `cargo test -p torrust-tracker --test cli-configuration`, `sha256sum` again, then the override run: the target binary changed (`1653421cdeec9d69` to `2f9d185fca66a2e5`, first 16 hex digits) while the copy stayed `1653421cdeec9d69`, and the override run passed 25 tests.
- Resolution reference: docs(issues): [#2681] test the preserved shipped-profile binary in the evidence
- Follow-up PR URL: N/A
- Reply URL: N/A

## Processing Log

- 2026-10-28 17:22 UTC - Round 1: review 5686786536 arrived with a timeout warning and three inline threads, F1 to F3. Each claim was checked against the tree; none required a change, so each thread got the prescribed `Superseded by F<n>:` reply and was resolved. Copilot was re-requested once because of the warning.
- 2026-10-28 17:37 UTC - Round 2: the re-requested review 5687032971 arrived with no open issue and one review-body finding without a thread (F4). It was reproduced (the plain test run rebuilds the target binary), fixed in the evidence in its own signed commit, and recorded as `FIXED`/`NON_RESOLVABLE`. This record is committed and pushed after the fix; da2ce7 is re-requested after the push.
- 2026-10-29 11:18 UTC - Correction to the 2026-10-28 17:37 UTC entry (finding F6 of this review; the earlier entries are kept as written). That entry was stamped 17:37 and described the F4 fix as already made in its own signed commit and da2ce7 as re-requested after the push. The recorded events are: the F4 fix commit was created at 2026-10-28 17:38:29 UTC and the audit update commit at 17:39:18 UTC (commit times from `git log`), so the entry was written before its fix existed. The PR timeline shows `review_requested` events for da2ce7 only at 2026-10-28 16:57:01 UTC and for Copilot at 16:57:00 and 17:19:54 UTC. The statements that da2ce7 is re-requested after the push in the 17:22 and 17:37 entries were intended actions: the REST requests made after the pushes listed da2ce7 as pending but produced no new timeline event, so no re-request event is recorded. From here on an intended or attempted action is written as such, and an event is stamped after it completes.

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
