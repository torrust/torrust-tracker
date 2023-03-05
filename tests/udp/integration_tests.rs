/// Integration tests for UDP tracker server
///
/// cargo test `udp_tracker_server` -- --nocapture
mod udp_tracker_server {

    // UDP tracker documentation:
    //
    // BEP 15. UDP Tracker Protocol for BitTorrent
    // https://www.bittorrent.org/beps/bep_0015.html

    use core::panic;

    use aquatic_udp_protocol::{ConnectRequest, ConnectionId, Response, TransactionId};
    use torrust_tracker::udp::MAX_PACKET_SIZE;

    use crate::udp::asserts::is_error_response;
    use crate::udp::client::{new_udp_client_connected, Client};
    use crate::udp::server::{start_udp_tracker, tracker_configuration};

    fn empty_udp_request() -> [u8; MAX_PACKET_SIZE] {
        [0; MAX_PACKET_SIZE]
    }

    fn empty_buffer() -> [u8; MAX_PACKET_SIZE] {
        [0; MAX_PACKET_SIZE]
    }

    async fn send_connection_request(transaction_id: TransactionId, client: &Client) -> ConnectionId {
        let connect_request = ConnectRequest { transaction_id };

        client.send(connect_request.into()).await;

        let response = client.receive().await;

        match response {
            Response::Connect(connect_response) => connect_response.connection_id,
            _ => panic!("error connecting to udp server {:?}", response),
        }
    }

    #[tokio::test]
    async fn should_return_a_bad_request_response_when_the_client_sends_an_empty_request() {
        let configuration = tracker_configuration();

        let udp_server = start_udp_tracker(&configuration);

        let client = new_udp_client_connected(&udp_server.bind_address.unwrap()).await;

        client.send(&empty_udp_request()).await;

        let mut buffer = empty_buffer();
        client.receive(&mut buffer).await;
        let response = Response::from_bytes(&buffer, true).unwrap();

        assert!(is_error_response(&response, "bad request"));
    }

    mod receiving_a_connection_request {
        use aquatic_udp_protocol::{ConnectRequest, TransactionId};

        use crate::udp::asserts::is_connect_response;
        use crate::udp::client::new_udp_tracker_client_connected;
        use crate::udp::server::{start_udp_tracker, tracker_configuration};

        #[tokio::test]
        async fn should_return_a_connect_response() {
            let configuration = tracker_configuration();

            let udp_server = start_udp_tracker(&configuration);

            let client = new_udp_tracker_client_connected(&udp_server.bind_address.unwrap()).await;

            let connect_request = ConnectRequest {
                transaction_id: TransactionId(123),
            };

            client.send(connect_request.into()).await;

            let response = client.receive().await;

            assert!(is_connect_response(&response, TransactionId(123)));
        }
    }

    mod receiving_an_announce_request {
        use std::net::Ipv4Addr;

        use aquatic_udp_protocol::{
            AnnounceEvent, AnnounceRequest, ConnectionId, InfoHash, NumberOfBytes, NumberOfPeers, PeerId, PeerKey, Port,
            TransactionId,
        };

        use crate::udp::asserts::is_ipv4_announce_response;
        use crate::udp::client::new_udp_tracker_client_connected;
        use crate::udp::integration_tests::udp_tracker_server::send_connection_request;
        use crate::udp::server::{start_udp_tracker, tracker_configuration};

        #[tokio::test]
        async fn should_return_an_announce_response() {
            let configuration = tracker_configuration();

            let udp_server = start_udp_tracker(&configuration);

            let client = new_udp_tracker_client_connected(&udp_server.bind_address.unwrap()).await;

            let connection_id = send_connection_request(TransactionId(123), &client).await;

            // Send announce request

            let announce_request = AnnounceRequest {
                connection_id: ConnectionId(connection_id.0),
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
                port: Port(client.udp_client.socket.local_addr().unwrap().port()),
            };

            client.send(announce_request.into()).await;

            let response = client.receive().await;

            assert!(is_ipv4_announce_response(&response));
        }
    }

    mod receiving_an_scrape_request {
        use aquatic_udp_protocol::{ConnectionId, InfoHash, ScrapeRequest, TransactionId};

        use crate::udp::asserts::is_scrape_response;
        use crate::udp::client::new_udp_tracker_client_connected;
        use crate::udp::integration_tests::udp_tracker_server::send_connection_request;
        use crate::udp::server::{start_udp_tracker, tracker_configuration};

        #[tokio::test]
        async fn should_return_a_scrape_response() {
            let configuration = tracker_configuration();

            let udp_server = start_udp_tracker(&configuration);

            let client = new_udp_tracker_client_connected(&udp_server.bind_address.unwrap()).await;

            let connection_id = send_connection_request(TransactionId(123), &client).await;

            // Send scrape request

            // Full scrapes are not allowed you need to pass an array of info hashes otherwise
            // it will return "bad request" error with empty vector
            let info_hashes = vec![InfoHash([0u8; 20])];

            let scrape_request = ScrapeRequest {
                connection_id: ConnectionId(connection_id.0),
                transaction_id: TransactionId(123i32),
                info_hashes,
            };

            client.send(scrape_request.into()).await;

            let response = client.receive().await;

            assert!(is_scrape_response(&response));
        }
    }
}
