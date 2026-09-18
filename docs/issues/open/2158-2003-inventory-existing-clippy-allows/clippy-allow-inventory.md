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
After direct-removal remediation, the anchored scan finds 219 remaining attributes, matching the
original 234 rows minus the 15 entries classified as **Remove**. Final source-location
reconciliation remains part of #2158's T6 validation pass.

Current-source reconciliation found two additional maintained-source attributes that were not
present in the recovered 2026-09-15 inventory: A235 and A236. A235 was removed in #2158 after a
focused repair of the benchmarking repository style and lock-scope diagnostics it exposed. A236 is
retained with an existing native reason because it converts a small non-negative collaboration-test
gauge value back to `usize` for assertions.

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

### Numeric-Domain Conversions

Numeric conversion work is provisionally grouped by invariant in the issue-local drafts below.
They are review inputs, not approved GitHub issue specifications. The maintainer will decide after
the complete inventory classification whether to resolve entries locally, promote a draft into a
standalone issue specification, or create a numeric-conversion EPIC that owns the cross-package
work.

| Candidate entries | Provisional disposition | Evidence and removal condition |
| ----------------- | ----------------------- | ------------------------------ |
| A080-A087, A113-A114, A143-A154, A170, A178-A222, A224-A227 | Temporary | [`metric-aggregate-conversion-safety.md`](numeric-conversion-follow-up-drafts/metric-aggregate-conversion-safety.md): establish typed or checked metric boundaries for negative, non-finite, fractional, and out-of-range `f64` values, then remove direct casts and allows. |
| A171 | Temporary | [`wire-numeric-conversion-validation.md`](numeric-conversion-follow-up-drafts/wire-numeric-conversion-validation.md): prove protocol-width bounds through types or checked conversions, then remove/narrow the allow. |
| A099, A123, A129 | Temporary | [`domain-numeric-conversion-contracts.md`](numeric-conversion-follow-up-drafts/domain-numeric-conversion-contracts.md): prove that each conversion is lossless or use deterministic checked conversions with boundary tests, then remove the allows. |
| A112, A120 | Retain | Benchmark-only bounds and precision needs are documented locally: generated peer values fit their target widths, and a short benchmark duration is well within the `f64` exact-integer range; the info-hash generator intentionally retains only four low-order bytes. Add native reasons in the remediation pass. |

The remaining numeric entries listed in the first three rows are intentionally not individually
marked final until the maintainer re-evaluates these drafts. They have concrete removal conditions,
but no follow-up GitHub issue will be created from them without explicit approval.

### UDP Protocol Crate Baseline

The UDP protocol crate has thirteen broad allowances inherited with its vendored foundation. They
cover separate concerns and cannot receive a shared retained rationale. Keep them temporary under
the issue-local [`udp-protocol-clippy-baseline-draft.md`](udp-protocol-clippy-baseline-draft.md)
until the maintainer re-evaluates the complete inventory. The follow-up must either remove each
allow or relocate it to the narrowest source scope with a native `reason` and concrete evidence.
The existing `empty_enums` comment is a preliminary source-specific constraint for macro-generated
wire-type helpers; it is not evidence for the other twelve lints.

### Type, Callable, Visibility, and Import Contracts

The three qBittorrent E2E private modules intentionally retain explicit `pub(crate)` declarations
as visibility documentation during their staged migration. Both versioned `Core` configuration
schemas retain three independently serialized public feature switches (`listed`, `private`, and
`tracker_usage_statistics`); grouping them would make the on-disk schema less direct. The matching
`Database` and `Logging` types have no excessive boolean fields, so their suppressions are stale.

`TrackerStatsAdapter::new` is invoked once at the REST route composition boundary and names six
separate metric repositories plus one configuration flag. A parameter object would only obscure
that wiring, so retain the narrow constructor exception. Similarly, `handle_error` is the UDP
request-dispatch boundary where each request, server, event, and response input must remain
visible; retain its narrow exception. The compact-peer parser, fixture `PeerBuilder` default, and
seed-return expressions have direct behavior-preserving Clippy fixes and are removal candidates.

### Trait and Public API Ergonomics

`async_trait` applies `must_use` to generated futures, while the compiler and Clippy also infer
that those futures are must-use. The resulting `double_must_use` reports occur in macro expansion,
outside the workspace's direct control. Retain these narrow trait-level allowances and add native
`reason` parameters during remediation.

The `result_large_err` allowances are all on configuration tests using `figment::Jail::expect_with`.
That framework requires each callback to return its `figment::error::Result<()>`, so its possibly
large error type is imposed by the test harness rather than returned by a tracker production API.
Retain the test-only suppressions and add native reasons during remediation.

The remaining trait/API cases need separate source-level remediation decisions. Do not infer that
the crate-level `udp-protocol` allowances are justified from this category's item-level evidence.

### Documentation and Panic Contracts

The deprecated REST API field has a fixed public compatibility name, so the prose must repeat that
identifier verbatim; retain its narrow `doc_markdown` allowance. The benchmark info-hash generator
and UDP checker each contain deliberate `unwrap` calls but lack a `# Panics` section. Add focused
documentation and remove their suppressions. The UDP protocol crate-level documentation allowances
remain for its dedicated baseline review rather than being inferred from these item-level cases.

Configuration construction takes ownership of the default path because it is forwarded to the
selected `ConfigTomlSource` variant. The REST adapter consumes `PeerKey` while constructing its
owned `AuthKey` DTO. Retain these ownership-preserving APIs. `CircularBuffer` implements the
standard `Write` contract, whose `write` and `flush` methods must return `io::Result`; retain those
allows. `Sqlite::new` retains a `Result` for driver API symmetry and planned fallibility, as its
existing comment explains. The three configuration default functions do not wrap a result or option
in their explicit return type, so their `unnecessary_wraps` attributes are stale removal candidates.

`RwLockTokio::write` exposes Tokio's non-`Send` write guard by design, so the future cannot be
`Send` without changing the lock contract. Retain that narrow allowance. The crate-level
`must_use_candidate` and `needless_pass_by_value` allowances remain pending the separate UDP
protocol crate-baseline review.

The next pass covers naming allowances. Retain them only where the repeated term describes an
essential protocol, lifecycle-state, transport, repository, or macro-generated role. The
`extra_unused_lifetimes` allowances are generated by `async_trait` in traits also processed by
`automock`; their existing nearby comments provide the required evidence.

### Direct Removal Implementation Evidence

| Entries | Remediation | Validation |
| ------- | ----------- | ---------- |
| A046, A047, A049, A050, A059, A061 | Removed stale configuration suppressions whose target items no longer trigger the recorded lints. | `cargo clippy -p torrust-tracker-configuration --all-targets --all-features -- -D warnings` |
| A090, A092, A093 | Replaced compact-peer parsing suppressions with fixed-size chunk parsing and replaced announce encoder `Into<Vec<u8>>` implementations with standard `From` implementations. | `cargo clippy -p torrust-tracker-http-protocol --all-targets --all-features -- -D warnings` |
| A100 | Derived `Default` for the fixture `PeerBuilder` instead of keeping a manual implementation. | `cargo clippy -p torrust-tracker-primitives --all-targets --all-features -- -D warnings` |
| A115 | Replaced the swarm registry test DTO `Into<TorrentEntryInfo>` implementation with `From<Coordinator> for TorrentEntryInfo`. | `cargo clippy -p torrust-tracker-swarm-coordination-registry --all-targets --all-features -- -D warnings` |
| A119 | Added the missing benchmark panic documentation for duplicate truncated info-hash values. | `cargo clippy -p torrust-tracker-torrent-repository-benchmarking --all-targets --all-features -- -D warnings` |
| A141, A142 | Removed seed-reference suppressions by making the zeroed test seed a `static`, matching the production `LazyLock` reference shape. | `cargo clippy -p torrust-tracker-udp-core --all-targets --all-features -- -D warnings` |
| A232 | Added the missing UDP checker panic documentation for the fixed sample info-hash literal. | `cargo clippy -p torrust-tracker-client --all-targets --all-features -- -D warnings` |

### Retained Rationale Implementation Evidence

| Entries | Remediation | Validation |
| ------- | ----------- | ---------- |
| A001, A009, A010, A094, A169, A228, A229, A230, A231, A234 | Added native `reason` parameters for retained CLI and executable-output suppressions. | `cargo clippy -p torrust-tracker -p torrust-tracker-persistence-benchmark -p torrust-tracker-udp-server -p torrust-tracker-client --all-targets --all-features -- -D warnings` |
| A011, A025, A088, A089, A091, A095, A096, A097, A098, A101, A102, A174, A177 | Added native `reason` parameters for retained `derive_more::Constructor` macro-expansion suppressions. | `cargo clippy -p torrust-tracker-axum-http-server -p torrust-tracker-axum-rest-api-server -p torrust-tracker-http-protocol -p torrust-tracker-primitives -p torrust-tracker-udp-server --all-targets --all-features -- -D warnings` |
| A002, A003, A004, A005, A015, A016, A017, A018, A019, A020, A021 | Added native `reason` parameters for retained async lifecycle, Axum extractor, and Axum handler suppressions. | `cargo clippy -p torrust-tracker -p torrust-tracker-axum-http-server --all-targets --all-features -- -D warnings` |
| A006, A007, A008 | Added native `reason` parameters for retained qBittorrent E2E staged-visibility suppressions. | `cargo clippy -p torrust-tracker --all-targets --all-features -- -D warnings` |
| A026, A027, A048, A060 | Added native `reason` parameters for retained configuration ownership and serialized feature-switch suppressions. | `cargo clippy -p torrust-tracker-configuration --all-targets --all-features -- -D warnings` |
| A028-A045, A051-A058, A062-A079 | Added native `reason` parameters for retained configuration Figment test-callback suppressions. | `cargo clippy -p torrust-tracker-configuration --all-targets --all-features -- -D warnings` |
| A012, A013, A014, A022, A023, A024, A121, A122, A124, A127, A128, A139, A173, A175, A176, A233 | Added native `reason` parameters for retained module-name repetition suppressions that preserve public API and benchmark abstraction names. | `cargo clippy -p torrust-tracker-axum-http-server -p torrust-tracker-axum-rest-api-server -p torrust-tracker-torrent-repository-benchmarking -p torrust-tracker-client -p torrust-tracker-core -p torrust-tracker-udp-server --all-targets --all-features -- -D warnings` |
| A103, A104, A105, A106, A131, A133, A135, A137, A155, A223 | Added native `reason` parameters for retained `async_trait` double-must-use suppressions. | `cargo clippy -p torrust-tracker-rest-api-application -p torrust-tracker-core -p torrust-tracker-udp-core -p torrust-tracker-udp-server --all-targets --all-features -- -D warnings` |
| A107, A110, A126, A132, A134, A136, A138 | Added native `reason` parameters for retained compatibility and API-shape suppressions. | `cargo clippy -p torrust-tracker-rest-api-client -p torrust-tracker-rest-api-runtime-adapter -p torrust-tracker-client-lib -p torrust-tracker-core --all-targets --all-features -- -D warnings` |
| A108, A109, A111, A116, A117, A118, A125, A140, A172 | Added native `reason` parameters for retained API-shape, standard trait, lock, benchmark, and UDP error-boundary suppressions. | `cargo clippy -p torrust-tracker-rest-api-protocol -p torrust-tracker-rest-api-runtime-adapter -p torrust-tracker-test-helpers -p torrust-tracker-torrent-repository-benchmarking -p torrust-tracker-core -p torrust-tracker-udp-core -p torrust-tracker-udp-server --all-targets --all-features -- -D warnings` |
| A112, A120 | Added native `reason` parameters for retained benchmark/test-data numeric suppressions. | `cargo clippy -p torrust-tracker-swarm-coordination-registry -p torrust-tracker-torrent-repository-benchmarking --all-targets --all-features -- -D warnings` |
| A236 | Reconciled a current-source collaboration-test gauge conversion already carrying a native retained reason. | `cargo clippy -p torrust-tracker-swarm-coordination-registry --all-targets --all-features -- -D warnings` |

### Temporary Follow-Up Link Evidence

| Entries | Follow-up | Validation |
| ------- | --------- | ---------- |
| A099, A123, A129 | Added native temporary `reason` parameters linking the domain numeric conversion suppressions to #2246. | `cargo clippy -p torrust-tracker-primitives -p torrust-tracker-torrent-repository-benchmarking -p torrust-tracker-core --all-targets --all-features -- -D warnings` |
| A156, A171 | Added native temporary `reason` parameters linking the wire numeric conversion suppressions to #2245. | `cargo clippy -p torrust-tracker-udp-protocol -p torrust-tracker-udp-server --all-targets --all-features -- -D warnings` |
| A080-A087, A113-A114, A143-A154, A170, A178-A222, A224-A227 | Added native temporary `reason` parameters linking metric aggregate suppressions to #2244. | `cargo clippy -p torrust-tracker-http-core -p torrust-tracker-swarm-coordination-registry -p torrust-tracker-udp-core -p torrust-tracker-udp-server --all-targets --all-features -- -D warnings` |
| A157-A168 | Created follow-up issue #2261 and added native temporary `reason` parameters linking the nonnumeric UDP protocol baseline suppressions to that issue. | `cargo clippy -p torrust-tracker-udp-protocol --all-targets --all-features -- -D warnings` |
| A235 | Removed the crate-level benchmarking style baseline by applying behavior-preserving repository style and lock-scope fixes. | `cargo clippy -p torrust-tracker-torrent-repository-benchmarking --all-targets --all-features -- -D warnings`; `cargo test -p torrust-tracker-torrent-repository-benchmarking --all-targets --all-features`; anchored source scan reports `missing_reason 0` |

## Entries

| ID | Source location | Scope | Lint name(s) | Rationale category | Evidence | Owner | Disposition |
| -- | --------------- | ----- | ------------ | ------------------ | -------- | ----- | ----------- |
| A001 | `src/bin/http_health_check.rs:1` | crate | `print_stdout`, `print_stderr`, `exit` | Health-check CLI contract | Container health-check binary reports probe outcomes and exit status to its caller | #2158 | Retain |
| A002 | `src/bootstrap/jobs/health_check_api.rs:58` | item | `async_yields_async` | Two-phase server lifecycle | Awaits startup and registration errors before returning the cancellation-aware runtime component future | #2158 | Retain |
| A003 | `src/bootstrap/jobs/http_tracker.rs:84` | item | `async_yields_async` | Two-phase server lifecycle | Awaits listener startup errors before returning the cancellation-aware HTTP runtime component future | #2158 | Retain |
| A004 | `src/bootstrap/jobs/tracker_apis.rs:110` | item | `async_yields_async` | Two-phase server lifecycle | Awaits listener startup errors before returning the cancellation-aware REST API runtime component future | #2158 | Retain |
| A005 | `src/bootstrap/jobs/udp_tracker.rs:44` | item | `async_yields_async` | Two-phase server lifecycle | Awaits listener startup errors before returning the cancellation-aware UDP runtime component future | #2158 | Retain |
| A006 | `src/console/ci/qbittorrent_e2e/qbittorrent/mod.rs:5` | crate | `redundant_pub_crate` | Staged private-module visibility | Existing comment: explicit `pub(crate)` documents intended visibility during staged migration | #2158 | Retain |
| A007 | `src/console/ci/qbittorrent_e2e/tracker/mod.rs:5` | crate | `redundant_pub_crate` | Staged private-module visibility | Existing comment: explicit `pub(crate)` documents intended visibility during staged migration | #2158 | Retain |
| A008 | `src/console/ci/qbittorrent_e2e/types/mod.rs:8` | crate | `redundant_pub_crate` | Staged private-module visibility | Existing comment: explicit `pub(crate)` documents intended visibility during staged migration | #2158 | Retain |
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
| A026 | `packages/configuration/src/lib.rs:164` | item | `needless_pass_by_value` | Ownership-preserving constructor API | Owned default path is forwarded into the selected configuration source | #2158 | Retain |
| A027 | `packages/configuration/src/lib.rs:179` | item | `needless_pass_by_value` | Ownership-preserving constructor API | Owned default path and optional explicit path are forwarded into configuration-source construction | #2158 | Retain |
| A028 | `packages/configuration/src/lib.rs:331` | item | `result_large_err` | Figment test callback contract | `Jail::expect_with` requires the framework `Result<()>` callback type | #2158 | Retain |
| A029 | `packages/configuration/src/lib.rs:352` | item | `result_large_err` | Figment test callback contract | `Jail::expect_with` requires the framework `Result<()>` callback type | #2158 | Retain |
| A030 | `packages/configuration/src/lib.rs:370` | item | `result_large_err` | Figment test callback contract | `Jail::expect_with` requires the framework `Result<()>` callback type | #2158 | Retain |
| A031 | `packages/configuration/src/lib.rs:390` | item | `result_large_err` | Figment test callback contract | `Jail::expect_with` requires the framework `Result<()>` callback type | #2158 | Retain |
| A032 | `packages/configuration/src/lib.rs:409` | item | `result_large_err` | Figment test callback contract | `Jail::expect_with` requires the framework `Result<()>` callback type | #2158 | Retain |
| A033 | `packages/configuration/src/lib.rs:433` | item | `result_large_err` | Figment test callback contract | `Jail::expect_with` requires the framework `Result<()>` callback type | #2158 | Retain |
| A034 | `packages/configuration/src/lib.rs:454` | item | `result_large_err` | Figment test callback contract | `Jail::expect_with` requires the framework `Result<()>` callback type | #2158 | Retain |
| A035 | `packages/configuration/src/lib.rs:476` | item | `result_large_err` | Figment test callback contract | `Jail::expect_with` requires the framework `Result<()>` callback type | #2158 | Retain |
| A036 | `packages/configuration/src/lib.rs:518` | item | `result_large_err` | Figment test callback contract | `Jail::expect_with` requires the framework `Result<()>` callback type | #2158 | Retain |
| A037 | `packages/configuration/src/lib.rs:537` | item | `result_large_err` | Figment test callback contract | `Jail::expect_with` requires the framework `Result<()>` callback type | #2158 | Retain |
| A038 | `packages/configuration/src/lib.rs:560` | item | `result_large_err` | Figment test callback contract | `Jail::expect_with` requires the framework `Result<()>` callback type | #2158 | Retain |
| A039 | `packages/configuration/src/lib.rs:594` | item | `result_large_err` | Figment test callback contract | `Jail::expect_with` requires the framework `Result<()>` callback type | #2158 | Retain |
| A040 | `packages/configuration/src/lib.rs:616` | item | `result_large_err` | Figment test callback contract | `Jail::expect_with` requires the framework `Result<()>` callback type | #2158 | Retain |
| A041 | `packages/configuration/src/lib.rs:640` | item | `result_large_err` | Figment test callback contract | `Jail::expect_with` requires the framework `Result<()>` callback type | #2158 | Retain |
| A042 | `packages/configuration/src/lib.rs:665` | item | `result_large_err` | Figment test callback contract | `Jail::expect_with` requires the framework `Result<()>` callback type | #2158 | Retain |
| A043 | `packages/configuration/src/lib.rs:686` | item | `result_large_err` | Figment test callback contract | `Jail::expect_with` requires the framework `Result<()>` callback type | #2158 | Retain |
| A044 | `packages/configuration/src/lib.rs:709` | item | `result_large_err` | Figment test callback contract | `Jail::expect_with` requires the framework `Result<()>` callback type | #2158 | Retain |
| A045 | `packages/configuration/src/lib.rs:734` | item | `result_large_err` | Figment test callback contract | `Jail::expect_with` requires the framework `Result<()>` callback type | #2158 | Retain |
| A046 | `packages/configuration/src/lib.rs:844` | item | `unnecessary_wraps` | Stale suppression | Function explicitly returns `Utf8PathBuf`, not a wrapper type; remove and confirm Clippy remains clean | #2158 | Remove |
| A047 | `packages/configuration/src/lib.rs:848` | item | `unnecessary_wraps` | Stale suppression | Function explicitly returns `Utf8PathBuf`, not a wrapper type; remove and confirm Clippy remains clean | #2158 | Remove |
| A048 | `packages/configuration/src/v2_0_0/core.rs:8` | item | `struct_excessive_bools` | Serialized feature-switch schema | Three independent public configuration switches (`listed`, `private`, and `tracker_usage_statistics`) intentionally map directly to the v2 schema | #2158 | Retain |
| A049 | `packages/configuration/src/v2_0_0/database.rs:4` | item | `struct_excessive_bools` | Stale suppression | `Database` has no boolean fields; remove and confirm Clippy remains clean | #2158 | Remove |
| A050 | `packages/configuration/src/v2_0_0/logging.rs:11` | item | `struct_excessive_bools` | Stale suppression | `Logging` has no boolean fields; remove and confirm Clippy remains clean | #2158 | Remove |
| A051 | `packages/configuration/src/v2_0_0/mod.rs:559` | item | `result_large_err` | Figment test callback contract | `Jail::expect_with` requires the framework `Result<()>` callback type | #2158 | Retain |
| A052 | `packages/configuration/src/v2_0_0/mod.rs:595` | item | `result_large_err` | Figment test callback contract | `Jail::expect_with` requires the framework `Result<()>` callback type | #2158 | Retain |
| A053 | `packages/configuration/src/v2_0_0/mod.rs:629` | item | `result_large_err` | Figment test callback contract | `Jail::expect_with` requires the framework `Result<()>` callback type | #2158 | Retain |
| A054 | `packages/configuration/src/v2_0_0/mod.rs:663` | item | `result_large_err` | Figment test callback contract | `Jail::expect_with` requires the framework `Result<()>` callback type | #2158 | Retain |
| A055 | `packages/configuration/src/v2_0_0/mod.rs:697` | item | `result_large_err` | Figment test callback contract | `Jail::expect_with` requires the framework `Result<()>` callback type | #2158 | Retain |
| A056 | `packages/configuration/src/v2_0_0/mod.rs:785` | item | `result_large_err` | Figment test callback contract | `Jail::expect_with` requires the framework `Result<()>` callback type | #2158 | Retain |
| A057 | `packages/configuration/src/v2_0_0/mod.rs:824` | item | `result_large_err` | Figment test callback contract | `Jail::expect_with` requires the framework `Result<()>` callback type | #2158 | Retain |
| A058 | `packages/configuration/src/v2_0_0/mod.rs:860` | item | `result_large_err` | Figment test callback contract | `Jail::expect_with` requires the framework `Result<()>` callback type | #2158 | Retain |
| A059 | `packages/configuration/src/v2_0_0/tracker_api.rs:50` | item | `unnecessary_wraps` | Stale suppression | Function explicitly returns `Option<TslConfig>` as a configuration default, not a fallible result wrapper; remove and confirm Clippy remains clean | #2158 | Remove |
| A060 | `packages/configuration/src/v3_0_0/core.rs:11` | item | `struct_excessive_bools` | Serialized feature-switch schema | Three independent public configuration switches (`listed`, `private`, and `tracker_usage_statistics`) intentionally map directly to the v3 schema | #2158 | Retain |
| A061 | `packages/configuration/src/v3_0_0/logging.rs:14` | item | `struct_excessive_bools` | Stale suppression | `Logging` has no boolean fields; remove and confirm Clippy remains clean | #2158 | Remove |
| A062 | `packages/configuration/src/v3_0_0/mod.rs:633` | item | `result_large_err` | Figment test callback contract | `Jail::expect_with` requires the framework `Result<()>` callback type | #2158 | Retain |
| A063 | `packages/configuration/src/v3_0_0/mod.rs:673` | item | `result_large_err` | Figment test callback contract | `Jail::expect_with` requires the framework `Result<()>` callback type | #2158 | Retain |
| A064 | `packages/configuration/src/v3_0_0/mod.rs:708` | item | `result_large_err` | Figment test callback contract | `Jail::expect_with` requires the framework `Result<()>` callback type | #2158 | Retain |
| A065 | `packages/configuration/src/v3_0_0/mod.rs:748` | item | `result_large_err` | Figment test callback contract | `Jail::expect_with` requires the framework `Result<()>` callback type | #2158 | Retain |
| A066 | `packages/configuration/src/v3_0_0/mod.rs:808` | item | `result_large_err` | Figment test callback contract | `Jail::expect_with` requires the framework `Result<()>` callback type | #2158 | Retain |
| A067 | `packages/configuration/src/v3_0_0/mod.rs:844` | item | `result_large_err` | Figment test callback contract | `Jail::expect_with` requires the framework `Result<()>` callback type | #2158 | Retain |
| A068 | `packages/configuration/src/v3_0_0/mod.rs:878` | item | `result_large_err` | Figment test callback contract | `Jail::expect_with` requires the framework `Result<()>` callback type | #2158 | Retain |
| A069 | `packages/configuration/src/v3_0_0/mod.rs:917` | item | `result_large_err` | Figment test callback contract | `Jail::expect_with` requires the framework `Result<()>` callback type | #2158 | Retain |
| A070 | `packages/configuration/src/v3_0_0/mod.rs:958` | item | `result_large_err` | Figment test callback contract | `Jail::expect_with` requires the framework `Result<()>` callback type | #2158 | Retain |
| A071 | `packages/configuration/src/v3_0_0/mod.rs:1008` | item | `result_large_err` | Figment test callback contract | `Jail::expect_with` requires the framework `Result<()>` callback type | #2158 | Retain |
| A072 | `packages/configuration/src/v3_0_0/mod.rs:1163` | item | `result_large_err` | Figment test callback contract | `Jail::expect_with` requires the framework `Result<()>` callback type | #2158 | Retain |
| A073 | `packages/configuration/src/v3_0_0/mod.rs:1209` | item | `result_large_err` | Figment test callback contract | `Jail::expect_with` requires the framework `Result<()>` callback type | #2158 | Retain |
| A074 | `packages/configuration/src/v3_0_0/mod.rs:1255` | item | `result_large_err` | Figment test callback contract | `Jail::expect_with` requires the framework `Result<()>` callback type | #2158 | Retain |
| A075 | `packages/configuration/src/v3_0_0/mod.rs:1299` | item | `result_large_err` | Figment test callback contract | `Jail::expect_with` requires the framework `Result<()>` callback type | #2158 | Retain |
| A076 | `packages/configuration/src/v3_0_0/mod.rs:1334` | item | `result_large_err` | Figment test callback contract | `Jail::expect_with` requires the framework `Result<()>` callback type | #2158 | Retain |
| A077 | `packages/configuration/src/v3_0_0/mod.rs:1370` | item | `result_large_err` | Figment test callback contract | `Jail::expect_with` requires the framework `Result<()>` callback type | #2158 | Retain |
| A078 | `packages/configuration/src/v3_0_0/mod.rs:1411` | item | `result_large_err` | Figment test callback contract | `Jail::expect_with` requires the framework `Result<()>` callback type | #2158 | Retain |
| A079 | `packages/configuration/src/v3_0_0/mod.rs:1441` | item | `result_large_err` | Figment test callback contract | `Jail::expect_with` requires the framework `Result<()>` callback type | #2158 | Retain |
| A080 | `packages/http-core/src/statistics/metrics.rs:48` | item | `cast_sign_loss` | Metric aggregate conversion | [Metric aggregate draft](numeric-conversion-follow-up-drafts/metric-aggregate-conversion-safety.md) defines the checked-boundary removal condition | #2158 | Temporary |
| A081 | `packages/http-core/src/statistics/metrics.rs:49` | item | `cast_possible_truncation` | Metric aggregate conversion | [Metric aggregate draft](numeric-conversion-follow-up-drafts/metric-aggregate-conversion-safety.md) defines the checked-boundary removal condition | #2158 | Temporary |
| A082 | `packages/http-core/src/statistics/metrics.rs:61` | item | `cast_sign_loss` | Metric aggregate conversion | [Metric aggregate draft](numeric-conversion-follow-up-drafts/metric-aggregate-conversion-safety.md) defines the checked-boundary removal condition | #2158 | Temporary |
| A083 | `packages/http-core/src/statistics/metrics.rs:62` | item | `cast_possible_truncation` | Metric aggregate conversion | [Metric aggregate draft](numeric-conversion-follow-up-drafts/metric-aggregate-conversion-safety.md) defines the checked-boundary removal condition | #2158 | Temporary |
| A084 | `packages/http-core/src/statistics/metrics.rs:74` | item | `cast_sign_loss` | Metric aggregate conversion | [Metric aggregate draft](numeric-conversion-follow-up-drafts/metric-aggregate-conversion-safety.md) defines the checked-boundary removal condition | #2158 | Temporary |
| A085 | `packages/http-core/src/statistics/metrics.rs:75` | item | `cast_possible_truncation` | Metric aggregate conversion | [Metric aggregate draft](numeric-conversion-follow-up-drafts/metric-aggregate-conversion-safety.md) defines the checked-boundary removal condition | #2158 | Temporary |
| A086 | `packages/http-core/src/statistics/metrics.rs:87` | item | `cast_sign_loss` | Metric aggregate conversion | [Metric aggregate draft](numeric-conversion-follow-up-drafts/metric-aggregate-conversion-safety.md) defines the checked-boundary removal condition | #2158 | Temporary |
| A087 | `packages/http-core/src/statistics/metrics.rs:88` | item | `cast_possible_truncation` | Metric aggregate conversion | [Metric aggregate draft](numeric-conversion-follow-up-drafts/metric-aggregate-conversion-safety.md) defines the checked-boundary removal condition | #2158 | Temporary |
| A088 | `packages/http-protocol/src/v1/responses/announce/data.rs:17` | item | `redundant_field_names` | Proc-macro expansion | Existing nearby comment: MSRV-compatible `derive_more::Constructor` emits `field: field`; remove when it emits shorthand | #2158 | Retain |
| A089 | `packages/http-protocol/src/v1/responses/announce/data.rs:28` | item | `redundant_field_names` | Proc-macro expansion | Existing nearby comment: MSRV-compatible `derive_more::Constructor` emits `field: field`; remove when it emits shorthand | #2158 | Retain |
| A090 | `packages/http-protocol/src/v1/responses/announce/deserialization.rs:82` | item | `chunks_exact_to_as_chunks`, `explicit_iter_loop` | Direct compact-peer parsing cleanup | Use fixed-size chunk conversion and iterator collection without changing BEP compact-peer decoding | #2158 | Remove |
| A091 | `packages/http-protocol/src/v1/responses/announce/encoding.rs:32` | item | `redundant_field_names` | Proc-macro expansion | Existing nearby comment: MSRV-compatible `derive_more::Constructor` emits `field: field`; remove when it emits shorthand | #2158 | Retain |
| A092 | `packages/http-protocol/src/v1/responses/announce/encoding.rs:68` | item | `from_over_into` | Standard conversion trait | Replace `Into<Vec<u8>> for Normal` with `From<Normal> for Vec<u8>`; the standard blanket implementation preserves `.into()` callers | #2158 | Remove |
| A093 | `packages/http-protocol/src/v1/responses/announce/encoding.rs:117` | item | `from_over_into` | Standard conversion trait | Replace `Into<Vec<u8>> for Compact` with `From<Compact> for Vec<u8>`; the standard blanket implementation preserves `.into()` callers | #2158 | Remove |
| A094 | `packages/persistence-benchmark/src/bin/persistence_benchmark/runner.rs:1` | crate | `print_stdout` | Benchmark CLI contract | Persistence benchmark runner intentionally emits progress and result output | #2158 | Retain |
| A095 | `packages/primitives/src/announce.rs:15` | item | `redundant_field_names` | Proc-macro expansion | Existing nearby comment: MSRV-compatible `derive_more::Constructor` emits `field: field`; remove when it emits shorthand | #2158 | Retain |
| A096 | `packages/primitives/src/announce.rs:87` | item | `redundant_field_names` | Proc-macro expansion | Existing nearby comment: MSRV-compatible `derive_more::Constructor` emits `field: field`; remove when it emits shorthand | #2158 | Retain |
| A097 | `packages/primitives/src/mode.rs:12` | item | `redundant_field_names` | Proc-macro expansion | Existing nearby comment: MSRV-compatible `derive_more::Constructor` emits `field: field`; remove when it emits shorthand | #2158 | Retain |
| A098 | `packages/primitives/src/pagination.rs:8` | item | `redundant_field_names` | Proc-macro expansion | Existing nearby comment: MSRV-compatible `derive_more::Constructor` emits `field: field`; remove when it emits shorthand | #2158 | Retain |
| A099 | `packages/primitives/src/peer.rs:141` | item | `cast_possible_truncation` | Domain conversion contract | [Domain conversion draft](numeric-conversion-follow-up-drafts/domain-numeric-conversion-contracts.md) defines the lossless-or-checked removal condition | #2158 | Temporary |
| A100 | `packages/primitives/src/peer.rs:504` | item | `derivable_impls` | Direct fixture-builder cleanup | `PeerBuilder` only wraps `Peer`, which implements `Default`; derive the builder default | #2158 | Remove |
| A101 | `packages/primitives/src/policy.rs:12` | item | `redundant_field_names` | Proc-macro expansion | Existing nearby comment: MSRV-compatible `derive_more::Constructor` emits `field: field`; remove when it emits shorthand | #2158 | Retain |
| A102 | `packages/primitives/src/swarm_metadata.rs:15` | item | `redundant_field_names` | Proc-macro expansion | Existing nearby comment: MSRV-compatible `derive_more::Constructor` emits `field: field`; remove when it emits shorthand | #2158 | Retain |
| A103 | `packages/rest-api-application/src/v1/ports/auth_key.rs:16` | item | `double_must_use` | Macro-generated async trait future | Nearby comment identifies the duplicate must-use annotation generated by `async_trait` | #2158 | Retain |
| A104 | `packages/rest-api-application/src/v1/ports/stats.rs:15` | item | `double_must_use` | Macro-generated async trait future | Nearby comment identifies the duplicate must-use annotation generated by `async_trait` | #2158 | Retain |
| A105 | `packages/rest-api-application/src/v1/ports/torrent.rs:13` | item | `double_must_use` | Macro-generated async trait future | Nearby comment identifies the duplicate must-use annotation generated by `async_trait` | #2158 | Retain |
| A106 | `packages/rest-api-application/src/v1/ports/whitelist.rs:16` | item | `double_must_use` | Macro-generated async trait future | Nearby comment identifies the duplicate must-use annotation generated by `async_trait` | #2158 | Retain |
| A107 | `packages/rest-api-client/src/v1/client.rs:226` | item | `struct_field_names` | HTTP transport role | Fields distinguish connection configuration, API path, and transport client in the low-level HTTP API client | #2158 | Retain |
| A108 | `packages/rest-api-protocol/src/v1/context/torrent/resources/peer.rs:22` | item | `doc_markdown` | Deprecated public API field | Documentation must repeat the compatibility field identifier `updated_milliseconds_ago` verbatim | #2158 | Retain |
| A109 | `packages/rest-api-runtime-adapter/src/v1/adapters/auth_key.rs:86` | item | `needless_pass_by_value` | Ownership-preserving DTO conversion | Conversion consumes `PeerKey` fields while building the owned API DTO | #2158 | Retain |
| A110 | `packages/rest-api-runtime-adapter/src/v1/adapters/stats.rs:18` | item | `struct_field_names` | Repository role names | Fields name distinct tracker statistics repositories that the adapter aggregates | #2158 | Retain |
| A111 | `packages/rest-api-runtime-adapter/src/v1/adapters/stats.rs:31` | item | `too_many_arguments` | REST composition boundary | Sole route-composition call explicitly wires six distinct repositories and one policy flag; a parameter object would obscure the dependency graph | #2158 | Retain |
| A112 | `packages/swarm-coordination-registry/examples/bench_peers.rs:34` | item | `cast_possible_truncation`, `cast_precision_loss`, `cast_sign_loss` | Benchmark input bounds | Nearby comment proves target-width bounds and `f64` precision sufficiency for the short benchmark duration | #2158 | Retain |
| A113 | `packages/swarm-coordination-registry/src/statistics/activity_metrics_updater.rs:79` | item | `cast_precision_loss` | Metric aggregate conversion | [Metric aggregate draft](numeric-conversion-follow-up-drafts/metric-aggregate-conversion-safety.md) assesses whether count gauges need a typed conversion boundary | #2158 | Temporary |
| A114 | `packages/swarm-coordination-registry/src/statistics/activity_metrics_updater.rs:93` | item | `cast_precision_loss` | Metric aggregate conversion | [Metric aggregate draft](numeric-conversion-follow-up-drafts/metric-aggregate-conversion-safety.md) assesses whether count gauges need a typed conversion boundary | #2158 | Temporary |
| A115 | `packages/swarm-coordination-registry/src/swarm/registry.rs:864` | item | `from_over_into` | Standard conversion trait | Replace `Into<TorrentEntryInfo> for Coordinator` with `From<Coordinator> for TorrentEntryInfo`; the standard blanket implementation preserves `.into()` callers | #2158 | Remove |
| A116 | `packages/test-helpers/src/logging.rs:130` | item | `unnecessary_wraps` | Standard `Write` trait contract | Circular buffer `write` implements the required `io::Result<usize>` signature | #2158 | Retain |
| A117 | `packages/test-helpers/src/logging.rs:146` | item | `unnecessary_wraps` | Standard `Write` trait contract | Circular buffer `flush` implements the required `io::Result<()>` signature | #2158 | Retain |
| A118 | `packages/test-helpers/src/logging.rs:147` | item | `unused_self` | Standard `Write` trait contract | Circular buffer `flush` requires its mutable receiver under the `Write` trait signature | #2158 | Retain |
| A119 | `packages/torrent-repository-benchmarking/benches/helpers/utils.rs:20` | item | `missing_panics_doc` | Missing benchmark panic documentation | Document the generator's deliberate `unwrap` precondition and remove the suppression | #2158 | Remove |
| A120 | `packages/torrent-repository-benchmarking/benches/helpers/utils.rs:25` | item | `cast_possible_truncation` | Info-hash test-data encoding | Generator intentionally retains four low-order bytes when assigning each byte; narrowing is its defined encoding | #2158 | Retain |
| A121 | `packages/torrent-repository-benchmarking/src/entry/mod.rs:52` | item | `module_name_repetitions` | Benchmark abstraction name | `EntrySync` distinguishes the synchronous entry benchmark abstraction from `EntryAsync` | #2158 | Retain |
| A122 | `packages/torrent-repository-benchmarking/src/entry/mod.rs:64` | item | `module_name_repetitions` | Benchmark abstraction name | `EntryAsync` distinguishes the asynchronous entry benchmark abstraction from `EntrySync` | #2158 | Retain |
| A123 | `packages/torrent-repository-benchmarking/src/entry/single.rs:13` | item | `cast_possible_truncation` | Domain conversion contract | [Domain conversion draft](numeric-conversion-follow-up-drafts/domain-numeric-conversion-contracts.md) defines the lossless-or-checked removal condition | #2158 | Temporary |
| A124 | `packages/torrent-repository-benchmarking/src/repository/mod.rs:29` | item | `module_name_repetitions` | Benchmark abstraction name | `RepositoryAsync` distinguishes the asynchronous repository benchmark abstraction | #2158 | Retain |
| A125 | `packages/torrent-repository-benchmarking/src/repository/rw_lock_tokio.rs:18` | item | `future_not_send` | Tokio lock guard contract | Tokio write future yields a non-`Send` write guard; adding `Send` would change the lock API contract | #2158 | Retain |
| A126 | `packages/tracker-client/src/http/client/mod.rs:27` | item | `struct_field_names` | HTTP client role | Fields distinguish the HTTP transport client and tracker base URL | #2158 | Retain |
| A127 | `packages/tracker-client/src/udp/client.rs:18` | item | `module_name_repetitions` | Protocol-specific public type | `UdpClient` identifies the UDP protocol client among tracker client types | #2158 | Retain |
| A128 | `packages/tracker-client/src/udp/client.rs:179` | item | `module_name_repetitions` | Protocol-specific public type | `UdpTrackerClient` distinguishes the tracker-facing wrapper from its underlying UDP client | #2158 | Retain |
| A129 | `packages/tracker-core/src/announce_handler.rs:277` | item | `cast_sign_loss` | Domain conversion contract | [Domain conversion draft](numeric-conversion-follow-up-drafts/domain-numeric-conversion-contracts.md) defines the lossless-or-checked removal condition | #2158 | Temporary |
| A130 | `packages/tracker-core/src/databases/driver/sqlite/mod.rs:34` | item | `unnecessary_wraps` | Driver API symmetry | Existing nearby comment preserves `Result` symmetry with MySQL and future fallibility | #2158 | Retain |
| A131 | `packages/tracker-core/src/databases/traits/auth_keys.rs:14` | item | `double_must_use` | Macro-generated async trait future | Nearby comment identifies the duplicate must-use annotation generated by `async_trait` | #2158 | Retain |
| A132 | `packages/tracker-core/src/databases/traits/auth_keys.rs:16` | item | `struct_field_names`, `extra_unused_lifetimes` | Macro-generated trait code | Nearby comment identifies `automock` fields ending in `keys` and `async_trait` lifetimes outside workspace control | #2158 | Retain |
| A133 | `packages/tracker-core/src/databases/traits/schema.rs:13` | item | `double_must_use` | Macro-generated async trait future | Nearby comment identifies the duplicate must-use annotation generated by `async_trait` | #2158 | Retain |
| A134 | `packages/tracker-core/src/databases/traits/schema.rs:15` | item | `extra_unused_lifetimes` | Macro-generated async trait code | Nearby comment identifies lifetimes generated by `async_trait` outside workspace control | #2158 | Retain |
| A135 | `packages/tracker-core/src/databases/traits/torrent_metrics.rs:18` | item | `double_must_use` | Macro-generated async trait future | Nearby comment identifies the duplicate must-use annotation generated by `async_trait` | #2158 | Retain |
| A136 | `packages/tracker-core/src/databases/traits/torrent_metrics.rs:20` | item | `extra_unused_lifetimes` | Macro-generated async trait code | Nearby comment identifies lifetimes generated by `async_trait` outside workspace control | #2158 | Retain |
| A137 | `packages/tracker-core/src/databases/traits/whitelist.rs:11` | item | `double_must_use` | Macro-generated async trait future | Nearby comment identifies the duplicate must-use annotation generated by `async_trait` | #2158 | Retain |
| A138 | `packages/tracker-core/src/databases/traits/whitelist.rs:13` | item | `extra_unused_lifetimes` | Macro-generated async trait code | Nearby comment identifies lifetimes generated by `async_trait` outside workspace control | #2158 | Retain |
| A139 | `packages/tracker-core/src/error.rs:108` | item | `module_name_repetitions` | Domain error type | `PeerKeyError` identifies the error's peer-key domain among tracker-core errors | #2158 | Retain |
| A140 | `packages/udp-core/benches/helpers/sync.rs:13` | item | `unused_async` | Benchmark call-shape consistency | Helper remains async so synchronous and asynchronous benchmark paths share an awaitable call shape | #2158 | Retain |
| A141 | `packages/udp-core/src/crypto/keys.rs:50` | item | `needless_borrow` | Direct seed reference cleanup | Return the re-exported seed through implicit borrowing; validate both production and test configurations | #2158 | Remove |
| A142 | `packages/udp-core/src/crypto/keys.rs:73` | item | `needless_borrow` | Direct seed reference cleanup | Return the test seed through implicit borrowing; validate the test configuration | #2158 | Remove |
| A143 | `packages/udp-core/src/statistics/metrics.rs:49` | item | `cast_sign_loss` | Metric aggregate conversion | [Metric aggregate draft](numeric-conversion-follow-up-drafts/metric-aggregate-conversion-safety.md) defines the checked-boundary removal condition | #2158 | Temporary |
| A144 | `packages/udp-core/src/statistics/metrics.rs:50` | item | `cast_possible_truncation` | Metric aggregate conversion | [Metric aggregate draft](numeric-conversion-follow-up-drafts/metric-aggregate-conversion-safety.md) defines the checked-boundary removal condition | #2158 | Temporary |
| A145 | `packages/udp-core/src/statistics/metrics.rs:62` | item | `cast_sign_loss` | Metric aggregate conversion | [Metric aggregate draft](numeric-conversion-follow-up-drafts/metric-aggregate-conversion-safety.md) defines the checked-boundary removal condition | #2158 | Temporary |
| A146 | `packages/udp-core/src/statistics/metrics.rs:63` | item | `cast_possible_truncation` | Metric aggregate conversion | [Metric aggregate draft](numeric-conversion-follow-up-drafts/metric-aggregate-conversion-safety.md) defines the checked-boundary removal condition | #2158 | Temporary |
| A147 | `packages/udp-core/src/statistics/metrics.rs:75` | item | `cast_sign_loss` | Metric aggregate conversion | [Metric aggregate draft](numeric-conversion-follow-up-drafts/metric-aggregate-conversion-safety.md) defines the checked-boundary removal condition | #2158 | Temporary |
| A148 | `packages/udp-core/src/statistics/metrics.rs:76` | item | `cast_possible_truncation` | Metric aggregate conversion | [Metric aggregate draft](numeric-conversion-follow-up-drafts/metric-aggregate-conversion-safety.md) defines the checked-boundary removal condition | #2158 | Temporary |
| A149 | `packages/udp-core/src/statistics/metrics.rs:88` | item | `cast_sign_loss` | Metric aggregate conversion | [Metric aggregate draft](numeric-conversion-follow-up-drafts/metric-aggregate-conversion-safety.md) defines the checked-boundary removal condition | #2158 | Temporary |
| A150 | `packages/udp-core/src/statistics/metrics.rs:89` | item | `cast_possible_truncation` | Metric aggregate conversion | [Metric aggregate draft](numeric-conversion-follow-up-drafts/metric-aggregate-conversion-safety.md) defines the checked-boundary removal condition | #2158 | Temporary |
| A151 | `packages/udp-core/src/statistics/metrics.rs:101` | item | `cast_sign_loss` | Metric aggregate conversion | [Metric aggregate draft](numeric-conversion-follow-up-drafts/metric-aggregate-conversion-safety.md) defines the checked-boundary removal condition | #2158 | Temporary |
| A152 | `packages/udp-core/src/statistics/metrics.rs:102` | item | `cast_possible_truncation` | Metric aggregate conversion | [Metric aggregate draft](numeric-conversion-follow-up-drafts/metric-aggregate-conversion-safety.md) defines the checked-boundary removal condition | #2158 | Temporary |
| A153 | `packages/udp-core/src/statistics/metrics.rs:114` | item | `cast_sign_loss` | Metric aggregate conversion | [Metric aggregate draft](numeric-conversion-follow-up-drafts/metric-aggregate-conversion-safety.md) defines the checked-boundary removal condition | #2158 | Temporary |
| A154 | `packages/udp-core/src/statistics/metrics.rs:115` | item | `cast_possible_truncation` | Metric aggregate conversion | [Metric aggregate draft](numeric-conversion-follow-up-drafts/metric-aggregate-conversion-safety.md) defines the checked-boundary removal condition | #2158 | Temporary |
| A155 | `packages/udp-core/src/statistics/repository.rs:15` | item | `double_must_use` | Macro-generated async trait future | Nearby comment identifies the duplicate must-use annotation generated by `async_trait` | #2158 | Retain |
| A156 | `packages/udp-protocol/src/lib.rs:8` | crate | `cast_possible_truncation` | UDP protocol crate baseline | [Baseline draft](udp-protocol-clippy-baseline-draft.md): replace with checked wire conversion or a narrow documented exception | #2158 | Temporary |
| A157 | `packages/udp-protocol/src/lib.rs:9` | crate | `default_trait_access` | UDP protocol crate baseline | [Baseline draft](udp-protocol-clippy-baseline-draft.md): remove or narrow with source-specific evidence | #2158 | Temporary |
| A158 | `packages/udp-protocol/src/lib.rs:10` | crate | `doc_markdown` | UDP protocol crate baseline | [Baseline draft](udp-protocol-clippy-baseline-draft.md): remove or narrow with source-specific evidence | #2158 | Temporary |
| A159 | `packages/udp-protocol/src/lib.rs:14` | crate | `empty_enums` | UDP protocol crate baseline | Existing comment identifies `FromBytes` macro-generated empty helper enums; [baseline draft](udp-protocol-clippy-baseline-draft.md) requires the narrowest supported scope | #2158 | Temporary |
| A160 | `packages/udp-protocol/src/lib.rs:15` | crate | `explicit_iter_loop` | UDP protocol crate baseline | [Baseline draft](udp-protocol-clippy-baseline-draft.md): remove or narrow with source-specific evidence | #2158 | Temporary |
| A161 | `packages/udp-protocol/src/lib.rs:16` | crate | `legacy_numeric_constants` | UDP protocol crate baseline | [Baseline draft](udp-protocol-clippy-baseline-draft.md): remove or narrow with source-specific evidence | #2158 | Temporary |
| A162 | `packages/udp-protocol/src/lib.rs:17` | crate | `match_same_arms` | UDP protocol crate baseline | [Baseline draft](udp-protocol-clippy-baseline-draft.md): remove or narrow with source-specific evidence | #2158 | Temporary |
| A163 | `packages/udp-protocol/src/lib.rs:18` | crate | `missing_errors_doc` | UDP protocol crate baseline | [Baseline draft](udp-protocol-clippy-baseline-draft.md): remove or narrow with source-specific evidence | #2158 | Temporary |
| A164 | `packages/udp-protocol/src/lib.rs:19` | crate | `missing_panics_doc` | UDP protocol crate baseline | [Baseline draft](udp-protocol-clippy-baseline-draft.md): remove or narrow with source-specific evidence | #2158 | Temporary |
| A165 | `packages/udp-protocol/src/lib.rs:20` | crate | `must_use_candidate` | UDP protocol crate baseline | [Baseline draft](udp-protocol-clippy-baseline-draft.md): remove or narrow with source-specific evidence | #2158 | Temporary |
| A166 | `packages/udp-protocol/src/lib.rs:21` | crate | `needless_pass_by_value` | UDP protocol crate baseline | [Baseline draft](udp-protocol-clippy-baseline-draft.md): remove or narrow with source-specific evidence | #2158 | Temporary |
| A167 | `packages/udp-protocol/src/lib.rs:22` | crate | `semicolon_if_nothing_returned` | UDP protocol crate baseline | [Baseline draft](udp-protocol-clippy-baseline-draft.md): remove or narrow with source-specific evidence | #2158 | Temporary |
| A168 | `packages/udp-protocol/src/lib.rs:23` | crate | `wildcard_imports` | UDP protocol crate baseline | [Baseline draft](udp-protocol-clippy-baseline-draft.md): remove or narrow with source-specific evidence | #2158 | Temporary |
| A169 | `packages/udp-server/examples/udp_only_public_tracker.rs:35` | crate | `print_stdout` | Example executable output | Runnable public-tracker example intentionally prints service information | #2158 | Retain |
| A170 | `packages/udp-server/src/banning/event/handler.rs:34` | item | `cast_precision_loss` | Metric aggregate conversion | [Metric aggregate draft](numeric-conversion-follow-up-drafts/metric-aggregate-conversion-safety.md) assesses whether count gauges need a typed conversion boundary | #2158 | Temporary |
| A171 | `packages/udp-server/src/handlers/announce.rs:132` | item | `cast_possible_truncation` | Wire conversion validation | [Wire conversion draft](numeric-conversion-follow-up-drafts/wire-numeric-conversion-validation.md) defines the protocol-bound removal condition | #2158 | Temporary |
| A172 | `packages/udp-server/src/handlers/error.rs:18` | item | `too_many_arguments` | UDP error-dispatch boundary | Request dispatcher forwards independently meaningful request, server, event, and response inputs; keeping them explicit preserves error-path traceability | #2158 | Retain |
| A173 | `packages/udp-server/src/server/mod.rs:54` | item | `module_name_repetitions` | Protocol-specific public type | `Server` is the UDP server module's canonical public state controller | #2158 | Retain |
| A174 | `packages/udp-server/src/server/spawner.rs:31` | item | `redundant_field_names` | Proc-macro expansion | Existing nearby comment: MSRV-compatible `derive_more::Constructor` emits `field: field`; remove when it emits shorthand | #2158 | Retain |
| A175 | `packages/udp-server/src/server/states.rs:33` | item | `module_name_repetitions` | Lifecycle-state API name | `StoppedUdpServer` distinguishes the UDP server state-controller alias from other server states | #2158 | Retain |
| A176 | `packages/udp-server/src/server/states.rs:37` | item | `module_name_repetitions` | Lifecycle-state API name | `RunningUdpServer` distinguishes the UDP server state-controller alias from other server states | #2158 | Retain |
| A177 | `packages/udp-server/src/server/states.rs:51` | item | `redundant_field_names` | Proc-macro expansion | Existing nearby comment: MSRV-compatible `derive_more::Constructor` emits `field: field`; remove when it emits shorthand | #2158 | Retain |
| A178 | `packages/udp-server/src/statistics/metrics.rs:55` | item | `cast_precision_loss` | Metric aggregate conversion | [Metric aggregate draft](numeric-conversion-follow-up-drafts/metric-aggregate-conversion-safety.md) defines the checked-boundary removal condition | #2158 | Temporary |
| A179 | `packages/udp-server/src/statistics/metrics.rs:86` | item | `cast_sign_loss` | Metric aggregate conversion | [Metric aggregate draft](numeric-conversion-follow-up-drafts/metric-aggregate-conversion-safety.md) defines the checked-boundary removal condition | #2158 | Temporary |
| A180 | `packages/udp-server/src/statistics/metrics.rs:87` | item | `cast_possible_truncation` | Metric aggregate conversion | [Metric aggregate draft](numeric-conversion-follow-up-drafts/metric-aggregate-conversion-safety.md) defines the checked-boundary removal condition | #2158 | Temporary |
| A181 | `packages/udp-server/src/statistics/metrics.rs:98` | item | `cast_sign_loss` | Metric aggregate conversion | [Metric aggregate draft](numeric-conversion-follow-up-drafts/metric-aggregate-conversion-safety.md) defines the checked-boundary removal condition | #2158 | Temporary |
| A182 | `packages/udp-server/src/statistics/metrics.rs:99` | item | `cast_possible_truncation` | Metric aggregate conversion | [Metric aggregate draft](numeric-conversion-follow-up-drafts/metric-aggregate-conversion-safety.md) defines the checked-boundary removal condition | #2158 | Temporary |
| A183 | `packages/udp-server/src/statistics/metrics.rs:107` | item | `cast_sign_loss` | Metric aggregate conversion | [Metric aggregate draft](numeric-conversion-follow-up-drafts/metric-aggregate-conversion-safety.md) defines the checked-boundary removal condition | #2158 | Temporary |
| A184 | `packages/udp-server/src/statistics/metrics.rs:108` | item | `cast_possible_truncation` | Metric aggregate conversion | [Metric aggregate draft](numeric-conversion-follow-up-drafts/metric-aggregate-conversion-safety.md) defines the checked-boundary removal condition | #2158 | Temporary |
| A185 | `packages/udp-server/src/statistics/metrics.rs:152` | item | `cast_sign_loss` | Metric aggregate conversion | [Metric aggregate draft](numeric-conversion-follow-up-drafts/metric-aggregate-conversion-safety.md) defines the checked-boundary removal condition | #2158 | Temporary |
| A186 | `packages/udp-server/src/statistics/metrics.rs:153` | item | `cast_possible_truncation` | Metric aggregate conversion | [Metric aggregate draft](numeric-conversion-follow-up-drafts/metric-aggregate-conversion-safety.md) defines the checked-boundary removal condition | #2158 | Temporary |
| A187 | `packages/udp-server/src/statistics/metrics.rs:163` | item | `cast_sign_loss` | Metric aggregate conversion | [Metric aggregate draft](numeric-conversion-follow-up-drafts/metric-aggregate-conversion-safety.md) defines the checked-boundary removal condition | #2158 | Temporary |
| A188 | `packages/udp-server/src/statistics/metrics.rs:164` | item | `cast_possible_truncation` | Metric aggregate conversion | [Metric aggregate draft](numeric-conversion-follow-up-drafts/metric-aggregate-conversion-safety.md) defines the checked-boundary removal condition | #2158 | Temporary |
| A189 | `packages/udp-server/src/statistics/metrics.rs:173` | item | `cast_sign_loss` | Metric aggregate conversion | [Metric aggregate draft](numeric-conversion-follow-up-drafts/metric-aggregate-conversion-safety.md) defines the checked-boundary removal condition | #2158 | Temporary |
| A190 | `packages/udp-server/src/statistics/metrics.rs:174` | item | `cast_possible_truncation` | Metric aggregate conversion | [Metric aggregate draft](numeric-conversion-follow-up-drafts/metric-aggregate-conversion-safety.md) defines the checked-boundary removal condition | #2158 | Temporary |
| A191 | `packages/udp-server/src/statistics/metrics.rs:183` | item | `cast_sign_loss` | Metric aggregate conversion | [Metric aggregate draft](numeric-conversion-follow-up-drafts/metric-aggregate-conversion-safety.md) defines the checked-boundary removal condition | #2158 | Temporary |
| A192 | `packages/udp-server/src/statistics/metrics.rs:184` | item | `cast_possible_truncation` | Metric aggregate conversion | [Metric aggregate draft](numeric-conversion-follow-up-drafts/metric-aggregate-conversion-safety.md) defines the checked-boundary removal condition | #2158 | Temporary |
| A193 | `packages/udp-server/src/statistics/metrics.rs:194` | item | `cast_sign_loss` | Metric aggregate conversion | [Metric aggregate draft](numeric-conversion-follow-up-drafts/metric-aggregate-conversion-safety.md) defines the checked-boundary removal condition | #2158 | Temporary |
| A194 | `packages/udp-server/src/statistics/metrics.rs:195` | item | `cast_possible_truncation` | Metric aggregate conversion | [Metric aggregate draft](numeric-conversion-follow-up-drafts/metric-aggregate-conversion-safety.md) defines the checked-boundary removal condition | #2158 | Temporary |
| A195 | `packages/udp-server/src/statistics/metrics.rs:208` | item | `cast_sign_loss` | Metric aggregate conversion | [Metric aggregate draft](numeric-conversion-follow-up-drafts/metric-aggregate-conversion-safety.md) defines the checked-boundary removal condition | #2158 | Temporary |
| A196 | `packages/udp-server/src/statistics/metrics.rs:209` | item | `cast_possible_truncation` | Metric aggregate conversion | [Metric aggregate draft](numeric-conversion-follow-up-drafts/metric-aggregate-conversion-safety.md) defines the checked-boundary removal condition | #2158 | Temporary |
| A197 | `packages/udp-server/src/statistics/metrics.rs:222` | item | `cast_sign_loss` | Metric aggregate conversion | [Metric aggregate draft](numeric-conversion-follow-up-drafts/metric-aggregate-conversion-safety.md) defines the checked-boundary removal condition | #2158 | Temporary |
| A198 | `packages/udp-server/src/statistics/metrics.rs:223` | item | `cast_possible_truncation` | Metric aggregate conversion | [Metric aggregate draft](numeric-conversion-follow-up-drafts/metric-aggregate-conversion-safety.md) defines the checked-boundary removal condition | #2158 | Temporary |
| A199 | `packages/udp-server/src/statistics/metrics.rs:236` | item | `cast_sign_loss` | Metric aggregate conversion | [Metric aggregate draft](numeric-conversion-follow-up-drafts/metric-aggregate-conversion-safety.md) defines the checked-boundary removal condition | #2158 | Temporary |
| A200 | `packages/udp-server/src/statistics/metrics.rs:237` | item | `cast_possible_truncation` | Metric aggregate conversion | [Metric aggregate draft](numeric-conversion-follow-up-drafts/metric-aggregate-conversion-safety.md) defines the checked-boundary removal condition | #2158 | Temporary |
| A201 | `packages/udp-server/src/statistics/metrics.rs:249` | item | `cast_sign_loss` | Metric aggregate conversion | [Metric aggregate draft](numeric-conversion-follow-up-drafts/metric-aggregate-conversion-safety.md) defines the checked-boundary removal condition | #2158 | Temporary |
| A202 | `packages/udp-server/src/statistics/metrics.rs:250` | item | `cast_possible_truncation` | Metric aggregate conversion | [Metric aggregate draft](numeric-conversion-follow-up-drafts/metric-aggregate-conversion-safety.md) defines the checked-boundary removal condition | #2158 | Temporary |
| A203 | `packages/udp-server/src/statistics/metrics.rs:262` | item | `cast_sign_loss` | Metric aggregate conversion | [Metric aggregate draft](numeric-conversion-follow-up-drafts/metric-aggregate-conversion-safety.md) defines the checked-boundary removal condition | #2158 | Temporary |
| A204 | `packages/udp-server/src/statistics/metrics.rs:263` | item | `cast_possible_truncation` | Metric aggregate conversion | [Metric aggregate draft](numeric-conversion-follow-up-drafts/metric-aggregate-conversion-safety.md) defines the checked-boundary removal condition | #2158 | Temporary |
| A205 | `packages/udp-server/src/statistics/metrics.rs:275` | item | `cast_sign_loss` | Metric aggregate conversion | [Metric aggregate draft](numeric-conversion-follow-up-drafts/metric-aggregate-conversion-safety.md) defines the checked-boundary removal condition | #2158 | Temporary |
| A206 | `packages/udp-server/src/statistics/metrics.rs:276` | item | `cast_possible_truncation` | Metric aggregate conversion | [Metric aggregate draft](numeric-conversion-follow-up-drafts/metric-aggregate-conversion-safety.md) defines the checked-boundary removal condition | #2158 | Temporary |
| A207 | `packages/udp-server/src/statistics/metrics.rs:288` | item | `cast_sign_loss` | Metric aggregate conversion | [Metric aggregate draft](numeric-conversion-follow-up-drafts/metric-aggregate-conversion-safety.md) defines the checked-boundary removal condition | #2158 | Temporary |
| A208 | `packages/udp-server/src/statistics/metrics.rs:289` | item | `cast_possible_truncation` | Metric aggregate conversion | [Metric aggregate draft](numeric-conversion-follow-up-drafts/metric-aggregate-conversion-safety.md) defines the checked-boundary removal condition | #2158 | Temporary |
| A209 | `packages/udp-server/src/statistics/metrics.rs:301` | item | `cast_sign_loss` | Metric aggregate conversion | [Metric aggregate draft](numeric-conversion-follow-up-drafts/metric-aggregate-conversion-safety.md) defines the checked-boundary removal condition | #2158 | Temporary |
| A210 | `packages/udp-server/src/statistics/metrics.rs:302` | item | `cast_possible_truncation` | Metric aggregate conversion | [Metric aggregate draft](numeric-conversion-follow-up-drafts/metric-aggregate-conversion-safety.md) defines the checked-boundary removal condition | #2158 | Temporary |
| A211 | `packages/udp-server/src/statistics/metrics.rs:315` | item | `cast_sign_loss` | Metric aggregate conversion | [Metric aggregate draft](numeric-conversion-follow-up-drafts/metric-aggregate-conversion-safety.md) defines the checked-boundary removal condition | #2158 | Temporary |
| A212 | `packages/udp-server/src/statistics/metrics.rs:316` | item | `cast_possible_truncation` | Metric aggregate conversion | [Metric aggregate draft](numeric-conversion-follow-up-drafts/metric-aggregate-conversion-safety.md) defines the checked-boundary removal condition | #2158 | Temporary |
| A213 | `packages/udp-server/src/statistics/metrics.rs:328` | item | `cast_sign_loss` | Metric aggregate conversion | [Metric aggregate draft](numeric-conversion-follow-up-drafts/metric-aggregate-conversion-safety.md) defines the checked-boundary removal condition | #2158 | Temporary |
| A214 | `packages/udp-server/src/statistics/metrics.rs:329` | item | `cast_possible_truncation` | Metric aggregate conversion | [Metric aggregate draft](numeric-conversion-follow-up-drafts/metric-aggregate-conversion-safety.md) defines the checked-boundary removal condition | #2158 | Temporary |
| A215 | `packages/udp-server/src/statistics/metrics.rs:341` | item | `cast_sign_loss` | Metric aggregate conversion | [Metric aggregate draft](numeric-conversion-follow-up-drafts/metric-aggregate-conversion-safety.md) defines the checked-boundary removal condition | #2158 | Temporary |
| A216 | `packages/udp-server/src/statistics/metrics.rs:342` | item | `cast_possible_truncation` | Metric aggregate conversion | [Metric aggregate draft](numeric-conversion-follow-up-drafts/metric-aggregate-conversion-safety.md) defines the checked-boundary removal condition | #2158 | Temporary |
| A217 | `packages/udp-server/src/statistics/metrics.rs:354` | item | `cast_sign_loss` | Metric aggregate conversion | [Metric aggregate draft](numeric-conversion-follow-up-drafts/metric-aggregate-conversion-safety.md) defines the checked-boundary removal condition | #2158 | Temporary |
| A218 | `packages/udp-server/src/statistics/metrics.rs:355` | item | `cast_possible_truncation` | Metric aggregate conversion | [Metric aggregate draft](numeric-conversion-follow-up-drafts/metric-aggregate-conversion-safety.md) defines the checked-boundary removal condition | #2158 | Temporary |
| A219 | `packages/udp-server/src/statistics/metrics.rs:367` | item | `cast_sign_loss` | Metric aggregate conversion | [Metric aggregate draft](numeric-conversion-follow-up-drafts/metric-aggregate-conversion-safety.md) defines the checked-boundary removal condition | #2158 | Temporary |
| A220 | `packages/udp-server/src/statistics/metrics.rs:368` | item | `cast_possible_truncation` | Metric aggregate conversion | [Metric aggregate draft](numeric-conversion-follow-up-drafts/metric-aggregate-conversion-safety.md) defines the checked-boundary removal condition | #2158 | Temporary |
| A221 | `packages/udp-server/src/statistics/metrics.rs:380` | item | `cast_sign_loss` | Metric aggregate conversion | [Metric aggregate draft](numeric-conversion-follow-up-drafts/metric-aggregate-conversion-safety.md) defines the checked-boundary removal condition | #2158 | Temporary |
| A222 | `packages/udp-server/src/statistics/metrics.rs:381` | item | `cast_possible_truncation` | Metric aggregate conversion | [Metric aggregate draft](numeric-conversion-follow-up-drafts/metric-aggregate-conversion-safety.md) defines the checked-boundary removal condition | #2158 | Temporary |
| A223 | `packages/udp-server/src/statistics/repository.rs:16` | item | `double_must_use` | Macro-generated async trait future | Nearby comment identifies the duplicate must-use annotation generated by `async_trait` | #2158 | Retain |
| A224 | `packages/udp-server/src/statistics/repository.rs:330` | item | `cast_sign_loss` | Metric aggregate conversion test | [Metric aggregate draft](numeric-conversion-follow-up-drafts/metric-aggregate-conversion-safety.md) defines the checked-boundary removal condition | #2158 | Temporary |
| A225 | `packages/udp-server/src/statistics/repository.rs:332` | item | `cast_possible_truncation` | Metric aggregate conversion test | [Metric aggregate draft](numeric-conversion-follow-up-drafts/metric-aggregate-conversion-safety.md) defines the checked-boundary removal condition | #2158 | Temporary |
| A226 | `packages/udp-server/src/statistics/repository.rs:340` | item | `cast_sign_loss` | Metric aggregate conversion test | [Metric aggregate draft](numeric-conversion-follow-up-drafts/metric-aggregate-conversion-safety.md) defines the checked-boundary removal condition | #2158 | Temporary |
| A227 | `packages/udp-server/src/statistics/repository.rs:342` | item | `cast_possible_truncation` | Metric aggregate conversion test | [Metric aggregate draft](numeric-conversion-follow-up-drafts/metric-aggregate-conversion-safety.md) defines the checked-boundary removal condition | #2158 | Temporary |
| A228 | `console/tracker-client/src/bin/http_tracker_client.rs:1` | crate | `print_stderr` | HTTP tracker CLI contract | HTTP tracker client intentionally reports command failures on standard error | #2158 | Retain |
| A229 | `console/tracker-client/src/bin/tracker_checker.rs:1` | crate | `print_stderr`, `exit` | Tracker checker CLI contract | Tracker checker intentionally reports failures and returns explicit process status | #2158 | Retain |
| A230 | `console/tracker-client/src/bin/tracker_client.rs:1` | crate | `print_stderr`, `exit` | Unified tracker CLI contract | Unified tracker client intentionally reports failures and returns explicit process status | #2158 | Retain |
| A231 | `console/tracker-client/src/bin/udp_tracker_client.rs:1` | crate | `print_stderr` | UDP tracker CLI contract | UDP tracker client intentionally reports command failures on standard error | #2158 | Retain |
| A232 | `console/tracker-client/src/console/clients/checker/checks/udp.rs:26` | item | `missing_panics_doc` | Missing UDP checker panic documentation | Document the deliberate sample-hash and socket-resolution `unwrap` preconditions and remove the suppression | #2158 | Remove |
| A233 | `console/tracker-client/src/console/clients/udp/responses/json.rs:5` | item | `module_name_repetitions` | Serialization extension trait | `ToJson` is the serialization extension-trait name in the UDP response JSON module | #2158 | Retain |
| A234 | `console/tracker-client/src/lib.rs:5` | crate | `print_stdout`, `print_stderr` | Shared console output contract | Library modules implement terminal output invoked by the console binary targets | #2158 | Retain |
| A235 | `packages/torrent-repository-benchmarking/src/lib.rs:1` | crate | `option_if_let_else`, `or_fun_call`, `significant_drop_tightening`, `iter_with_drain` | Current-source benchmarking style baseline | Removed the crate-level baseline by applying Clippy style fixes and tightening lock/drop scopes across benchmark repository implementations | #2158 | Remove |
| A236 | `packages/swarm-coordination-registry/src/statistics/mod.rs:207` | item | `cast_possible_truncation`, `cast_sign_loss` | Collaboration-test gauge assertion | Existing native reason: the gauge is set from a small non-negative peer count | #2158 | Retain |
