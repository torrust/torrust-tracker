---
doc-type: manual-verification-evidence
issue-spec: docs/issues/open/2925-2392-migrate-tests-to-guarded-sqlite-database/ISSUE.md
last-updated-utc: "2026-11-26 16:04"
---

# Manual Verification Evidence

## Purpose

Record real, human-oriented verification of the completed behavior. This is evidence from commands
actually performed; nothing here is invented.

## Preserving Verification Artifacts

The runs wrote only files under a fresh temporary directory. The counts and the failing-test
classification are embedded below; the random file names are not recorded (they are random).

## Environment and Prerequisites

- Date and time (UTC): 2026-11-26; V1 08:05:50 to 08:07:08, V1c finished 08:07:20 (the time of its
  output file), V1b rerun 16:01:51 to 16:03:05. The first V1b run (finished 08:05:15 by the
  modification time of its log) is replaced by the rerun; its times were not stamped.
- Artifact under test: `torrust/develop` at `f5329f758`, no code change for V1 and V1c; V1b
  changed one line of `ephemeral()` in the working tree and restored it
- Operating system / environment: Linux 7.0.0-44-generic; `cargo +stable` 1.100.0
- Prerequisites and setup performed: a fresh directory from `mktemp -d`, exported as `TMPDIR`
  inside the script that ran the command

## Verification Processes

### V1 - Reproduce (before)

- Goal: count what the full suite leaves in a fresh `TMPDIR`.
- Initial state: empty directory.
- Status: `DONE`

#### Steps Performed

One script ran the steps in order: `export TMPDIR=$(mktemp -d)` (08:05:50 UTC), then
`cargo +stable test --tests --benches --examples --workspace --all-targets --all-features --no-fail-fast`
(exit 0, finished 08:07:08), then `ls "$TMPDIR" | grep -c '^data_.*\.db$'` and `ls "$TMPDIR" | wc -l`.

#### Observed Result

```text
data_db=249 total=249
```

#### Conclusion

The suite leaves 249 `data_<16 characters>.db` files and nothing else (no guard directory, no
TOML or TLS file: #2907 and #2924 already removed those). #2907 counted 301 before those fixes;
the `ephemeral*()` configurations account for these 249.

### V1b - Classification experiment (no database in `ephemeral()`)

- Goal: find the tests that fail when `ephemeral()` sets no database, and count what the suite
  leaves then.
- Status: `DONE`

#### Steps Performed

The first V1b run set `TMPDIR` inline for `cargo` only and then looked for the directory by name
afterwards, so its count of 0 did not provably describe the run's directory (review thread on
this file, CodeRabbit and da2ce7). It was replaced by this rerun, one script run in order:

1. `git apply` of the one-line patch that sets `config.core.database = None;` in `ephemeral()`
   (the patch is the T4 change and is not kept).
2. `export TMPDIR=$(mktemp -d)` (16:01:51 UTC, directory `/tmp/tmp.NdIoYb6g2u`).
3. `cargo +stable test --tests --benches --examples --workspace --all-targets --all-features --no-fail-fast`
   (exit 101 because tests fail, finished 16:03:05).
4. `ls -A "$TMPDIR" | wc -l` and `ls "$TMPDIR" | grep -c '^data_.*\.db$'` in the same shell.
5. `git checkout packages/test-helpers/src/configuration.rs`, then
   `git diff --quiet HEAD -- packages/test-helpers/src/configuration.rs` exited 0 (16:03:05).

#### Observed Result

```text
entries=0 data_db=0
```

77 tests failed (`grep -cE "^test .* FAILED$"` on the log), in 7 test binaries and the
`http_tracker_core_benchmark` bench (8 `error: test failed` lines). `tracker-core` does not fail:
it already uses its own guard-owned wrappers.

#### Conclusion

The failing tests, with the class each one belongs to, are listed in the appendix (taken from the
first run; the rerun counts the same 77 failures and 8 failing binaries). 48 need persistence, 29
only run a helper that unwraps the database configuration. The reasons seen in the panic
messages: `database configuration is required to initialize persistence` (`initialize_database`,
30), tests of the REST API, the HTTP tracker and the `app` module that expect persistence to be
composed (the rest), and `HttpTrackerCoreContainer::initialize` falling back to the default
SQLite path (2, "unable to open database file"). With no database in `ephemeral()` the suite
leaves no entry in the temporary directory.

### V1c - Per-test attribution of the `http-core` leaks (#2924 recheck)

- Goal: attribute the 6 `data_*.db` files the `http-core` binaries leave (#2924 recheck).
- Status: `DONE`

#### Steps Performed

Run at about 08:07 UTC (output file time 08:07:20); start and end were not stamped. Each test of the `http-core` lib binary was run alone (`<binary> --exact <test>`) with its own
`mktemp -d` `TMPDIR`, counting `data_*.db`; the bench binary was run once the same way.

#### Observed Result

```text
1 container::tests::the_container_initialized_from_the_core_configuration::it_should_compose_the_tracker_core_over_the_configured_database_with_the_given_core_configuration
1 container::tests::the_container_initialized_from_the_core_configuration::it_should_publish_the_events_of_its_instance_with_the_public_url_to_its_own_event_bus
bench http_tracker_core_benchmark: 4 files
```

#### Conclusion

6 = 2 (the two container tests) + 4 (the benchmark, `benches/helpers/util.rs`). No other
`http-core` test leaves a file.

## Failures and Follow-up

No process failed. V1b was rerun once (see its steps) because the first run's directory was not
provably the one counted. The V1c run has no start and end stamps; its result is a per-test count
that the output file records.

## Appendix: tests that fail without a database (V1b)

### torrust-tracker (--lib), 3 tests

| Test | Class |
| --- | --- |
| `app::tests::it_should_stop_every_started_component_cooperatively_and_each_producer_before_its_listeners_when_the_manager_is_cancelled` | needs persistence |
| `app::tests::it_should_subscribe_every_started_swarm_listener_before_its_task_runs_or_any_producer_starts` | needs persistence |
| `app::tests::it_should_retain_the_loader_error_when_initial_peer_key_loading_fails` | needs persistence |

### torrust-tracker-axum-http-server (--lib), 5 tests

| Test | Class |
| --- | --- |
| `v1::handlers::announce::tests::with_tracker_on_reverse_proxy::it_should_fail_when_the_right_most_x_forwarded_for_header_ip_is_not_available` | helper only |
| `v1::handlers::announce::tests::with_tracker_in_private_mode::it_should_fail_when_the_authentication_key_is_missing` | helper only |
| `v1::handlers::announce::tests::with_tracker_not_on_reverse_proxy::it_should_fail_when_the_client_ip_from_the_connection_info_is_not_available` | helper only |
| `v1::handlers::announce::tests::with_tracker_in_listed_mode::it_should_fail_when_the_announced_torrent_is_not_whitelisted` | helper only |
| `v1::handlers::announce::tests::with_tracker_in_private_mode::it_should_fail_when_the_authentication_key_is_invalid` | helper only |

### torrust-tracker-axum-http-server (tests/integration.rs), 9 tests

| Test | Class |
| --- | --- |
| `server::v1::contract::configured_as_private_and_whitelisted::receiving_an_scrape_request::it_should_return_listed_torrent_stats_to_an_authenticated_peer` | needs persistence |
| `server::v1::contract::configured_as_private::receiving_an_scrape_request::should_return_the_real_file_stats_when_the_client_is_authenticated` | needs persistence |
| `server::v1::contract::configured_as_whitelisted::and_receiving_an_announce_request::should_allow_announcing_a_whitelisted_torrent` | needs persistence |
| `server::v1::contract::configured_as_private_and_whitelisted::receiving_an_scrape_request::it_should_hide_listed_torrent_stats_from_an_unauthenticated_peer` | needs persistence |
| `server::v1::contract::configured_as_private::and_receiving_an_announce_request::should_respond_to_authenticated_peers` | needs persistence |
| `server::v1::contract::configured_as_private_and_whitelisted::and_receiving_an_announce_request::it_should_return_a_whitelist_error_for_an_authenticated_peer_announcing_an_unlisted_torrent` | needs persistence |
| `server::v1::contract::configured_as_private_and_whitelisted::and_receiving_an_announce_request::it_should_respond_to_an_authenticated_peer_announcing_a_listed_torrent` | needs persistence |
| `server::v1::contract::configured_as_whitelisted::receiving_an_scrape_request::should_return_the_file_stats_when_the_requested_file_is_whitelisted` | needs persistence |
| `server::v1::contract::configured_as_private_and_whitelisted::receiving_an_scrape_request::it_should_hide_unlisted_torrent_stats_from_an_authenticated_peer` | needs persistence |

### torrust-tracker-axum-rest-api-server (--lib), 2 tests

| Test | Class |
| --- | --- |
| `v1::routes::tests::it_should_serve_the_whitelist_routes_when_the_tracker_is_listed_and_has_persistence` | needs persistence |
| `v1::routes::tests::it_should_serve_the_auth_key_routes_when_the_tracker_is_private_and_has_persistence` | needs persistence |

### torrust-tracker-axum-rest-api-server (tests/integration.rs), 31 tests

| Test | Class |
| --- | --- |
| `server::v1::contract::context::auth_key::should_allow_deleting_an_auth_key` | needs persistence |
| `server::v1::contract::context::auth_key::should_not_allow_reloading_keys_for_unauthenticated_users` | needs persistence |
| `server::v1::contract::context::auth_key::should_fail_deleting_an_auth_key_when_the_key_id_is_invalid` | needs persistence |
| `server::v1::contract::context::auth_key::should_allow_generating_a_new_random_auth_key` | needs persistence |
| `server::v1::contract::context::auth_key::should_reject_auth_key_requests_when_private_mode_is_disabled_without_database_access` | needs persistence |
| `server::v1::contract::context::auth_key::should_fail_generating_a_new_auth_key_when_the_provided_key_is_invalid` | needs persistence |
| `server::v1::contract::context::auth_key::should_fail_when_the_auth_key_cannot_be_deleted` | needs persistence |
| `server::v1::contract::context::auth_key::deprecated_generate_key_endpoint::should_fail_when_the_auth_key_cannot_be_generated` | needs persistence |
| `server::v1::contract::context::auth_key::should_fail_without_a_source_path_when_the_same_auth_key_is_uploaded_twice` | needs persistence |
| `server::v1::contract::context::auth_key::should_fail_adding_a_key_that_already_exists_without_echoing_it` | needs persistence |
| `server::v1::contract::context::auth_key::should_fail_when_keys_cannot_be_reloaded` | needs persistence |
| `server::v1::contract::context::auth_key::should_not_allow_deleting_an_auth_key_for_unauthenticated_users` | needs persistence |
| `server::v1::contract::context::auth_key::should_fail_when_the_auth_key_cannot_be_generated` | needs persistence |
| `server::v1::contract::context::stats::should_expose_an_enabled_zero_value_persisted_metric` | needs persistence |
| `server::v1::contract::context::auth_key::should_fail_without_a_source_path_when_the_auth_key_to_delete_does_not_exist` | needs persistence |
| `server::v1::contract::context::auth_key::should_allow_reloading_keys` | needs persistence |
| `server::v1::contract::context::auth_key::should_allow_uploading_a_preexisting_auth_key` | needs persistence |
| `server::v1::contract::context::auth_key::deprecated_generate_key_endpoint::should_fail_generating_a_new_auth_key_when_the_key_duration_is_invalid` | needs persistence |
| `server::v1::contract::context::auth_key::deprecated_generate_key_endpoint::should_allow_generating_a_new_auth_key` | needs persistence |
| `server::v1::contract::context::whitelist::should_allow_reload_the_whitelist_from_the_database` | needs persistence |
| `server::v1::contract::context::whitelist::should_allow_removing_a_torrent_from_the_whitelist` | needs persistence |
| `server::v1::contract::context::whitelist::should_allow_whitelisting_a_torrent` | needs persistence |
| `server::v1::contract::context::whitelist::should_allow_whitelisting_a_torrent_that_has_been_already_whitelisted` | needs persistence |
| `server::v1::contract::context::whitelist::should_fail_removing_a_torrent_from_the_whitelist_when_the_provided_infohash_is_invalid` | needs persistence |
| `server::v1::contract::context::whitelist::should_fail_when_the_torrent_cannot_be_removed_from_the_whitelist` | needs persistence |
| `server::v1::contract::context::whitelist::should_fail_when_the_torrent_cannot_be_whitelisted` | needs persistence |
| `server::v1::contract::context::whitelist::should_fail_when_the_whitelist_cannot_be_reloaded_from_the_database` | needs persistence |
| `server::v1::contract::context::whitelist::should_fail_whitelisting_a_torrent_when_the_provided_infohash_is_invalid` | needs persistence |
| `server::v1::contract::context::whitelist::should_not_allow_removing_a_torrent_from_the_whitelist_for_unauthenticated_users` | needs persistence |
| `server::v1::contract::context::whitelist::should_not_fail_trying_to_remove_a_non_whitelisted_torrent_from_the_whitelist` | needs persistence |
| `server::v1::contract::context::whitelist::should_reject_whitelist_requests_when_listed_mode_is_disabled_without_database_access` | needs persistence |

### torrust-tracker-http-core (--lib), 2 tests

| Test | Class |
| --- | --- |
| `container::tests::the_container_initialized_from_the_core_configuration::it_should_publish_the_events_of_its_instance_with_the_public_url_to_its_own_event_bus` | needs persistence |
| `container::tests::the_container_initialized_from_the_core_configuration::it_should_compose_the_tracker_core_over_the_configured_database_with_the_given_core_configuration` | needs persistence |

### torrust-tracker-udp-server (--lib), 25 tests

| Test | Class |
| --- | --- |
| `handlers::announce::tests::announce_request::using_ipv4::it_should_reject_an_announce_with_a_negative_number_of_bytes_left` | helper only |
| `handlers::announce::tests::announce_request::it_should_encode_only_peer_addresses_matching_the_request_family` | helper only |
| `handlers::announce::tests::announce_request::using_ipv4::an_announced_peer_should_be_added_to_the_tracker` | helper only |
| `handlers::announce::tests::announce_request::using_ipv4::from_a_loopback_ip::each_listener_should_use_its_own_configured_external_ip` | helper only |
| `handlers::announce::tests::announce_request::using_ipv4::should_send_the_udp4_announce_event` | helper only |
| `handlers::announce::tests::announce_request::using_ipv4::the_announced_peer_should_not_be_included_in_the_response` | helper only |
| `handlers::announce::tests::announce_request::using_ipv4::the_tracker_should_always_use_the_remote_client_ip_but_not_the_port_in_the_udp_request_header_instead_of_the_peer_address_in_the_announce_request` | helper only |
| `handlers::announce::tests::announce_request::using_ipv4::it_should_preserve_an_announce_service_failure_for_packet_error_routing` | helper only |
| `handlers::announce::tests::announce_request::using_ipv6::an_announced_peer_should_be_added_to_the_tracker` | helper only |
| `handlers::announce::tests::announce_request::using_ipv4::when_the_announce_request_comes_from_a_client_using_ipv4_the_response_should_not_include_peers_using_ipv6` | helper only |
| `handlers::announce::tests::announce_request::using_ipv6::the_announced_peer_should_not_be_included_in_the_response` | helper only |
| `handlers::announce::tests::announce_request::using_ipv6::should_send_the_udp6_announce_event` | helper only |
| `handlers::announce::tests::announce_request::using_ipv6::the_tracker_should_always_use_the_remote_client_ip_but_not_the_port_in_the_udp_request_header_instead_of_the_peer_address_in_the_announce_request` | helper only |
| `handlers::announce::tests::announce_request::using_ipv6::when_the_announce_request_comes_from_a_client_using_ipv6_the_response_should_not_include_peers_using_ipv4` | helper only |
| `handlers::scrape::tests::scrape_request::it_should_preserve_a_scrape_service_failure_for_packet_error_routing` | helper only |
| `handlers::scrape::tests::scrape_request::it_should_preserve_the_order_of_eight_requested_info_hashes` | helper only |
| `handlers::scrape::tests::scrape_request::it_should_publish_an_accepted_scrape_event_for_an_ipv4_client` | helper only |
| `handlers::scrape::tests::scrape_request::it_should_publish_a_cookie_error_and_answer_when_cookie_validation_is_disabled` | helper only |
| `handlers::scrape::tests::scrape_request::it_should_publish_an_accepted_scrape_event_for_an_ipv6_client` | helper only |
| `handlers::scrape::tests::scrape_request::it_should_return_an_entry_for_each_duplicate_requested_info_hash` | helper only |
| `handlers::scrape::tests::scrape_request::it_should_return_the_persisted_downloads_of_torrents_absent_from_memory_in_request_order` | needs persistence |
| `handlers::scrape::tests::scrape_request::it_should_return_zeroed_statistics_when_the_tracker_does_not_have_the_requested_torrent` | helper only |
| `handlers::scrape::tests::scrape_request::with_a_public_tracker::it_should_return_statistics_when_the_public_tracker_has_the_requested_torrent` | helper only |
| `handlers::scrape::tests::scrape_request::with_a_whitelisted_tracker::it_should_return_statistics_when_the_listed_tracker_has_a_whitelisted_torrent` | helper only |
| `handlers::scrape::tests::scrape_request::with_a_whitelisted_tracker::it_should_return_zeroed_statistics_when_the_listed_tracker_has_not_whitelisted_the_torrent` | helper only |
