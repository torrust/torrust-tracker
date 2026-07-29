# Event Metrics Normalization Evidence

## Purpose

This file records the progressive manual baseline and post-change probes
required by the draft specification. Each code-changing task must have one
entry before its change and one entry after it.

## Entry Format

| Field | Record |
| --- | --- |
| Task | Implementation task identifier and title |
| Phase | `baseline` or `post-change` |
| Configuration | Complete isolated tracker configuration or its stable path |
| Endpoints | Final listener bindings used by the probe |
| Commands | Exact commands or client interactions |
| Observed output | Relevant counters, responses, and ban behavior |
| Expected delta | Intended difference from baseline, if any |
| Automated coverage | Focused tests run for the task |
| Result | `DONE`, `FAILED`, or `BLOCKED`, with diagnosis |

## Task Evidence Matrix

| Task | Baseline | Post-change | Result |
| --- | --- | --- | --- |
| T2 | TODO | TODO | BLOCKED on #2036 |
| T3 | TODO | TODO | TODO |
| T4 | TODO | TODO | TODO |
| T5 | TODO | TODO | TODO |
| T6 | TODO | TODO | TODO |
| T7 | TODO | TODO | TODO |
| T8 | TODO | TODO | TODO |
| T9 | TODO | TODO | TODO |
| T10 | TODO | TODO | TODO |

## Required Probe Outcomes

Every applicable baseline and post-change record must state whether:

- traffic from an enabled listener changes aggregate metrics;
- traffic from a disabled listener changes aggregate metrics; and
- UDP cookie errors from a disabled listener reach shared ban enforcement.

The post-change record must also state how the probe identifies repeated
port-zero listeners without relying on their configured socket address.
