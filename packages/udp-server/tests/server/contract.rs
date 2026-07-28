// UDP tracker documentation:
//
// BEP 15. UDP Tracker Protocol for BitTorrent
// https://www.bittorrent.org/beps/bep_0015.html

use core::panic;
use std::sync::Arc;
use std::time::Duration;

use torrust_tracker_client::udp::client::UdpTrackerClient;
use torrust_tracker_test_helpers::{configuration, logging};
use torrust_tracker_udp_protocol::{ConnectRequest, ConnectionId, MAX_PACKET_SIZE, Response, TransactionId};

use crate::server::asserts::get_error_response_message;

const DEFAULT_UDP_TIMEOUT: Duration = Duration::from_secs(5);

fn empty_udp_request() -> [u8; MAX_PACKET_SIZE] {
    [0; MAX_PACKET_SIZE]
}

async fn send_connection_request(transaction_id: TransactionId, client: &UdpTrackerClient) -> ConnectionId {
    let connect_request = ConnectRequest { transaction_id };

    match client.send(connect_request.into()).await {
        Ok(_) => (),
        Err(err) => panic!("{err}"),
    }

    let response = match client.receive().await {
        Ok(response) => response,
        Err(err) => panic!("{err}"),
    };

    match response {
        Response::Connect(connect_response) => connect_response.connection_id,
        _ => panic!("error connecting to udp server {response:?}"),
    }
}

#[tokio::test]
async fn should_return_a_bad_request_response_when_the_client_sends_an_empty_request() {
    logging::setup();

    let cfg = configuration::ephemeral();
    let core_config = Arc::new(cfg.core.clone());
    let udp_tracker_config = Arc::new(cfg.udp_trackers.unwrap()[0].clone());
    let env = torrust_tracker_udp_server::testing::environment::Started::new(&core_config, &udp_tracker_config).await;

    let client = match UdpTrackerClient::new(env.bind_address(), DEFAULT_UDP_TIMEOUT).await {
        Ok(udp_client) => udp_client,
        Err(err) => panic!("{err}"),
    };

    match client.client.send(&empty_udp_request()).await {
        Ok(_) => (),
        Err(err) => panic!("{err}"),
    }

    let response = match client.client.receive().await {
        Ok(response) => response,
        Err(err) => panic!("{err}"),
    };

    let response = Response::parse_bytes(&response, true).unwrap();

    assert!(
        get_error_response_message(&response)
            .unwrap()
            .contains("Protocol identifier missing")
    );

    env.stop().await;
}

mod receiving_a_connection_request {
    use std::sync::Arc;

    use torrust_tracker_client::udp::client::UdpTrackerClient;
    use torrust_tracker_test_helpers::{configuration, logging};
    use torrust_tracker_udp_protocol::{ConnectRequest, TransactionId};

    use super::DEFAULT_UDP_TIMEOUT;
    use crate::server::asserts::is_connect_response;

    #[tokio::test]
    async fn should_return_a_connect_response() {
        logging::setup();

        let cfg = configuration::ephemeral();
        let core_config = Arc::new(cfg.core.clone());
        let udp_tracker_config = Arc::new(cfg.udp_trackers.unwrap()[0].clone());
        let env = torrust_tracker_udp_server::testing::environment::Started::new(&core_config, &udp_tracker_config).await;

        let client = match UdpTrackerClient::new(env.bind_address(), DEFAULT_UDP_TIMEOUT).await {
            Ok(udp_tracker_client) => udp_tracker_client,
            Err(err) => panic!("{err}"),
        };

        let connect_request = ConnectRequest {
            transaction_id: TransactionId::new(123),
        };

        match client.send(connect_request.into()).await {
            Ok(_) => (),
            Err(err) => panic!("{err}"),
        }

        let response = match client.receive().await {
            Ok(response) => response,
            Err(err) => panic!("{err}"),
        };

        assert!(is_connect_response(&response, TransactionId::new(123)));

        env.stop().await;
    }
}

mod receiving_an_announce_request {
    use std::net::Ipv4Addr;
    use std::sync::Arc;

    use torrust_peer_id::PeerId;
    use torrust_tracker_client::udp::client::UdpTrackerClient;
    use torrust_tracker_test_helpers::logging::logs_contains_a_line_with;
    use torrust_tracker_test_helpers::{configuration, logging};
    use torrust_tracker_udp_protocol::{
        AnnounceActionPlaceholder, AnnounceEvent, AnnounceRequest, ConnectionId, InfoHash, NumberOfBytes, NumberOfPeers, PeerKey,
        Port, TransactionId,
    };

    use super::DEFAULT_UDP_TIMEOUT;
    use crate::common::fixtures::{random_info_hash, random_transaction_id};
    use crate::server::asserts::is_ipv4_announce_response;
    use crate::server::contract::send_connection_request;

    pub async fn assert_send_and_get_announce(
        tx_id: TransactionId,
        c_id: ConnectionId,
        info_hash: torrust_info_hash::InfoHash,
        client: &UdpTrackerClient,
    ) {
        let response = send_and_get_announce(tx_id, c_id, info_hash, client).await;
        assert!(is_ipv4_announce_response(&response));
    }

    pub async fn send_and_get_announce(
        tx_id: TransactionId,
        c_id: ConnectionId,
        info_hash: torrust_info_hash::InfoHash,
        client: &UdpTrackerClient,
    ) -> torrust_tracker_udp_protocol::Response {
        let announce_request =
            build_sample_announce_request(tx_id, c_id, client.client.socket.local_addr().unwrap().port(), info_hash);

        match client.send(announce_request.into()).await {
            Ok(_) => (),
            Err(err) => panic!("{err}"),
        }

        match client.receive().await {
            Ok(response) => response,
            Err(err) => panic!("{err}"),
        }
    }

    fn build_sample_announce_request(
        tx_id: TransactionId,
        c_id: ConnectionId,
        port: u16,
        info_hash: torrust_info_hash::InfoHash,
    ) -> AnnounceRequest {
        AnnounceRequest {
            connection_id: ConnectionId(c_id.0),
            action_placeholder: AnnounceActionPlaceholder::default(),
            transaction_id: tx_id,
            info_hash: InfoHash(info_hash.0),
            peer_id: PeerId([255u8; 20]),
            bytes_downloaded: NumberOfBytes(0i64.into()),
            bytes_uploaded: NumberOfBytes(0i64.into()),
            bytes_left: NumberOfBytes(0i64.into()),
            event: AnnounceEvent::Started.into(),
            ip_address: Ipv4Addr::UNSPECIFIED.into(),
            key: PeerKey::new(0i32),
            peers_wanted: NumberOfPeers(1i32.into()),
            port: Port(port.into()),
        }
    }

    #[tokio::test]
    async fn should_return_an_announce_response() {
        logging::setup();

        let cfg = configuration::ephemeral();
        let core_config = Arc::new(cfg.core.clone());
        let udp_tracker_config = Arc::new(cfg.udp_trackers.unwrap()[0].clone());
        let env = torrust_tracker_udp_server::testing::environment::Started::new(&core_config, &udp_tracker_config).await;

        let client = match UdpTrackerClient::new(env.bind_address(), DEFAULT_UDP_TIMEOUT).await {
            Ok(udp_tracker_client) => udp_tracker_client,
            Err(err) => panic!("{err}"),
        };

        let tx_id = TransactionId::new(123);

        let c_id = send_connection_request(tx_id, &client).await;

        let info_hash = random_info_hash();

        assert_send_and_get_announce(tx_id, c_id, info_hash, &client).await;

        env.stop().await;
    }

    #[tokio::test]
    async fn should_return_many_announce_response() {
        logging::setup();

        let cfg = configuration::ephemeral();
        let core_config = Arc::new(cfg.core.clone());
        let udp_tracker_config = Arc::new(cfg.udp_trackers.unwrap()[0].clone());
        let env = torrust_tracker_udp_server::testing::environment::Started::new(&core_config, &udp_tracker_config).await;

        let client = match UdpTrackerClient::new(env.bind_address(), DEFAULT_UDP_TIMEOUT).await {
            Ok(udp_tracker_client) => udp_tracker_client,
            Err(err) => panic!("{err}"),
        };

        let tx_id = TransactionId::new(123);

        let c_id = send_connection_request(tx_id, &client).await;

        let info_hash = random_info_hash();

        for x in 0..1000 {
            tracing::info!("req no: {x}");
            assert_send_and_get_announce(tx_id, c_id, info_hash, &client).await;
        }

        env.stop().await;
    }

    #[tokio::test]
    async fn should_ban_the_client_ip_if_it_sends_more_than_10_requests_with_a_cookie_value_not_normal() {
        logging::setup();

        let cfg = configuration::ephemeral();
        let core_config = Arc::new(cfg.core.clone());
        let udp_tracker_config = Arc::new(cfg.udp_trackers.unwrap()[0].clone());
        let env = torrust_tracker_udp_server::testing::environment::Started::new(&core_config, &udp_tracker_config).await;
        let ban_service = env.container.udp_tracker_core_container.ban_service.clone();

        let client = match UdpTrackerClient::new(env.bind_address(), DEFAULT_UDP_TIMEOUT).await {
            Ok(udp_tracker_client) => udp_tracker_client,
            Err(err) => panic!("{err}"),
        };

        let udp_banned_ips_total_before = ban_service.read().await.get_banned_ips_total();

        // The eleven first requests should be fine

        let invalid_connection_id = ConnectionId::new(0); // Zero is one of the not normal values.

        let info_hash = random_info_hash();

        for x in 0..=10 {
            tracing::info!("req no: {x}");

            let tx_id = random_transaction_id();

            send_and_get_announce(tx_id, invalid_connection_id, info_hash, &client).await;

            let transaction_id = tx_id.0.to_string();

            assert!(
                logs_contains_a_line_with(&["WARN", "UDP TRACKER", &transaction_id]),
                "Expected logs to contain: WARN ... UDP TRACKER ... transaction_id={transaction_id}"
            );
        }

        // The twelfth request should be banned (timeout error)

        let tx_id = random_transaction_id();

        let announce_request = build_sample_announce_request(
            tx_id,
            invalid_connection_id,
            client.client.socket.local_addr().unwrap().port(),
            info_hash,
        );

        let udp_requests_banned_before = env
            .container
            .udp_tracker_server_container
            .stats_repository
            .get_stats()
            .await
            .udp_requests_banned_total();

        // This should return a timeout error
        match client.send(announce_request.into()).await {
            Ok(_) => (),
            Err(err) => panic!("{err}"),
        }

        assert!(client.receive().await.is_err());

        let udp_requests_banned_after = env
            .container
            .udp_tracker_server_container
            .stats_repository
            .get_stats()
            .await
            .udp_requests_banned_total();
        let udp_banned_ips_total_after = ban_service.read().await.get_banned_ips_total();

        // UDP counter for banned requests should be increased by 1
        assert_eq!(udp_requests_banned_after, udp_requests_banned_before + 1);

        // UDP counter for banned IPs should be increased by 1
        assert_eq!(udp_banned_ips_total_after, udp_banned_ips_total_before + 1);

        env.stop().await;
    }
}

mod receiving_an_scrape_request {
    use std::sync::Arc;

    use torrust_tracker_client::udp::client::UdpTrackerClient;
    use torrust_tracker_test_helpers::{configuration, logging};
    use torrust_tracker_udp_protocol::{ConnectionId, InfoHash, ScrapeRequest, TransactionId};

    use super::DEFAULT_UDP_TIMEOUT;
    use crate::server::asserts::is_scrape_response;
    use crate::server::contract::send_connection_request;

    #[tokio::test]
    async fn should_return_a_scrape_response() {
        logging::setup();

        let cfg = configuration::ephemeral();
        let core_config = Arc::new(cfg.core.clone());
        let udp_tracker_config = Arc::new(cfg.udp_trackers.unwrap()[0].clone());
        let env = torrust_tracker_udp_server::testing::environment::Started::new(&core_config, &udp_tracker_config).await;

        let client = match UdpTrackerClient::new(env.bind_address(), DEFAULT_UDP_TIMEOUT).await {
            Ok(udp_tracker_client) => udp_tracker_client,
            Err(err) => panic!("{err}"),
        };

        let connection_id = send_connection_request(TransactionId::new(123), &client).await;

        // Send scrape request

        // Full scrapes are not allowed you need to pass an array of info hashes otherwise
        // it will return "bad request" error with empty vector

        let empty_info_hash = vec![InfoHash([0u8; 20])];

        let scrape_request = ScrapeRequest {
            connection_id: ConnectionId(connection_id.0),
            transaction_id: TransactionId::new(123i32),
            info_hashes: empty_info_hash,
        };

        match client.send(scrape_request.into()).await {
            Ok(_) => (),
            Err(err) => panic!("{err}"),
        }

        let response = match client.receive().await {
            Ok(response) => response,
            Err(err) => panic!("{err}"),
        };

        assert!(is_scrape_response(&response));

        env.stop().await;
    }
}

mod using_ipv6_v6only {
    use std::net::{IpAddr, Ipv6Addr, SocketAddr};
    use std::sync::Arc;

    use torrust_tracker_client::udp::client::UdpTrackerClient;
    use torrust_tracker_test_helpers::{configuration, logging};
    use torrust_tracker_udp_protocol::{ConnectRequest, TransactionId};

    use super::DEFAULT_UDP_TIMEOUT;
    use crate::server::asserts::is_connect_response;

    #[tokio::test]
    async fn should_accept_ipv6_connections_with_ipv6_v6only_enabled() {
        logging::setup();

        let cfg = configuration::ephemeral();
        let core_config = Arc::new(cfg.core.clone());
        let mut udp_tracker_config = cfg.udp_trackers.unwrap()[0].clone();
        udp_tracker_config.bind_address = SocketAddr::new(IpAddr::V6(Ipv6Addr::UNSPECIFIED), 0);
        udp_tracker_config.ipv6_v6only = true;
        let udp_tracker_config = Arc::new(udp_tracker_config);
        let env = torrust_tracker_udp_server::testing::environment::Started::new(&core_config, &udp_tracker_config).await;

        let client = UdpTrackerClient::new(env.bind_address(), DEFAULT_UDP_TIMEOUT).await.unwrap();

        let connect_request = ConnectRequest {
            transaction_id: TransactionId::new(123),
        };

        client.send(connect_request.into()).await.unwrap();

        let response = client.receive().await.unwrap();

        assert!(is_connect_response(&response, TransactionId::new(123)));

        env.stop().await;
    }
}

/// Tests for the disabled connection ID validation policy.
///
/// When `connection_id_validation = "disabled"`, announce and scrape requests
/// succeed even with arbitrary/invalid connection IDs. Connect requests still
/// issue valid connection IDs. The IP-ban enforcement is also disabled.
///
/// See ADR-20260727000000 (events are objective facts) and
/// issue #1136 for the full rationale.
mod using_disabled_connection_id_validation {
    use std::sync::Arc;

    use torrust_peer_id::PeerId;
    use torrust_tracker_client::udp::client::UdpTrackerClient;
    use torrust_tracker_test_helpers::{configuration, logging};
    use torrust_tracker_udp_core::ConnectionIdValidationPolicy;
    use torrust_tracker_udp_protocol::{
        AnnounceActionPlaceholder, AnnounceEvent, AnnounceRequest, ConnectRequest, ConnectionId, InfoHash, NumberOfBytes,
        NumberOfPeers, PeerKey, Port, ScrapeRequest, TransactionId,
    };

    use super::DEFAULT_UDP_TIMEOUT;
    use crate::common::fixtures::random_info_hash;
    use crate::server::asserts::is_connect_response;

    #[tokio::test]
    async fn connect_still_issues_a_valid_connection_id() {
        logging::setup();

        let cfg = configuration::ephemeral();
        let core_config = Arc::new(cfg.core.clone());
        let udp_tracker_config = Arc::new(cfg.udp_trackers.unwrap()[0].clone());
        let env = torrust_tracker_udp_server::testing::environment::Unstarted::new(&core_config, &udp_tracker_config)
            .await
            .with_connection_id_validation(ConnectionIdValidationPolicy::Disabled)
            .start()
            .await;

        let client = UdpTrackerClient::new(env.bind_address(), DEFAULT_UDP_TIMEOUT).await.unwrap();

        let connect_request = ConnectRequest {
            transaction_id: TransactionId::new(123),
        };

        client.send(connect_request.into()).await.unwrap();
        let response = client.receive().await.unwrap();

        assert!(is_connect_response(&response, TransactionId::new(123)));

        env.stop().await;
    }

    #[tokio::test]
    async fn announce_succeeds_with_an_arbitrary_connection_id() {
        logging::setup();

        let cfg = configuration::ephemeral();
        let core_config = Arc::new(cfg.core.clone());
        let udp_tracker_config = Arc::new(cfg.udp_trackers.unwrap()[0].clone());
        let env = torrust_tracker_udp_server::testing::environment::Unstarted::new(&core_config, &udp_tracker_config)
            .await
            .with_connection_id_validation(ConnectionIdValidationPolicy::Disabled)
            .start()
            .await;

        let client = UdpTrackerClient::new(env.bind_address(), DEFAULT_UDP_TIMEOUT).await.unwrap();

        let info_hash = random_info_hash();

        // An arbitrary connection ID that would fail strict validation (zero
        // is a "not normal" value that triggers a cookie error).
        let invalid_connection_id = ConnectionId::new(0);

        let announce_request = AnnounceRequest {
            connection_id: invalid_connection_id,
            action_placeholder: AnnounceActionPlaceholder::default(),
            transaction_id: TransactionId::new(1),
            info_hash: InfoHash(info_hash.0),
            peer_id: PeerId([255u8; 20]),
            bytes_downloaded: NumberOfBytes(0i64.into()),
            bytes_uploaded: NumberOfBytes(0i64.into()),
            bytes_left: NumberOfBytes(0i64.into()),
            event: AnnounceEvent::Started.into(),
            ip_address: std::net::Ipv4Addr::UNSPECIFIED.into(),
            key: PeerKey::new(0i32),
            peers_wanted: NumberOfPeers(1i32.into()),
            port: Port(client.client.socket.local_addr().unwrap().port().into()),
        };

        client.send(announce_request.into()).await.unwrap();

        let response = client.receive().await.unwrap();

        assert!(
            crate::server::asserts::is_ipv4_announce_response(&response),
            "announce should succeed with a valid announce response even with an invalid connection ID when validation is disabled"
        );

        env.stop().await;
    }

    #[tokio::test]
    async fn scrape_succeeds_with_an_arbitrary_connection_id() {
        logging::setup();

        let cfg = configuration::ephemeral();
        let core_config = Arc::new(cfg.core.clone());
        let udp_tracker_config = Arc::new(cfg.udp_trackers.unwrap()[0].clone());
        let env = torrust_tracker_udp_server::testing::environment::Unstarted::new(&core_config, &udp_tracker_config)
            .await
            .with_connection_id_validation(ConnectionIdValidationPolicy::Disabled)
            .start()
            .await;

        let client = UdpTrackerClient::new(env.bind_address(), DEFAULT_UDP_TIMEOUT).await.unwrap();

        // An arbitrary connection ID that would fail strict validation.
        let invalid_connection_id = ConnectionId::new(0);

        let empty_info_hash = vec![InfoHash([0u8; 20])];

        let scrape_request = ScrapeRequest {
            connection_id: invalid_connection_id,
            transaction_id: TransactionId::new(1),
            info_hashes: empty_info_hash,
        };

        client.send(scrape_request.into()).await.unwrap();

        let response = client.receive().await.unwrap();

        assert!(
            crate::server::asserts::is_scrape_response(&response),
            "scrape should succeed with a valid scrape response even with an invalid connection ID when validation is disabled"
        );

        env.stop().await;
    }
}
