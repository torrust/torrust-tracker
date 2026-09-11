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

The following nine categories cover all 461 report errors. C1 and C9 share one proposed URL-pattern boundary but remain separate to preserve their distinct observed diagnostics.

### C1: GitHub review-comment missing fragments — 416 occurrences

- **Pattern:** `https://github.com/torrust/torrust-tracker/pull/<number>#discussion_r<id>` with `Cannot find fragment`, primarily in `docs/copilot-pr-reviews/`.
- **Disposition:** Candidate for a narrow online-only exclusion. GitHub review-comment DOM anchors are dynamic and cannot be reliably fetched as fragments by Lychee; the URLs remain useful audit references.
- **Next action:** Propose a path-and-pattern-specific exclusion and prove a hosted rerun removes only this category while other GitHub links remain checked.

### C2: Local service examples — 7 occurrences

- **Pattern:** `localhost` or `127.0.0.1` service URLs with connection-refused or cached diagnostics.
- **Disposition:** Candidate for a narrow online-only exclusion. These intentional local configuration and debugging examples cannot resolve on a GitHub-hosted runner.
- **Next action:** Propose an online-only loopback-only exclusion that preserves non-loopback HTTP/HTTPS validation.

### C3: Unavailable docs.rs crate pages — 14 occurrences

- **Pattern:** `https://docs.rs/torrust-*` and `https://docs.rs/bittorrent-udp-protocol` returning `404`.
- **Disposition:** Repair candidate. These package README links point at unavailable documentation pages and are externally observable stale references.
- **Next action:** Verify current published crate names and replacement documentation locations, then repair them in a small dedicated slice.

### C4: Stale repository-controlled GitHub links — 3 occurrences

- **Pattern:** Repository GitHub URLs returning `404`.
- **Disposition:** Repair candidate. The affected targets are a removed issue-local research file and two removed test paths.
- **Next action:** Verify the intended current repository targets or remove obsolete references, then repair them in a small dedicated slice.

### C5: Stale Caddy documentation link — 1 occurrence

- **Pattern:** `https://caddyserver.com/docs/protocol/http3` returning `404`.
- **Disposition:** Repair candidate.
- **Next action:** Identify the current authoritative Caddy HTTP/3 documentation target before modifying `docs/containers.md`.

### C6: Other missing fragments — 5 occurrences

- **Pattern:** Docker Cloud ACI fragments, GitHub issue-comment fragments, and the Star History fragment returning `Cannot find fragment`.
- **Disposition:** Unresolved investigation. Unlike C1, these have distinct target-page semantics and must not be hidden by a broad fragment exclusion.
- **Next action:** Verify each fragment or replacement page individually; repair stale fragments, or propose a specific exclusion only if target rendering makes automated validation impossible.

### C7: Third-party access-controlled links — 3 occurrences

- **Pattern:** Medium and Stack Overflow URLs returning `403`.
- **Disposition:** Transient/access-controlled candidate. A `403` does not prove the referenced content is stale.
- **Next action:** Rerun once before deciding between retaining the signal, replacing the citation, or documenting a narrowly scoped exception.

### C8: FSF transport failures — 4 occurrences

- **Pattern:** `https://www.fsf.org/` cached errors and one TLS handshake failure.
- **Disposition:** Transient/network candidate. The same target has multiple diagnostics across documents.
- **Next action:** Rerun once and compare results before any configuration decision.

### C9: Cached GitHub review-comment-anchor errors — 8 occurrences

- **Pattern:** `https://github.com/torrust/torrust-tracker/pull/<number>#discussion_r<id>` with cached-error diagnostics, including two references from open issue specs.
- **Disposition:** Covered by C1's exact URL pattern, but retained separately because the report cached a different diagnostic.
- **Next action:** Complete offline. `lychee --dump` with the configured online policy found zero matching pull-request review-comment anchors and retained both GitHub issue-comment anchors. A hosted rerun remains required only to verify the aggregate report, artifact upload, and unrelated failures.

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

## First Remediation Slice

C1 and C9 are the first remediation boundary: the exact GitHub pull-request review-comment URL pattern is responsible for 424 of 461 reported errors. The online-only configuration excludes exactly that pattern. A two-link local boundary test excluded a matching pull-request review-comment anchor while retaining a non-matching GitHub issue-comment anchor as a visible error. A hosted workflow rerun must still retain unrelated `404`, `403`, local-example, and third-party diagnostics.

## Deferred Work

C3-C9 are intentionally deferred until C1 is independently reviewed and its exclusion boundary is validated. This prevents the first configuration change from mixing clearly uncheckable dynamic anchors with potentially stale or transient external URLs.
