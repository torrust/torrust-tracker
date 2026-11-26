---
semantic-links:
  skill-links:
    - create-adr
    - wait-for-pr-review
    - process-pr-review
    - merge-pull-request
    - watch-pull-request-checks
  related-artifacts:
    - .claude/skills/create-adr/SKILL.md
    - .claude/skills/wait-for-pr-review/SKILL.md
    - .claude/skills/process-pr-review/SKILL.md
    - .claude/skills/merge-pull-request/SKILL.md
    - .claude/skills/watch-pull-request-checks/SKILL.md
    - "issue #3037"
---

<!-- skill-link: create-adr -->

# Review Pull Requests Without a Mandatory Reviewer

## Scope

Repository-level. The decision governs how every contributor, person or AI agent, gets a pull
request of this repository ready for merge, whatever packages it changes.

## Description

Until 2026-11-24 every pull request waited for a Copilot review on its head commit, and the agent
re-requested Copilot and the other maintainer (`da2ce7`) after each round of fixes. The
`wait-for-pr-review` exit condition, the `merge-pull-request` Merge Readiness checks and the poller's
default reviewer set all named Copilot.

On 2026-11-24 the maintainer stopped requesting Copilot reviews by default: they cost about 500 €
per month at the current pace, and the affordable level did not review well enough. CodeRabbit and
Greptile were enabled on trial; they review by themselves, but on free tiers with rate limits. The
other maintainer reviews with his own agents, but not on a fixed schedule, and waiting for his
reviews made pull requests stack up. Requiring any one of these reviewers would let a silent or
rate-limited reviewer block every pull request, which is the problem the change is meant to remove.

## Agreement

1. **No review from a named reviewer is required.** A pull request is ready for the maintainer when
   the checks pass, no actionable review thread is unresolved, and every review received so far is
   processed with `process-pr-review`. The `ACK <full head hash>` rule of Merge Readiness does not
   change. A review that arrives after the pull request looked ready reopens the loop.
2. **One process for every reviewer.** Findings from a person, Copilot, CodeRabbit, Greptile or any
   other bot are processed the same way. The 5-round limit counts rounds from every reviewer.
3. **Only a maintainer requests a review.** An AI agent never triggers one: no reviewer request, no
   review-bot command such as `@coderabbitai review`, no `@`-mention that asks for a review. A
   reviewer that reviews by itself after a push is not triggered by the agent; its new findings are
   processed like any other. Human contributors may still ask a reviewer to look.
4. **The poller watches every reviewer.** Without `--reviewer`, `wait-for-pr-review` reports reviews
   and comments from every account except the pull request's author. No reviewer login is
   hard-coded, so a change of reviewers needs no code change.
5. **Audit records name the reviewer by login.** New `PR-REVIEW.md` records have a `Reviewer` column
   with the GitHub login instead of the `Author class` column (`Copilot`, `Human`, `Unknown`).
   Existing records keep their column and stay valid.
6. **Review-bot check runs do not gate readiness.** Only check runs created by a GitHub Actions
   workflow count. A check run that a review service posts from its own GitHub App (for example
   `CodeRabbit` or `Greptile Review`) neither blocks readiness nor is waited for. The external
   reviewers are on trial and not trusted to gate a pull request; this may be revisited when the
   trial ends.

### Alternatives Considered

| Alternative | Why it was rejected |
| --- | --- |
| Keep Copilot as the required reviewer | The cost and review quality that started this change. |
| Require CodeRabbit or Greptile on the head commit | Unproven, rate-limited reviewers would block pull requests again. |
| Hard-code the new reviewer logins as poller defaults | Needs a code change whenever the reviewer set changes. |
| Read the reviewer list from a repository configuration file | Another file to keep current; watching every account except the author needs none. |
| Keep the author classes and add a generic `Bot` class, or one class per bot | The login is exact, needs no list, and the class can be derived from it. |
| Count review-bot check runs like workflow checks | Would let an external reviewer gate readiness. |

### Consequences

- A pull request can reach the maintainer with no AI review at all. Accepted: the maintainer reviews
  it himself.
- The poller wakes more often: on empty thread-reply reviews of a review bot and on comments from
  any account. Each wake-up costs one look.
- A late review reopens a pull request that looked ready; the agent keeps watching until it is
  merged or closed.
- Recognizing a review-bot check run relies on GitHub reporting no workflow for it.

## Affected Code

- `.claude/skills/wait-for-pr-review/tool/src/event.rs`: `Reviewers`, the reviewer set the poller
  watches (decision 4).
- `.claude/skills/wait-for-pr-review/tool/src/main.rs`: the `--reviewer` flag with no default
  logins, and the `watch` field of the `wait_started` record.
- `contrib/dev-tools/checks/agent-review-report-contract/src/main.rs`: the pinned sentences of the
  `Reviewer` column and the forbidden author classes (decision 5).
- The skills `wait-for-pr-review`, `watch-pull-request-checks`, `merge-pull-request` and
  `process-pr-review`, and `docs/templates/PR-REVIEW-TEMPLATE.md` (decisions 1 to 6).

## Date

2026-11-24

## References

- EPIC #2003 - Overhaul: Automation Tools and AI Agent Guardrails
- Issue #2977 (reviewer deadline and hand-over report)
- [`20260821172000_establish_ai_agent_context_capability_and_portability_governance.md`](20260821172000_establish_ai_agent_context_capability_and_portability_governance.md)
