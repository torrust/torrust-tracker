# Agent Documentation Refactor Plan

## Goal

Refactor the repository's agent documentation so that:

- repository-wide policies remain easy to find and maintain,
- detailed operational workflows live in the right skills,
- custom agents carry only role-specific execution rules,
- new engineering rules are introduced without making `AGENTS.md` harder to use.

This plan is focused on documentation and agent-guidance changes only. It does not include
implementation of product features.

## Problems To Solve

### 1. `AGENTS.md` is too large and mixes levels of abstraction

The root `AGENTS.md` currently contains both:

- repository constitution-level rules, and
- detailed procedures and command-heavy operational guidance.

That makes it harder to maintain, harder to read, and more likely to drift from the specialized
skills that already exist.

### 2. Some desired engineering rules are not encoded clearly enough

The repository needs stronger, clearer guidance for:

- preferring the latest stable Rust crate versions when possible,
- preferring current supported base container images,
- preferring Rust over non-trivial shell logic,
- maximizing maintainable automated test coverage and documenting justified gaps,
- documenting public APIs and non-obvious invariants with Rust docs.

### 3. Role-specific behaviour and repository-wide policy are not fully separated

Some rules primarily affect the Implementer agent, but their intent is still repository-wide.
Those rules should be split between:

- short policy statements in `AGENTS.md`,
- operational rules in `.github/agents/implementer.agent.md`, and
- repeatable procedures in skills under `.github/skills/dev/`.

## Refactor Principles

Use this split consistently:

- `AGENTS.md`: repository-wide policy, quality bar, governance, and high-level conventions.
- Custom agents: role-specific execution behaviour and handoff rules.
- Skills: detailed workflows, command sequences, decision trees, and maintenance procedures.

Rule of thumb:

- If the guidance says "always" or "never" across the repository, keep it in `AGENTS.md`.
- If the guidance says "when doing X, follow these steps," move it to a skill.
- If the guidance says "this role must behave like Y," put it in the relevant custom agent.

## Planned Changes

### A. Refactor the root `AGENTS.md`

#### A1. Keep `AGENTS.md` as a policy-first document

Retain short, durable statements for:

- quality gates,
- security constraints,
- review and commit governance,
- testing philosophy,
- dependency freshness policy,
- container base image freshness policy,
- scripting-language threshold (`bash` for simple orchestration, Rust for non-trivial logic),
- documentation expectations,
- spec-first and review-first workflow expectations.

#### A2. Remove or compress command-heavy procedures

Reduce `AGENTS.md` detail for areas already handled better by skills, including:

- detailed setup sequences,
- detailed lint troubleshooting sequences,
- detailed issue and ADR authoring workflows,
- detailed PR review workflows,
- detailed dependency update procedures,
- detailed testing recipes.

Replace large procedural sections with short summaries and explicit links to the relevant skills.

#### A3. Add the new repository-wide policy rules

Add short policy statements for:

1. **Dependency freshness**
   Prefer the latest stable Rust crate version when adding or upgrading dependencies unless a
   compatibility reason requires otherwise. If not using the latest stable version, document why.

2. **Container base image freshness**
   Prefer current supported base images in `Containerfile` and compose-related artifacts. If an
   older image is retained, document the compatibility or operational reason.

3. **Bash vs Rust threshold**
   Use shell scripts only for simple orchestration. When logic becomes non-trivial, stateful,
   safety-critical, or worth testing independently, prefer Rust.

4. **Testing philosophy**
   Aim for high maintainable automated coverage. If behaviour is left untested, document the
   reason explicitly. Treat difficult testing as a design signal first, not just a testing
   inconvenience.

5. **Rust documentation expectations**
   Document public APIs and non-obvious internal invariants. Prefer high-signal Rust docs over
   boilerplate commentary.

### B. Tighten `.github/agents/implementer.agent.md`

Add or refine Implementer-specific operational rules so the agent applies the repository policies
consistently during implementation work.

#### B1. Dependency introduction rule

When adding a new dependency:

- check whether the standard library or an existing workspace dependency already solves the need,
- check the latest stable crate version first,
- justify any decision to use an older version,
- run `cargo machete` after the dependency is introduced.

#### B2. Container image rule

When touching `Containerfile`, compose files, or container setup artifacts:

- check whether the base image should be updated,
- avoid carrying forward outdated images without justification.

#### B3. Scripting rule

Add an explicit rule such as:

- do not grow shell scripts into application logic,
- migrate non-trivial logic to Rust when it needs types, tests, or safe reuse.

#### B4. Testing rule

Strengthen the existing TDD/test guidance so that the Implementer:

- adds unit tests to the maximum practical extent,
- prefers maintainable tests over brittle tests,
- documents justified test gaps,
- treats poor testability as a design problem to improve when possible.

#### B5. Rust docs rule

Require the Implementer to:

- add or update Rust doc comments for changed public APIs,
- document invariants, edge cases, and non-obvious constraints when the code is not self-evident.

### C. Update related custom agents where policy verification matters

#### C1. Reviewer agent

Update `.github/agents/reviewer.agent.md` so the Reviewer verifies:

- documented test gaps are justified,
- new public APIs or important behavior changes have adequate Rust docs,
- dependency/version choices are justified when not using the latest stable version.

#### C2. Committer agent

Keep the Committer focused on commit readiness, but consider a short reminder that repository
policy violations discovered at commit time should block the commit and be returned for repair.

### D. Add or expand skills under `.github/skills/dev/`

#### D1. New skill: `dev/maintenance/add-rust-dependency`

Create a new skill dedicated to introducing a Rust dependency.

Expected scope:

- confirm the dependency is truly needed,
- check the latest stable version on crates.io,
- review feature flags and prefer the smallest viable feature set,
- document why the crate was chosen,
- document why an older version is used if applicable,
- run `cargo machete`, linting, and relevant tests.

This should stay separate from bulk dependency upgrades handled by
`.github/skills/dev/maintenance/update-dependencies/SKILL.md`.

#### D2. Expand `write-unit-test`

Update `.github/skills/dev/testing/write-unit-test/SKILL.md` to include:

- the expectation of high maintainable coverage,
- acceptable reasons for leaving behaviour untested,
- guidance on documenting test gaps,
- the preference order of unit tests over heavier test layers when maintainable.

#### D3. Possibly expand `create-issue` or issue templates later

If test-gap documentation or dependency-justification notes repeatedly need issue-spec support,
consider extending the issue templates or planning skill with explicit fields for:

- testing exclusions and rationale,
- dependency/version choice notes.

This is optional and should be done only if it clearly improves review quality.

### E. Cross-link documentation semantically

Where relevant, add or update semantic links so that:

- policies link to the skills or agents that put them into practice,
- skills link back to the templates or artifacts they govern,
- future documentation drift is easier to detect.

This should follow the convention in
`docs/skills/semantic-skill-link-convention.md`.

## Concrete Edit List

### Files to update

- `AGENTS.md`
- `.github/agents/implementer.agent.md`
- `.github/agents/reviewer.agent.md`
- `.github/agents/committer.agent.md` (only if needed for policy enforcement wording)
- `.github/skills/dev/testing/write-unit-test/SKILL.md`
- `.github/skills/dev/maintenance/update-dependencies/SKILL.md` (only if cross-references are helpful)

### Files to add

- `.github/skills/dev/maintenance/add-rust-dependency/SKILL.md`

### Files to review for semantic-link alignment

- `docs/skills/semantic-skill-link-convention.md`
- any touched templates or policy docs that become part of the workflow graph

## Suggested Execution Order

1. Refactor `AGENTS.md` into a policy-first structure.
2. Update the Implementer agent with the new operational rules.
3. Update the Reviewer agent so the new rules are actually verified.
4. Create the new `add-rust-dependency` skill.
5. Expand the `write-unit-test` skill.
6. Add semantic links where needed.
7. Run pre-commit checks and commit the documentation changes.

## Review Questions

Please review these points before implementation:

1. Should the root `AGENTS.md` keep short examples for some policies, or should it become almost
   entirely policy-only with links out to skills?

   I think only policy-only and general summary of the project.

2. Do you want the Rust documentation rule to require docs only for public APIs, or also for
   important internal modules/types by default?

   Also for internal important modules by default.

3. Should the Reviewer explicitly block merges when public API docs are missing, or only flag it
   as a strong expectation?

   Block.

4. Do you want the new dependency skill to cover both Rust crates and container base image
   selection, or should those stay separate?

   Separate.

5. Do you want test-gap justification documented in code comments, issue specs, PR descriptions,
   or any of the above depending on scope?

   Any of the above depending on scope.

## Out of Scope for This Refactor

- Enforcing these rules via scripts or CI beyond the current lint/test gates.
- Automatic dependency freshness checking.
- Automatic crates.io or container registry integration.
- Broad restructuring of unrelated documentation.

## Expected Outcome

After this refactor:

- `AGENTS.md` is shorter, clearer, and more durable.
- The Implementer agent has stronger, more actionable engineering rules.
- Skills own the operational detail for repeated workflows.
- New repository rules are visible without duplicating long procedures everywhere.
- Documentation is easier for both humans and agents to navigate and maintain.
