---
doc-type: issue
issue-type: task
status: done
priority: p2
github-issue: 1750
spec-path: docs/issues/closed/1750-refactor-run-tracker-skill-semantic-coupling.md
branch: 1750-refactor-run-tracker-skill-semantic-coupling
related-pr: null
last-updated-utc: null
semantic-links:
  skill-links:
    - create-issue
  related-artifacts:
    - docs/issues/README.md
    - .github/skills/
---

# Refactor `run-tracker-locally` Skill with Semantic Artifact Coupling

## Goal

Refactor the skill at [`.github/skills/dev/environment-setup/run-tracker-locally/SKILL.md`](../../../.github/skills/dev/environment-setup/run-tracker-locally/SKILL.md) to align better with the Agent Skills specification and to reduce documentation drift by introducing explicit, maintainable links between the skill and the repository artifacts it depends on.

## Motivation

The current skill works, but it is vulnerable to becoming stale when referenced artifacts change.
A typical example is changing the default configuration path: the implementation may be updated in code while the skill remains unchanged.

This issue is motivated by three goals:

- Make skill maintenance proactive instead of memory-based.
- Add explicit semantic coupling between skill instructions and implementation artifacts.
- Establish a repeatable pattern so future skills do not repeat the same drift problem.

In short, this is not only a content update; it is a refactor of how we represent and maintain skill-to-artifact relationships.

This issue is intentionally **experimental**. It proposes a significant change in how the repository uses AI skills, and should be implemented behind a cautious review workflow.

## Problem

The skill currently references project artifacts (files, commands, defaults) in plain narrative Markdown.
Those references are human-readable but not operationally coupled.

As a consequence:

- moving or renaming a referenced artifact can silently invalidate the skill,
- changing semantic meaning in an artifact (not only file existence) can invalidate guidance,
- there is no built-in reminder at artifact-change time that a skill review is needed.

## Scope

In scope:

- Refactor [`.github/skills/dev/environment-setup/run-tracker-locally/SKILL.md`](../../../.github/skills/dev/environment-setup/run-tracker-locally/SKILL.md).
- Add explicit back-link reminders in artifacts that influence this skill.
- Define a lightweight semantic-link convention that works across Rust, TOML, and Markdown.
- Update the meta-skill [`.github/skills/add-new-skill/SKILL.md`](../../../.github/skills/add-new-skill/SKILL.md) so future skills adopt the same pattern.

Out of scope:

- Building a full ontology framework or a generic DSL for all project documentation.
- Migrating all existing skills in one shot.

## Experimental Rollout and Review Strategy

This issue should be implemented as an experimental branch and left as an open PR for maintainers to review before merge.

- Keep the PR open for cross-maintainer feedback (including maintainers like Cameron).
- Treat this work as a repository-level policy experiment, not a routine docs edit.
- Prefer incremental commits that make review easy: convention first, then skill refactor, then validation automation.
- Do not force immediate adoption across all skills; validate this approach with one skill first.

The implementation should make it easy to evaluate:

- maintenance cost,
- reviewer confidence,
- failure modes,
- and whether this should become a general project convention.

## Trust Model

The refactor should explicitly follow this trust model:

- The agent can propose and execute changes.
- Scripts and checks validate structural/semantic integrity.
- Maintainers decide policy acceptance.

Agent self-reporting is not sufficient for link integrity or semantic coupling correctness. Validation must be objective and reproducible.

## Proposed Changes

### Task 1: Refactor the target skill structure

- [ ] Restructure [`.github/skills/dev/environment-setup/run-tracker-locally/SKILL.md`](../../../.github/skills/dev/environment-setup/run-tracker-locally/SKILL.md) to better match Agent Skills best practices:
  - concise core workflow,
  - explicit defaults,
  - gotchas,
  - validation loop.
- [ ] Keep main instructions focused and move secondary details to `references/` when needed.
- [ ] Add clear default behavior (preferred commands and fallback guidance).

### Task 2: Add semantic back links in impacted artifacts

Add explicit reminder links in artifacts that this skill depends on, using a small structured marker convention (for example: `skill-link: run-tracker-locally`).

- [ ] Add back-link marker in [`src/bootstrap/config.rs`](../../../src/bootstrap/config.rs) near `DEFAULT_PATH_CONFIG`.
- [ ] Add back-link marker in [`share/default/config/tracker.development.sqlite3.toml`](../../../share/default/config/tracker.development.sqlite3.toml).
- [ ] Add back-link marker in [`src/lib.rs`](../../../src/lib.rs) where default config behavior is documented.
- [ ] Add back-link marker in [`README.md`](../../../README.md) where local run/config copy instructions are documented.

Notes:

- Use language-appropriate syntax (Rust comments, TOML comments, Markdown comments/text).
- The marker is a maintenance signal, not runtime logic.

### Task 3: Define minimal semantic-link convention

- [ ] Document a minimal convention for cross-artifact links, including:
  - marker name,
  - allowed values,
  - placement rules,
  - when to add/update/remove links.
- [ ] Publish this convention in a canonical repository document that can be referenced by skills and reviewers.
- [ ] Keep convention intentionally small and pragmatic.

### Task 3b: Add a marker catalog

- [ ] Add a repository catalog defining supported marker types (starting with `skill-link`).
- [ ] Keep the marker catalog intentionally small and grow it only when a concrete need appears.
- [ ] Document marker semantics and expected usage patterns for reviewers and contributors.

### Task 4: Update the skill-creation meta-skill

- [ ] Update [`.github/skills/add-new-skill/SKILL.md`](../../../.github/skills/add-new-skill/SKILL.md) so new skills include semantic coupling considerations from day one.
- [ ] Add guidance for:
  - declaring critical artifact dependencies,
  - adding backlinks in touched artifacts,
  - validating those links during skill maintenance.

### Task 5: Add lightweight validation (optional in first iteration)

- [ ] Add a basic validation script under the skill directory (`scripts/`) or shared dev tooling to detect broken file references/backlinks.
- [ ] Integrate as non-blocking initially (warning), then evaluate promoting to CI gate.

### Task 6: Add explicit experimental governance in the implementation PR

- [ ] Open a dedicated PR labeled as experimental and architecture-affecting for AI workflow conventions.
- [ ] Request review from maintainers who own development workflow and documentation conventions.
- [ ] Keep merge decision separate from implementation completion: a finished implementation may still remain unmerged pending consensus.
- [ ] Capture review feedback in the issue/PR and update the convention proposal accordingly.

## Acceptance Criteria

- [ ] [`.github/skills/dev/environment-setup/run-tracker-locally/SKILL.md`](../../../.github/skills/dev/environment-setup/run-tracker-locally/SKILL.md) is refactored with a concise, maintainable structure.
- [ ] The key dependent artifacts include explicit back-link reminders to `run-tracker-locally`.
- [ ] A documented minimal semantic-link convention exists and is understandable by contributors.
- [ ] A canonical document exists for the `skill-link` convention and is referenced from skill-authoring guidance.
- [ ] A marker catalog exists, starts minimal, and documents how new markers can be added organically.
- [ ] [`.github/skills/add-new-skill/SKILL.md`](../../../.github/skills/add-new-skill/SKILL.md) includes the new guidance for semantic coupling.
- [ ] The approach remains lightweight and does not introduce an over-engineered ontology system.
- [ ] The implementation is submitted as an explicit experimental PR and reviewed by maintainers before any merge decision.

## Risks and Trade-offs

- Too little structure keeps drift risk high.
- Too much structure creates maintenance overhead and poor adoption.
- The proposed design intentionally targets the middle ground: explicit links + lightweight conventions + incremental validation.

## References

- Agent Skills overview: <https://agentskills.io/home>
- Agent Skills specification: <https://agentskills.io/specification>
- Best practices: <https://agentskills.io/skill-creation/best-practices>
- Optimizing descriptions: <https://agentskills.io/skill-creation/optimizing-descriptions>
- Evaluating skills: <https://agentskills.io/skill-creation/evaluating-skills>
- Using scripts: <https://agentskills.io/skill-creation/using-scripts>
- Target skill: [`.github/skills/dev/environment-setup/run-tracker-locally/SKILL.md`](../../../.github/skills/dev/environment-setup/run-tracker-locally/SKILL.md)
- Meta-skill: [`.github/skills/add-new-skill/SKILL.md`](../../../.github/skills/add-new-skill/SKILL.md)
