---
name: implement-task
description: "Implement one task of an approved issue spec with TDD, manual evidence and signed commits. Use for the overall sequence on a local branch; a bug's analysis follows fix-bug."
metadata:
  author: torrust
  version: "1.0"
  semantic-links:
    related-artifacts:
      - .claude/agents/implementer.md
      - .claude/skills/commit-changes/SKILL.md
      - .claude/skills/write-unit-test/SKILL.md
      - .claude/skills/fix-bug/SKILL.md
      - .claude/skills/add-rust-dependency/SKILL.md
      - .claude/skills/handle-errors-in-code/SKILL.md
      - .claude/skills/review-task/SKILL.md
---

# Implementing A Task

Use this skill when an issue specification is approved and one of its tasks, or a bug, must be
implemented on a local branch. It holds the sequence of an implementation. The practices inside each
step belong to other skills, which are linked and not copied. The session that implements follows it
itself, or through the [`implementer`](../../agents/implementer.md) agent, which preloads it.

Follow `AGENTS.md` for the repository's rules: the Beck rules of simple design, the quality gate
(`./contrib/dev-tools/git/hooks/pre-commit.sh`), the code style and the engineering policies.

## Workflow

1. **Analyse.** Read `AGENTS.md`, the skills of the area you change, and the task in full. State what
   must change and what must not. Decide whether the work is substantively a bug (broken, incorrect,
   stale, misleading, unexpectedly failing or regressed behavior), whatever its labels say; if it is,
   follow [`fix-bug`](../fix-bug/SKILL.md) in order: reproduce the defect against the real artifact,
   then choose the regression-test boundary before the red test. When the specification is ambiguous,
   incomplete or does not match the code, or a decision matters and you cannot take it, ask, and do
   not guess.
2. **Decompose.** Break the task into the smallest independent, verifiable steps. Each has one
   intent, can be checked by a test or an observable behavior, and fits one commit. Follow the Commit
   Points of the specification.
3. **Implement each step with TDD.** Write a failing test (red), the minimal code that passes it
   (green), then refactor to remove duplication while the tests stay green. The
   [`write-unit-test`](../write-unit-test/SKILL.md) skill holds the naming and the Arrange/Act/Assert
   pattern. When TDD is not practical (command-line wiring, configuration plumbing), implement
   defensively and add the tests as the next step. Run `cargo test -p <package>` before moving on.
4. **Apply the policies of the change.** For a new dependency, follow
   [`add-rust-dependency`](../add-rust-dependency/SKILL.md) and run `cargo machete` after it. For
   error handling, follow [`handle-errors-in-code`](../handle-errors-in-code/SKILL.md). For container
   artifacts, check the base images. Update the Rust documentation of a changed public API and of the
   invariants the code does not show. Keep shell for orchestration and move non-trivial logic to Rust.
5. **Verify against the finished artifact.** Run the manual scenarios of the specification and record
   the actual commands, output, logs and conclusions in the issue-local
   `manual-verification-evidence.md`; for a bug, include the first reproduction and the final
   like-for-like recheck. The output of automated tests is not manual evidence. Do not leave meaningful
   behavior untested without writing the reason in the code, the specification or the pull-request
   notes. See the [verification types](../../../docs/testing.md#verification-types).
6. **Record the completion review.** Compare the result with the specification. Create the
   issue-local `implementation-retrospective.md` for a reusable lesson, a material design change or a
   meaningful deviation from the plan; otherwise add a short progress-log entry saying why none was
   needed (see the "Implementation Completion Review" of the issue template). A lesson that applies
   beyond the issue goes to the skill, agent, template or document it concerns, or to a follow-up.
7. **Commit.** Follow [`commit-changes`](../commit-changes/SKILL.md): GPG-signed Conventional Commits,
   one per step, after the quality gate passes. Never skip the hook and never bypass signing.

## Boundaries

- Do not do more than the task asks: scope creep is a defect.
- Do not suppress a compiler warning or a Clippy lint without a documented reason.
- Do not commit code that fails the quality gate.
- Do not verify your own acceptance criteria and do not tick them in the specification: the task
  reviewer does (see [`review-task`](../review-task/SKILL.md)).
