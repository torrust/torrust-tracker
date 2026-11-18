---
pr-number: 2952
pr-url: https://github.com/torrust/torrust-tracker/pull/2952
last-updated-utc: "2026-11-18 19:17"
---

<!-- skill-link: process-pr-review -->

# PR #2952 Review Audit

Source: pull-request reviews and inline review threads for https://github.com/torrust/torrust-tracker/pull/2952.

<!-- Copied verbatim into each audit record after placeholder replacement. -->

## Ownership

The PR author owns this tracked audit record. Reviewers, including repository review agents,
deliver findings through GitHub and have no repository-artifact obligation.

- Post-merge workflow approval: N/A

<!-- Copied verbatim into each audit record. -->

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

Round 1 is Copilot's review `5963279729` (COMMENTED, three comments without IDs, recorded as F1 to F3) on the first head.

| Finding ID | Review finding reference | Author class | Severity | Category | Relationship | Disposition | Thread state |
| ---------- | ------------------------ | ------------ | -------- | -------- | ------------ | ----------- | ------------ |
| F1 | `review-finding:pr-2952-f1` | Copilot | Minor (inferred) | documentation | ORIGINAL | FIXED | RESOLVED |
| F2 | `review-finding:pr-2952-f2` | Copilot | Suggestion (inferred) | maintainability | ORIGINAL | NO_ACTION | SUPERSEDED |
| F3 | `review-finding:pr-2952-f3` | Copilot | Minor (inferred) | testing | ORIGINAL | FIXED | RESOLVED |

## Finding Details

### F1 - `--version` is a usage error

- PR number: 2952
- Source review ID: 5963279729
- Reviewer finding ID: N/A
- Source URL: https://github.com/torrust/torrust-tracker/pull/2952#discussion_r4619375545
- Concern: clap's `DisplayVersion` would be treated as a usage error; either support `--version` or make its absence intentional and stable.
- Solution: The command has no version flag: clap adds one only with `#[command(version)]`, so `--version` is an unknown argument, a usage error with exit 2. The doc comment of the arguments now says so; the command test of the usage-error record already runs `--version` and expects exit 2.
- Current-tree verification: `cargo test -q -p agent-reply-validator --test cli` passed, including `it_should_write_the_usage_error_keys_in_the_documented_order`, which runs `--version`.
- Resolution reference: `docs(agent-reply-validator): [#2904] say that the command has no --version`
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2952#discussion_r4619474947

### F2 - `--all` stops at the first runtime error

- PR number: 2952
- Source review ID: 5963279729
- Reviewer finding ID: N/A
- Source URL: https://github.com/torrust/torrust-tracker/pull/2952#discussion_r4619375594
- Concern: a non-UTF-8 candidate path or file content stops `--all`, which can hide problems of other reply files in the same run; the suggestion is to report each runtime error and go on.
- Solution: No change. A runtime error is a failure of the environment (git, the index, the encoding of a file), not a finding about a reply, and the run exits 1 with the record that names the path, as `frontmatter-validator` does. Fixing it and running again shows the rest.
- Current-tree verification: `check_index` in `src/bin/agent-reply-validator/main.rs` returns at the first runtime error; the command tests `it_should_fail_with_a_runtime_error_when_a_staged_candidate_path_is_not_utf8` and the non-UTF-8 content test expect one `runtime_error` record and exit 1.
- Resolution reference: https://github.com/torrust/torrust-tracker/pull/2952#discussion_r4619475451
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2952#discussion_r4619475451

### F3 - The pattern tests use the Rust `regex` engine, not ECMA-262

- PR number: 2952
- Source review ID: 5963279729
- Reviewer finding ID: N/A
- Source URL: https://github.com/torrust/torrust-tracker/pull/2952#discussion_r4619375673
- Concern: JSON Schema patterns are ECMA-262; the tests read them with Rust's `regex`, so their messages can read as proof of equivalence.
- Solution: A comment on the tests and their assertion messages say the patterns are read by the Rust `regex` engine, an approximation, and that the tests show agreement on their samples only; the module doc already names the known gap (U+0085, U+FEFF).
- Current-tree verification: `cargo test -q -p agent-reply-validator --lib` passed (154 tests).
- Resolution reference: `test(agent-reply-validator): [#2904] say the pattern tests use the Rust regex engine`
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2952#discussion_r4619475894

## Processing Log

- 2026-11-18 19:13 UTC - Started audit, before the replies of round 1.
- 2026-11-18 19:17 UTC - Round 1: Copilot's review `5963279729` (submitted 19:09:46Z, forge `submitted_at`) with three comments, recorded as F1 to F3. The fixes of F1 and F3 were committed at 19:11:51Z and 19:12:29Z (`git log --format=%cI`, in UTC) and pushed; `git ls-remote` showed the fork branch at the local head at 19:17:15Z (`date -u`). The three replies were posted between 19:17:27Z and 19:17:32Z (forge `created_at`), after the push. `validate-audit-record.py --pr-number 2952 --base torrust/develop`, with its `gh` calls made through `gh-bot`, exited 0 (3 rows, 0 failures) at 19:17:42Z; the three threads were then resolved by their thread IDs, and a GraphQL refresh showed 0 unresolved threads. No review of `da2ce7`, who was requested, arrived in the first 60 minutes.

<!-- Copied verbatim into each audit record. -->

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
