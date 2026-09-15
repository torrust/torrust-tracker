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

## Entries

| ID | Source location | Scope | Lint name(s) | Rationale category | Evidence | Owner | Disposition |
| -- | --------------- | ----- | ------------ | ------------------ | -------- | ----- | ----------- |
| A001 | `src/bin/http_health_check.rs:1` | crate | `print_stdout`, `print_stderr`, `exit` | Pending review | Source inspection required | #2158 | Pending |
| A002 | `src/bootstrap/jobs/health_check_api.rs:58` | item | `async_yields_async` | Pending review | Source inspection required | #2158 | Pending |
| A003 | `src/bootstrap/jobs/http_tracker.rs:84` | item | `async_yields_async` | Pending review | Source inspection required | #2158 | Pending |
| A004 | `src/bootstrap/jobs/tracker_apis.rs:110` | item | `async_yields_async` | Pending review | Source inspection required | #2158 | Pending |
| A005 | `src/bootstrap/jobs/udp_tracker.rs:44` | item | `async_yields_async` | Pending review | Source inspection required | #2158 | Pending |
| A006 | `src/console/ci/qbittorrent_e2e/qbittorrent/mod.rs:5` | crate | `redundant_pub_crate` | Pending review | Source inspection required | #2158 | Pending |
| A007 | `src/console/ci/qbittorrent_e2e/tracker/mod.rs:5` | crate | `redundant_pub_crate` | Pending review | Source inspection required | #2158 | Pending |
| A008 | `src/console/ci/qbittorrent_e2e/types/mod.rs:8` | crate | `redundant_pub_crate` | Pending review | Source inspection required | #2158 | Pending |
| A009 | `src/console/profiling.rs:1` | crate | `print_stdout`, `print_stderr` | Pending review | Source inspection required | #2158 | Pending |
| A010 | `src/main.rs:112` | item | `print_stderr` | Pending review | Source inspection required | #2158 | Pending |
| A011 | `packages/axum-http-server/src/server.rs:64` | item | `redundant_field_names` | Pending review | Source inspection required | #2158 | Pending |
| A012 | `packages/axum-http-server/src/server.rs:183` | item | `module_name_repetitions` | Pending review | Source inspection required | #2158 | Pending |
| A013 | `packages/axum-http-server/src/server.rs:187` | item | `module_name_repetitions` | Pending review | Source inspection required | #2158 | Pending |
| A014 | `packages/axum-http-server/src/server.rs:205` | item | `module_name_repetitions` | Pending review | Source inspection required | #2158 | Pending |
| A015 | `packages/axum-http-server/src/v1/extractors/authentication_key.rs:75` | item | `manual_async_fn` | Pending review | Source inspection required | #2158 | Pending |
| A016 | `packages/axum-http-server/src/v1/extractors/client_ip_sources.rs:56` | item | `manual_async_fn` | Pending review | Source inspection required | #2158 | Pending |
| A017 | `packages/axum-http-server/src/v1/handlers/announce.rs:24` | item | `unused_async` | Pending review | Source inspection required | #2158 | Pending |
| A018 | `packages/axum-http-server/src/v1/handlers/announce.rs:37` | item | `unused_async` | Pending review | Source inspection required | #2158 | Pending |
| A019 | `packages/axum-http-server/src/v1/handlers/health_check.rs:3` | item | `unused_async` | Pending review | Source inspection required | #2158 | Pending |
| A020 | `packages/axum-http-server/src/v1/handlers/scrape.rs:24` | item | `unused_async` | Pending review | Source inspection required | #2158 | Pending |
| A021 | `packages/axum-http-server/src/v1/handlers/scrape.rs:39` | item | `unused_async` | Pending review | Source inspection required | #2158 | Pending |
| A022 | `packages/axum-rest-api-server/src/server.rs:75` | item | `module_name_repetitions` | Pending review | Source inspection required | #2158 | Pending |
| A023 | `packages/axum-rest-api-server/src/server.rs:79` | item | `module_name_repetitions` | Pending review | Source inspection required | #2158 | Pending |
| A024 | `packages/axum-rest-api-server/src/server.rs:87` | item | `module_name_repetitions` | Pending review | Source inspection required | #2158 | Pending |
| A025 | `packages/axum-rest-api-server/src/server.rs:240` | item | `redundant_field_names` | Pending review | Source inspection required | #2158 | Pending |
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
| A088 | `packages/http-protocol/src/v1/responses/announce/data.rs:17` | item | `redundant_field_names` | Pending review | Source inspection required | #2158 | Pending |
| A089 | `packages/http-protocol/src/v1/responses/announce/data.rs:28` | item | `redundant_field_names` | Pending review | Source inspection required | #2158 | Pending |
| A090 | `packages/http-protocol/src/v1/responses/announce/deserialization.rs:82` | item | `chunks_exact_to_as_chunks`, `explicit_iter_loop` | Pending review | Source inspection required | #2158 | Pending |
| A091 | `packages/http-protocol/src/v1/responses/announce/encoding.rs:32` | item | `redundant_field_names` | Pending review | Source inspection required | #2158 | Pending |
| A092 | `packages/http-protocol/src/v1/responses/announce/encoding.rs:68` | item | `from_over_into` | Pending review | Source inspection required | #2158 | Pending |
| A093 | `packages/http-protocol/src/v1/responses/announce/encoding.rs:117` | item | `from_over_into` | Pending review | Source inspection required | #2158 | Pending |
| A094 | `packages/persistence-benchmark/src/bin/persistence_benchmark/runner.rs:1` | crate | `print_stdout` | Pending review | Source inspection required | #2158 | Pending |
| A095 | `packages/primitives/src/announce.rs:15` | item | `redundant_field_names` | Pending review | Source inspection required | #2158 | Pending |
| A096 | `packages/primitives/src/announce.rs:87` | item | `redundant_field_names` | Pending review | Source inspection required | #2158 | Pending |
| A097 | `packages/primitives/src/mode.rs:12` | item | `redundant_field_names` | Pending review | Source inspection required | #2158 | Pending |
| A098 | `packages/primitives/src/pagination.rs:8` | item | `redundant_field_names` | Pending review | Source inspection required | #2158 | Pending |
| A099 | `packages/primitives/src/peer.rs:141` | item | `cast_possible_truncation` | Pending review | Source inspection required | #2158 | Pending |
| A100 | `packages/primitives/src/peer.rs:504` | item | `derivable_impls` | Pending review | Source inspection required | #2158 | Pending |
| A101 | `packages/primitives/src/policy.rs:12` | item | `redundant_field_names` | Pending review | Source inspection required | #2158 | Pending |
| A102 | `packages/primitives/src/swarm_metadata.rs:15` | item | `redundant_field_names` | Pending review | Source inspection required | #2158 | Pending |
| A103 | `packages/rest-api-application/src/v1/ports/auth_key.rs:16` | item | `double_must_use` | Pending review | Source inspection required | #2158 | Pending |
| A104 | `packages/rest-api-application/src/v1/ports/stats.rs:15` | item | `double_must_use` | Pending review | Source inspection required | #2158 | Pending |
| A105 | `packages/rest-api-application/src/v1/ports/torrent.rs:13` | item | `double_must_use` | Pending review | Source inspection required | #2158 | Pending |
| A106 | `packages/rest-api-application/src/v1/ports/whitelist.rs:16` | item | `double_must_use` | Pending review | Source inspection required | #2158 | Pending |
| A107 | `packages/rest-api-client/src/v1/client.rs:226` | item | `struct_field_names` | Pending review | Source inspection required | #2158 | Pending |
| A108 | `packages/rest-api-protocol/src/v1/context/torrent/resources/peer.rs:22` | item | `doc_markdown` | Pending review | Source inspection required | #2158 | Pending |
| A109 | `packages/rest-api-runtime-adapter/src/v1/adapters/auth_key.rs:86` | item | `needless_pass_by_value` | Pending review | Source inspection required | #2158 | Pending |
| A110 | `packages/rest-api-runtime-adapter/src/v1/adapters/stats.rs:18` | item | `struct_field_names` | Pending review | Source inspection required | #2158 | Pending |
| A111 | `packages/rest-api-runtime-adapter/src/v1/adapters/stats.rs:31` | item | `too_many_arguments` | Pending review | Source inspection required | #2158 | Pending |
| A112 | `packages/swarm-coordination-registry/examples/bench_peers.rs:34` | item | `cast_possible_truncation`, `cast_precision_loss`, `cast_sign_loss` | Pending review | Source inspection required | #2158 | Pending |
| A113 | `packages/swarm-coordination-registry/src/statistics/activity_metrics_updater.rs:79` | item | `cast_precision_loss` | Pending review | Source inspection required | #2158 | Pending |
| A114 | `packages/swarm-coordination-registry/src/statistics/activity_metrics_updater.rs:93` | item | `cast_precision_loss` | Pending review | Source inspection required | #2158 | Pending |
| A115 | `packages/swarm-coordination-registry/src/swarm/registry.rs:864` | item | `from_over_into` | Pending review | Source inspection required | #2158 | Pending |
| A116 | `packages/test-helpers/src/logging.rs:130` | item | `unnecessary_wraps` | Pending review | Source inspection required | #2158 | Pending |
| A117 | `packages/test-helpers/src/logging.rs:146` | item | `unnecessary_wraps` | Pending review | Source inspection required | #2158 | Pending |
| A118 | `packages/test-helpers/src/logging.rs:147` | item | `unused_self` | Pending review | Source inspection required | #2158 | Pending |
| A119 | `packages/torrent-repository-benchmarking/benches/helpers/utils.rs:20` | item | `missing_panics_doc` | Pending review | Source inspection required | #2158 | Pending |
| A120 | `packages/torrent-repository-benchmarking/benches/helpers/utils.rs:25` | item | `cast_possible_truncation` | Pending review | Source inspection required | #2158 | Pending |
| A121 | `packages/torrent-repository-benchmarking/src/entry/mod.rs:52` | item | `module_name_repetitions` | Pending review | Source inspection required | #2158 | Pending |
| A122 | `packages/torrent-repository-benchmarking/src/entry/mod.rs:64` | item | `module_name_repetitions` | Pending review | Source inspection required | #2158 | Pending |
| A123 | `packages/torrent-repository-benchmarking/src/entry/single.rs:13` | item | `cast_possible_truncation` | Pending review | Source inspection required | #2158 | Pending |
| A124 | `packages/torrent-repository-benchmarking/src/repository/mod.rs:29` | item | `module_name_repetitions` | Pending review | Source inspection required | #2158 | Pending |
| A125 | `packages/torrent-repository-benchmarking/src/repository/rw_lock_tokio.rs:18` | item | `future_not_send` | Pending review | Source inspection required | #2158 | Pending |
| A126 | `packages/tracker-client/src/http/client/mod.rs:27` | item | `struct_field_names` | Pending review | Source inspection required | #2158 | Pending |
| A127 | `packages/tracker-client/src/udp/client.rs:18` | item | `module_name_repetitions` | Pending review | Source inspection required | #2158 | Pending |
| A128 | `packages/tracker-client/src/udp/client.rs:179` | item | `module_name_repetitions` | Pending review | Source inspection required | #2158 | Pending |
| A129 | `packages/tracker-core/src/announce_handler.rs:277` | item | `cast_sign_loss` | Pending review | Source inspection required | #2158 | Pending |
| A130 | `packages/tracker-core/src/databases/driver/sqlite/mod.rs:34` | item | `unnecessary_wraps` | Pending review | Source inspection required | #2158 | Pending |
| A131 | `packages/tracker-core/src/databases/traits/auth_keys.rs:14` | item | `double_must_use` | Pending review | Source inspection required | #2158 | Pending |
| A132 | `packages/tracker-core/src/databases/traits/auth_keys.rs:16` | item | `struct_field_names`, `extra_unused_lifetimes` | Pending review | Source inspection required | #2158 | Pending |
| A133 | `packages/tracker-core/src/databases/traits/schema.rs:13` | item | `double_must_use` | Pending review | Source inspection required | #2158 | Pending |
| A134 | `packages/tracker-core/src/databases/traits/schema.rs:15` | item | `extra_unused_lifetimes` | Pending review | Source inspection required | #2158 | Pending |
| A135 | `packages/tracker-core/src/databases/traits/torrent_metrics.rs:18` | item | `double_must_use` | Pending review | Source inspection required | #2158 | Pending |
| A136 | `packages/tracker-core/src/databases/traits/torrent_metrics.rs:20` | item | `extra_unused_lifetimes` | Pending review | Source inspection required | #2158 | Pending |
| A137 | `packages/tracker-core/src/databases/traits/whitelist.rs:11` | item | `double_must_use` | Pending review | Source inspection required | #2158 | Pending |
| A138 | `packages/tracker-core/src/databases/traits/whitelist.rs:13` | item | `extra_unused_lifetimes` | Pending review | Source inspection required | #2158 | Pending |
| A139 | `packages/tracker-core/src/error.rs:108` | item | `module_name_repetitions` | Pending review | Source inspection required | #2158 | Pending |
| A140 | `packages/udp-core/benches/helpers/sync.rs:13` | item | `unused_async` | Pending review | Source inspection required | #2158 | Pending |
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
| A169 | `packages/udp-server/examples/udp_only_public_tracker.rs:35` | crate | `print_stdout` | Pending review | Source inspection required | #2158 | Pending |
| A170 | `packages/udp-server/src/banning/event/handler.rs:34` | item | `cast_precision_loss` | Pending review | Source inspection required | #2158 | Pending |
| A171 | `packages/udp-server/src/handlers/announce.rs:132` | item | `cast_possible_truncation` | Pending review | Source inspection required | #2158 | Pending |
| A172 | `packages/udp-server/src/handlers/error.rs:18` | item | `too_many_arguments` | Pending review | Source inspection required | #2158 | Pending |
| A173 | `packages/udp-server/src/server/mod.rs:54` | item | `module_name_repetitions` | Pending review | Source inspection required | #2158 | Pending |
| A174 | `packages/udp-server/src/server/spawner.rs:31` | item | `redundant_field_names` | Pending review | Source inspection required | #2158 | Pending |
| A175 | `packages/udp-server/src/server/states.rs:33` | item | `module_name_repetitions` | Pending review | Source inspection required | #2158 | Pending |
| A176 | `packages/udp-server/src/server/states.rs:37` | item | `module_name_repetitions` | Pending review | Source inspection required | #2158 | Pending |
| A177 | `packages/udp-server/src/server/states.rs:51` | item | `redundant_field_names` | Pending review | Source inspection required | #2158 | Pending |
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
| A228 | `console/tracker-client/src/bin/http_tracker_client.rs:1` | crate | `print_stderr` | Pending review | Source inspection required | #2158 | Pending |
| A229 | `console/tracker-client/src/bin/tracker_checker.rs:1` | crate | `print_stderr`, `exit` | Pending review | Source inspection required | #2158 | Pending |
| A230 | `console/tracker-client/src/bin/tracker_client.rs:1` | crate | `print_stderr`, `exit` | Pending review | Source inspection required | #2158 | Pending |
| A231 | `console/tracker-client/src/bin/udp_tracker_client.rs:1` | crate | `print_stderr` | Pending review | Source inspection required | #2158 | Pending |
| A232 | `console/tracker-client/src/console/clients/checker/checks/udp.rs:26` | item | `missing_panics_doc` | Pending review | Source inspection required | #2158 | Pending |
| A233 | `console/tracker-client/src/console/clients/udp/responses/json.rs:5` | item | `module_name_repetitions` | Pending review | Source inspection required | #2158 | Pending |
| A234 | `console/tracker-client/src/lib.rs:5` | crate | `print_stdout`, `print_stderr` | Pending review | Source inspection required | #2158 | Pending |
