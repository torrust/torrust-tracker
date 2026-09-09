---
source-run: https://github.com/torrust/torrust-tracker/actions/runs/34347690674
source-revision: 7abc30b2b9fb85b235e7b2ef2a40f5d5f7ed1555
collected-utc: 2026-09-09
---

# External Link Check Baseline

This record classifies every failure reported by the retained `lychee-external-link-report` artifact from [External Link Check run 34347690674](https://github.com/torrust/torrust-tracker/actions/runs/34347690674). It is evidence for issue [#2185](https://github.com/torrust/torrust-tracker/issues/2185), not a replacement for the artifact.

## Run Summary

| Metric            | Count |
| ----------------- | ----: |
| Total checks      | 1,677 |
| Unique links      | 1,114 |
| Successful checks | 1,216 |
| Redirects         |    24 |
| Timeouts          |     0 |
| Exclusions        |     0 |
| Errors            |   461 |

Lychee reported 421 `Cannot find fragment` diagnostics, 18 `404` responses, 13 cached errors, five connection refusals, three `403` responses, and one TLS handshake failure. The disposition table uses URL pattern and diagnostic together, so it does not assume every generic `ERROR` has the same cause.

## Classification Method

1. Parse every `Errors per input` entry in the Markdown artifact.
2. Group recurring URLs only when their host, path shape, and diagnostic establish one technical cause.
3. Keep different diagnostics for the same host separate; for example, GitHub review-comment anchors and stale GitHub repository paths have different dispositions.
4. Treat `404` as an actionable stale-reference candidate unless the referenced page is intentionally unpublished and that decision is documented.
5. Treat `403`, cached errors, and transport failures as transient/access-controlled until a rerun establishes a durable category.
6. Propose no host-wide, scheme-wide, or fragment-wide exclusion.

## Failure Dispositions

The following categories cover all 461 report errors:

| ID  | Reported failure pattern                                                                                                                         | Occurrences | Current disposition                                                                                                                                                                         | Next action                                                                                                                                                                          |
| --- | ------------------------------------------------------------------------------------------------------------------------------------------------ | ----------: | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------ |
| C1  | `https://github.com/torrust/torrust-tracker/pull/<number>#discussion_r<id>` with `Cannot find fragment`, primarily in `docs/copilot-pr-reviews/` |         416 | Candidate for a narrow online-only exclusion. GitHub review-comment DOM anchors are dynamic and cannot be reliably fetched as fragments by Lychee. The URLs remain useful audit references. | Propose a path-and-pattern-specific exclusion; prove a hosted rerun removes only this category while other GitHub links remain checked.                                              |
| C2  | `localhost` or `127.0.0.1` service URLs with connection-refused/cached diagnostics                                                               |           7 | Candidate for a narrow online-only exclusion. These are intentional local configuration and debugging examples that cannot resolve on a GitHub-hosted runner.                               | Propose an online-only URL-pattern exclusion covering only loopback hosts; preserve non-loopback HTTP/HTTPS validation.                                                              |
| C3  | `https://docs.rs/torrust-*` and `https://docs.rs/bittorrent-udp-protocol` returning `404`                                                        |          14 | Repair candidate. These package README links point at unavailable documentation pages and are externally observable stale references.                                                       | Verify current published crate names and replacement documentation locations, then repair in a small dedicated slice.                                                                |
| C4  | Repository GitHub URLs returning `404`                                                                                                           |           3 | Repair candidate. The affected targets are a removed issue-local research file and two removed test paths.                                                                                  | Verify the intended current repository targets or remove obsolete references, then repair in a small dedicated slice.                                                                |
| C5  | `https://caddyserver.com/docs/protocol/http3` returning `404`                                                                                    |           1 | Repair candidate.                                                                                                                                                                           | Identify the current authoritative Caddy HTTP/3 documentation target before modifying `docs/containers.md`.                                                                          |
| C6  | Docker Cloud ACI fragments, GitHub issue-comment fragments, and the Star History fragment returning `Cannot find fragment`                       |           5 | Unresolved investigation. Unlike C1, these are distinct target-page semantics and must not be hidden by a broad fragment exclusion.                                                         | Verify each fragment or replacement page individually; repair stale fragments, or propose a specific exclusion only if the target’s rendering makes automated validation impossible. |
| C7  | Medium and Stack Overflow URLs returning `403`                                                                                                   |           3 | Transient/access-controlled candidate. A `403` does not prove the referenced content is stale.                                                                                              | Rerun once before deciding between retaining the signal, replacing the citation, or documenting a narrowly scoped exception.                                                         |
| C8  | `https://www.fsf.org/` cached errors and one TLS handshake failure                                                                               |           4 | Transient/network candidate. The same target has multiple diagnostics across documents.                                                                                                     | Rerun once and compare results before any configuration decision.                                                                                                                    |
| C9  | Cached GitHub review-comment-anchor errors not reported as missing fragments                                                                     |           6 | Covered by the C1 URL pattern, but counted separately because the report cached a different diagnostic.                                                                                     | Verify the C1 exclusion removes both cached and missing-fragment diagnostics for the same exact review-comment URL pattern.                                                          |
| C10 | Other cached errors in two open issue specs                                                                                                      |           2 | Unresolved investigation. The current artifact’s cached diagnostic is insufficient evidence of stale content.                                                                               | Reproduce or rerun each URL before classifying it as repair, transient failure, or a separate durable exclusion candidate.                                                           |

The C1-C10 occurrence totals equal the report’s 461 errors. C1 and C9 share one proposed URL-pattern boundary but remain separate rows to preserve their distinct observed diagnostics.

## Affected Reference Inventory

### C3: Unavailable docs.rs pages

The 14 affected package README files are:

- `packages/axum-health-check-api-server/README.md`
- `packages/axum-http-server/README.md`
- `packages/axum-rest-api-server/README.md`
- `packages/axum-server/README.md`
- `packages/events/README.md`
- `packages/http-core/README.md`
- `packages/http-protocol/README.md`
- `packages/rest-api-application/README.md`
- `packages/rest-api-protocol/README.md`
- `packages/rest-api-runtime-adapter/README.md`
- `packages/tracker-core/README.md`
- `packages/udp-core/README.md`
- `packages/udp-protocol/README.md`
- `packages/udp-server/README.md`

### C4-C6: Individually actionable or investigatory URLs

| URL                                                                                                                                         | Diagnostic       | Source                                                              | Disposition |
| ------------------------------------------------------------------------------------------------------------------------------------------- | ---------------- | ------------------------------------------------------------------- | ----------- |
| `https://github.com/torrust/torrust-tracker/blob/develop/docs/issues/open/1671-ipv4-ipv6-client-metrics/research-dual-stack-portability.md` | `404`            | `docs/adrs/20260620000000_add_ipv6_v6only_config_option.md`         | C4          |
| `https://github.com/torrust/torrust-tracker/blob/develop/tests/stats.rs`                                                                    | `404`            | `docs/issues/drafts/increase-main-app-integration-test-coverage.md` | C4          |
| `https://github.com/torrust/torrust-tracker/tree/develop/tests/servers/api/contract/stats`                                                  | `404`            | `docs/issues/drafts/increase-main-app-integration-test-coverage.md` | C4          |
| `https://caddyserver.com/docs/protocol/http3`                                                                                               | `404`            | `docs/containers.md`                                                | C5          |
| `https://docs.docker.com/cloud/aci-container-features/#persistent-volumes`                                                                  | Missing fragment | `docs/containers.md`                                                | C6          |
| `https://docs.docker.com/cloud/aci-integration/#exposing-ports`                                                                             | Missing fragment | `docs/containers.md`                                                | C6          |
| `https://github.com/torrust/torrust-tracker/issues/1669#issuecomment-4010991467`                                                            | Missing fragment | `docs/issues/open/1669-overhaul-packages/EPIC.md`                   | C6          |
| `https://github.com/torrust/torrust-tracker/issues/269#issuecomment-1749443211`                                                             | Missing fragment | `docs/issues/open/269-review-dependency-licenses/ISSUE.md`          | C6          |
| `https://star-history.dera.page/#torrust/torrust-tracker`                                                                                   | Missing fragment | `README.md`                                                         | C6          |

### C7-C9: Rerun-first URLs

| URL or URL pattern                                                                    | Diagnostic                            | Current disposition |
| ------------------------------------------------------------------------------------- | ------------------------------------- | ------------------- |
| `medium.com/@kentbeck_7670/*`                                                         | `403`                                 | C7                  |
| `stackoverflow.com/a/56768087/3012842`                                                | `403`                                 | C7                  |
| `https://www.fsf.org/`                                                                | Cached error or TLS handshake failure | C8                  |
| `github.com/torrust/torrust-tracker/pull/<number>#discussion_r<id>` with cached error | Cached error                          | C9                  |
| Remaining one-off cached-error URLs                                                   | Cached error                          | C10                 |

## First Remediation Slice

C1 and C9 are the first proposed remediation boundary: the exact GitHub pull-request review-comment URL pattern is responsible for 424 of 461 reported errors. No exclusion has been added yet. The next change must be online-only, documented, and verified by a hosted workflow rerun that retains unrelated `404`, `403`, local-example, and third-party diagnostics.

## Deferred Work

C3-C9 are intentionally deferred until C1 is independently reviewed and its exclusion boundary is validated. This prevents the first configuration change from mixing clearly uncheckable dynamic anchors with potentially stale or transient external URLs.
