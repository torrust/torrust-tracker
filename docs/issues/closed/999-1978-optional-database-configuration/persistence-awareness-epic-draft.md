---
doc-type: epic
status: approved-draft
intended-destination: docs/issues/drafts/
github-issue: null
related-issue: 999
related-github-issue: 144
last-updated-utc: 2026-08-25 00:00
semantic-links:
  related-artifacts:
    - docs/issues/closed/999-1978-optional-database-configuration/ISSUE.md
    - docs/issues/closed/999-1978-optional-database-configuration/analysis.md
    - docs/issues/closed/999-1978-optional-database-configuration/solution.md
    - docs/issues/closed/999-1978-optional-database-configuration/adr-draft.md
    - docs/issues/drafts/144-make-rest-api-persistence-aware.md
---

# Draft EPIC - Progressively make tracker capabilities persistence-aware

> **Approved Phase 2 draft:** Refine this document against the merged #999,
> Issue #1980, and persistence-free activation-follow-up implementations. Then
> move it to `docs/issues/drafts/`, update the scope from final evidence, and
> create the GitHub EPIC. Do not create it during #999 Phase 2 or Phase 3 unless
> its scope becomes a blocker.

## Goal

Progressively remove implicit persistence assumptions from tracker capabilities
after the explicit v3 persistence-free deployment is activated by the small
post-#1980 follow-up drafted alongside #999.
Every capability, API response, configuration option, and test fixture should
make clear whether it needs persistence, uses session-only state, exposes
historical state, or is unavailable by configuration.

## Why This Is Needed

Issue #999 introduces optional v3 representation and optional container
dependencies. Its post-#1980 activation follow-up makes the tracker and
public UDP/HTTP services run without a database. The management REST API
remains persistence-required until the next-major API work under GitHub issue
144 implements its approved disabled-capability contract. The existing system
has broader historical coupling:

- management routes currently assume persistence-backed whitelist and key
  services exist;
- completed metrics can represent session and persisted history differently;
- torrent and statistics responses can expose in-memory values seeded from
  persistence without explicitly identifying their provenance;
- tests, examples, container artifacts, and deployment documentation often
  provision SQLite by default.

Those concerns require staged API, model, test, and operational changes. The
next-major REST API compatibility work is drafted in
`docs/issues/drafts/144-make-rest-api-persistence-aware.md` under GitHub issue 144. This EPIC must coordinate with it and must not delay the
configuration-overhaul EPIC once #999 and its activation follow-up supply a
safe persistence-free UDP/HTTP-tracker baseline.

## Scope

### In Scope

- Make application and REST API composition explicitly capability-aware.
- Standardize API behavior for a capability disabled by configuration.
- Make session and historical metric semantics explicit in API models.
- Expand persistence-free coverage across unit, integration, container, example,
  benchmark, and operational paths.
- Identify and remove remaining implicit persistence assumptions incrementally.

### Out of Scope

- Reverting the #999 v3 persistence-free boundary.
- Changing v2 configuration behavior.
- Creating separate feature-specific schemas or migration streams.
- Requiring all possible persistence-related improvements to land in one PR.

## Candidate Subissues

These are intentionally detailed candidates, not yet-created GitHub issues.
Refine ordering and boundaries after #999 merges, coordinating API contract
work with GitHub issue #144.

| Order | Candidate subissue                            | Problem to solve                                                                        | Expected outcome                                                                                    |
| ----- | --------------------------------------------- | --------------------------------------------------------------------------------------- | --------------------------------------------------------------------------------------------------- |
| 1     | Inventory remaining persistence assumptions   | #999 will identify a known baseline, but merged code/tests may reveal more assumptions. | Evidence-backed follow-up plan with ownership and priorities.                                       |
| 2     | Standardize disabled-capability API responses | Routes should distinguish disabled-by-configuration from database operational failure.  | Shared response model/status policy and contract tests.                                             |
| 3     | Refine REST capability composition            | Remove remaining direct route/service assumptions that a persistence store exists.      | Routes receive only the capability services they may use; disabled routes do not reach persistence. |
| 4     | Define metric provenance                      | Current-process counters and restored historical values have different meanings.        | Explicit session/historical fields or metadata, no numeric sentinels.                               |
| 5     | Define per-torrent completed semantics        | In-memory torrent counts can be lazily seeded from persisted counts.                    | Documented session versus lifetime semantics and compatible API model.                              |
| 6     | Expand persistence-free test infrastructure   | Existing helpers commonly create SQLite regardless of capability configuration.         | Reusable no-database fixtures and focused regression coverage.                                      |
| 7     | Audit operational artifacts                   | Examples, benchmarks, container paths, and docs may silently assume SQLite.             | Accurate deployment guidance and only intentional persistence setup.                                |

## Delivery Strategy

1. Start after #999 and the configuration-overhaul EPIC have merged or are no
   longer affected by the work.
2. Begin with an evidence refresh based on the merged #999 implementation.
3. Establish one API contract for configuration-disabled capabilities before
   changing individual routes.
4. Deliver metric-provenance changes as explicitly versioned API work with
   migration guidance where needed.
5. Keep each subissue independently testable and avoid reintroducing feature
   checks scattered through repositories.

## Progress Tracking

### Workflow Checkpoints

- [x] Draft created as a follow-up artifact for Issue #999.
- [x] Draft approved as a post-merge starting point.
- [ ] #999 implementation merged and draft reconciled with its final behavior.
- [ ] #1980 and persistence-free activation follow-up merged and draft
      reconciled with their final behavior.
- [ ] Epic specification moved to `docs/issues/drafts/` and approved.
- [ ] GitHub EPIC created and linked.
- [ ] Candidate subissues refined, created, and linked.

### Progress Log

- 2026-08-25 00:00 UTC - GitHub Copilot/User - Created initial follow-up EPIC
  draft while defining #999’s persistence-free v3 direction. The draft is not a
  created GitHub issue and must not block #999 or #1980.
- 2026-08-25 00:00 UTC - User - Approved this draft as the post-merge starting
  point. Its scope must be reconciled with merged #999, #1980, activation, and
  API #144 work before the GitHub EPIC is created.

## Acceptance Criteria

- [ ] The merged #999 implementation is the documented baseline for follow-up work.
- [ ] Every remaining persistence assumption has an explicit disposition.
- [ ] API semantics distinguish disabled capability, operational persistence
      failure, session-only values, and historical values.
- [ ] Persistence-free regression coverage does not silently provision SQLite.
- [ ] Operational artifacts accurately describe optional persistence.

## Risks and Trade-offs

- **API compatibility:** More explicit metric semantics can require client
  changes. Mitigation: version and document response-model changes deliberately.
- **Scope growth:** Persistence touches several layers. Mitigation: maintain
  small capability-focused subissues and an explicit order.
- **Behavior drift:** Configuration-aware checks can be duplicated. Mitigation:
  keep each capability decision at its composition boundary and cover it with
  contract tests.

## References

- Related issues: #999, #144
- Configuration-overhaul EPIC #1978
- `docs/issues/closed/999-1978-optional-database-configuration/analysis.md`
- `docs/issues/closed/999-1978-optional-database-configuration/solution.md`
- `docs/issues/closed/999-1978-optional-database-configuration/adr-draft.md`
