---
pr-number: 2748
pr-url: https://github.com/torrust/torrust-tracker/pull/2748
last-updated-utc: "2026-11-06 14:14"
---

# PR #2748 Review Audit

Source: pull-request reviews and inline review threads for https://github.com/torrust/torrust-tracker/pull/2748.

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

One row per independent concern from Copilot's reviews of this pull request (round 1: reviews 5799093230 and 5799313195; round 2: review 5801235274).

| Finding ID | Review finding reference | Author class | Severity | Category | Relationship | Disposition | Thread state |
| ---------- | ------------------------ | ------------ | -------- | -------- | ------------ | ----------- | ------------ |
| F1 | review-finding:pr-2748-f1 | Copilot | Major (inferred) | documentation | ORIGINAL | FIXED | RESOLVED |
| F2 | review-finding:pr-2748-f2 | Copilot | Minor (inferred) | documentation | ORIGINAL | FIXED | RESOLVED |
| F3 | review-finding:pr-2748-f3 | Copilot | Minor (inferred) | correctness | ORIGINAL | FIXED | RESOLVED |
| F4 | review-finding:pr-2748-f4 | Copilot | Minor (inferred) | correctness | ORIGINAL | FIXED | RESOLVED |
| F5 | review-finding:pr-2748-f5 | Copilot | Minor | documentation | ORIGINAL | FIXED | RESOLVED |
| F6 | review-finding:pr-2748-f6 | Copilot | Minor | documentation | ORIGINAL | FIXED | RESOLVED |

## Finding Details

### F1 - ADR claims a tracked follow-up issue without a reference

- PR number: 2748
- Source review ID: 5799093230
- Reviewer finding ID: N/A
- Source URL: https://github.com/torrust/torrust-tracker/pull/2748#discussion_r4491639334
- Concern: The ADR says the non-compliance is tracked in a follow-up issue, but no issue reference exists and the spec says the follow-up is pending.
- Solution: Valid. The ADR now says the changes are listed as task T4 of #2715 until their follow-up issues exist.
- Current-tree verification: `grep -n 'T4' docs/adrs/20261106102632_*.md` shows the new wording; no 'tracked in a follow-up issue' remains.
- Resolution reference: docs(adrs): [#2715] point the follow-up work to task T4 of the issue
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2748#discussion_r4493021635

### F2 - New progress-log entries omit the time of day

- PR number: 2748
- Source review ID: 5799093230
- Reviewer finding ID: N/A
- Source URL: https://github.com/torrust/torrust-tracker/pull/2748#discussion_r4491639400
- Concern: The two new entries in the spec's Progress Log carry only a date, while earlier entries carry date and time.
- Solution: Valid. The entries now read 10:25 UTC and 10:27 UTC, from the commit times of the work they describe.
- Current-tree verification: Read the Progress Log of the #2715 spec: every entry carries a time of day.
- Resolution reference: docs(issues): [#2715] add the time of day to the progress log entries
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2748#discussion_r4493021999

### F3 - no-stdout-result paragraph contradicts exit 2 for usage errors

- PR number: 2748
- Source review ID: 5799313195
- Reviewer finding ID: N/A
- Source URL: https://github.com/torrust/torrust-tracker/pull/2748#discussion_r4491811016
- Concern: The ADR said a no-stdout-result command's only non-success exit is 1, which contradicts the table that assigns usage errors to 2.
- Solution: Valid. The paragraph now says every failure that is not a usage error exits 1 and usage errors keep exit 2.
- Current-tree verification: Read section 2 of the ADR: the paragraph and the table agree.
- Resolution reference: docs(adrs): [#2715] keep exit 2 for usage errors in no-stdout-result commands
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2748#discussion_r4493022347

### F4 - package-coverage-check is in scope but not classified

- PR number: 2748
- Source review ID: 5799313195
- Reviewer finding ID: N/A
- Source URL: https://github.com/torrust/torrust-tracker/pull/2748#discussion_r4491811065
- Concern: The tool is consumed by `generate_coverage_pr.yaml`, so it falls under the new scope, but it is absent from the table and the follow-ups; its errors are plain text and a usage error exits 1.
- Solution: Valid. It is classified `stdout-result-data` in the binary table of ADR 20260519000000, recorded as not consistent in the new ADR, and covered by the T4 follow-up work. No binary changes.
- Current-tree verification: Read `contrib/dev-tools/checks/package-coverage-check/src/main.rs` (plain-text stderr, `ExitCode::FAILURE` for every error) and `.github/workflows/generate_coverage_pr.yaml` lines 49-54; the table row and the ADR bullet exist.
- Resolution reference: docs(adrs): [#2715] classify package-coverage-check in the CLI output contract
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2748#discussion_r4493022682

### F5 - Non-canonical severity labels in the audit

- PR number: 2748
- Source review ID: 5801235274
- Reviewer finding ID: F5
- Source URL: https://github.com/torrust/torrust-tracker/pull/2748#discussion_r4493281564
- Concern: The audit used `Medium` and `Low`, which are outside the severity vocabulary; the source comments state no severity, so the canonical level must be marked inferred.
- Solution: Valid. F1 is `Major (inferred)` and F2 to F4 are `Minor (inferred)`.
- Current-tree verification: `grep -n "^| F" docs/pr-reviews/pr-2748-review/PR-REVIEW.md` shows only canonical labels.
- Resolution reference: docs(pr-reviews): [#2715] use canonical severity labels in the PR 2748 audit
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2748#discussion_r4493304493

### F6 - Summary JSON requirement left open

- PR number: 2748
- Source review ID: 5801235274
- Reviewer finding ID: F6
- Source URL: https://github.com/torrust/torrust-tracker/pull/2748#discussion_r4493281446
- Concern: The ADR left open whether `package-coverage-check summary` must emit JSON, which contradicts its `stdout-result-data` class.
- Solution: Valid. The ADR now requires one JSON result, with the Markdown report carried inside it and extracted by the workflow, and records no exception.
- Current-tree verification: Read the `package-coverage-check` bullet in section 3 of the ADR.
- Resolution reference: docs(adrs): [#2715] require the package-coverage-check summary to be a JSON result
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2748#discussion_r4493304043

## Processing Log

- 2026-11-06 13:43 UTC - Started audit; round 1 (Copilot reviews 5799093230 and 5799313195) processed: F1 to F4 fixed in the four commits cited above, pushed, and replied. The review loop should have started right after the PR was opened; it started when the maintainer asked.
- 2026-11-06 14:14 UTC - Round 2 (Copilot review 5801235274, poller result at 14:09 UTC) processed: F5 and F6 fixed in the two commits cited above, pushed, replied and resolved; zero unresolved threads after refreshing GraphQL.

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
