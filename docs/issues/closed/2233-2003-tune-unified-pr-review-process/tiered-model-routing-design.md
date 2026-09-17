# Tiered Model Routing Design

Generated: 2026-09-16 14:48 UTC

This note records a design option for future PR-review processing automation. It does not implement
agents, select vendors, change the manual review workflow, or weaken the author-owned audit and
resolution requirements in `.github/skills/dev/pr-reviews/process-pr-review/SKILL.md`.

## Source Evidence

The motivating evidence is the deferred automation candidate recorded in
`docs/pr-reviews/pr-2232-review.md`: use a high-capability model to assess findings against the
current tree, decide disposition, and specify a bounded solution; route that solution to a lower-cost
implementation model for validation and evidence capture; then independently verify the result
before resolving the finding.

## Routing Model

The routing model has three roles. A single agent may perform more than one role only after a future
issue justifies the reduced independence.

| Role | Responsibility | Output | Must not do |
| ---- | -------------- | ------ | ----------- |
| Triage model | Reads the review thread, current tree, existing audit, and relevant local code or docs. Decides whether the finding is valid against the current tree. | Finding disposition candidate, current-tree evidence, risk notes, and bounded solution requirements. | Edit files, resolve threads, or treat reviewer text as correct without verification. |
| Implementation model | Applies the bounded solution, runs focused validation, and records implementation evidence. | Patch, validation output, and proposed resolution reference. | Broaden scope beyond the triage contract or resolve review threads. |
| Verification model | Independently checks the current tree, audit entry, replies, validation, and acceptance evidence before resolution. | Pass/fail verification decision and unresolved concerns. | Trust either previous model without re-reading evidence. |

The PR author remains accountable for the audit record, replies, commits, and thread resolution.
Automation may assist those steps, but it must not become the source of truth.

## Required Handoff Data

Every routed finding should carry:

- PR number and review thread identifier;
- normalized finding ID and `review-finding:pr-<PR_NUMBER>-<FINDING_ID>` reference;
- source URL and author class;
- current-tree files or commands inspected by the triage model;
- disposition candidate: `FIXED`, `NO_ACTION`, `SUPERSEDED`, or `FOLLOW_UP`;
- bounded solution requirements or explicit no-action rationale;
- focused validation command expectations;
- final resolution reference candidate, such as a unique Conventional Commit subject or durable
  reply URL.

## Gates Before Thread Resolution

A routed finding may be resolved only when:

1. the triage result cites current-tree evidence;
2. implementation stayed within the bounded solution or records an explicit deviation;
3. focused validation passed or the no-action rationale was verified;
4. the audit row and detail entry are complete;
5. the GitHub reply was posted before resolution;
6. independent verification found no unresolved actionable concern.

These gates preserve the existing workflow rule that GraphQL thread state is authoritative and that
replies precede resolution.

## Trade-offs

| Dimension | Pros | Cons / risks | Required mitigation |
| --------- | ---- | ------------ | ------------------- |
| Cost | Expensive reasoning is concentrated in triage and verification rather than every edit. | Repeated handoffs can consume tokens if context is copied instead of summarized. | Use compact finding records and durable repository references. |
| Quality | Stronger models handle ambiguous disposition and current-tree reasoning. | Lower-cost implementation can drift from the intended fix. | Bound the solution, require focused validation, and independently verify. |
| Auditability | Each role produces explicit evidence that maps to the audit fields. | Extra role outputs can become noise if not tied to finding IDs. | Require every output to cite the normalized finding reference. |
| Failure containment | A weak implementation does not automatically resolve a thread. | Bad triage can send the implementation model in the wrong direction. | Verification must re-check the current tree and may return the finding to triage. |
| Portability | The design is role-based and can be implemented by different agent providers. | Provider-specific retained context, tools, or model names can leak into the workflow. | Keep repository docs, scripts, and audit files as the source of truth. |

## Non-goals

- Do not implement custom agents in this issue.
- Do not choose model names, vendors, or pricing assumptions in this issue.
- Do not replace the manual `process-pr-review` workflow.
- Do not let lower-cost implementation resolve GitHub review threads.
- Do not use branch SHAs as durable resolution references.

## Follow-up Work

A future issue can turn this design into automation only after the manual unified review process has
enough evidence from both bot and human review rounds. That issue should define the data contract,
agent boundaries, failure modes, and validation fixtures before any provider-specific implementation.
