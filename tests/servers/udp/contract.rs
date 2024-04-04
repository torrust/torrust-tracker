// UDP tracker documentation:
//
// BEP 15. UDP Tracker Protocol for BitTorrent
// https://www.bittorrent.org/beps/bep_0015.html

use aquatic_udp_protocol::Response;
use torrust_tracker::shared::bit_torrent::tracker::udp::Client;
use torrust_tracker_configuration::{CLIENT_TIMEOUT_DEFAULT, MAX_PACKET_SIZE};
use torrust_tracker_test_helpers::configuration;

use crate::common::udp::Started;
use crate::servers::udp::asserts::is_error_response;

fn empty_udp_request() -> [u8; MAX_PACKET_SIZE] {
    [0; MAX_PACKET_SIZE]
}

fn empty_buffer() -> [u8; MAX_PACKET_SIZE] {
    [0; MAX_PACKET_SIZE]
}

#[tokio::test]
async fn should_return_a_bad_request_response_when_the_client_sends_an_empty_request() {
    let env = Started::new(&configuration::ephemeral().into()).await;

    let client = Client::connect(env.bind_address(), CLIENT_TIMEOUT_DEFAULT)
        .await
        .expect("it should connect");

    client.send(&empty_udp_request()).await.expect("it should send request");

    let mut buffer = empty_buffer();
    client.receive(&mut buffer).await.expect("it should receive a response");

    let response = Response::from_bytes(&buffer, true).expect("it should parse a response");

    assert!(is_error_response(&response, "bad request"));

    env.stop().await;
}

mod receiving_a_connection_request {
    use aquatic_udp_protocol::{ConnectRequest, TransactionId};
    use torrust_tracker::shared::bit_torrent::tracker::udp::Client;
    use torrust_tracker_configuration::CLIENT_TIMEOUT_DEFAULT;
    use torrust_tracker_test_helpers::configuration;

    use crate::common::udp::Started;
    use crate::servers::udp::asserts::is_connect_response;

    #[tokio::test]
    async fn should_return_a_connect_response() {
        let env = Started::new(&configuration::ephemeral().into()).await;

        let client = Client::connect(env.bind_address(), CLIENT_TIMEOUT_DEFAULT)
            .await
            .expect("it should connect");

        let connect_request = ConnectRequest {
            transaction_id: TransactionId(123),
        };

        client
            .send_request(connect_request.into())
            .await
            .expect("it should send request");

        let response = client.receive_response().await.expect("it should get response");

        assert!(is_connect_response(&response, TransactionId(123)));

        env.stop().await;
    }
}

mod receiving_an_announce_request {
    use std::net::Ipv4Addr;

    use aquatic_udp_protocol::{
        AnnounceEvent, AnnounceRequest, ConnectionId, InfoHash, NumberOfBytes, NumberOfPeers, PeerId, PeerKey, Port,
        TransactionId,
    };
    use torrust_tracker::shared::bit_torrent::tracker::udp::Client;
    use torrust_tracker_configuration::CLIENT_TIMEOUT_DEFAULT;
    use torrust_tracker_test_helpers::configuration;

    use crate::common::udp::Started;
    use crate::servers::udp::asserts::is_ipv4_announce_response;

    #[tokio::test]
    async fn should_return_an_announce_response() {
        let env = Started::new(&configuration::ephemeral().into()).await;

        let client = Client::connect(env.bind_address(), CLIENT_TIMEOUT_DEFAULT)
            .await
            .expect("it should connect");

        let ctx = client
            .do_connection_request(TransactionId(123))
            .await
            .expect("it should do connection");

        // Send announce request

        let announce_request = AnnounceRequest {
            connection_id: ConnectionId(ctx.connection_id.0),
            transaction_id: TransactionId(123i32),
            info_hash: InfoHash([0u8; 20]),
            peer_id: PeerId([255u8; 20]),
            bytes_downloaded: NumberOfBytes(0i64),
            bytes_uploaded: NumberOfBytes(0i64),
            bytes_left: NumberOfBytes(0i64),
            event: AnnounceEvent::Started,
            ip_address: Some(Ipv4Addr::new(0, 0, 0, 0)),
            key: PeerKey(0u32),
            peers_wanted: NumberOfPeers(1i32),
            port: Port(client.local_addr().expect("it should get the local address").port()),
        };

        client
            .send_request(announce_request.into())
            .await
            .expect("it should send a request");

        let response = client.receive_response().await.expect("it should receive a response");

        println!("test response {response:?}");

        assert!(is_ipv4_announce_response(&response));

        env.stop().await;
    }
}

mod receiving_an_scrape_request {
    use aquatic_udp_protocol::{ConnectionId, InfoHash, ScrapeRequest, TransactionId};
    use torrust_tracker::shared::bit_torrent::tracker::udp::Client;
    use torrust_tracker_configuration::CLIENT_TIMEOUT_DEFAULT;
    use torrust_tracker_test_helpers::configuration;

    use crate::common::udp::Started;
    use crate::servers::udp::asserts::is_scrape_response;

    #[tokio::test]
    async fn should_return_a_scrape_response() {
        let env = Started::new(&configuration::ephemeral().into()).await;

        let client = Client::connect(env.bind_address(), CLIENT_TIMEOUT_DEFAULT)
            .await
            .expect("it should connect");

        let ctx = client
            .do_connection_request(TransactionId(123))
            .await
            .expect("it should connect");

        // Send scrape request

        // Full scrapes are not allowed you need to pass an array of info hashes otherwise
        // it will return "bad request" error with empty vector
        let info_hashes = vec![InfoHash([0u8; 20])];

        let scrape_request = ScrapeRequest {
            connection_id: ConnectionId(ctx.connection_id.0),
            transaction_id: TransactionId(123i32),
            info_hashes,
        };

        client
            .send_request(scrape_request.into())
            .await
            .expect("it should send a request");

        let response = client.receive_response().await.expect("it should receive a response");

        assert!(is_scrape_response(&response));

        env.stop().await;
    }
}
