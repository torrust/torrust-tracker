---
semantic-links:
  related-artifacts:
    - .github/agents/complexity-auditor.agent.md
    - .github/agents/task-reviewer.agent.md
    - .github/agents/pr-reviewer.agent.md
    - docs/agents/orchestration.md
---

# Agent Review Reports - Issue #2266 - Implement the Rust Frontmatter Model and Initial Validator

> Append one completed independent-review entry at a time. Do not modify, reorder, or remove
> earlier entries. A correction is a new entry that names the earlier conclusion.

## Reports

### 2026-09-21 19:45 UTC - GitHub Copilot Task Reviewer

- Invocation scope: Independently reviewed commits `ba85ac19` and `0c5b628b`, the clean current
  working tree, and the delivered #2266 structural-model scope: crate bootstrap, extraction and
  universal envelope, strict issue/EPIC profiles, compatibility dispatch, and frozen v1 references.
- Inputs: `ISSUE.md`; predecessor v1 contract and fixture manifest; the two commit diffs; crate
  implementation and changed unit tests.
- Evidence: `cargo test --package frontmatter-validator` passed 20 unit tests; `cargo +nightly fmt
  --all -- --check` passed; `cargo clippy --package frontmatter-validator -- -D warnings` passed;
  language diagnostics reported no errors. The working tree was clean. No local #2280 or #2281
  issue specification was found, so the mandatory split handoff could be verified only from the
  #2266 progress log.
- Acceptance-criteria matrix:
  - AC1: PASS - Canonical Rust envelope and strict issue/EPIC types exist.
  - AC2: PASS - Accepted predecessor fixtures and all four rejected fixture categories have focused tests.
  - AC3: PENDING - Extraction and structural parsing are separated, but no repository-aware boundary
    implementation or independently testable adapter exists in this delivery.
  - AC4: PENDING - The intended reference union is implemented, but coverage omits invalid
    skill-name syntax and strict `spec-path` is not repository-relative validated.
  - AC5: PENDING - Deferred to #2280 by the recorded mandatory split.
  - AC6-AC8: PENDING - Deferred to #2281 by the recorded mandatory split.
  - AC9-AC10: PENDING - Crate bootstrap is replaceable, but invocation, integration, and documentation
    are deferred to #2281.
  - AC11-AC12: PENDING - Deferred to #2281.
- Findings:
  - FAIL - `src/profile.rs` accepts impossible calendar/time values such as `2026-99-99 99:99` because
    timestamp validation checks only digit positions and separators. Parse and validate a real UTC
    minute timestamp, with focused boundary tests.
  - FAIL - `src/profile.rs` checks only that strict `spec-path` is non-empty, although the approved
    contract requires a repository-relative path. Reject absolute and traversal paths, with tests.
  - FAIL - `src/lib.rs` validates only top-level `semantic-links`; the approved compatibility boundary
    requires Agent Skill and agent-profile documents to validate only `metadata.semantic-links` and
    ignore top-level `semantic-links` for v1 semantics. Add external-document dispatch and fixtures.
  - WARN - The mandatory split is stated in the issue log, but #2280 and #2281 are absent from this
    working tree; verify that their tracked specifications and acceptance criteria own the deferred work.
  - WARN - The issue records prose-first review summaries, and changed tests are readable, deterministic,
    and retain visible Acts/assertions, but it does not preserve per-test prose-first comparison evidence.
    Record that comparison for the changed test set and add coverage for invalid skill-name syntax.
  - WARN - Focused package gates pass, but `linter all`, broader required tests, manual verification,
    and implementation-completion evidence remain incomplete.
- Completion-review finding: FAIL - No `implementation-retrospective.md` or progress-log rationale
  establishes whether the implementation differed materially from plan; the original issue also remains
  incomplete because schema and command-surface work are deferred.
- Issue-spec updates: No acceptance checkbox was marked complete; no implementation workflow checkpoint
  was changed.
- Verdict: REVIEW FAILED
- Follow-up actions:
  - Repair timestamp and `spec-path` validation and add focused tests.
  - Implement and test the externally governed `metadata.semantic-links` compatibility boundary.
  - Confirm and document the #2280/#2281 issue-local handoffs before treating T6-T8 as completed.
  - Complete the deferred issues, quality gates, manual evidence, and completion review before closing #2266.

### 2026-09-21 19:55 UTC - GitHub Copilot - Correction

- Response to the three FAIL findings: the strict timestamp validator now rejects impossible dates
  and clock values; `spec-path` requires a non-empty repository-relative path without absolute,
  traversal, tagged, or whitespace forms; extraction reads only `metadata.semantic-links` whenever
  an external metadata mapping is present and ignores a simultaneous top-level extension.
- Regression coverage: added deterministic AAA tests for impossible timestamps, traversal paths,
  invalid skill names, nested external metadata links, and ignored external top-level links.
- Verification: `cargo test --package frontmatter-validator` passed 25 tests; nightly Rust
  `cargo +nightly fmt --all -- --check` and `cargo clippy --package frontmatter-validator -- -D
  warnings` passed.
- Deferred-scope clarification: #2280 and #2281 are native GitHub sub-issues created under #2264;
  they intentionally have no local issue-spec folders yet. Their GitHub bodies own the split schema
  and command/integration acceptance criteria respectively. This issue remains open until its
  in-scope structural work and the deferred follow-ups are completed and reviewed.

### 2026-09-21 20:15 UTC - GitHub Copilot - Follow-up Correction

- Response to the re-review finding: external dispatch no longer treats any `metadata` mapping as
  sufficient. It recognizes the observed Agent Skill (`name` plus mapping `metadata`) and agent
  profile (`name`, `description`, and `tools` or `argument-hint`) shapes. Those documents ignore
  top-level `semantic-links`, whether or not nested metadata links exist; ordinary unknown records
  still parse the universal top-level extension.
- Regression coverage: added a deterministic agent-profile-without-metadata case alongside the
  nested Agent Skill and conflicting-extension cases.
- Verification: `cargo test --package frontmatter-validator` passed 26 tests; nightly Rust
  `cargo +nightly fmt --all -- --check` and `cargo clippy --package frontmatter-validator -- -D
  warnings` passed.

### 2026-09-21 20:35 UTC - GitHub Copilot - Final Structural Correction

- Response to the final review findings: present semantic-link fields now first deserialize to a
  YAML value and must be sequences of strings, preventing `serde_yaml` from coercing `null` to an
  empty sequence. External documents with `name` and `description` are dispatched to the
  externally governed compatibility path whether or not `metadata` exists.
- Regression coverage: added isolated tests for `skill-links: null` and an Agent Skill without
  nested metadata whose invalid top-level extension must be ignored.
- Verification: `cargo test --package frontmatter-validator` passed 28 tests; nightly Rust
  `cargo +nightly fmt --all -- --check` and `cargo clippy --package frontmatter-validator -- -D
  warnings` passed.

### 2026-09-21 20:55 UTC - GitHub Copilot - Strict Scalar Correction

- Response to the final scalar findings: issue/EPIC candidates with a present but non-integer
  `schema-version` now receive `wrong-scalar-type`; the parsed frontmatter retains its YAML source
  so strict `last-updated-utc` validation can require a double-quoted scalar in addition to valid
  calendar and clock values.
- Regression coverage: added isolated quoted-schema-version and unquoted-timestamp cases.
- Verification: `cargo test --package frontmatter-validator` passed 30 tests; nightly Rust
  `cargo +nightly fmt --all -- --check` and `cargo clippy --package frontmatter-validator -- -D
  warnings` passed.

### 2026-09-21 21:10 UTC - GitHub Copilot - Timestamp Comment Correction

- Response to the final lexical finding: strict timestamp source validation now ignores an optional
  YAML comment after the scalar before checking the mandatory surrounding double quotes.
- Regression coverage: added an accepted v1 timestamp followed by `# updated`.
- Verification: `cargo test --package frontmatter-validator` passed 31 tests; nightly Rust
  `cargo +nightly fmt --all -- --check` and `cargo clippy --package frontmatter-validator -- -D
  warnings` passed.
