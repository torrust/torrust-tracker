# Clippy Allow Inventory

## Scope and Generation

This inventory covers actual Rust attributes in maintained source roots: `src/`, `packages/`,
`console/`, `tests/`, and `contrib/`. It excludes generated and runtime roots (`target/`, `storage/`).
The inventory is an issue-local human audit record for #2158, not the prospective enforcement baseline.

Generated on 2026-09-15 by recursively scanning each source root for an attribute that begins at
a source line and has the form `#![allow(clippy::...)]` or `#[allow(clippy::...)]`. Anchoring the
match at the line start excludes attribute-shaped fixture text embedded in string literals.

Completeness method: rerun the anchored scan and compare its `path:line` keys with the **Source
location** column below. Each row represents one allow attribute; the **Lint name(s)** column lists
every Clippy lint named by that attribute. The initial scan found 234 attributes containing 244 lint allowances.

This inventory contains all 234 source locations from the initial scan. Classification and
remediation remain separate work; an entry marked **Pending** is recorded but not yet decided.

## Classification Vocabulary

- **Retain** — necessary documented exception, with a nearby source rationale.
- **Remove** — suppression can be deleted alongside its focused fix and validation.
- **Temporary** — requires a stable removal condition and an approved, linked follow-up issue.
- **Pending** — not yet classified; no decision is implied.

## Grouped Classification Process

Classify source occurrences by rationale category before remediating them. This establishes a
consistent decision for repeated lint families while preserving per-entry evidence when a source
occurrence differs. A category decision is not a blanket exception: every entry must still receive
a disposition, evidence, and (when retained) a nearby source rationale.

| Order | Rationale category | Lint families | Review approach |
| ----- | ------------------ | ------------- | --------------- |
| 1 | Compatibility and external API shape | `struct_field_names`, `module_name_repetitions`, `redundant_field_names`, `from_over_into`, `extra_unused_lifetimes` | Identify external, wire, and public API naming contracts; retain only the narrow necessary scope. |
| 2 | Intentional executable or CLI output | `print_stdout`, `print_stderr`, `exit` | Confirm the binary or command-line output contract; retain output only where it is an intentional interface. |
| 3 | Framework-required async signatures | `unused_async`, `manual_async_fn`, `async_yields_async` | Confirm framework trait or handler signatures that require the async shape. |
| 4 | Numeric-domain conversions | `cast_possible_truncation`, `cast_sign_loss`, `cast_precision_loss` | Establish bounds and loss-tolerance invariants for each conversion family; do not apply a blanket decision. |
| 5 | Trait and public API ergonomics | `double_must_use`, `must_use_candidate`, `needless_pass_by_value`, `unnecessary_wraps`, `result_large_err`, `future_not_send` | Evaluate public trait contracts, error-size trade-offs, async Send boundaries, and API compatibility. |
| 6 | Documentation and panic contracts | `doc_markdown`, `missing_errors_doc`, `missing_panics_doc` | Add or correct focused documentation unless the public or test-only contract provides concrete evidence for retaining the exception. |
| 7 | Type and callable-shape constraints | `struct_excessive_bools`, `empty_enums`, `too_many_arguments`, `unused_self` | Evaluate whether an externally constrained schema, trait shape, or focused design change best expresses the contract. |
| 8 | Test, example, benchmark, and fixture code | `missing_panics_doc`, `unused_async`, output and conversion lints | Confirm the non-production target and whether a focused local change is clearer than a suppression. |
| 9 | Module visibility and imports | `redundant_pub_crate`, `wildcard_imports` | Confirm module visibility and import intent; prefer the narrowest visibility and explicit imports unless compatibility evidence requires otherwise. |
| 10 | UDP protocol crate baseline | All crate-level lints in `packages/udp-protocol/src/lib.rs` | Review each lint independently; reduce scope or remove where practical before retaining any crate-wide exception. |
| 11 | Likely direct removals | `derivable_impls`, `needless_borrow`, `explicit_iter_loop`, `chunks_exact_to_as_chunks`, `default_trait_access`, `legacy_numeric_constants`, `match_same_arms`, `semicolon_if_nothing_returned` | Apply the focused Clippy remediation and validate the owning package. |

After classification, remediate one category and package-scoped batch at a time. Do not create a
temporary GitHub follow-up until its folder-style draft specification has been reviewed and approved.

## Classification Decisions

### Compatibility and External API Shape

The `redundant_field_names` allowances associated with `derive_more::Constructor` are retained.
On the current MSRV-compatible `derive_more` version, Nightly Clippy diagnoses the macro-generated
`field: field` initializer even though it is outside this workspace's source. Each affected source
location already documents the condition for removal: remove the allowance when `derive_more` emits
field-init shorthand. The next remediation batch must add the native `reason` parameter while
preserving that nearby explanation.

The three `from_over_into` allowances are removal candidates. Replacing each `Into<T>` implementation
with `From<Source> for T` preserves `.into()` call sites through the standard library's blanket
implementation and satisfies Clippy without an API regression. They are classified before their
focused remediation batch.

The `extra_unused_lifetimes` allowances are generated by `async_trait` in traits also processed by
`automock`; their existing nearby comments provide the required evidence. Remaining naming
allowances are retained only when the repeated term describes an essential protocol, lifecycle,
transport, repository, or generated-code role. Their remediation batch must add native, specific
`reason` parameters and missing nearby rationale comments.

### Intentional Executable and CLI Output

All allowances in this category are retained. The affected binaries expose command-line behavior:
they report results on standard output, failures on standard error, or terminate with an explicit
process status. The console-client library contains the shared implementation of that output
contract. The next remediation batch must add a native `reason` parameter to every retained
attribute; no behavior change is appropriate merely to avoid these lints.

### Async Signature and Lifecycle Boundaries

The `async_yields_async` allowances are retained for two-phase server startup. Each function awaits
listener startup and service registration so it can return startup errors synchronously to its
caller, then returns the long-running cancellation-aware component future to the job manager.
Collapsing this boundary solely to satisfy the lint would obscure the startup versus runtime error
contract.

The `manual_async_fn` allowances are retained for Axum's `FromRequestParts` trait implementations.
Their explicit `impl Future + Send` return type is required by the trait's Send-capable extractor
contract. The `unused_async` handler allowances are retained because the router-facing handlers
must preserve their asynchronous Axum handler signature while delegating work to shared helpers.
The benchmark helper remains async to preserve a uniform asynchronous benchmark call shape.
The remediation batch must add native, specific `reason` parameters and nearby rationale comments
where absent.

The next pass covers naming allowances. Retain them only where the repeated term describes an
essential protocol, lifecycle-state, transport, repository, or macro-generated role. The
`extra_unused_lifetimes` allowances are generated by `async_trait` in traits also processed by
`automock`; their existing nearby comments provide the required evidence.

## Entries

| ID | Source location | Scope | Lint name(s) | Rationale category | Evidence | Owner | Disposition |
| -- | --------------- | ----- | ------------ | ------------------ | -------- | ----- | ----------- |
| A001 | `src/bin/http_health_check.rs:1` | crate | `print_stdout`, `print_stderr`, `exit` | Health-check CLI contract | Container health-check binary reports probe outcomes and exit status to its caller | #2158 | Retain |
| A002 | `src/bootstrap/jobs/health_check_api.rs:58` | item | `async_yields_async` | Two-phase server lifecycle | Awaits startup and registration errors before returning the cancellation-aware runtime component future | #2158 | Retain |
| A003 | `src/bootstrap/jobs/http_tracker.rs:84` | item | `async_yields_async` | Two-phase server lifecycle | Awaits listener startup errors before returning the cancellation-aware HTTP runtime component future | #2158 | Retain |
| A004 | `src/bootstrap/jobs/tracker_apis.rs:110` | item | `async_yields_async` | Two-phase server lifecycle | Awaits listener startup errors before returning the cancellation-aware REST API runtime component future | #2158 | Retain |
| A005 | `src/bootstrap/jobs/udp_tracker.rs:44` | item | `async_yields_async` | Two-phase server lifecycle | Awaits listener startup errors before returning the cancellation-aware UDP runtime component future | #2158 | Retain |
| A006 | `src/console/ci/qbittorrent_e2e/qbittorrent/mod.rs:5` | crate | `redundant_pub_crate` | Pending review | Source inspection required | #2158 | Pending |
| A007 | `src/console/ci/qbittorrent_e2e/tracker/mod.rs:5` | crate | `redundant_pub_crate` | Pending review | Source inspection required | #2158 | Pending |
| A008 | `src/console/ci/qbittorrent_e2e/types/mod.rs:8` | crate | `redundant_pub_crate` | Pending review | Source inspection required | #2158 | Pending |
| A009 | `src/console/profiling.rs:1` | crate | `print_stdout`, `print_stderr` | Profiling CLI contract | Profiling console command reports status and failures to the terminal | #2158 | Retain |
| A010 | `src/main.rs:112` | item | `print_stderr` | Startup failure reporting | Process entry point intentionally reports a user-facing startup failure on standard error | #2158 | Retain |
| A011 | `packages/axum-http-server/src/server.rs:64` | item | `redundant_field_names` | Proc-macro expansion | Existing nearby comment: MSRV-compatible `derive_more::Constructor` emits `field: field`; remove when it emits shorthand | #2158 | Retain |
| A012 | `packages/axum-http-server/src/server.rs:183` | item | `module_name_repetitions` | Lifecycle-state API name | `StoppedHttpServer` distinguishes the HTTP server state-controller alias from other server states | #2158 | Retain |
| A013 | `packages/axum-http-server/src/server.rs:187` | item | `module_name_repetitions` | Lifecycle-state API name | `RunningHttpServer` distinguishes the HTTP server state-controller alias from other server states | #2158 | Retain |
| A014 | `packages/axum-http-server/src/server.rs:205` | item | `module_name_repetitions` | Protocol-specific public type | `HttpServer` identifies the HTTP server controller among workspace server types | #2158 | Retain |
| A015 | `packages/axum-http-server/src/v1/extractors/authentication_key.rs:75` | item | `manual_async_fn` | Axum extractor trait contract | `FromRequestParts` implementation exposes the trait-required Send-capable future explicitly | #2158 | Retain |
| A016 | `packages/axum-http-server/src/v1/extractors/client_ip_sources.rs:56` | item | `manual_async_fn` | Axum extractor trait contract | `FromRequestParts` implementation exposes the trait-required Send-capable future explicitly | #2158 | Retain |
| A017 | `packages/axum-http-server/src/v1/handlers/announce.rs:24` | item | `unused_async` | Axum route-handler signature | Public router handler preserves its asynchronous signature while delegating to shared announce logic | #2158 | Retain |
| A018 | `packages/axum-http-server/src/v1/handlers/announce.rs:37` | item | `unused_async` | Axum route-handler signature | Public router handler preserves its asynchronous signature while delegating to shared announce logic | #2158 | Retain |
| A019 | `packages/axum-http-server/src/v1/handlers/health_check.rs:3` | item | `unused_async` | Axum route-handler signature | Router handler preserves a uniform asynchronous handler signature | #2158 | Retain |
| A020 | `packages/axum-http-server/src/v1/handlers/scrape.rs:24` | item | `unused_async` | Axum route-handler signature | Public router handler preserves its asynchronous signature while delegating to shared scrape logic | #2158 | Retain |
| A021 | `packages/axum-http-server/src/v1/handlers/scrape.rs:39` | item | `unused_async` | Axum route-handler signature | Public router handler preserves its asynchronous signature while delegating to shared scrape logic | #2158 | Retain |
| A022 | `packages/axum-rest-api-server/src/server.rs:75` | item | `module_name_repetitions` | Lifecycle-state API name | `StoppedApiServer` distinguishes the REST API server state-controller alias from other server states | #2158 | Retain |
| A023 | `packages/axum-rest-api-server/src/server.rs:79` | item | `module_name_repetitions` | Lifecycle-state API name | `RunningApiServer` distinguishes the REST API server state-controller alias from other server states | #2158 | Retain |
| A024 | `packages/axum-rest-api-server/src/server.rs:87` | item | `module_name_repetitions` | REST API public type | `ApiServer` identifies the management REST API server controller among workspace server types | #2158 | Retain |
| A025 | `packages/axum-rest-api-server/src/server.rs:240` | item | `redundant_field_names` | Proc-macro expansion | Existing nearby comment: MSRV-compatible `derive_more::Constructor` emits `field: field`; remove when it emits shorthand | #2158 | Retain |
| A026 | `packages/configuration/src/lib.rs:164` | item | `needless_pass_by_value` | Pending review | Source inspection required | #2158 | Pending |
| A027 | `packages/configuration/src/lib.rs:179` | item | `needless_pass_by_value` | Pending review | Source inspection required | #2158 | Pending |
| A028 | `packages/configuration/src/lib.rs:331` | item | `result_large_err` | Pending review | Source inspection required | #2158 | Pending |
| A029 | `packages/configuration/src/lib.rs:352` | item | `result_large_err` | Pending review | Source inspection required | #2158 | Pending |
| A030 | `packages/configuration/src/lib.rs:370` | item | `result_large_err` | Pending review | Source inspection required | #2158 | Pending |
| A031 | `packages/configuration/src/lib.rs:390` | item | `result_large_err` | Pending review | Source inspection required | #2158 | Pending |
| A032 | `packages/configuration/src/lib.rs:409` | item | `result_large_err` | Pending review | Source inspection required | #2158 | Pending |
| A033 | `packages/configuration/src/lib.rs:433` | item | `result_large_err` | Pending review | Source inspection required | #2158 | Pending |
| A034 | `packages/configuration/src/lib.rs:454` | item | `result_large_err` | Pending review | Source inspection required | #2158 | Pending |
| A035 | `packages/configuration/src/lib.rs:476` | item | `result_large_err` | Pending review | Source inspection required | #2158 | Pending |
| A036 | `packages/configuration/src/lib.rs:518` | item | `result_large_err` | Pending review | Source inspection required | #2158 | Pending |
| A037 | `packages/configuration/src/lib.rs:537` | item | `result_large_err` | Pending review | Source inspection required | #2158 | Pending |
| A038 | `packages/configuration/src/lib.rs:560` | item | `result_large_err` | Pending review | Source inspection required | #2158 | Pending |
| A039 | `packages/configuration/src/lib.rs:594` | item | `result_large_err` | Pending review | Source inspection required | #2158 | Pending |
| A040 | `packages/configuration/src/lib.rs:616` | item | `result_large_err` | Pending review | Source inspection required | #2158 | Pending |
| A041 | `packages/configuration/src/lib.rs:640` | item | `result_large_err` | Pending review | Source inspection required | #2158 | Pending |
| A042 | `packages/configuration/src/lib.rs:665` | item | `result_large_err` | Pending review | Source inspection required | #2158 | Pending |
| A043 | `packages/configuration/src/lib.rs:686` | item | `result_large_err` | Pending review | Source inspection required | #2158 | Pending |
| A044 | `packages/configuration/src/lib.rs:709` | item | `result_large_err` | Pending review | Source inspection required | #2158 | Pending |
| A045 | `packages/configuration/src/lib.rs:734` | item | `result_large_err` | Pending review | Source inspection required | #2158 | Pending |
| A046 | `packages/configuration/src/lib.rs:844` | item | `unnecessary_wraps` | Pending review | Source inspection required | #2158 | Pending |
| A047 | `packages/configuration/src/lib.rs:848` | item | `unnecessary_wraps` | Pending review | Source inspection required | #2158 | Pending |
| A048 | `packages/configuration/src/v2_0_0/core.rs:8` | item | `struct_excessive_bools` | Pending review | Source inspection required | #2158 | Pending |
| A049 | `packages/configuration/src/v2_0_0/database.rs:4` | item | `struct_excessive_bools` | Pending review | Source inspection required | #2158 | Pending |
| A050 | `packages/configuration/src/v2_0_0/logging.rs:11` | item | `struct_excessive_bools` | Pending review | Source inspection required | #2158 | Pending |
| A051 | `packages/configuration/src/v2_0_0/mod.rs:559` | item | `result_large_err` | Pending review | Source inspection required | #2158 | Pending |
| A052 | `packages/configuration/src/v2_0_0/mod.rs:595` | item | `result_large_err` | Pending review | Source inspection required | #2158 | Pending |
| A053 | `packages/configuration/src/v2_0_0/mod.rs:629` | item | `result_large_err` | Pending review | Source inspection required | #2158 | Pending |
| A054 | `packages/configuration/src/v2_0_0/mod.rs:663` | item | `result_large_err` | Pending review | Source inspection required | #2158 | Pending |
| A055 | `packages/configuration/src/v2_0_0/mod.rs:697` | item | `result_large_err` | Pending review | Source inspection required | #2158 | Pending |
| A056 | `packages/configuration/src/v2_0_0/mod.rs:785` | item | `result_large_err` | Pending review | Source inspection required | #2158 | Pending |
| A057 | `packages/configuration/src/v2_0_0/mod.rs:824` | item | `result_large_err` | Pending review | Source inspection required | #2158 | Pending |
| A058 | `packages/configuration/src/v2_0_0/mod.rs:860` | item | `result_large_err` | Pending review | Source inspection required | #2158 | Pending |
| A059 | `packages/configuration/src/v2_0_0/tracker_api.rs:50` | item | `unnecessary_wraps` | Pending review | Source inspection required | #2158 | Pending |
| A060 | `packages/configuration/src/v3_0_0/core.rs:11` | item | `struct_excessive_bools` | Pending review | Source inspection required | #2158 | Pending |
| A061 | `packages/configuration/src/v3_0_0/logging.rs:14` | item | `struct_excessive_bools` | Pending review | Source inspection required | #2158 | Pending |
| A062 | `packages/configuration/src/v3_0_0/mod.rs:633` | item | `result_large_err` | Pending review | Source inspection required | #2158 | Pending |
| A063 | `packages/configuration/src/v3_0_0/mod.rs:673` | item | `result_large_err` | Pending review | Source inspection required | #2158 | Pending |
| A064 | `packages/configuration/src/v3_0_0/mod.rs:708` | item | `result_large_err` | Pending review | Source inspection required | #2158 | Pending |
| A065 | `packages/configuration/src/v3_0_0/mod.rs:748` | item | `result_large_err` | Pending review | Source inspection required | #2158 | Pending |
| A066 | `packages/configuration/src/v3_0_0/mod.rs:808` | item | `result_large_err` | Pending review | Source inspection required | #2158 | Pending |
| A067 | `packages/configuration/src/v3_0_0/mod.rs:844` | item | `result_large_err` | Pending review | Source inspection required | #2158 | Pending |
| A068 | `packages/configuration/src/v3_0_0/mod.rs:878` | item | `result_large_err` | Pending review | Source inspection required | #2158 | Pending |
| A069 | `packages/configuration/src/v3_0_0/mod.rs:917` | item | `result_large_err` | Pending review | Source inspection required | #2158 | Pending |
| A070 | `packages/configuration/src/v3_0_0/mod.rs:958` | item | `result_large_err` | Pending review | Source inspection required | #2158 | Pending |
| A071 | `packages/configuration/src/v3_0_0/mod.rs:1008` | item | `result_large_err` | Pending review | Source inspection required | #2158 | Pending |
| A072 | `packages/configuration/src/v3_0_0/mod.rs:1163` | item | `result_large_err` | Pending review | Source inspection required | #2158 | Pending |
| A073 | `packages/configuration/src/v3_0_0/mod.rs:1209` | item | `result_large_err` | Pending review | Source inspection required | #2158 | Pending |
| A074 | `packages/configuration/src/v3_0_0/mod.rs:1255` | item | `result_large_err` | Pending review | Source inspection required | #2158 | Pending |
| A075 | `packages/configuration/src/v3_0_0/mod.rs:1299` | item | `result_large_err` | Pending review | Source inspection required | #2158 | Pending |
| A076 | `packages/configuration/src/v3_0_0/mod.rs:1334` | item | `result_large_err` | Pending review | Source inspection required | #2158 | Pending |
| A077 | `packages/configuration/src/v3_0_0/mod.rs:1370` | item | `result_large_err` | Pending review | Source inspection required | #2158 | Pending |
| A078 | `packages/configuration/src/v3_0_0/mod.rs:1411` | item | `result_large_err` | Pending review | Source inspection required | #2158 | Pending |
| A079 | `packages/configuration/src/v3_0_0/mod.rs:1441` | item | `result_large_err` | Pending review | Source inspection required | #2158 | Pending |
| A080 | `packages/http-core/src/statistics/metrics.rs:48` | item | `cast_sign_loss` | Pending review | Source inspection required | #2158 | Pending |
| A081 | `packages/http-core/src/statistics/metrics.rs:49` | item | `cast_possible_truncation` | Pending review | Source inspection required | #2158 | Pending |
| A082 | `packages/http-core/src/statistics/metrics.rs:61` | item | `cast_sign_loss` | Pending review | Source inspection required | #2158 | Pending |
| A083 | `packages/http-core/src/statistics/metrics.rs:62` | item | `cast_possible_truncation` | Pending review | Source inspection required | #2158 | Pending |
| A084 | `packages/http-core/src/statistics/metrics.rs:74` | item | `cast_sign_loss` | Pending review | Source inspection required | #2158 | Pending |
| A085 | `packages/http-core/src/statistics/metrics.rs:75` | item | `cast_possible_truncation` | Pending review | Source inspection required | #2158 | Pending |
| A086 | `packages/http-core/src/statistics/metrics.rs:87` | item | `cast_sign_loss` | Pending review | Source inspection required | #2158 | Pending |
| A087 | `packages/http-core/src/statistics/metrics.rs:88` | item | `cast_possible_truncation` | Pending review | Source inspection required | #2158 | Pending |
| A088 | `packages/http-protocol/src/v1/responses/announce/data.rs:17` | item | `redundant_field_names` | Proc-macro expansion | Existing nearby comment: MSRV-compatible `derive_more::Constructor` emits `field: field`; remove when it emits shorthand | #2158 | Retain |
| A089 | `packages/http-protocol/src/v1/responses/announce/data.rs:28` | item | `redundant_field_names` | Proc-macro expansion | Existing nearby comment: MSRV-compatible `derive_more::Constructor` emits `field: field`; remove when it emits shorthand | #2158 | Retain |
| A090 | `packages/http-protocol/src/v1/responses/announce/deserialization.rs:82` | item | `chunks_exact_to_as_chunks`, `explicit_iter_loop` | Pending review | Source inspection required | #2158 | Pending |
| A091 | `packages/http-protocol/src/v1/responses/announce/encoding.rs:32` | item | `redundant_field_names` | Proc-macro expansion | Existing nearby comment: MSRV-compatible `derive_more::Constructor` emits `field: field`; remove when it emits shorthand | #2158 | Retain |
| A092 | `packages/http-protocol/src/v1/responses/announce/encoding.rs:68` | item | `from_over_into` | Standard conversion trait | Replace `Into<Vec<u8>> for Normal` with `From<Normal> for Vec<u8>`; the standard blanket implementation preserves `.into()` callers | #2158 | Remove |
| A093 | `packages/http-protocol/src/v1/responses/announce/encoding.rs:117` | item | `from_over_into` | Standard conversion trait | Replace `Into<Vec<u8>> for Compact` with `From<Compact> for Vec<u8>`; the standard blanket implementation preserves `.into()` callers | #2158 | Remove |
| A094 | `packages/persistence-benchmark/src/bin/persistence_benchmark/runner.rs:1` | crate | `print_stdout` | Benchmark CLI contract | Persistence benchmark runner intentionally emits progress and result output | #2158 | Retain |
| A095 | `packages/primitives/src/announce.rs:15` | item | `redundant_field_names` | Proc-macro expansion | Existing nearby comment: MSRV-compatible `derive_more::Constructor` emits `field: field`; remove when it emits shorthand | #2158 | Retain |
| A096 | `packages/primitives/src/announce.rs:87` | item | `redundant_field_names` | Proc-macro expansion | Existing nearby comment: MSRV-compatible `derive_more::Constructor` emits `field: field`; remove when it emits shorthand | #2158 | Retain |
| A097 | `packages/primitives/src/mode.rs:12` | item | `redundant_field_names` | Proc-macro expansion | Existing nearby comment: MSRV-compatible `derive_more::Constructor` emits `field: field`; remove when it emits shorthand | #2158 | Retain |
| A098 | `packages/primitives/src/pagination.rs:8` | item | `redundant_field_names` | Proc-macro expansion | Existing nearby comment: MSRV-compatible `derive_more::Constructor` emits `field: field`; remove when it emits shorthand | #2158 | Retain |
| A099 | `packages/primitives/src/peer.rs:141` | item | `cast_possible_truncation` | Pending review | Source inspection required | #2158 | Pending |
| A100 | `packages/primitives/src/peer.rs:504` | item | `derivable_impls` | Pending review | Source inspection required | #2158 | Pending |
| A101 | `packages/primitives/src/policy.rs:12` | item | `redundant_field_names` | Proc-macro expansion | Existing nearby comment: MSRV-compatible `derive_more::Constructor` emits `field: field`; remove when it emits shorthand | #2158 | Retain |
| A102 | `packages/primitives/src/swarm_metadata.rs:15` | item | `redundant_field_names` | Proc-macro expansion | Existing nearby comment: MSRV-compatible `derive_more::Constructor` emits `field: field`; remove when it emits shorthand | #2158 | Retain |
| A103 | `packages/rest-api-application/src/v1/ports/auth_key.rs:16` | item | `double_must_use` | Pending review | Source inspection required | #2158 | Pending |
| A104 | `packages/rest-api-application/src/v1/ports/stats.rs:15` | item | `double_must_use` | Pending review | Source inspection required | #2158 | Pending |
| A105 | `packages/rest-api-application/src/v1/ports/torrent.rs:13` | item | `double_must_use` | Pending review | Source inspection required | #2158 | Pending |
| A106 | `packages/rest-api-application/src/v1/ports/whitelist.rs:16` | item | `double_must_use` | Pending review | Source inspection required | #2158 | Pending |
| A107 | `packages/rest-api-client/src/v1/client.rs:226` | item | `struct_field_names` | HTTP transport role | Fields distinguish connection configuration, API path, and transport client in the low-level HTTP API client | #2158 | Retain |
| A108 | `packages/rest-api-protocol/src/v1/context/torrent/resources/peer.rs:22` | item | `doc_markdown` | Pending review | Source inspection required | #2158 | Pending |
| A109 | `packages/rest-api-runtime-adapter/src/v1/adapters/auth_key.rs:86` | item | `needless_pass_by_value` | Pending review | Source inspection required | #2158 | Pending |
| A110 | `packages/rest-api-runtime-adapter/src/v1/adapters/stats.rs:18` | item | `struct_field_names` | Repository role names | Fields name distinct tracker statistics repositories that the adapter aggregates | #2158 | Retain |
| A111 | `packages/rest-api-runtime-adapter/src/v1/adapters/stats.rs:31` | item | `too_many_arguments` | Pending review | Source inspection required | #2158 | Pending |
| A112 | `packages/swarm-coordination-registry/examples/bench_peers.rs:34` | item | `cast_possible_truncation`, `cast_precision_loss`, `cast_sign_loss` | Pending review | Source inspection required | #2158 | Pending |
| A113 | `packages/swarm-coordination-registry/src/statistics/activity_metrics_updater.rs:79` | item | `cast_precision_loss` | Pending review | Source inspection required | #2158 | Pending |
| A114 | `packages/swarm-coordination-registry/src/statistics/activity_metrics_updater.rs:93` | item | `cast_precision_loss` | Pending review | Source inspection required | #2158 | Pending |
| A115 | `packages/swarm-coordination-registry/src/swarm/registry.rs:864` | item | `from_over_into` | Standard conversion trait | Replace `Into<TorrentEntryInfo> for Coordinator` with `From<Coordinator> for TorrentEntryInfo`; the standard blanket implementation preserves `.into()` callers | #2158 | Remove |
| A116 | `packages/test-helpers/src/logging.rs:130` | item | `unnecessary_wraps` | Pending review | Source inspection required | #2158 | Pending |
| A117 | `packages/test-helpers/src/logging.rs:146` | item | `unnecessary_wraps` | Pending review | Source inspection required | #2158 | Pending |
| A118 | `packages/test-helpers/src/logging.rs:147` | item | `unused_self` | Pending review | Source inspection required | #2158 | Pending |
| A119 | `packages/torrent-repository-benchmarking/benches/helpers/utils.rs:20` | item | `missing_panics_doc` | Pending review | Source inspection required | #2158 | Pending |
| A120 | `packages/torrent-repository-benchmarking/benches/helpers/utils.rs:25` | item | `cast_possible_truncation` | Pending review | Source inspection required | #2158 | Pending |
| A121 | `packages/torrent-repository-benchmarking/src/entry/mod.rs:52` | item | `module_name_repetitions` | Benchmark abstraction name | `EntrySync` distinguishes the synchronous entry benchmark abstraction from `EntryAsync` | #2158 | Retain |
| A122 | `packages/torrent-repository-benchmarking/src/entry/mod.rs:64` | item | `module_name_repetitions` | Benchmark abstraction name | `EntryAsync` distinguishes the asynchronous entry benchmark abstraction from `EntrySync` | #2158 | Retain |
| A123 | `packages/torrent-repository-benchmarking/src/entry/single.rs:13` | item | `cast_possible_truncation` | Pending review | Source inspection required | #2158 | Pending |
| A124 | `packages/torrent-repository-benchmarking/src/repository/mod.rs:29` | item | `module_name_repetitions` | Benchmark abstraction name | `RepositoryAsync` distinguishes the asynchronous repository benchmark abstraction | #2158 | Retain |
| A125 | `packages/torrent-repository-benchmarking/src/repository/rw_lock_tokio.rs:18` | item | `future_not_send` | Pending review | Source inspection required | #2158 | Pending |
| A126 | `packages/tracker-client/src/http/client/mod.rs:27` | item | `struct_field_names` | HTTP client role | Fields distinguish the HTTP transport client and tracker base URL | #2158 | Retain |
| A127 | `packages/tracker-client/src/udp/client.rs:18` | item | `module_name_repetitions` | Protocol-specific public type | `UdpClient` identifies the UDP protocol client among tracker client types | #2158 | Retain |
| A128 | `packages/tracker-client/src/udp/client.rs:179` | item | `module_name_repetitions` | Protocol-specific public type | `UdpTrackerClient` distinguishes the tracker-facing wrapper from its underlying UDP client | #2158 | Retain |
| A129 | `packages/tracker-core/src/announce_handler.rs:277` | item | `cast_sign_loss` | Pending review | Source inspection required | #2158 | Pending |
| A130 | `packages/tracker-core/src/databases/driver/sqlite/mod.rs:34` | item | `unnecessary_wraps` | Pending review | Source inspection required | #2158 | Pending |
| A131 | `packages/tracker-core/src/databases/traits/auth_keys.rs:14` | item | `double_must_use` | Pending review | Source inspection required | #2158 | Pending |
| A132 | `packages/tracker-core/src/databases/traits/auth_keys.rs:16` | item | `struct_field_names`, `extra_unused_lifetimes` | Macro-generated trait code | Nearby comment identifies `automock` fields ending in `keys` and `async_trait` lifetimes outside workspace control | #2158 | Retain |
| A133 | `packages/tracker-core/src/databases/traits/schema.rs:13` | item | `double_must_use` | Pending review | Source inspection required | #2158 | Pending |
| A134 | `packages/tracker-core/src/databases/traits/schema.rs:15` | item | `extra_unused_lifetimes` | Macro-generated async trait code | Nearby comment identifies lifetimes generated by `async_trait` outside workspace control | #2158 | Retain |
| A135 | `packages/tracker-core/src/databases/traits/torrent_metrics.rs:18` | item | `double_must_use` | Pending review | Source inspection required | #2158 | Pending |
| A136 | `packages/tracker-core/src/databases/traits/torrent_metrics.rs:20` | item | `extra_unused_lifetimes` | Macro-generated async trait code | Nearby comment identifies lifetimes generated by `async_trait` outside workspace control | #2158 | Retain |
| A137 | `packages/tracker-core/src/databases/traits/whitelist.rs:11` | item | `double_must_use` | Pending review | Source inspection required | #2158 | Pending |
| A138 | `packages/tracker-core/src/databases/traits/whitelist.rs:13` | item | `extra_unused_lifetimes` | Macro-generated async trait code | Nearby comment identifies lifetimes generated by `async_trait` outside workspace control | #2158 | Retain |
| A139 | `packages/tracker-core/src/error.rs:108` | item | `module_name_repetitions` | Domain error type | `PeerKeyError` identifies the error's peer-key domain among tracker-core errors | #2158 | Retain |
| A140 | `packages/udp-core/benches/helpers/sync.rs:13` | item | `unused_async` | Benchmark call-shape consistency | Helper remains async so synchronous and asynchronous benchmark paths share an awaitable call shape | #2158 | Retain |
| A141 | `packages/udp-core/src/crypto/keys.rs:50` | item | `needless_borrow` | Pending review | Source inspection required | #2158 | Pending |
| A142 | `packages/udp-core/src/crypto/keys.rs:73` | item | `needless_borrow` | Pending review | Source inspection required | #2158 | Pending |
| A143 | `packages/udp-core/src/statistics/metrics.rs:49` | item | `cast_sign_loss` | Pending review | Source inspection required | #2158 | Pending |
| A144 | `packages/udp-core/src/statistics/metrics.rs:50` | item | `cast_possible_truncation` | Pending review | Source inspection required | #2158 | Pending |
| A145 | `packages/udp-core/src/statistics/metrics.rs:62` | item | `cast_sign_loss` | Pending review | Source inspection required | #2158 | Pending |
| A146 | `packages/udp-core/src/statistics/metrics.rs:63` | item | `cast_possible_truncation` | Pending review | Source inspection required | #2158 | Pending |
| A147 | `packages/udp-core/src/statistics/metrics.rs:75` | item | `cast_sign_loss` | Pending review | Source inspection required | #2158 | Pending |
| A148 | `packages/udp-core/src/statistics/metrics.rs:76` | item | `cast_possible_truncation` | Pending review | Source inspection required | #2158 | Pending |
| A149 | `packages/udp-core/src/statistics/metrics.rs:88` | item | `cast_sign_loss` | Pending review | Source inspection required | #2158 | Pending |
| A150 | `packages/udp-core/src/statistics/metrics.rs:89` | item | `cast_possible_truncation` | Pending review | Source inspection required | #2158 | Pending |
| A151 | `packages/udp-core/src/statistics/metrics.rs:101` | item | `cast_sign_loss` | Pending review | Source inspection required | #2158 | Pending |
| A152 | `packages/udp-core/src/statistics/metrics.rs:102` | item | `cast_possible_truncation` | Pending review | Source inspection required | #2158 | Pending |
| A153 | `packages/udp-core/src/statistics/metrics.rs:114` | item | `cast_sign_loss` | Pending review | Source inspection required | #2158 | Pending |
| A154 | `packages/udp-core/src/statistics/metrics.rs:115` | item | `cast_possible_truncation` | Pending review | Source inspection required | #2158 | Pending |
| A155 | `packages/udp-core/src/statistics/repository.rs:15` | item | `double_must_use` | Pending review | Source inspection required | #2158 | Pending |
| A156 | `packages/udp-protocol/src/lib.rs:8` | crate | `cast_possible_truncation` | Pending review | Source inspection required | #2158 | Pending |
| A157 | `packages/udp-protocol/src/lib.rs:9` | crate | `default_trait_access` | Pending review | Source inspection required | #2158 | Pending |
| A158 | `packages/udp-protocol/src/lib.rs:10` | crate | `doc_markdown` | Pending review | Source inspection required | #2158 | Pending |
| A159 | `packages/udp-protocol/src/lib.rs:14` | crate | `empty_enums` | Pending review | Source inspection required | #2158 | Pending |
| A160 | `packages/udp-protocol/src/lib.rs:15` | crate | `explicit_iter_loop` | Pending review | Source inspection required | #2158 | Pending |
| A161 | `packages/udp-protocol/src/lib.rs:16` | crate | `legacy_numeric_constants` | Pending review | Source inspection required | #2158 | Pending |
| A162 | `packages/udp-protocol/src/lib.rs:17` | crate | `match_same_arms` | Pending review | Source inspection required | #2158 | Pending |
| A163 | `packages/udp-protocol/src/lib.rs:18` | crate | `missing_errors_doc` | Pending review | Source inspection required | #2158 | Pending |
| A164 | `packages/udp-protocol/src/lib.rs:19` | crate | `missing_panics_doc` | Pending review | Source inspection required | #2158 | Pending |
| A165 | `packages/udp-protocol/src/lib.rs:20` | crate | `must_use_candidate` | Pending review | Source inspection required | #2158 | Pending |
| A166 | `packages/udp-protocol/src/lib.rs:21` | crate | `needless_pass_by_value` | Pending review | Source inspection required | #2158 | Pending |
| A167 | `packages/udp-protocol/src/lib.rs:22` | crate | `semicolon_if_nothing_returned` | Pending review | Source inspection required | #2158 | Pending |
| A168 | `packages/udp-protocol/src/lib.rs:23` | crate | `wildcard_imports` | Pending review | Source inspection required | #2158 | Pending |
| A169 | `packages/udp-server/examples/udp_only_public_tracker.rs:35` | crate | `print_stdout` | Example executable output | Runnable public-tracker example intentionally prints service information | #2158 | Retain |
| A170 | `packages/udp-server/src/banning/event/handler.rs:34` | item | `cast_precision_loss` | Pending review | Source inspection required | #2158 | Pending |
| A171 | `packages/udp-server/src/handlers/announce.rs:132` | item | `cast_possible_truncation` | Pending review | Source inspection required | #2158 | Pending |
| A172 | `packages/udp-server/src/handlers/error.rs:18` | item | `too_many_arguments` | Pending review | Source inspection required | #2158 | Pending |
| A173 | `packages/udp-server/src/server/mod.rs:54` | item | `module_name_repetitions` | Protocol-specific public type | `Server` is the UDP server module's canonical public state controller | #2158 | Retain |
| A174 | `packages/udp-server/src/server/spawner.rs:31` | item | `redundant_field_names` | Proc-macro expansion | Existing nearby comment: MSRV-compatible `derive_more::Constructor` emits `field: field`; remove when it emits shorthand | #2158 | Retain |
| A175 | `packages/udp-server/src/server/states.rs:33` | item | `module_name_repetitions` | Lifecycle-state API name | `StoppedUdpServer` distinguishes the UDP server state-controller alias from other server states | #2158 | Retain |
| A176 | `packages/udp-server/src/server/states.rs:37` | item | `module_name_repetitions` | Lifecycle-state API name | `RunningUdpServer` distinguishes the UDP server state-controller alias from other server states | #2158 | Retain |
| A177 | `packages/udp-server/src/server/states.rs:51` | item | `redundant_field_names` | Proc-macro expansion | Existing nearby comment: MSRV-compatible `derive_more::Constructor` emits `field: field`; remove when it emits shorthand | #2158 | Retain |
| A178 | `packages/udp-server/src/statistics/metrics.rs:55` | item | `cast_precision_loss` | Pending review | Source inspection required | #2158 | Pending |
| A179 | `packages/udp-server/src/statistics/metrics.rs:86` | item | `cast_sign_loss` | Pending review | Source inspection required | #2158 | Pending |
| A180 | `packages/udp-server/src/statistics/metrics.rs:87` | item | `cast_possible_truncation` | Pending review | Source inspection required | #2158 | Pending |
| A181 | `packages/udp-server/src/statistics/metrics.rs:98` | item | `cast_sign_loss` | Pending review | Source inspection required | #2158 | Pending |
| A182 | `packages/udp-server/src/statistics/metrics.rs:99` | item | `cast_possible_truncation` | Pending review | Source inspection required | #2158 | Pending |
| A183 | `packages/udp-server/src/statistics/metrics.rs:107` | item | `cast_sign_loss` | Pending review | Source inspection required | #2158 | Pending |
| A184 | `packages/udp-server/src/statistics/metrics.rs:108` | item | `cast_possible_truncation` | Pending review | Source inspection required | #2158 | Pending |
| A185 | `packages/udp-server/src/statistics/metrics.rs:152` | item | `cast_sign_loss` | Pending review | Source inspection required | #2158 | Pending |
| A186 | `packages/udp-server/src/statistics/metrics.rs:153` | item | `cast_possible_truncation` | Pending review | Source inspection required | #2158 | Pending |
| A187 | `packages/udp-server/src/statistics/metrics.rs:163` | item | `cast_sign_loss` | Pending review | Source inspection required | #2158 | Pending |
| A188 | `packages/udp-server/src/statistics/metrics.rs:164` | item | `cast_possible_truncation` | Pending review | Source inspection required | #2158 | Pending |
| A189 | `packages/udp-server/src/statistics/metrics.rs:173` | item | `cast_sign_loss` | Pending review | Source inspection required | #2158 | Pending |
| A190 | `packages/udp-server/src/statistics/metrics.rs:174` | item | `cast_possible_truncation` | Pending review | Source inspection required | #2158 | Pending |
| A191 | `packages/udp-server/src/statistics/metrics.rs:183` | item | `cast_sign_loss` | Pending review | Source inspection required | #2158 | Pending |
| A192 | `packages/udp-server/src/statistics/metrics.rs:184` | item | `cast_possible_truncation` | Pending review | Source inspection required | #2158 | Pending |
| A193 | `packages/udp-server/src/statistics/metrics.rs:194` | item | `cast_sign_loss` | Pending review | Source inspection required | #2158 | Pending |
| A194 | `packages/udp-server/src/statistics/metrics.rs:195` | item | `cast_possible_truncation` | Pending review | Source inspection required | #2158 | Pending |
| A195 | `packages/udp-server/src/statistics/metrics.rs:208` | item | `cast_sign_loss` | Pending review | Source inspection required | #2158 | Pending |
| A196 | `packages/udp-server/src/statistics/metrics.rs:209` | item | `cast_possible_truncation` | Pending review | Source inspection required | #2158 | Pending |
| A197 | `packages/udp-server/src/statistics/metrics.rs:222` | item | `cast_sign_loss` | Pending review | Source inspection required | #2158 | Pending |
| A198 | `packages/udp-server/src/statistics/metrics.rs:223` | item | `cast_possible_truncation` | Pending review | Source inspection required | #2158 | Pending |
| A199 | `packages/udp-server/src/statistics/metrics.rs:236` | item | `cast_sign_loss` | Pending review | Source inspection required | #2158 | Pending |
| A200 | `packages/udp-server/src/statistics/metrics.rs:237` | item | `cast_possible_truncation` | Pending review | Source inspection required | #2158 | Pending |
| A201 | `packages/udp-server/src/statistics/metrics.rs:249` | item | `cast_sign_loss` | Pending review | Source inspection required | #2158 | Pending |
| A202 | `packages/udp-server/src/statistics/metrics.rs:250` | item | `cast_possible_truncation` | Pending review | Source inspection required | #2158 | Pending |
| A203 | `packages/udp-server/src/statistics/metrics.rs:262` | item | `cast_sign_loss` | Pending review | Source inspection required | #2158 | Pending |
| A204 | `packages/udp-server/src/statistics/metrics.rs:263` | item | `cast_possible_truncation` | Pending review | Source inspection required | #2158 | Pending |
| A205 | `packages/udp-server/src/statistics/metrics.rs:275` | item | `cast_sign_loss` | Pending review | Source inspection required | #2158 | Pending |
| A206 | `packages/udp-server/src/statistics/metrics.rs:276` | item | `cast_possible_truncation` | Pending review | Source inspection required | #2158 | Pending |
| A207 | `packages/udp-server/src/statistics/metrics.rs:288` | item | `cast_sign_loss` | Pending review | Source inspection required | #2158 | Pending |
| A208 | `packages/udp-server/src/statistics/metrics.rs:289` | item | `cast_possible_truncation` | Pending review | Source inspection required | #2158 | Pending |
| A209 | `packages/udp-server/src/statistics/metrics.rs:301` | item | `cast_sign_loss` | Pending review | Source inspection required | #2158 | Pending |
| A210 | `packages/udp-server/src/statistics/metrics.rs:302` | item | `cast_possible_truncation` | Pending review | Source inspection required | #2158 | Pending |
| A211 | `packages/udp-server/src/statistics/metrics.rs:315` | item | `cast_sign_loss` | Pending review | Source inspection required | #2158 | Pending |
| A212 | `packages/udp-server/src/statistics/metrics.rs:316` | item | `cast_possible_truncation` | Pending review | Source inspection required | #2158 | Pending |
| A213 | `packages/udp-server/src/statistics/metrics.rs:328` | item | `cast_sign_loss` | Pending review | Source inspection required | #2158 | Pending |
| A214 | `packages/udp-server/src/statistics/metrics.rs:329` | item | `cast_possible_truncation` | Pending review | Source inspection required | #2158 | Pending |
| A215 | `packages/udp-server/src/statistics/metrics.rs:341` | item | `cast_sign_loss` | Pending review | Source inspection required | #2158 | Pending |
| A216 | `packages/udp-server/src/statistics/metrics.rs:342` | item | `cast_possible_truncation` | Pending review | Source inspection required | #2158 | Pending |
| A217 | `packages/udp-server/src/statistics/metrics.rs:354` | item | `cast_sign_loss` | Pending review | Source inspection required | #2158 | Pending |
| A218 | `packages/udp-server/src/statistics/metrics.rs:355` | item | `cast_possible_truncation` | Pending review | Source inspection required | #2158 | Pending |
| A219 | `packages/udp-server/src/statistics/metrics.rs:367` | item | `cast_sign_loss` | Pending review | Source inspection required | #2158 | Pending |
| A220 | `packages/udp-server/src/statistics/metrics.rs:368` | item | `cast_possible_truncation` | Pending review | Source inspection required | #2158 | Pending |
| A221 | `packages/udp-server/src/statistics/metrics.rs:380` | item | `cast_sign_loss` | Pending review | Source inspection required | #2158 | Pending |
| A222 | `packages/udp-server/src/statistics/metrics.rs:381` | item | `cast_possible_truncation` | Pending review | Source inspection required | #2158 | Pending |
| A223 | `packages/udp-server/src/statistics/repository.rs:16` | item | `double_must_use` | Pending review | Source inspection required | #2158 | Pending |
| A224 | `packages/udp-server/src/statistics/repository.rs:330` | item | `cast_sign_loss` | Pending review | Source inspection required | #2158 | Pending |
| A225 | `packages/udp-server/src/statistics/repository.rs:332` | item | `cast_possible_truncation` | Pending review | Source inspection required | #2158 | Pending |
| A226 | `packages/udp-server/src/statistics/repository.rs:340` | item | `cast_sign_loss` | Pending review | Source inspection required | #2158 | Pending |
| A227 | `packages/udp-server/src/statistics/repository.rs:342` | item | `cast_possible_truncation` | Pending review | Source inspection required | #2158 | Pending |
| A228 | `console/tracker-client/src/bin/http_tracker_client.rs:1` | crate | `print_stderr` | HTTP tracker CLI contract | HTTP tracker client intentionally reports command failures on standard error | #2158 | Retain |
| A229 | `console/tracker-client/src/bin/tracker_checker.rs:1` | crate | `print_stderr`, `exit` | Tracker checker CLI contract | Tracker checker intentionally reports failures and returns explicit process status | #2158 | Retain |
| A230 | `console/tracker-client/src/bin/tracker_client.rs:1` | crate | `print_stderr`, `exit` | Unified tracker CLI contract | Unified tracker client intentionally reports failures and returns explicit process status | #2158 | Retain |
| A231 | `console/tracker-client/src/bin/udp_tracker_client.rs:1` | crate | `print_stderr` | UDP tracker CLI contract | UDP tracker client intentionally reports command failures on standard error | #2158 | Retain |
| A232 | `console/tracker-client/src/console/clients/checker/checks/udp.rs:26` | item | `missing_panics_doc` | Pending review | Source inspection required | #2158 | Pending |
| A233 | `console/tracker-client/src/console/clients/udp/responses/json.rs:5` | item | `module_name_repetitions` | Serialization extension trait | `ToJson` is the serialization extension-trait name in the UDP response JSON module | #2158 | Retain |
| A234 | `console/tracker-client/src/lib.rs:5` | crate | `print_stdout`, `print_stderr` | Shared console output contract | Library modules implement terminal output invoked by the console binary targets | #2158 | Retain |
