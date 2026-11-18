---
pr-number: 2952
pr-url: https://github.com/torrust/torrust-tracker/pull/2952
last-updated-utc: "2026-11-18 20:40"
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
Round 2 is Copilot's review `5964554730` (COMMENTED, two findings in its body with its own IDs F4 and F6, which collide
with no row) on the head that carried the round 1 audit.

| Finding ID | Review finding reference | Author class | Severity | Category | Relationship | Disposition | Thread state |
| ---------- | ------------------------ | ------------ | -------- | -------- | ------------ | ----------- | ------------ |
| F1 | `review-finding:pr-2952-f1` | Copilot | Minor (inferred) | documentation | ORIGINAL | FIXED | RESOLVED |
| F2 | `review-finding:pr-2952-f2` | Copilot | Suggestion (inferred) | maintainability | ORIGINAL | NO_ACTION | SUPERSEDED |
| F3 | `review-finding:pr-2952-f3` | Copilot | Minor (inferred) | testing | ORIGINAL | FIXED | RESOLVED |
| F4 | `review-finding:pr-2952-f4` | Copilot | Major | correctness | ORIGINAL | FIXED | NON_RESOLVABLE |
| F6 | `review-finding:pr-2952-f6` | Copilot | Minor | correctness | ORIGINAL | FIXED | NON_RESOLVABLE |

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

### F4 - Verdicts and severities were read from the object form too

- PR number: 2952
- Source review ID: 5964554730
- Reviewer finding ID: F4
- Source URL: https://github.com/torrust/torrust-tracker/pull/2952#pullrequestreview-5964554730
- Concern: Serde's derived enum reader accepts `"verdict": {"REVIEW_PASSED": null}` and `"severity": {"Major": null}`, which the schema does not allow, and the shape check did not catch them.
- Solution: Both fields are read from a JSON string and then as the enum, so the object form is rejected with "invalid type: map, expected a string". The schema is unchanged.
- Current-tree verification: the rejection cases `verdict_as_an_object` and `severity_as_an_object` pass; removing the string-only reading of either field fails its case; `agent-reply-schema` regenerated the artifact with no change.
- Resolution reference: `fix(agent-reply-validator): [#2904] read verdicts and severities from strings only`
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2952#issuecomment-6990977404

### F6 - `--all` accepted a reply file staged as a symbolic link

- PR number: 2952
- Source review ID: 5964554730
- Reviewer finding ID: F6
- Source URL: https://github.com/torrust/torrust-tracker/pull/2952#pullrequestreview-5964554730
- Concern: `--all` discarded the index modes, so a symbolic link whose stored target text is a valid reply passed, while path mode rejects links.
- Solution: `--all` reads each entry with `git ls-files --stage -z` and rejects a candidate with mode `120000` before reading its content, with the same message as path mode.
- Current-tree verification: the command test `it_should_fail_with_a_runtime_error_when_a_staged_candidate_is_a_symbolic_link` passes; removing the check fails it.
- Resolution reference: `fix(agent-reply-validator): [#2904] reject a reply file staged as a symbolic link`
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2952#issuecomment-6990977404

## Processing Log

- 2026-11-18 19:13 UTC - Started audit, before the replies of round 1.
- 2026-11-18 19:17 UTC - Round 1: Copilot's review `5963279729` (submitted 19:09:46Z, forge `submitted_at`) with three comments, recorded as F1 to F3. The fixes of F1 and F3 were committed at 19:11:51Z and 19:12:29Z (`git log --format=%cI`, in UTC) and pushed; `git ls-remote` showed the fork branch at the local head at 19:17:15Z (`date -u`). The three replies were posted between 19:17:27Z and 19:17:32Z (forge `created_at`), after the push. `validate-audit-record.py --pr-number 2952 --base torrust/develop`, with its `gh` calls made through `gh-bot`, exited 0 (3 rows, 0 failures) at 19:17:42Z; the three threads were then resolved by their thread IDs, and a GraphQL refresh showed 0 unresolved threads. No review of `da2ce7`, who was requested, arrived in the first 60 minutes.
- 2026-11-18 20:40 UTC - Round 2: Copilot's review `5964554730` (submitted 20:31:43Z, forge `submitted_at`) with two findings in its body, recorded as F4 and F6 under its IDs. They have no inline thread, so their rows are `NON_RESOLVABLE`. The fixes were committed at 20:34:01Z and 20:35:06Z (`git log --format=%cI`, in UTC) and pushed; `git ls-remote` showed the fork branch at the local head at 20:39:25Z (`date -u`). One conversation response, naming the review and both findings, was posted at 20:39:36Z (forge `created_at`), after the push, and edited at 20:39:44Z to correct its first line, which had named the round 1 review.

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
