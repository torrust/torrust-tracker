---
semantic-links:
  skill-links:
    - process-pr-review
  related-artifacts:
    - .github/skills/dev/pr-reviews/process-pr-review/SKILL.md
---

<!-- skill-link: process-pr-review -->

# PR #2651 Review Audit

Source: pull-request review and inline thread for <https://github.com/torrust/torrust-tracker/pull/2651>.

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

Review `5644927806` (<https://github.com/torrust/torrust-tracker/pull/2651#pullrequestreview-5644927806>)
was submitted at 2026-10-25T19:43:55Z with one independently actionable Major finding.
The reviewer-provided ID `UTF8-001` is retained without collision. This is the first
audit row, so there is no earlier finding to deduplicate against.

| Finding ID | Review finding reference | Author class | Severity | Category | Relationship | Disposition | Thread state |
| ---------- | ------------------------ | ------------ | -------- | -------- | ------------ | ----------- | ------------ |
| UTF8-001 | review-finding:pr-2651-utf8-001 | Copilot | Major | correctness | ORIGINAL | FIXED | RESOLVED |

## Finding Details

### UTF8-001 - Restrict exemptions to indexed attribute declarations

- PR number: 2651
- Source review ID: 5644927806
- Reviewer finding ID: N/A
- Source URL: https://github.com/torrust/torrust-tracker/pull/2651#discussion_r4372151594
- Concern: Cached attribute lookup still honors local, global, and system rules, allowing external configuration to change which indexed files are validated. Local and global `* binary` rules hid invalid staged text in reproduction. A local `*.png -binary` rule rejected a declared PNG. The reported global negative rule did not override a staged positive declaration, because indexed declarations take precedence over global attributes.
- Solution: Resolve attributes in a template-free temporary Git directory sharing the real index and object store, including alternates, while excluding repository configuration and local attributes. Disable global/system configuration and system attributes for the isolated process, set `core.attributesFile` to the null device, and preserve indexed declarations and built-in macros in a single batched query. Update the hook and workflow documentation.
- Current-tree verification: `bash contrib/dev-tools/checks/tests/test-check-utf8-files.sh` passed all 28 cases, including local/global overrides, injected system configuration, linked worktrees, a submodule checkout, alternate object storage, the system-attribute disable switch, one batched query, and exit 2 errors. `./contrib/dev-tools/checks/check-utf8-files.sh` passed with 2,240 checked and 7 skipped. Reproduction now rejects both undeclared Latin-1 bypasses and accepts the declared PNG despite the local negative rule. `git diff --check` over the branch range from `develop` exited 0.
- Resolution reference: `fix(dev-tools): isolate indexed binary attribute resolution`
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2651#discussion_r4372203397

## Processing Log

- 2026-10-25 19:43 UTC - Review `5644927806` submitted with finding `UTF8-001` at 19:43:55Z.
- 2026-10-25 19:52 UTC - Reproduced the finding, retained the original ID and Major severity, assigned category correctness and the immutable reference, implemented the fix, and completed the focused checks: all 28 test cases passed, the UTF-8 check passed with 2,240 checked and 7 skipped, and `git diff --check` passed.
- 2026-10-25 20:02 UTC - Posted the FIXED reply at 20:02:40Z: https://github.com/torrust/torrust-tracker/pull/2651#discussion_r4372203397.
- 2026-10-25 20:03 UTC - Resolved the thread. Its outdated flag does not change the FIXED disposition or RESOLVED state.

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
