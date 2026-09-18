---
semantic-links:
  related-artifacts:
    - docs/issues/open/2185-2003-triage-advisory-external-link-check-findings/ISSUE.md
    - docs/issues/open/2185-2003-triage-advisory-external-link-check-findings/external-link-baseline.md
    - docs/issues/drafts/refactor-semantic-link-conventions/EPIC.md
    - docs/issues/drafts/refactor-semantic-link-conventions/external-link-check-residual-failures-2026-09-18.md
    - .github/lychee-online.toml
    - .github/workflows/external-link-check.yaml
---

# Agent Review Reports - Issue #2185

> Append one completed independent-review entry at a time. Do not modify, reorder, or remove
> earlier entries. A correction is a new entry that names the earlier conclusion.

## Reports

### 2026-09-11 08:32 UTC - Task Reviewer

- Invocation scope: Read-only review of the hosted C1/C9 verification evidence in `ISSUE.md` and `external-link-baseline.md`.
- Inputs: GitHub Actions runs [34577938048](https://github.com/torrust/torrust-tracker/actions/runs/34577938048) and [34578523069](https://github.com/torrust/torrust-tracker/actions/runs/34578523069), the downloaded replacement artifact, `.github/lychee-online.toml`, and `.github/workflows/external-link-check.yaml`.
- Evidence: Replacement run 34578523069 completed with a visible Lychee failure on `427b0c93b01f1264f7ef824097ac5669d899486c`; its `Upload Lychee Report` step succeeded. The unexpired 1,743-byte `lychee-external-link-report` artifact expires on 2026-09-25 and records 1,816 total checks, 1,288 successes, 26 redirects, 476 exclusions, 52 errors, and no timeouts. It contains no exact `#discussion_r` pull-request review-comment URLs, while unrelated `404`, `403`, loopback, GitHub issue-comment/pull-request-review fragments, other missing fragments, and a `429` remain visible.
- Findings:
  - Resolved: The exact online-only C1/C9 exclusion is sufficiently narrow and its hosted boundary behavior is verified.
  - Correction required: Cancelled run 34577938048 retained an empty, unusable report artifact; it did not provide usable Lychee output but should not be described as having no artifact.
  - Pending: Keep issue #2185 open. T5, AC2, AC6, remaining quality/completion checkpoints, and C2-C8 triage remain incomplete.
- Verdict: REVIEW PASSED after the cancelled-run artifact wording is corrected.
- Follow-up actions:
  - Record the cancelled-run correction in `ISSUE.md`.
  - Commit and review the hosted-verification evidence without closing issue #2185.
  - Continue with the next independently reviewable category after the evidence is merged.

### 2026-09-11 15:50 UTC - Task Reviewer

- Invocation scope: Read-only review of C2 hosted-verification evidence in `ISSUE.md` and `external-link-baseline.md`.
- Inputs: GitHub Actions [run 34616458439](https://github.com/torrust/torrust-tracker/actions/runs/34616458439), its downloaded report artifact, and the merged online configuration.
- Evidence: The run completed with a visible Lychee failure on `f6df96bf1ad59812db47457b84dcd5926c267c61`; `Upload Lychee Report` succeeded. The unexpired `lychee-external-link-report` artifact is 1,533 bytes and expires on 2026-09-25. Its report records 1,857 total checks, 1,318 successes, 25 redirects, 495 exclusions, 44 errors, and no timeouts. No `localhost`, `127.0.0.1`, or exact `#discussion_r` pull-request review-comment URL remained, while unrelated `404`, `403`, FSF transport, issue-comment, pull-request-review, and other missing-fragment failures remained visible.
- Findings:
  - Resolved: C2 loopback filtering is hosted-verified and remains narrow.
  - Resolved: T3, AC3, AC5, and M4 can be complete for the C1/C9+C2 exclusion slices.
  - Pending: Keep issue #2185 open for C3-C8, AC2/AC6, final quality/manual evidence, acceptance re-review, and completion review.
- Verdict: REVIEW PASSED.
- Follow-up actions:
  - Commit and review the C2 hosted-verification evidence without closing issue #2185.
  - Continue with the next independently reviewable category after the evidence is merged.

### 2026-09-11 09:42 UTC - Task Reviewer

- Invocation scope: Independent review of the uncommitted C2 loopback-exclusion slice for issue #2185 in `.github/lychee-online.toml`, `ISSUE.md`, and `external-link-baseline.md`.
- Inputs: Uncommitted diff, `.github/workflows/external-link-check.yaml`, `lychee.toml`, installed Lychee `0.24.2` implementation, and an explicit three-link online-config boundary check.
- Evidence: `.github/lychee-online.toml` alone adds `exclude_loopback = true`; the unchanged `lychee.toml` remains offline and has no loopback exclusion. The online workflow continues to use the online config. Lychee `0.24.2` maps this option exclusively to loopback filtering: IPv4 `127.0.0.0/8`, IPv6 `::1/128`, and the `localhost` hostname; private and link-local filtering are separate disabled options. The explicit three-link check excluded `http://127.0.0.1:9/` and `http://localhost:9/`; it checked `https://www.rust-lang.org/` successfully after one redirect. It therefore reported 3 total links, 2 exclusions, 1 success, 1 redirect for that successful public URL, and 0 errors. The issue and baseline record the same test and correctly state that hosted verification is pending.
- Findings:
  - Resolved: C2 is online-only, narrowly limited to loopback addresses and `localhost`, and does not enable broad private, link-local, or public-address exclusion.
  - Pending: The required hosted rerun has not yet verified C2 removal in the workflow report while retaining unrelated failures and its uploaded artifact. The prior hosted evidence only validates C1/C9.
  - Correction applied: AC3 and AC5 were marked complete despite the new C2 slice awaiting its required hosted evidence; both checkboxes and the acceptance-verification table now correctly show pending status.
  - Scope: The three-file diff contains no unrelated workflow, local-policy, timeout, retry, concurrency, path, or URL-pattern policy changes. `git diff --check` passed.
- Verdict: REVIEW WARNED.
- Follow-up actions:
  - Manually dispatch the hosted External Link Check with the C2 change, retain the report artifact, and record its URL, revision, counts, C2 absence, remaining unrelated failures, and upload outcome.
  - After that evidence is independently reviewed, restore AC3 and AC5 only if the hosted boundary behavior passes.

### 2026-09-11 16:45 UTC - Task Reviewer

- Invocation scope: Read-only review of the uncommitted C3 docs.rs repair slice for issue #2185: all changed package `README.md` files, `ISSUE.md`, and `external-link-baseline.md`.
- Inputs: Uncommitted diff and status; the C3 inventory in `external-link-baseline.md`; root and package Cargo manifests; resolved `cargo metadata --no-deps --format-version 1`; and live HTTP header checks for the replacement and all replaced docs.rs URLs.
- Evidence: Exactly 14 package README files changed, exactly matching the 14 C3 inventory entries, with no missing or extra paths. Each changes only its Crate documentation URL: the prior individual docs.rs target returns HTTP 404, and the replacement is `https://docs.rs/crate/torrust-tracker/`. Every affected package declares `documentation.workspace = true`; resolved Cargo metadata gives all 14 the inherited `https://docs.rs/crate/torrust-tracker/` value from `[workspace.package]`. The replacement returns HTTP 200 after its expected redirect to `/crate/torrust-tracker/latest`. The five non-C3 package documentation links remain unchanged. `git diff --check` passes.
- Findings:
  - Resolved: The C3 repair is precisely scoped and each replacement target agrees with its package's resolved Cargo documentation metadata.
  - Pending: Hosted External Link Check verification is correctly recorded as pending. Do not mark T2, AC2, M3, or a C3 hosted-verification result complete until a merged hosted run and retained report prove the 14 stale URLs no longer appear while unrelated failures remain visible.
- Verdict: REVIEW WARNED.
- Follow-up actions:
  - Merge the C3 slice, manually dispatch the advisory workflow, retain its report artifact, and record the run URL, revision, counts, absence of all 14 replaced URLs, remaining failures, and upload outcome.
  - Independently review that hosted evidence before completing C3-related issue criteria.

### 2026-09-14 10:00 UTC - Task Reviewer

- Invocation scope: Read-only review of the uncommitted C3 hosted-verification evidence in `ISSUE.md`, `external-link-baseline.md`, the supplied downloaded report, GitHub Actions run 34829466145, and `.github/workflows/external-link-check.yaml`.
- Evidence: Run 34829466145 completed on `952911af2321e848ac95ad183e8e8b96d8fd354a` with the expected visible `Check External Links` failure. `Upload Lychee Report` succeeded under the workflow's unconditional upload policy. The retained `lychee-external-link-report` artifact is 1,357 bytes, is not expired, and expires at `2026-09-28T09:51:01Z`. The supplied report contains none of C3's 14 retired package-specific docs.rs targets while retaining representative unrelated FSF transport, Docker fragment, GitHub comment/review-fragment, stale-reference, third-party `403`, and Star History fragment failures.
- Findings:
  - Resolved: The merged hosted evidence verifies the C3 repair boundary without adding an exclusion or suppressing unrelated failures. M3 may remain DONE.
  - Correction required: The supplied report summary and its individual entries both show 31 errors, not 24. Correct the C3 count in `ISSUE.md` and `external-link-baseline.md`.
  - Correction required: AC2 must remain pending for remaining repair slices, but its wording must no longer state that C3 hosted verification is pending.
  - Pending: Keep T2 and AC2 pending, and keep all issue-wide completion, quality, manual-verification, acceptance-review, and implementation-completion-review criteria pending.
- Verdict: REVIEW WARNED.
- Follow-up actions:
  - Correct the C3 error count and stale AC2-hosted-verification wording.
  - Preserve this review entry as append-only evidence; do not close issue #2185.

### 2026-09-14 12:10 UTC - Task Reviewer

- Invocation scope: Independent read-only review of the uncommitted C4 repository-controlled stale-link repair slice for issue #2185 in `docs/adrs/20260620000000_add_ipv6_v6only_config_option.md`, `docs/issues/drafts/increase-main-app-integration-test-coverage.md`, `ISSUE.md`, and `external-link-baseline.md`.
- Inputs: Scoped uncommitted diff; `torrust/develop` tree; commits `dc449858` and `8151e920`; current `tests/scaffold.rs` and `tests/metrics/` sources; prior issue-local review reports.
- Evidence: Commit `dc449858` renames the #1671 research document from `docs/issues/open/` to `docs/issues/closed/`; `torrust/develop` contains that archived target. Commit `8151e920` deletes `tests/stats.rs` and `tests/servers/api/contract/stats/mod.rs`; current `torrust/develop` contains `tests/scaffold.rs` and four `tests/metrics/` targets. The repaired ADR and draft contain the three intended replacement destinations. The old URLs remain only as inline-code historical entries in `external-link-baseline.md`, not as active repaired-document links. Exactly the four requested documentation files changed; no online/local Lychee configuration or workflow diff exists, and `git diff --check` passes.
- Findings:
  - Correction required: The draft statement that current integration coverage is organized under `tests/metrics/` is overly broad. Current `tests/` also contains `banning/`, `configuration/`, and `lifecycle/` suites. Describe `tests/metrics/` as the current metrics-focused coverage instead.
  - Resolved: The C4 archive and test-path replacement claims follow from repository history and the current `torrust/develop` tree.
  - Pending: Keep T2 and AC2 pending until a hosted post-merge External Link Check confirms the three C4 stale URLs are absent while unrelated failures and report upload remain visible.
- Verdict: REVIEW WARNED.
- Follow-up actions:
  - Correct the overly broad test-layout sentence in the draft.
  - Merge the coherent C4 slice, manually dispatch or observe the hosted External Link Check, retain its report artifact, and append the run URL, revision, counts, C4 URL absence, remaining failures, and upload outcome before completing T2 or AC2.

### 2026-09-14 16:05 UTC - Task Reviewer

- Invocation scope: Independent read-only review of the uncommitted C4 hosted-verification evidence in `ISSUE.md`, `external-link-baseline.md`, the supplied downloaded report, GitHub Actions run 34843399874, and `.github/workflows/external-link-check.yaml`.
- Evidence: Run 34843399874 completed on `618723d49283432a99ea5604fc31becfa1a63a84`. `Check External Links` failed visibly and `Upload Lychee Report` succeeded under the workflow's `if: always()` upload policy. The official unexpired `lychee-external-link-report` artifact is 1,191 bytes and expires at `2026-09-28T12:50:59Z`. The supplied report records 1,782 total checks, 1,224 unique links, 1,196 successful checks, 25 redirects, 558 exclusions, 28 errors, and zero timeouts.
- Findings:
  - Resolved: The report contains none of C4's three retired repository-controlled URLs.
  - Resolved: Representative unrelated Caddy `404`, Medium and Stack Overflow `403`, FSF transport, Docker missing-fragment, GitHub issue-comment/pull-request-review-anchor, and Star History fragment failures remain visible.
  - Resolved: The C4 evidence correctly cautions that its 28 errors are not directly comparable with C3's 31 errors because intervening merged changes modified the checked document set.
  - Correction required: Add run 34843399874 to AC5's evidence row so the documented evidence explicitly includes the hosted rerun for the C4 remediation slice.
  - Pending: Keep T2 and AC2 pending for C5-C8 and remaining repair verification. Keep all issue-wide quality, manual-verification, acceptance-review, implementation-completion-review, and closure criteria pending.
- Verdict: REVIEW WARNED.
- Follow-up actions:
  - Make the AC5 evidence-row correction.
  - Preserve this review as append-only evidence; do not close issue #2185.

### 2026-09-15 07:07 UTC - Task Reviewer

- Invocation scope: Independent read-only review of the uncommitted C5 Caddy HTTP/3 documentation-link repair in `docs/containers.md`, `ISSUE.md`, and `external-link-baseline.md` for issue #2185.
- Evidence: The retired active reference `https://caddyserver.com/docs/protocol/http3` returns HTTP 404. Its replacement, `https://caddyserver.com/docs/caddyfile/options#servers`, returns HTTP 200 and is Caddy's authoritative server-options documentation. It documents the `protocols` option, including HTTP/3 as `h3`, directly supporting the documented `servers :443 { protocols h1 h2 h3 }` configuration.
- Findings:
  - Resolved: The patch replaces exactly one active stale Caddy reference and retains the retired URL as historical baseline evidence.
  - Resolved: No online or local Lychee configuration, workflow, testing policy, or exclusion category changes.
  - Resolved: `git diff --check`, `linter all`, focused local-link validation, and the active-source absence check pass.
  - Pending: Keep T2, M3, and AC2 in progress until a hosted External Link Check confirms the retired URL is absent while unrelated failures and report upload remain visible.
- Verdict: PASS.
- Follow-up actions:
  - Commit and merge the C5 repair as an isolated slice, then dispatch the hosted External Link Check and record independently reviewed artifact evidence.
  - Do not close issue #2185.

### 2026-09-15 10:21 UTC - Task Reviewer

- Invocation scope: Independent review of the uncommitted C5 follow-up repair for issue #2185 in `docs/containers.md`, `ISSUE.md`, and `external-link-baseline.md`.
- Inputs: Scoped uncommitted diff and status; `docs/containers.md`; C5 issue and baseline entries; supplied `/tmp/c5-lychee-report/lychee-report.md`; GitHub Actions run 34953081017; and focused/full validation output.
- Evidence:
  - Run 34953081017 completed with conclusion `failure` on `a1ddcaa01968ee227b3bdf937b9257b420345931`. Its `Check External Links` step visibly failed with exit code 2, while `Upload Lychee Report` succeeded.
  - The official unexpired `lychee-external-link-report` artifact is 1,342 bytes and expires at `2026-09-29T09:39:45Z`. The supplied artifact report records 1,952 total checks, 1,379 unique links, 1,226 successful checks, 25 redirects, 686 exclusions, 38 errors, and 2 timeouts.
  - It contains no retired C5 URL and records `https://caddyserver.com/docs/caddyfile/options#servers` as a missing-fragment error.
  - The repaired Caddy server-options URL without a fragment returns HTTP 200, supports the documented `servers` / `protocols` configuration, and is Caddy's authoritative options page.
  - The diff removes only that fragment and updates issue evidence; it changes no Lychee configuration or workflow. `git diff --check`, `linter all`, and focused local-link validation passed.
- Acceptance criteria matrix:
  - PASS: The prior hosted run is accurately recorded as failed diagnostic evidence while retaining a successful report upload.
  - PASS: C5's retired URL is absent from the supplied hosted report, and the prior replacement fragment is accurately identified as missing.
  - PASS: The active replacement is the narrow authoritative URL without a fragment and is reachable with HTTP 200.
  - PENDING: C5 hosted verification remains required because run 34953081017 exercised the invalid `#servers` replacement rather than the current URL without a fragment.
  - PASS: T2, M3, AC2, and broader issue completion remain pending; no issue-wide completion criterion was prematurely checked.
- Findings:
  - Pending: Manually dispatch or observe a hosted External Link Check containing the C5 URL without a fragment. Record its run URL, revision, counts, C5 absence, representative unrelated failures, and successful report upload before treating C5, M3, T2, or AC2 as complete.
  - Resolved: No broad suppression, Lychee-policy, or workflow change is present.
- Completion-review finding: Correctly pending. The issue records neither a retrospective nor a no-retrospective rationale because the issue is still in progress; do not complete the implementation review yet.
- Issue-spec updates: No checkboxes changed. The C5 progress and baseline entries correctly retain hosted verification, T2/M3/AC2, and broader completion as pending.
- Verdict: REVIEW WARNED.
- Follow-up actions:
  - Obtain and independently review the hosted verification for the C5 target without a fragment.
  - Keep issue #2185 open and do not complete its remaining issue-wide verification or completion checkpoints from this diagnostic run.

### 2026-09-15 13:04 UTC - Task Reviewer

- Invocation scope: Read-only review of the uncommitted C5 hosted-verification evidence in `ISSUE.md` and `external-link-baseline.md`, with the active Caddy reference in `docs/containers.md`.
- Inputs: Merged PR #2225 commit `bbb58fa802ff1cd2f330ac9146fee3dc76ce496b`, GitHub Actions [run 34971438822](https://github.com/torrust/torrust-tracker/actions/runs/34971438822), and its downloaded `lychee-external-link-report` artifact.
- Evidence: The run completed with its expected advisory failure on the merged C5 revision. `Check External Links` failed visibly while `Upload Lychee Report` succeeded. The retained 1,277-byte artifact expires on 2026-09-29. Its report records 2,006 total checks, 1,424 unique links, 1,249 successful checks, 25 redirects, 720 exclusions, 37 errors, and no timeouts. It contains neither the retired `https://caddyserver.com/docs/protocol/http3` URL nor the current Caddy options URL without a fragment as an error, while unrelated Docker missing fragments, third-party `403` responses, FSF transport errors, GitHub comment/review fragments, and the Star History fragment remain visible.
- Findings:
  - Resolved: C5 is hosted-verified without adding an exclusion or hiding unrelated external-link failures.
  - Correction required: Add run 34971438822 to AC5's consolidated evidence row.
  - Pending: Keep T2 and M3 in progress for C6-C8. Keep AC2, AC6, issue-wide quality, manual-verification, acceptance-review, implementation-completion-review, and closure checkpoints pending.
- Verdict: REVIEW WARNED.
- Follow-up actions:
  - Add the C5 run to AC5's evidence row and commit the C5 hosted-verification record.
  - Continue C6-C8 under their independent dispositions; do not close issue #2185.

### 2026-09-15 15:08 UTC - Task Reviewer

- Invocation scope: Read-only review of the uncommitted C6 Docker ACI fragment repair in `docs/containers.md`, `ISSUE.md`, and `external-link-baseline.md`.
- Inputs: Scoped uncommitted diff; the retired Docker ACI URLs; Azure Files volume documentation; and Azure Container Instances troubleshooting documentation.
- Evidence: Both retired Docker URLs redirect to Docker's retired-page notice. Azure's Azure Files documentation states that mounting a share over a container directory obscures existing files or directories at the mount path while the container runs. Azure's troubleshooting documentation states that ACI does not support Docker-style port mapping. The old URLs remain as historical baseline inventory entries; no Lychee configuration, workflow, testing policy, or broad exclusion changed. Focused `git diff --check`, Markdown, spelling, and local-link validation passed.
- Findings:
  - Resolved: The volume-mount and port-mapping wording is limited to claims supported by the authoritative Azure documentation.
  - Resolved: C6's GitHub issue-comment and Star History fragments remain separate investigations; no broad fragment exclusion is proposed.
  - Pending: Hosted verification is required before completing the C6 Docker repair, M3, T2, or AC2. Keep the issue-wide completion criteria pending.
- Verdict: REVIEW WARNED.
- Follow-up actions:
  - Commit and merge this isolated repair, then obtain and independently review a hosted External Link Check report.
  - Continue the remaining C6 fragments independently; do not close issue #2185.

### 2026-09-16 17:55 UTC - Task Reviewer

- Invocation scope: Read-only review of the merged C6 Docker ACI hosted-verification evidence in `ISSUE.md`, `external-link-baseline.md`, `agent-review-reports.md`, and `docs/containers.md`.
- Inputs: PR #2231 merge commit `6e1e9d29b5d88b763e1748ad45afabe5d725d62f`, GitHub Actions [run 35128890381](https://github.com/torrust/torrust-tracker/actions/runs/35128890381), and its downloaded `lychee-external-link-report` artifact.
- Evidence: The run completed with the expected advisory failure. `Check External Links` failed visibly while `Upload Lychee Report` succeeded. The retained 1,229-byte artifact expires on 2026-09-30. Its report records 2,117 total checks, 1,483 unique links, 1,343 successful checks, 25 redirects, 737 exclusions, 33 errors, and 4 timeouts. It contains neither retired Docker ACI URL as an error and retains unrelated third-party `403`, FSF, GitHub comment/review fragment, Star History fragment, and timeout failures.
- Findings:
  - Resolved: The two Docker ACI repairs are hosted-verified without adding an exclusion or hiding unrelated external-link failures.
  - Pending: The GitHub issue-comment and Star History fragments remain separate C6 investigations; C7 and C8 remain pending. Keep AC2, AC6, final quality, manual-verification, acceptance-review, implementation-completion-review, and closure checkpoints pending.
- Verdict: REVIEW PASSED for the C6 Docker sub-slice.
- Follow-up actions:
  - Record run 35128890381 in the issue and baseline evidence while preserving the remaining C6-C8 work as pending.
  - Do not close issue #2185.

### 2026-09-17 12:08 UTC - Task Reviewer

- Invocation scope: Read-only review of the uncommitted C6 dynamic-fragment exclusion slice in `.github/lychee-online.toml`, `ISSUE.md`, and `external-link-baseline.md`.
- Inputs: Scoped diff, `.github/workflows/external-link-check.yaml`, unchanged `lychee.toml`, the referenced source links, GitHub issue-comment API responses, Star History page behavior, and the focused Lychee boundary fixture.
- Evidence: The three new fully anchored patterns match only the two exact GitHub issue-comment URLs and the exact Star History project selector. GitHub's issue-comment API returned each referenced comment's exact URL. Star History reads the repository selector from the client-side fragment, while its fragment-free root does not preserve that project view. `lychee --config .github/lychee-online.toml --dump --format json .tmp/c6-dynamic-fragment-boundary.md` emitted only the non-matching GitHub issue and unrelated Star History SVG controls.
- Findings:
  - Resolved: The exclusions are online-only and retain all other GitHub URLs, fragments, and Star History targets for checking.
  - Resolved: The C6 Docker hosted-verification state remains distinct from the three pending dynamic-fragment exclusions.
  - Pending: A hosted External Link Check must prove that the exact URLs are absent while unrelated failures, the visible Lychee failure, and report upload remain intact.
- Verdict: REVIEW PASSED.
- Follow-up actions:
  - Commit and merge this isolated C6 exclusion slice.
  - Run and independently review the required hosted boundary verification before restoring T3/T4, AC3/AC5, and M4 to complete.

### 2026-09-17 14:50 UTC - Task Reviewer

- Invocation scope: Read-only review of the uncommitted C6 dynamic-fragment hosted-verification evidence in `ISSUE.md` and `external-link-baseline.md`, the current C6 online policy, GitHub Actions [run 35224905794](https://github.com/torrust/torrust-tracker/actions/runs/35224905794), and its downloaded report artifact.
- Inputs: Merged PR #2251 revision `3bad98d1587ae396f87eb0297531e8fa7b4f15f1`, run and job metadata, downloaded `lychee-external-link-report`, `.github/lychee-online.toml`, and the C6 evidence diff.
- Evidence: Run 35224905794 failed visibly with Lychee exit code 2, while `Upload Lychee Report` succeeded. Its unexpired 1,039-byte artifact, expiring 2026-10-01, contains a 4,347-byte report with 2,172 total checks, 1,391 successes, 26 redirects, 751 exclusions, 30 errors, and no timeouts. The report contains none of the two exact GitHub issue-comment anchors or the Star History project selector, while retaining 23 unrelated GitHub-fragment errors, three `403` responses, and four FSF cached or TLS failures.
- Findings:
  - Resolved: The exact C6 online-only exclusions are hosted-verified without suppressing unrelated failures, and the run supports completing T3/T4, AC3/AC5, and M4.
  - Correction applied: The baseline's stale `C6-C8 remain deferred` summary conflicted with C6's hosted-verified state; it now defers only C7-C8.
  - Pending: C7 and C8 remain rerun-first categories; T2, T5, AC2, AC6, and issue-wide completion evidence remain pending.
- Verdict: REVIEW PASSED after the deferred-work correction.
- Follow-up actions:
  - Commit and review this evidence-only slice without closing issue #2185.
  - Continue C7 and C8 independently.

### 2026-09-18 08:20 UTC - Task Reviewer

- Invocation scope: Independent review of the #2185 closure and semantic-link EPIC handoff documentation in `ISSUE.md`, `external-link-baseline.md`, `docs/issues/drafts/refactor-semantic-link-conventions/EPIC.md`, `docs/issues/drafts/refactor-semantic-link-conventions/external-link-check-residual-failures-2026-09-18.md`, and `project-words.txt`.
- Inputs: Current uncommitted diff, hosted External Link Check runs 35224905794, 35238419294, and 35315382956, the closing residual-failures artifact, and local validation output from `linter markdown`, `linter cspell`, `linter lychee`, and `git diff --check`.
- Evidence: The final hosted run 35315382956 contains no GitHub URL with any fragment as an error, retains the residual Medium, Stack Overflow, FSF, and GNU license findings, and uploaded the retained `lychee-external-link-report` artifact. The EPIC states that these residual cases are input for S13 and that no policy solution is chosen there. The EPIC also records the maintainer's proposed policy option for critical unstable URLs: preserve a vetted copy, excerpt, or distilled in-repository context for maintainers and AI agents when legal and maintainable, and treat the original URL as provenance, background reading, or an online-check exception.
- Findings:
  - Resolved: The substantive #2185 closure is supported by recorded evidence: stale references are repaired or found non-stale, exact online-only exclusions are hosted-verified, and residual cases are handed to the semantic-link EPIC instead of decided in this task.
  - Resolved: The EPIC handoff preserves the residual report and policy questions without selecting a checker or semantic-link policy prematurely.
  - Resolved: The issue completion-review placeholder was replaced with the EPIC-handoff conclusion, and this final independent review is recorded in the issue-local report.
- Verdict: REVIEW PASSED.
- Follow-up actions:
  - Commit the closure documentation and residual report.
  - Open the closing PR for issue #2185; after merge, close the GitHub issue and move the spec from `docs/issues/open/` to `docs/issues/closed/` in the normal completed-issue cleanup flow.
