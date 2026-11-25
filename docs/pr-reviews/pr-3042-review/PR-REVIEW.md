---
semantic-links:
  skill-links:
    - process-pr-review
  related-artifacts:
    - .claude/skills/process-pr-review/SKILL.md
    - "issue #2999"
---

<!-- skill-link: process-pr-review -->

<!-- cspell:ignore nnounce oken -->

# PR #3042 Review Audit

Source: pull-request reviews and inline review threads for
<https://github.com/torrust/torrust-tracker/pull/3042>.

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

Review 6044736873 (Greptile, 2026-11-24 18:06 UTC) posted two inline comments, F1 and F2, and a summary comment (<https://github.com/torrust/torrust-tracker/pull/3042#issuecomment-7140177515>) that lists the same two findings and adds none. The CodeRabbit comment on the pull request only says its free review limit was reached; it holds no finding.

| Finding ID | Review finding reference | Author class | Severity | Category | Relationship | Disposition | Thread state |
| ---------- | ------------------------ | ------------ | -------- | -------- | ------------ | ----------- | ------------ |
| F1 | review-finding:pr-3042-f1 | Unknown | Blocker | security | ORIGINAL | FIXED | RESOLVED |
| F2 | review-finding:pr-3042-f2 | Unknown | Nit | maintainability | ORIGINAL | FIXED | RESOLVED |

## Finding Details

### F1 - Encoded tokens reach logs

- PR number: 3042
- Source review ID: 6044736873
- Reviewer finding ID: N/A
- Source URL: <https://github.com/torrust/torrust-tracker/pull/3042#discussion_r4684413746>
- Concern: `redact_query` compared query parameter names before decoding them, but the server decodes them, so `/api/v1/stats?%74oken=<admin-token>` authenticates and was logged unredacted in the request event and the span.
- Solution: names are percent-decoded before they are compared, for query parameters and for the `announce`, `scrape` and `key` path segments; the logged text keeps the original spelling. Four cases were added to the redaction table (`%74oken`, `%74%6F%6B%65%6E`, `to%6ben`, `/%61nnounce/<key>`).
- Current-tree verification: `cargo test -p torrust-tracker-axum-server --lib request_log` failed on the four new cases before the change (21 passed, 4 failed) and passes after it (25 passed); on a running private tracker `GET /api/v1/stats?%74oken=TestAdminToken0001` returned the statistics and the log held `uri=/api/v1/stats?%74oken=***` and 0 lines with the token (section R1 of `docs/issues/open/2999-2993-si-a-keep-secrets-out-of-errors-and-logs/manual-verification-evidence.md`).
- Resolution reference: fix(axum-server): [#2999] decode query and path names before redacting the request URI
- Follow-up PR URL: N/A
- Reply URL: <https://github.com/torrust/torrust-tracker/pull/3042#discussion_r4684525263>

### F2 - Imports break repository style

- PR number: 3042
- Source review ID: 6044736873
- Reviewer finding ID: N/A
- Source URL: <https://github.com/torrust/torrust-tracker/pull/3042#discussion_r4684413764>
- Concern: the code this branch added imported `Arc` inside a function and used full paths such as `std::fmt::Debug`, against the repository style (imports at the top, short names).
- Solution: the imports moved to the file headers with short names in `handler.rs`, `peer_key.rs`, `add_key_form.rs`, `databases/error.rs` and `test-helpers/src/logging.rs`. Behaviour is unchanged.
- Current-tree verification: `grep -n "std::fmt::\|std::borrow::Cow"` on those five files prints no full path in the added code (only the header imports of `databases/error.rs`); `cargo build --workspace --tests`, `cargo test --workspace --no-fail-fast` and the pre-commit gate pass.
- Resolution reference: fix(axum-server): [#2999] decode query and path names before redacting the request URI
- Follow-up PR URL: N/A
- Reply URL: <https://github.com/torrust/torrust-tracker/pull/3042#discussion_r4684525681>

## Processing Log

- 2026-11-24 18:06 UTC - Greptile review 6044736873 arrived with two inline findings.
- 2026-11-24 18:16 UTC - Replied on both threads after the fix commit was pushed (replies 4684525263 and 4684525681).
- 2026-11-24 18:20 UTC - `validate-audit-record.py --pr-number 3042` run with the bot token set for that one command (identity `josecelano-bot`) printed `{"status": "ok", "rows": 2, "log_entries": 2, "failures": 0}`. A refreshed GraphQL listing showed the F1 thread already resolved by `greptile-apps[bot]`; the F2 thread was then resolved by `josecelano-bot`. No unresolved thread remains.

## Completion Rules

- Re-derive the reply claim against the current tree before replying or resolving a thread.
- Reply on every resolvable thread before resolving it.
- For an outdated thread whose concern was fixed, record `Disposition=FIXED` and
  `Thread state=RESOLVED`, even if GitHub marks the thread outdated after the push.
- Cite a fix by its unique Conventional Commit subject or durable reply URL, never by a branch SHA
  that can change after a rebase.
- Refresh review threads using GraphQL and confirm that no unresolved actionable thread remains.
