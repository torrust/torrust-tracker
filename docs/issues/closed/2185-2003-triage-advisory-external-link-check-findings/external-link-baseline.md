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

The baseline errors fall into nine categories (C1-C9) covering all 461 report errors. C1 and C9 share one proposed URL-pattern boundary but remain separate to preserve their distinct observed diagnostics. C10 records a post-baseline class that entered the checked document set later and is listed after C9 for completeness.

### C1: GitHub review-comment missing fragments — 416 occurrences

- **Pattern:** `https://github.com/torrust/torrust-tracker/pull/<number>#discussion_r<id>` with `Cannot find fragment`, primarily in `docs/copilot-pr-reviews/`.
- **Disposition:** Candidate for a narrow online-only exclusion. GitHub review-comment DOM anchors are dynamic and cannot be reliably fetched as fragments by Lychee; the URLs remain useful audit references.
- **Next action:** Propose a path-and-pattern-specific exclusion and prove a hosted rerun removes only this category while other GitHub links remain checked.

### C2: Local service examples — 7 occurrences

    - **Pattern:** `localhost` or `127.0.0.1` service URLs with connection-refused or cached diagnostics.
    - **Disposition:** Resolved with online-only `exclude_loopback = true`. These intentional local configuration and debugging examples cannot resolve on a GitHub-hosted runner.
    - **Next action:** Complete. The local boundary test excluded `127.0.0.1` and `localhost` while checking `https://www.rust-lang.org/` successfully. Hosted [run 34616458439](https://github.com/torrust/torrust-tracker/actions/runs/34616458439) excluded 495 links; its remaining failures contained no loopback URLs because Lychee excluded them, while retaining 44 unrelated errors and its report artifact.

### C3: Unavailable docs.rs crate pages — 14 occurrences

- **Pattern:** `https://docs.rs/torrust-*` and `https://docs.rs/bittorrent-udp-protocol` returning `404`.
- **Disposition:** Repaired. These package README links pointed at unavailable individual crate pages; the corresponding workspace packages inherit the shared `https://docs.rs/crate/torrust-tracker/latest` documentation target.
- **Next action:** Complete. All 14 affected package manifests report the shared target through Cargo metadata, docs.rs returned HTTP 200 for it, and hosted [run 34829466145](https://github.com/torrust/torrust-tracker/actions/runs/34829466145) contained none of the replaced URLs while retaining unrelated errors and its report artifact.

### C4: Stale repository-controlled GitHub links — 3 occurrences

- **Pattern:** Repository GitHub URLs returning `404`.
- **Disposition:** Repaired and hosted-verified. The IPv6 research file moved from the open issue folder to the closed issue folder; the removed historical test paths are replaced by the current `tests/scaffold.rs` example and `tests/metrics/` targets.
- **Next action:** Complete. Hosted [run 34843399874](https://github.com/torrust/torrust-tracker/actions/runs/34843399874) contains none of the three replaced URLs while retaining unrelated errors and its report artifact.

### C5: Stale Caddy documentation link — 1 occurrence

- **Pattern:** `https://caddyserver.com/docs/protocol/http3` returning `404`.
- **Disposition:** Repaired and hosted-verified. Caddy's current server-options reference documents the `protocols` setting, including HTTP/3 as `h3`.
- **Replacement:** `https://caddyserver.com/docs/caddyfile/options` returned HTTP 200.
- **Evidence:** Hosted [run 34953081017](https://github.com/torrust/torrust-tracker/actions/runs/34953081017) contains no retired C5 URL but reports the prior `#servers` replacement fragment as missing. Follow-up [run 34971438822](https://github.com/torrust/torrust-tracker/actions/runs/34971438822) ran on merged revision `bbb58fa8`, contains neither the retired URL nor the replacement without a fragment as an error, retained 37 unrelated errors with no timeouts, and successfully uploaded its report artifact.
- **Next action:** Complete. Keep the C6-C8 categories under their independent dispositions.

### C6: Other missing fragments — 5 baseline occurrences (hosted-verified)

- **Pattern:** Docker Cloud ACI fragments, GitHub issue-comment fragments, and the Star History fragment returning `Cannot find fragment`.
- **Disposition:** Two Docker Cloud ACI fragments repaired and hosted-verified. The two GitHub issue-comment anchors and the Star History project selector are hosted-verified through exact online-only exclusions; their dynamic fragments preserve the specific comment or project view, while their page URLs alone do not.
- **Evidence:** Both Docker Cloud ACI pages redirect to Docker's retired-page notice. Azure's current Azure Files documentation describes the replacement mount-path behavior, and Azure's troubleshooting documentation confirms that ACI does not support Docker-style port mapping. Hosted [run 35128890381](https://github.com/torrust/torrust-tracker/actions/runs/35128890381) on merged revision `6e1e9d29` contains neither retired Docker URL as an error, retains 33 unrelated errors and 4 timeouts, and successfully uploads its report artifact. GitHub's issue-comment API confirms each referenced comment exists, while page-level checks confirm the Star History root does not preserve the repository-specific view. Hosted [run 35224905794](https://github.com/torrust/torrust-tracker/actions/runs/35224905794) on merged revision `3bad98d1` contains neither exact GitHub issue-comment anchor nor the Star History project selector as an error, retains unrelated GitHub fragments, `403` responses, and FSF transport failures, and successfully uploads its report artifact.
- **Next action:** Complete. Keep C7 and C8 under their independent rerun-first dispositions.

### C7: Third-party access-controlled links — 3 occurrences

- **Pattern:** Medium and Stack Overflow URLs returning `403`.
- **Disposition:** Persistent and not stale; handed off. The `403` responses persisted across hosted [run 35224905794](https://github.com/torrust/torrust-tracker/actions/runs/35224905794), [run 35238419294](https://github.com/torrust/torrust-tracker/actions/runs/35238419294), and [run 35315382956](https://github.com/torrust/torrust-tracker/actions/runs/35315382956). Medium returns `403` to a browser user agent as well as to Lychee. The Stack Overflow short permalink redirects to the full question URL, which also returns `403` to automated clients, while the answer still exists. Replacing the citations or excluding the hosts is a policy choice, so the cases are recorded in the semantic-link EPIC draft instead.
- **Next action:** None in this issue. See `docs/issues/drafts/refactor-semantic-link-conventions/external-link-check-residual-failures-2026-09-18.md`.

### C8: FSF transport failures — 4 occurrences

- **Pattern:** `https://www.fsf.org/` cached errors and one TLS handshake failure.
- **Disposition:** Persistent checker limitation; handed off. The failure persisted across the same three hosted runs as C7. `curl` and `openssl` reach `www.fsf.org` with HTTP 200 and a valid certificate, but the server offers only finite-field `DHE` cipher suites; rustls, which Lychee uses, negotiates `ECDHE` only, so the handshake can never complete from the checker. All four occurrences are the AGPL license footer repeated in `README.md`, `console/tracker-client/README.md`, `packages/rest-api-client/README.md`, and `packages/tracker-client/README.md`.
- **Next action:** None in this issue. Recorded with C7 in the EPIC handoff artifact.

### C9: Cached GitHub review-comment-anchor errors — 8 occurrences

- **Pattern:** `https://github.com/torrust/torrust-tracker/pull/<number>#discussion_r<id>` with cached-error diagnostics, including two references from open issue specs.
- **Disposition:** Covered by C1's exact URL pattern, but retained separately because the report cached a different diagnostic.
- **Next action:** Complete offline. `lychee --dump` with the configured online policy found zero matching pull-request review-comment anchors and retained both GitHub issue-comment anchors. A hosted rerun remains required only to verify the aggregate report, artifact upload, and unrelated failures.

### C10: Pull-request comment and review anchors — 23 occurrences (post-baseline)

- **Pattern:** `https://github.com/torrust/torrust-tracker/pull/<number>#issuecomment-<id>` and `https://github.com/torrust/torrust-tracker/pull/<number>#pullrequestreview-<id>` with `Cannot find fragment` or cached errors, all in `docs/pr-reviews/`.
- **Disposition:** Resolved with two exact online-only patterns. These records were added to the checked set after the baseline, so the class is not part of the 461-error total. GitHub renders the anchors client-side, exactly as for C1/C9, and the unified PR review process keeps generating them.
- **Next action:** Complete. A five-link boundary test excluded the three dynamic pull-request anchor forms and the exact C6 issue-comment anchor while retaining `https://github.com/torrust/torrust-tracker/pull/123/files`. Hosted [run 35315382956](https://github.com/torrust/torrust-tracker/actions/runs/35315382956) contains no GitHub fragment error.

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

All 14 links now use `https://docs.rs/crate/torrust-tracker/latest`. Cargo metadata reports the same inherited documentation value for each workspace package, and docs.rs returns HTTP 200 for this canonical target.

### C4-C6: Individually actionable or investigatory URLs

| URL                                                                                                                                         | Diagnostic       | Source                                                              | Disposition |
| ------------------------------------------------------------------------------------------------------------------------------------------- | ---------------- | ------------------------------------------------------------------- | ----------- |
| `https://github.com/torrust/torrust-tracker/blob/develop/docs/issues/open/1671-ipv4-ipv6-client-metrics/research-dual-stack-portability.md` | `404`            | `docs/adrs/20260620000000_add_ipv6_v6only_config_option.md`         | C4: replaced with archived research document |
| `https://github.com/torrust/torrust-tracker/blob/develop/tests/stats.rs`                                                                    | `404`            | `docs/issues/drafts/increase-main-app-integration-test-coverage/ISSUE.md` | C4: replaced with current scaffold example |
| `https://github.com/torrust/torrust-tracker/tree/develop/tests/servers/api/contract/stats`                                                  | `404`            | `docs/issues/drafts/increase-main-app-integration-test-coverage/ISSUE.md` | C4: replaced with current metrics targets |
| `https://caddyserver.com/docs/protocol/http3`                                                                                               | `404`            | `docs/containers.md`                                                | C5: replaced with `https://caddyserver.com/docs/caddyfile/options` |
| `https://docs.docker.com/cloud/aci-container-features/#persistent-volumes`                                                                  | Missing fragment | `docs/containers.md`                                                | C6: replaced with Azure Files mount-path documentation |
| `https://docs.docker.com/cloud/aci-integration/#exposing-ports`                                                                             | Missing fragment | `docs/containers.md`                                                | C6: replaced with Azure ACI troubleshooting documentation |
| `https://github.com/torrust/torrust-tracker/issues/1669#issuecomment-4010991467`                                                            | Missing fragment | `docs/issues/open/1669-overhaul-packages/EPIC.md`                   | C6: exact online-only exclusion hosted-verified |
| `https://github.com/torrust/torrust-tracker/issues/269#issuecomment-1749443211`                                                             | Missing fragment | `docs/issues/open/269-review-dependency-licenses/ISSUE.md`          | C6: exact online-only exclusion hosted-verified |
| `https://star-history.dera.page/#torrust/torrust-tracker`                                                                                   | Missing fragment | `README.md`                                                         | C6: exact online-only exclusion hosted-verified |

### C7-C9: Rerun-first URLs

| URL or URL pattern                                                                    | Diagnostic                            | Current disposition |
| ------------------------------------------------------------------------------------- | ------------------------------------- | ------------------- |
| `medium.com/@kentbeck_7670/*`                                                         | `403`                                 | C7: persistent bot protection, handed off |
| `stackoverflow.com/a/56768087/3012842`                                                | `403`                                 | C7: persistent bot protection, handed off |
| `https://www.fsf.org/`                                                                | Cached error or TLS handshake failure | C8: rustls cannot negotiate the server's `DHE`-only suites, handed off |
| `github.com/torrust/torrust-tracker/pull/<number>#discussion_r<id>` with cached error | Cached error                          | C9: exact online-only exclusion hosted-verified |

## First Remediation Slice

C1 and C9 are the first remediation boundary: the exact GitHub pull-request review-comment URL pattern is responsible for 424 of 461 reported errors. The online-only configuration excludes exactly that pattern. A two-link local boundary test excluded a matching pull-request review-comment anchor while retaining a non-matching GitHub issue-comment anchor as a visible error. A hosted workflow rerun must still retain unrelated `404`, `403`, local-example, and third-party diagnostics.

## Hosted Verification

[External Link Check run 34578523069](https://github.com/torrust/torrust-tracker/actions/runs/34578523069) ran the merged configuration on revision `427b0c93b01f1264f7ef824097ac5669d899486c`. The `Check External Links` step failed visibly and `Upload Lychee Report` succeeded; the retained `lychee-external-link-report` artifact is 1,743 bytes and expires on 2026-09-25.

The report recorded 1,816 total checks, 1,288 successful checks, 26 redirects, 476 excluded links, 52 errors, and no timeouts. No URL matching the exact `https://github.com/torrust/torrust-tracker/pull/<number>#discussion_r<id>` pattern remained in the report. The excluded count is greater than the 424 baseline occurrences because the merged `develop` revision contained additional matching review-comment links.

The report still contained visible `404`, `403`, local loopback, GitHub issue-comment and pull-request-review fragments, third-party missing-fragment, and rate-limited diagnostics. This proves the C1/C9 rule suppresses the intended dynamic review-comment-anchor pattern without hiding unrelated categories. The remaining 52 errors are the next triage input; their exact count is not directly comparable to the 461-error baseline because the checked document set changed between revisions.

### C3 repair verification

[External Link Check run 34829466145](https://github.com/torrust/torrust-tracker/actions/runs/34829466145) ran after PR #2208 merged on revision `952911af2321e848ac95ad183e8e8b96d8fd354a`. The `Check External Links` step failed visibly with exit code 2 and `Upload Lychee Report` succeeded. The retained `lychee-external-link-report` artifact is 1,357 bytes and expires on 2026-09-28.

The downloaded report contains 31 errors and no occurrence of any of C3's 14 replaced package-specific docs.rs URLs. It retains unrelated FSF transport, Docker missing-fragment, GitHub issue-comment and pull-request-review-anchor, stale-reference, third-party `403`, and Star History fragment errors. The count is not directly comparable to C2's 44 errors because PR #2207 archived issue specifications that were part of C2's checked document set. This verifies the C3 repair without adding an exclusion or hiding remaining external-link failures.

### C4 repair verification

[External Link Check run 34843399874](https://github.com/torrust/torrust-tracker/actions/runs/34843399874) ran after PR #2212 merged on revision `618723d49283432a99ea5604fc31becfa1a63a84`. The `Check External Links` step failed visibly and `Upload Lychee Report` succeeded. The retained `lychee-external-link-report` artifact is 1,191 bytes and expires on 2026-09-28.

The downloaded report records 1,782 total checks, 1,224 unique links, 1,196 successful checks, 25 redirects, 558 excluded links, 28 errors, and no timeouts. It contains none of C4's three replaced repository-controlled URLs. It retains unrelated Caddy `404`, Medium and Stack Overflow `403`, FSF transport, Docker missing-fragment, GitHub issue-comment and pull-request-review-anchor, and Star History fragment errors. The count is not directly comparable to C3's 31 errors because intervening merged changes modified the checked document set. This verifies the C4 repair without adding an exclusion or hiding remaining external-link failures.

### C5 repair verification

[External Link Check run 34971438822](https://github.com/torrust/torrust-tracker/actions/runs/34971438822) ran after PR #2225 merged on revision `bbb58fa802ff1cd2f330ac9146fee3dc76ce496b`. The `Check External Links` step failed visibly and `Upload Lychee Report` succeeded. The retained `lychee-external-link-report` records 2,006 total checks, 1,424 unique links, 1,249 successful checks, 25 redirects, 720 exclusions, 37 errors, and no timeouts.

The downloaded report contains neither the retired `https://caddyserver.com/docs/protocol/http3` URL nor the `https://caddyserver.com/docs/caddyfile/options` replacement without a fragment as an error. It retains unrelated Docker missing fragments, Medium and Stack Overflow `403` responses, FSF transport errors, GitHub comment and review fragments, and the Star History fragment. This verifies the C5 repair without adding an exclusion or hiding remaining external-link failures.

### C6 Docker repair verification

[External Link Check run 35128890381](https://github.com/torrust/torrust-tracker/actions/runs/35128890381) ran after PR #2231 merged on revision `6e1e9d29`. The `Check External Links` step failed visibly and `Upload Lychee Report` succeeded. The retained `lychee-external-link-report` artifact is 1,229 bytes and expires on 2026-09-30.

The report records 2,117 total checks, 1,483 unique links, 1,343 successful checks, 25 redirects, 737 exclusions, 33 errors, and 4 timeouts. It contains neither `https://docs.docker.com/cloud/aci-container-features/#persistent-volumes` nor `https://docs.docker.com/cloud/aci-integration/#exposing-ports` as an error. It retains unrelated third-party `403` responses, FSF transport errors, GitHub issue-comment and review fragments, the Star History fragment, and timeout failures. This verifies the Docker ACI repair without adding an exclusion or hiding remaining external-link failures.

### C6 dynamic-fragment exclusion verification

[External Link Check run 35224905794](https://github.com/torrust/torrust-tracker/actions/runs/35224905794) ran after PR #2251 merged on revision `3bad98d1`. The `Check External Links` step failed visibly with Lychee exit code 2 and `Upload Lychee Report` succeeded. The retained `lychee-external-link-report` artifact is 1,039 bytes, contains a 4,347-byte report, and expires on 2026-10-01.

The report records 2,172 total checks, 1,391 successful checks, 26 redirects, 751 exclusions, 30 errors, and no timeouts. It contains neither `https://github.com/torrust/torrust-tracker/issues/1669#issuecomment-4010991467`, `https://github.com/torrust/torrust-tracker/issues/269#issuecomment-1749443211`, nor `https://star-history.dera.page/#torrust/torrust-tracker` as an error. It retains unrelated GitHub issue-comment and review fragments, Medium and Stack Overflow `403` responses, and FSF cached and TLS handshake failures. This verifies the exact C6 exclusions without suppressing unrelated failures.

### C7-C8 rerun-first verification

[External Link Check run 35238419294](https://github.com/torrust/torrust-tracker/actions/runs/35238419294) was dispatched on merged revision `37c0bea5` with no configuration change, as the single rerun the policy in `docs/testing.md` requires before treating a `403` or transport failure as persistent. The `Check External Links` step failed visibly and `Upload Lychee Report` succeeded.

The report records 2,177 total checks, 1,509 unique links, 1,394 successful checks, 26 redirects, 751 exclusions, 30 errors, and 2 timeouts. All three C7 `403` responses and all four C8 FSF failures are present with the same diagnostics as in run 35224905794, so both categories are persistent. The two timeouts are `https://martinfowler.com/bliki/BeckDesignRules.html`, which had not failed before and did not fail in the next run; that is the transient case the rerun-first policy is designed to catch. The 23 remaining errors are the C10 pull-request comment and review anchors.

### C10 exclusion verification and closing run

[External Link Check run 35315382956](https://github.com/torrust/torrust-tracker/actions/runs/35315382956) ran after PR #2255 merged on revision `e6dd8918bf6964794882f60486f2da3e296cbb13`. The `Check External Links` step failed visibly and `Upload Lychee Report` succeeded. The retained `lychee-external-link-report` artifact is 782 bytes, contains a 1,782-byte report, and expires on 2026-10-02.

The report records 2,224 total checks, 1,537 unique links, 1,327 successful checks, 27 redirects, 886 exclusions, 7 errors, and 4 timeouts. It contains no GitHub URL with any fragment as an error. The seven errors are the two Medium `403` responses, the Stack Overflow `403`, and four FSF failures (one TLS `HandshakeFailure` and three cached). The four timeouts are `https://www.gnu.org/licenses/` in the same four license footers as the FSF links; this is the first run in which that URL failed. This verifies the C10 exclusions without suppressing unrelated failures and is the last report produced under this issue. It is preserved verbatim in `docs/issues/drafts/refactor-semantic-link-conventions/external-link-check-residual-failures-2026-09-18.md`.

## Handoff

C7 and C8 were rerun and probed to their root causes and are not stale references: one is bot protection on hosts that still serve the content, the other is a TLS cipher-suite mismatch between the server and Lychee's rustls stack. The GNU licenses timeouts are a first observation and remain a rerun-first candidate. Deciding whether such links should be replaced, excluded by host, or treated as a lower-importance class is a policy question, so these cases are handed to the semantic-link EPIC draft with the closing report and the insights from this triage. `.github/lychee-online.toml` keeps only the exact exclusions verified above.
