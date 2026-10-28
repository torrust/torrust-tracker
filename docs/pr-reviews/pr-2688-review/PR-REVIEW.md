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

One review of `copilot-pull-request-reviewer` has arrived so far: 5686786536 (COMMENTED, 2026-10-28 17:17 UTC, on the head whose subject is `docs(issues): [#2681] record the pre-PR review, the completion review and the retrospective`). It carries a "didn't start before the timeout" warning, so Copilot was requested again once; the re-requested review had not arrived when this record was written. It lists three open issues, each with one inline thread (F1 to F3). Copilot was requested through the `request_copilot_review` MCP tool. da2ce7 was requested through the REST `requested_reviewers` endpoint and has not reviewed. The maintainer asked on 2026-10-28 to address Copilot's review.

| Finding ID | Review finding reference | Author class | Severity | Category | Relationship | Disposition | Thread state |
| ---------- | ------------------------ | ------------ | -------- | -------- | ------------ | ----------- | ------------ |
| F1 | review-finding:pr-2688-f1 | Copilot | Suggestion | maintainability | ORIGINAL | NO_ACTION | SUPERSEDED |
| F2 | review-finding:pr-2688-f2 | Copilot | Suggestion | maintainability | ORIGINAL | NO_ACTION | SUPERSEDED |
| F3 | review-finding:pr-2688-f3 | Copilot | Suggestion | maintainability | ORIGINAL | NO_ACTION | SUPERSEDED |

## Finding Details

### F1 - Inner `#[cfg(test)]` and `pub mod tests` in `src/test_helpers.rs`

- PR number: 2688
- Source review ID: 5686786536
- Reviewer finding ID: N/A
- Source URL: <https://github.com/torrust/torrust-tracker/pull/2688#discussion_r4403778680>
- Concern: the module is already declared `#[cfg(test)] mod test_helpers;`, so the inner `#[cfg(test)]` is redundant, and `pub mod tests` reads like exported API. The reviewer suggests removing the inner attribute and using `mod tests`, or keeping `pub(crate)` with a targeted `#[allow(clippy::redundant_pub_crate)]` (or `pub(super)`).
- Solution: No change. `mod tests` (or `pub(super)`) cannot work: 28 references in 19 files of `tracker-core` import `crate::test_helpers::tests::...` or `crate::whitelist::test_helpers::tests::...`, so the inner module must be visible crate-wide. `pub(crate)` with an `allow` is the option the spec rejected in D5 (a lint suppression for no benefit; `pub` is clippy's own suggestion), and a comment above the module now explains the `pub`. The inner `#[cfg(test)]` stays because rule C of ADR 20261021190318 says the module is declared `#[cfg(test)] mod` and its content is `#[cfg(test)]`.
- Current-tree verification: `grep -rn "test_helpers::tests" packages/tracker-core/src` counted 28 references in 19 files; `cargo clippy -p torrust-tracker-core --all-targets -- -D warnings` passes with `pub mod tests` and fails with `pub(crate) mod tests` (evidence of the earlier prototype).
- Resolution reference: https://github.com/torrust/torrust-tracker/pull/2688#discussion_r4403811220
- Follow-up PR URL: N/A
- Reply URL: <https://github.com/torrust/torrust-tracker/pull/2688#discussion_r4403811220>

### F2 - Same pattern in `src/whitelist/test_helpers.rs`

- PR number: 2688
- Source review ID: 5686786536
- Reviewer finding ID: N/A
- Source URL: <https://github.com/torrust/torrust-tracker/pull/2688#discussion_r4403778758>
- Concern: the same two observations for the whitelist helper module: the inner `#[cfg(test)]` is redundant and `pub mod tests` can mislead; the reviewer prefers `mod tests` or `pub(super)`/`pub(crate)` with an `allow`.
- Solution: No change, for the reason of F1: `crate::whitelist::test_helpers::tests` is imported from `whitelist/mod.rs`, and the same 28 references need crate-wide visibility; `pub(crate)` needs a lint suppression (D5); rule C keeps the content `#[cfg(test)]`. The comment above the module explains the `pub`.
- Current-tree verification: `grep -rn "whitelist::test_helpers::tests" packages/tracker-core/src` finds the import in `whitelist/mod.rs`; the module is private and `#[cfg(test)]` in `whitelist/mod.rs`, so it is unreachable outside the crate.
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

## Processing Log

- 2026-10-28 17:22 UTC - Round 1: review 5686786536 arrived with a timeout warning and three inline threads, F1 to F3. Each claim was checked against the tree; none required a change, so each thread got the prescribed `Superseded by F<n>:` reply and was resolved. Copilot was re-requested once because of the warning.

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
