---
semantic-links:
  skill-links:
    - write-markdown-docs
  related-artifacts:
    - docs/templates/ISSUE.md
    - .github/skills/dev/planning/create-issue/SKILL.md
    - docs/issues/open/2691-reduce-agents-md-token-footprint/ISSUE.md
    - docs/issues/open/2691-reduce-agents-md-token-footprint/research.md
    - docs/issues/open/2691-reduce-agents-md-token-footprint/section-disposition.md
    - docs/issues/open/2691-reduce-agents-md-token-footprint/rule-inventory.md
    - docs/issues/open/2691-reduce-agents-md-token-footprint/manual-verification-evidence.md
---

# Implementation Retrospective — Reduce the AGENTS.md Token Footprint

> Create this concrete artifact only as lowercase
> `implementation-retrospective.md` inside a folder-style issue specification.

## Purpose

Record evidence-based process improvements discovered while implementing issue #2691. This is a
blameless review of the implementation approach; it does not replace acceptance-criteria
verification.

## Outcome

- The root `AGENTS.md` went from 12.4k to 4.8k Claude Code tokens in `/context` (-61%), from 523 to
  229 lines and from 35,910 to 13,027 bytes, below the agreed target of about 5k.
- All 99 rules and facts of the baseline file are accounted for in
  [`rule-inventory.md`](rule-inventory.md): each has a final location, or a recorded reason for its
  removal.
- Two fresh-session checks passed (V2 and V3 in
  [`manual-verification-evidence.md`](manual-verification-evidence.md)): an agent stated the commit,
  signing and branch rules correctly, and found the moved package and port content through the
  pointers.
- The package catalog now lives in `docs/packages.md`; the `open-pull-request` skill gained the
  history and PR-title rules; the `setup-dev-environment` and `run-linters` skills gained small
  additions.

## What Went Well

1. **Spec first, review before moving anything.** Research (T1 to T3), a disposition per section
   (T4) and a maintainer review (T5) came before any edit to `AGENTS.md`. The maintainer's decisions
   (D1 to D7) were recorded in the spec, so every later trim had a documented basis.
2. **Destination before removal.** For each move the destination was checked first and the content
   added there in its own commit when it was missing. The order of commits therefore never left a
   rule without a home.
3. **A real measurement, not only a proxy.** The proxy tokenizer was used for quick ratios, and
   Claude Code's own `/context` for the result. The ratio between them (1.66 at baseline, 1.60 at
   the end) let the T7 estimate land within 3% of the measured value.
4. **The maintainer ran the behavioral checks one step at a time.** The agent that wrote the change
   could not fairly test whether another agent still finds the rules; the one-step-at-a-time protocol
   made the runs reproducible and the evidence exact.

## What Changed During Implementation

1. **The completeness checklist was seeded from the wrong source.** The plan's first prerequisite
   table listed the destinations of the planned moves, which could only confirm those moves. An
   outside comment on the issue pointed this out. The plan gained task T4a and acceptance criterion
   AC7: a rule inventory with 99 rows built from the baseline file before restructuring, with a
   distinctive phrase per row.
2. **A destination pointed back at the source.** The `open-pull-request` skill referred to "the
   history-rewrite rules in the repository instructions", that is, to `AGENTS.md`, and held only a
   stub of the PR-title rules. Removing the text from `AGENTS.md` first would have left those rules
   nowhere. They were added to the skill first and the reference fixed. The package catalog had the
   same shape: `docs/packages.md` and `packages/AGENTS.md` named `AGENTS.md` as the canonical source,
   so three links had to be reversed.
3. **The baseline contained stale statements.** Moving content exposed four statements that were
   already wrong: `cargo test --test integration` (no such test target), `.cspell.gitignore` (no such
   file; cspell uses `ignorePaths`), four cargo aliases listed where the file defines five, and the
   contents of `tests/` (`integration.rs` and `servers/` do not exist). They were dropped, not
   carried forward.
4. **The linter changed the evidence files silently.** The pre-commit run re-aligns Markdown tables,
   so an inventory update that matched the original row format found nothing and appeared to
   succeed. It was caught only because the count of asserted rows did not increase.
5. **A measurement used the wrong file.** The first `/context` run after the trims reported 12.4k,
   the baseline. The session had loaded the old `AGENTS.md`. The reading was discarded and the checkout
   state (`git rev-parse`, `wc -l`) was added to the procedure before the final measurement.
6. **A wording-sensitive trim was not explicitly approved at the review gate.** Decision D5 said to
   keep the eight Engineering Policies inline; shortening their text was proposed in T4 but not
   called out. The maintainer asked whether the trims were approved; the question was answered with
   decision D6 and the work continued.
7. **Citations from memory were wrong.** Several inventory row numbers quoted in the V2 evidence were
   wrong. They were caught by checking them against the inventory before committing.
8. **Field checks changed the risk picture.** Repository skills under `.github/skills/` are not
   auto-loaded by Claude Code, so moving text into a skill costs no description tokens there; and a
   nested `AGENTS.md` loaded on demand in this repository, which has no `CLAUDE.md`. See
   [`research.md`](research.md#field-checks-against-the-docs).

## Root Cause

The issue template and the `create-issue` skill describe how to verify behavior but have nothing
specific to work that relocates content between documents. Three assumptions followed from that gap:
that a documented destination already holds the content it will receive, that nothing in the
destination refers back to the source, and that a checklist of the planned moves is a checklist of
the rules. None was checked before the plan was approved; the issue's own review gates caught them
during implementation.

## Improvements for Future Work

These are proposals. None is implemented by this issue; the maintainer decides which, if any, become
a follow-up.

1. **Add a relocation checklist to the `create-issue` guidance** for issues that move or delete
   documentation content: build an inventory from the source file before editing, give each row a
   locator phrase and a final location, and end with an assertion step.
2. **Check reverse references before removal.** Before deleting a section, search for links that
   point at it and for destinations that refer back to the source.
3. **Record the checkout state with every measurement taken from a tool's own report**, such as the
   commit and a line count of the measured file.
4. **Generate cross-references from the data**, for example inventory row ids, instead of quoting them
   by hand.
5. **Call out wording-sensitive changes at the review gate**, such as policies whose exact text issues
   cite by number, so the approval covers them explicitly.

## Avoiding Overcorrection

The evidence does not justify:

- an automated size check or a documented budget for `AGENTS.md` (decision D7: not now, and may
  change);
- a CI gate on `AGENTS.md` length;
- a `CLAUDE.md` adapter or Claude-specific path-scoped rules (`AGENTS.md` is read directly);
- nested `AGENTS.md` files for every package, which would add many files for an unmeasured gain;
- a reusable tool for the rule inventory: the one-off scripts were enough and their method is
  recorded in the evidence;
- an ADR for this work, which is re-evaluated in task T10.

## Evidence

- Issue specification: [`ISSUE.md`](ISSUE.md), with the progress log
- Research and baseline: [`research.md`](research.md)
- Dispositions and maintainer decisions: [`section-disposition.md`](section-disposition.md)
- Completeness checklist: [`rule-inventory.md`](rule-inventory.md)
- Verification: [`manual-verification-evidence.md`](manual-verification-evidence.md)
- Pull requests: #2692 (specification), #2704 (implementation)
- Branch commits, by subject: `docs(packages): [#2691] make docs/packages.md the canonical package
  catalog`, `docs(skills): [#2691] add history and PR-title rules to the open-pull-request skill`,
  `docs(agents): [#2691] move Git Workflow detail to the open-pull-request skill`,
  `docs(agents): [#2691] condense the remaining AGENTS.md sections`
