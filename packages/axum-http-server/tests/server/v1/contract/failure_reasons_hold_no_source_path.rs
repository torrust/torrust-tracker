//! The `failure reason` of an HTTP tracker response reaches the `BitTorrent`
//! client, which shows it to its user. It must name what failed and hold no
//! source path of the tracker (see
//! `docs/adrs/20261118045443_errors_do_not_record_source_locations.md`).
//!
//! One test per failure family of request parsing, key extraction, peer-IP
//! resolution, whitelist authorization and key authentication, through a
//! running HTTP tracker.
use std::sync::Arc;
use std::time::Duration;

use torrust_tracker_axum_http_server::testing::environment::Started;
use torrust_tracker_client::http::client::Client;
use torrust_tracker_configuration::v3_0_0::Configuration;
use torrust_tracker_test_helpers::assertions::assert_names_no_source_path;
use torrust_tracker_test_helpers::{configuration, logging};

use crate::server::asserts::failure_reason_of;

const VALID_INFO_HASH: &str = "%3B%24U%04%CF%5F%11%BB%DB%E1%20%1C%EAjk%F4Z%EE%1B%C0";
const VALID_PEER_ID: &str = "-qB00000000000000001";
const SHORT_INFO_HASH: &str = "0123456789";
const SHORT_PEER_ID: &str = "-qB0000000000000000";

/// Starts an HTTP tracker with `cfg`, sends `GET /{path_and_query}` and returns
/// the `failure reason` of the response.
async fn failure_reason_for(cfg: Configuration, path_and_query: &str) -> String {
    logging::setup();

    let core_config = Arc::new(cfg.core.clone());
    let http_tracker_config = Arc::new(cfg.http_trackers.unwrap()[0].clone());
    let env = Started::new(&core_config, &http_tracker_config).await;

    let response = Client::new(env.base_url(), Duration::from_secs(5))
        .unwrap()
        .get(path_and_query)
        .await
        .unwrap();
    let failure_reason = failure_reason_of(response).await;

    env.stop().await;

    failure_reason
}

#[tokio::test]
async fn an_announce_with_an_unparsable_query() {
    let failure_reason = failure_reason_for(configuration::ephemeral_public(), "announce?a=b=c").await;

    assert_names_no_source_path(&failure_reason);
    assert!(failure_reason.contains("invalid param a=b=c"), "{failure_reason:?}");
}

#[tokio::test]
async fn an_announce_without_query_params() {
    let failure_reason = failure_reason_for(configuration::ephemeral_public(), "announce").await;

    assert_names_no_source_path(&failure_reason);
    assert!(
        failure_reason.contains("missing query params for announce request"),
        "{failure_reason:?}"
    );
}

#[tokio::test]
async fn an_announce_without_the_info_hash_param() {
    let failure_reason = failure_reason_for(
        configuration::ephemeral_public(),
        &format!("announce?peer_id={VALID_PEER_ID}&port=17548"),
    )
    .await;

    assert_names_no_source_path(&failure_reason);
    assert!(failure_reason.contains("missing param info_hash"), "{failure_reason:?}");
}

#[tokio::test]
async fn an_announce_with_an_invalid_numwant_param() {
    let failure_reason = failure_reason_for(
        configuration::ephemeral_public(),
        &format!("announce?info_hash={VALID_INFO_HASH}&peer_id={VALID_PEER_ID}&port=17548&numwant=x"),
    )
    .await;

    assert_names_no_source_path(&failure_reason);
    assert!(
        failure_reason.contains("invalid param value x for numwant"),
        "{failure_reason:?}"
    );
}

#[tokio::test]
async fn an_announce_with_a_short_info_hash() {
    let failure_reason = failure_reason_for(
        configuration::ephemeral_public(),
        &format!("announce?info_hash={SHORT_INFO_HASH}&peer_id={VALID_PEER_ID}&port=17548"),
    )
    .await;

    assert_names_no_source_path(&failure_reason);
    assert!(
        failure_reason.contains(&format!("invalid param value {SHORT_INFO_HASH} for info_hash")),
        "{failure_reason:?}"
    );
}

#[tokio::test]
async fn an_announce_with_a_short_peer_id() {
    let failure_reason = failure_reason_for(
        configuration::ephemeral_public(),
        &format!("announce?info_hash={VALID_INFO_HASH}&peer_id={SHORT_PEER_ID}&port=17548"),
    )
    .await;

    assert_names_no_source_path(&failure_reason);
    assert!(
        failure_reason.contains(&format!("invalid param value {SHORT_PEER_ID} for peer_id")),
        "{failure_reason:?}"
    );
}

#[tokio::test]
async fn a_scrape_without_query_params() {
    let failure_reason = failure_reason_for(configuration::ephemeral_public(), "scrape").await;

    assert_names_no_source_path(&failure_reason);
    assert!(
        failure_reason.contains("missing query params for scrape request"),
        "{failure_reason:?}"
    );
}

#[tokio::test]
async fn a_scrape_without_the_info_hash_param() {
    let failure_reason = failure_reason_for(configuration::ephemeral_public(), "scrape?another_param=NOT_RELEVANT").await;

    assert_names_no_source_path(&failure_reason);
    assert!(failure_reason.contains("missing param info_hash"), "{failure_reason:?}");
}

#[tokio::test]
async fn a_scrape_with_a_short_info_hash() {
    let failure_reason = failure_reason_for(
        configuration::ephemeral_public(),
        &format!("scrape?info_hash={SHORT_INFO_HASH}"),
    )
    .await;

    assert_names_no_source_path(&failure_reason);
    assert!(
        failure_reason.contains(&format!("invalid param value {SHORT_INFO_HASH} for info_hash")),
        "{failure_reason:?}"
    );
}

#[tokio::test]
async fn an_announce_to_a_private_tracker_with_a_malformed_key() {
    let failure_reason = failure_reason_for(
        configuration::ephemeral_private(),
        &format!("announce/INVALID_KEY?info_hash={VALID_INFO_HASH}&peer_id={VALID_PEER_ID}&port=17548"),
    )
    .await;

    assert_names_no_source_path(&failure_reason);
    assert!(
        failure_reason.contains("Invalid authentication key: expected 32 characters, got 11"),
        "{failure_reason:?}"
    );
    assert!(!failure_reason.contains("INVALID_KEY"), "the failure reason holds the key");
}

#[tokio::test]
async fn an_announce_to_a_tracker_behind_a_reverse_proxy_without_the_x_forwarded_for_header() {
    let failure_reason = failure_reason_for(
        configuration::ephemeral_with_reverse_proxy(),
        &format!("announce?info_hash={VALID_INFO_HASH}&peer_id={VALID_PEER_ID}&port=17548"),
    )
    .await;

    assert_names_no_source_path(&failure_reason);
    assert!(failure_reason.contains("X-Forwarded-For"), "{failure_reason:?}");
}

#[tokio::test]
async fn an_announce_to_a_listed_tracker_for_a_torrent_that_is_not_whitelisted() {
    let failure_reason = failure_reason_for(
        configuration::ephemeral_listed(),
        &format!("announce?info_hash={VALID_INFO_HASH}&peer_id={VALID_PEER_ID}&port=17548"),
    )
    .await;

    assert_names_no_source_path(&failure_reason);
    assert!(
        failure_reason.contains("The torrent: 3b245504cf5f11bbdbe1201cea6a6bf45aee1bc0, is not whitelisted"),
        "{failure_reason:?}"
    );
}

#[tokio::test]
async fn an_announce_to_a_private_tracker_without_a_key() {
    let failure_reason = failure_reason_for(
        configuration::ephemeral_private(),
        &format!("announce?info_hash={VALID_INFO_HASH}&peer_id={VALID_PEER_ID}&port=17548"),
    )
    .await;

    assert_names_no_source_path(&failure_reason);
    assert!(
        failure_reason.contains("Missing authentication key: this tracker is private, use /announce/<key>"),
        "{failure_reason:?}"
    );
}

#[tokio::test]
async fn an_announce_to_a_private_tracker_with_an_unregistered_key() {
    let failure_reason = failure_reason_for(
        configuration::ephemeral_private(),
        &format!("announce/YZSl4lMZupRuOpSRC3krIKR5BPB14nrJ?info_hash={VALID_INFO_HASH}&peer_id={VALID_PEER_ID}&port=17548"),
    )
    .await;

    assert_names_no_source_path(&failure_reason);
    assert!(
        failure_reason.contains("Authentication key not registered"),
        "{failure_reason:?}"
    );
    assert!(
        !failure_reason.contains("YZSl4lMZupRuOpSRC3krIKR5BPB14nrJ"),
        "the failure reason holds the key"
    );
}
