---
semantic-links:
  related-artifacts:
    - "issue #2003"
    - "issue #2417"
    - docs/discussions/2003-overhaul-guardrails-and-automation/20261003-goals-and-boundaries/README.md
    - docs/discussions/2003-overhaul-guardrails-and-automation/20261003-semantic-linking-knowledge-graph/initial-proposal.md
    - docs/issues/open/2417-2411-verify-http-scrape-info-hash-limit/ISSUE.md
    - docs/issues/closed/README.md
    - docs/templates/ISSUE.md
    - docs/adrs/20260821172000_establish_ai_agent_context_capability_and_portability_governance.md
---

<!-- cspell:ignore Addy Adzic Birgitta Böckeler Codely Codeplain Gojko Horthy HumanLayer Kiro OpenSpec Osmani RPI Tessl -->

# Specifications and Rationale: What Outlives an Issue Specification

| Field        | Value                                                                                                            |
| ------------ | ---------------------------------------------------------------------------------------------------------------- |
| Status       | Draft for review in the pull request that adds it                                                                |
| Started      | 2026-10-03                                                                                                       |
| Participants | Jose Celano (sources, the position on tests, the missing homes for the why); GitHub Copilot (analysis and draft) |
| Reviewer     | Cameron (`da2ce7`), assignee of EPIC #2003                                                                       |
| Informs      | EPIC #2003 - Overhaul: Automation Tools and AI Agent Guardrails                                                  |
| Scope        | Specification lifecycle in aspect 2 of [Goals and Boundaries](../20261003-goals-and-boundaries/README.md)        |

## Why This Discussion

[Goals and Boundaries](../20261003-goals-and-boundaries/README.md#aspect-2-deterministic-workflows)
found that issue specifications are provisional: closed specifications are a temporary buffer
before permanent removal ([`docs/issues/closed/README.md`](../../../issues/closed/README.md)), so
knowledge that must outlive one has to move somewhere durable first. This discussion asks what that
knowledge is and where it should go. It answers in two parts: tests specify what the program does,
and a set of durable records explains why.

Jose Celano brought the sources: a Codely newsletter on spec-driven development, the article it
draws on, a guide to writing specifications for AI agents, and the position on tests recorded
below.

Nothing here changes a specification. A conclusion takes effect only when the owner of EPIC #2003
records it.

## Kinds of Specification

A Codely newsletter in Spanish, shared by Jose Celano on 2026-10-03, describes spec-driven
development (SDD) as an intermediate step between the prompt and the code: a specification that
drives the implementation. Using an agent's plan mode is already SDD. Translated and summarized, it
names three approaches by the weight the specification keeps:

- **Spec first:** write the specification, implement it, and move on. This is the Research, Plan,
  Implement (RPI) flow, the one that tools such as Kiro follow.
- **Spec anchored:** the specification and the code stay synchronized in both directions; when
  one changes, the other is updated (OpenSpec, Predictable Code).
- **Spec as source:** the specification is the only source of truth, and the code is regenerated
  from it. The code is volatile and is not even committed (Codeplain).

The newsletter calls RPI its current sweet spot, while expecting better models and harnesses to
end it.

The classification comes from Birgitta Böckeler's article
[Understanding Spec-Driven-Development: Kiro, spec-kit, and Tessl](https://martinfowler.com/articles/exploring-gen-ai/sdd-3-tools.html),
published on martinfowler.com on 2025-10-15 in the "Exploring Gen AI" series. It names three
implementation levels, each building on the previous one:

1. **Spec-first:** "A well thought-out spec is written first, and then used in the AI-assisted
   development workflow for the task at hand."
2. **Spec-anchored:** "The spec is kept even after the task is complete, to continue using it for
   evolution and maintenance of the respective feature."
3. **Spec-as-source:** "The spec is the main source file over time, and only the spec is edited by
   the human, the human never touches the code."

The newsletter adds two things the article does not say: its spec-anchored approach requires
two-way synchronization, where the article only keeps and edits the specification, and its
spec-as-source approach leaves the generated code uncommitted. The article also observes that
every SDD approach it found is spec-first. It compares spec-as-source with model-driven
development, warning that it may combine that approach's inflexibility with the non-determinism of
language models. And it separates specifications, which serve one change, from the "memory bank"
of context files such as `AGENTS.md` that every session uses. The RPI flow is described in Dex
Horthy's
[Getting AI to Work in Complex Codebases](https://github.com/humanlayer/advanced-context-engineering-for-coding-agents/blob/main/ace-fca.md)
(HumanLayer, August 2025).

The repository's artifacts map onto these kinds as follows:

| Artifact | Kind | Reason |
| -------- | ---- | ------ |
| Issue and EPIC specifications in `docs/issues/` | Spec-first | Written before the work, kept in `closed/` as a temporary buffer, and eventually deleted; a later change gets a new specification. This is the lifecycle the article draws for spec-first. |
| Behavioral and acceptance tests | Spec-anchored, and executable | Kept with the feature and changed with it. Section 4 of the semantic-linking discussion's [initial proposal](../20261003-semantic-linking-knowledge-graph/initial-proposal.md) calls them the durable specification. |
| Current-state guides such as `docs/packages.md` and `docs/architecture/` | Explanation, not specification | They describe the system for readers. When a guide and an acceptance test disagree, the test is right. |
| ADRs | None of the three | They record a decision and are superseded rather than edited. |
| `AGENTS.md`, skills, and agent profiles | Memory bank | Context for every session, not a specification of one change. |
| The v1 frontmatter model | The reverse of spec-as-source | The Rust types are the source, and the JSON schema is generated from them. |

Spec-as-source is the "mirror program" that section 21 of the initial proposal warns against, and
the article draws the same lesson from model-driven development. No conclusion here proposes it.

## Acceptance Tests as the Long-Term Specification

**Position (Jose Celano).** Issue specifications are disposable: they exist to build a feature.
After that, the long-term specification is the high-level acceptance tests, the only kind of
specification that has to change when the code's behavior changes. The repository does not need to
invent a verifiable, programmable specification format, because that is what a test is. Both
articles point the same way: Böckeler's spec-anchored level kept verifiable is a test suite, and
Osmani's conformance suites ([Writing a Good Specification](#writing-a-good-specification)) are
tests. Nor does it need natural-language scenarios that are translated into code, as Gherkin step
definitions do: an acceptance test written in Rust that reads like prose is specification enough.

Böckeler defines a specification as "a structured, behavior-oriented artifact [...] written in
natural language". Behavior-driven development (BDD) and its Gherkin language already meet that
definition with tests: Given/When/Then scenarios that domain experts can read as prose and that run
deterministically. Kiro writes its acceptance criteria in the same Given/When/Then form. So
acceptance tests can be the long-term specification without leaving her definition, if they are
written to be read. The position takes the lighter path: Rust tests that read like prose, without
Gherkin's translation layer.

The position holds within these limits:

- **Tests say what, not why.** The reasons belong in the homes listed in
  [Where the Why Lives](#where-the-why-lives), and a test can link to the decision it enforces.
- **Only some tests are specifications.** High-level tests through public interfaces (the UDP and
  HTTP tracker protocols, the REST API, the configuration), named by behavior, survive refactoring.
  Unit tests coupled to the implementation change with it and specify nothing durable.
- **Untested behavior is unspecified.** Before an issue specification is deleted, each acceptance
  criterion that must last needs an acceptance test. That makes the promotion step concrete and
  checkable.
- **Some requirements are hard to test.** Performance, security, and operability need benchmarks
  or dedicated checks, and what stays in prose belongs in an ADR or a guide.
- **A test suite is hard to browse.** Readers and agents need a way to find the behavior a feature
  has. The next section shows one that already exists.

### Reading the Tests as a Specification

Rust tests can already be read as an outline. [`cargo-pretty-test`](https://crates.io/crates/cargo-pretty-test)
runs `cargo test` and prints the results as a tree of test modules and test names, the way
JavaScript test runners print nested `describe` and `it` blocks. An excerpt for the UDP scrape
handler (`cargo pretty-test -p torrust-tracker-udp-server --lib scrape`, version 0.2.5):

```text
└── handlers
    └── scrape
        └── tests
            └── scrape_request
                ├─ ✅ it_should_preserve_the_order_of_eight_requested_info_hashes
                ├─ ✅ it_should_return_an_entry_for_each_duplicate_requested_info_hash
                ├─ ✅ it_should_return_zeroed_statistics_when_the_tracker_does_not_have_the_requested_torrent
                ├── with_a_public_tracker
                │   └─ ✅ it_should_return_statistics_when_the_public_tracker_has_the_requested_torrent
                └── with_a_whitelisted_tracker
                    ├─ ✅ it_should_return_statistics_when_the_listed_tracker_has_a_whitelisted_torrent
                    └─ ✅ it_should_return_zeroed_statistics_when_the_listed_tracker_has_not_whitelisted_the_torrent
```

The tree reads as a specification: "a scrape request, with a whitelisted tracker, should return
zeroed statistics when the tracker has not whitelisted the torrent". This supports the position:

- **It answers "hard to browse" without a new artifact.** The outline is generated from module and
  test names, so it cannot drift from the tests. The tool is an optional adapter in the sense of
  the
  [AI-agent portability ADR](../../../adrs/20260821172000_establish_ai_agent_context_capability_and_portability_governance.md):
  the source of truth is the names, and any runner that prints them works.
- **It shows what is not specified.** Nothing in the scrape tree mentions the 74-torrent limit:
  no test references `MAX_SCRAPE_TORRENTS`. By the limit "untested behavior is unspecified", the
  scrape limit is currently specified only in prose, and that prose has drifted: issue #2417
  found that HTTP does not enforce the limit its documentation states.
- **It makes names part of the specification.** Module names such as `with_a_whitelisted_tracker`
  and test names starting with `it_should_` are what make the tree read as prose. Naming
  conventions for acceptance tests become part of the contract (open question 1).

## Where the Why Lives

Acceptance tests record what the program does, not why. Today the repository records the why in
two places: ADRs for technical decisions, and Rust documentation next to the code. Product
decisions have no durable home. Their reasons are written in issue specifications, which are
deleted.

[`MAX_SCRAPE_TORRENTS`](../../../../packages/tracker-core/src/lib.rs) shows the gap. Its
documentation records two reasons of different kinds:

1. **An external constraint.** BEP 15 says "Up to about 74 torrents can be scraped at once": the
   limit follows from the size of a UDP datagram.
2. **A product decision.** The documentation says the same limit applies to HTTP scrapes, which
   BEP 48 does not require, and gives where the limit is implemented as the reason ("it's applied
   at the domain level"). Issue #2417 found that the HTTP path does not enforce it: 75 and 1,000
   distinct hashes were returned in full. Its specification
   ([2417 ISSUE.md](../../../issues/open/2417-2411-verify-http-scrape-info-hash-limit/ISSUE.md))
   separates the reasons: for UDP, a protocol constraint; for HTTP, any cap is a policy choice to
   bound the work one request causes.

The history shows how the HTTP documentation drifted. The first HTTP tracker, built on Warp,
rejected a scrape with more than `MAX_SCRAPE_TORRENTS` info hashes. The commit
`refactor: [#229] remove Warp HTTP Tracker` (2023-03-10) removed that check with the rest of the
Warp code, and the Axum tracker that replaced it never added one. Three weeks later,
`docs: [#266] crate docs for servers::http mod` documented the limit as still in force. The
history records no decision to drop the HTTP limit. A plausible reason is that the limit comes from
the UDP datagram size, which does not constrain an HTTP URL, but nothing says so.

A comment next to the code drifted from the code exactly as a specification would: prose that
restates behavior does not fail when the behavior changes. A test of the limit through the public
HTTP interface would have failed when the check went away, and the removal would have been a
visible decision instead of a silent loss.

| Kind of why | Example | Durable home | Today |
| ----------- | ------- | ------------ | ----- |
| External constraint | The BEP 15 scrape limit | Rust documentation on the item, citing the source | Exists |
| Technical decision | [Keep `Database` as an aggregate supertrait](../../../adrs/20260429000000_keep_database_as_aggregate_supertrait.md) | ADR | Exists |
| Product decision | Whether HTTP scrapes have a per-request cap, and which value (issue #2417) | A product decision log, organized by capability | Missing |
| Goal | Why a capability is built at all | An impact map | Missing |

A proposal for the two missing homes:

- **Product decision log.** One file per capability, such as scrape, announce, or private mode.
  Each entry has a stable ID, the decision, its reason, its source (a BEP, an issue, a discussion),
  and the acceptance tests that verify it. The log records decisions, not behavior: the what stays
  in the tests, so the log changes only when a decision changes, as ADRs do.
- **Citations and a check.** Tests and code documentation cite the decision ID, and a
  deterministic check confirms that every ID is cited by at least one test. That makes "untested
  behavior is unspecified" enforceable, and keeps the log from drifting.
- **Promotion.** A decision is still first written in the issue specification. At close-out it
  moves into the log before the specification is archived.
- **No restating.** External constraints stay in Rust documentation, and the log links to them.
- **Impact maps.** Gojko Adzic's *Impact Mapping* (2012) links a goal to the actors who can
  contribute to it, the impacts wanted on their behavior, and the deliverables that produce those
  impacts. The goal, impact, and capability levels are durable and can live in Git as Mermaid
  diagrams. Issues hang off deliverables only while they are open: an issue delivers a capability,
  it is not the reason for it.

The functional-technical boundary needs a simple rule, given Böckeler's caution
([Further Points from Böckeler](#further-points-from-böckeler)): a reason that would still hold
with a different implementation is a product decision; a reason that depends on the implementation
belongs in an ADR. By that rule, the documented reason for the HTTP scrape limit is an
implementation reason, while the reason issue #2417 gives for an HTTP cap, bounding per-request
work, would hold for any implementation: it is a product decision. The issue plans to record it in
an ADR because no product home exists, and until then its reasoning lives in an issue
specification that will be deleted.

## Writing a Good Specification

Addy Osmani's [How to write a good spec for AI agents](https://addyosmani.com/blog/good-spec/)
(2026-01-13) gives five principles:

1. Start from a short statement of the goal and let the agent draft the detailed specification in
   a read-only planning mode, before any code is written.
2. Structure the specification like a product requirements document. Cover the six areas that
   GitHub's study of more than 2,500 agent configuration files found in the most effective ones:
   commands, testing, project structure, code style, Git workflow, and boundaries.
3. Give the agent one focused task at a time, with only the context it needs: adherence drops as
   instructions pile up (the "curse of instructions").
4. Build in self-checks and constraints: three-tier boundaries ("always do", "ask first", "never
   do"), a review of the result against the specification, a second agent judging subjective
   criteria, and conformance tests derived from the specification.
5. Test continuously, and keep the specification current as decisions change.

Read with Böckeler's distinction, the article mixes two artifacts. The six areas describe the
memory bank, which serves every task; the goal, success criteria, and task breakdown describe the
specification of one change. In this repository:

- `AGENTS.md` already covers the six areas. Its rules include all three boundary tiers, spread
  across Essential Rules, Collaboration Principles, and Git Workflow rather than grouped by tier.
- The [issue template](../../../templates/ISSUE.md) covers the change: Goal, Scope, Implementation
  Plan, Acceptance Criteria, and Verification Plan. Its Implementation Plan and Commit Points are
  the article's small, reviewable tasks.
- Keeping the specification current makes it spec-anchored. Issue specifications here are kept
  current only while the work is open; after that, promotion has to carry their lasting content
  elsewhere.
- Principle 3 is the purpose of aspect 1 (knowledge graph): links let an agent load the context
  one task needs instead of everything.
- Conformance tests are spec-anchored and executable: the long-term specification of conclusion 1.
  Self-review and a judge agent are techniques in the sense of aspect 3 (orchestration).
- For parallel agents, the article asks for independent tasks, dependencies noted in the
  specification, and never two agents writing one file: the planning rule that aspect 4
  (coordination) describes.

## Further Points from Böckeler

The article's observations on the tools it tried bear on #2003:

- **A false sense of control.** Spec-kit's checklists are "interpreted by AI, so there is no 100%
  guarantee that they will be respected", and the agent both ignored instructions and followed
  them too eagerly. This supports the contract-and-technique conclusion of
  [Goals and Boundaries](../20261003-goals-and-boundaries/README.md#draft-conclusions): a gate must
  be deterministic, not a checklist an agent reads.
- **One workflow for all sizes.** Kiro turned a small bug into 4 user stories with 16 acceptance
  criteria. The repository's single [issue template](../../../templates/ISSUE.md) asks every issue
  to address a bug-fix process, a regression test strategy, and a design and ownership review,
  even if only to mark them `Not applicable`, so it carries the same risk.
- **Reviewing Markdown instead of code.** The generated Markdown was repetitive and tedious to
  review, and she would "rather review code". Each specification, audit, and report a change
  produces here adds to that review load.
- **Separating functional from technical specifications is hard,** and the profession has a poor
  record of doing it. [Where the Why Lives](#where-the-why-lives) depends on that separation.

## Draft Conclusions

1. **Issue specifications are disposable; acceptance tests are the long-term specification.**
   Behavior that must last is promoted to high-level acceptance tests before its issue
   specification is deleted. No separate executable specification format is needed, and
   spec-as-source is not a goal.
2. **Acceptance tests are written to be read.** Module and test names read like prose, with no
   Gherkin translation layer, and an outline generated from them is the view readers browse.
3. **Give every kind of why a durable home.** External constraints go in Rust documentation,
   technical decisions in ADRs, product decisions in a decision log that tests cite, and goals in
   impact maps. The last two are new artifacts and need the overhaul owner's approval.
4. **Promotion has three targets.** At close-out, behavior moves into acceptance tests, product
   decisions into the decision log, and technical decisions into ADRs. This is the concrete form of
   the promotion step in Goals and Boundaries.

## Open Questions for the Reviewer

1. Which tests count as the long-term specification, how are they marked or indexed so readers and
   agents can find a feature's behavior, and what naming makes a Rust acceptance test read like
   prose enough to serve as one?
2. Should `AGENTS.md` group its rules by the three boundary tiers, and should the issue template be
   reviewed against Osmani's principles as part of the overhaul?
3. Should the repository add a product decision log and impact maps? If so, where should they
   live: a new `docs/product/` folder, or next to the existing `docs/features/`?
4. Should the issue template come in sizes, for example a short form for small bugs, so that the
   workflow fits the change?

## Outcome

Pending review.
