//! UDP tracker test helpers.

use std::net::SocketAddr;
use std::num::NonZeroU16;
use std::time::Duration;

use torrust_tracker_client::udp::client::UdpTrackerClient;
use torrust_tracker_udp_protocol::{
    AnnounceActionPlaceholder, AnnounceEvent, AnnounceRequest, ConnectRequest, ConnectionId, NumberOfBytes, NumberOfPeers,
    PeerKey, Port, TransactionId,
};

/// Sends a UDP announce to the given tracker address.
///
/// Performs the connect → announce handshake and returns the announce response.
///
/// # Panics
///
/// Panics if the client cannot connect, send, or receive.
pub async fn udp_announce(
    remote_addr: SocketAddr,
    info_hash: &[u8; 20],
    peer_id: &[u8; 20],
    port: u16,
) -> torrust_tracker_udp_protocol::Response {
    let client = UdpTrackerClient::new(remote_addr, Duration::from_secs(5))
        .await
        .expect("failed to create UDP client");

    // Connect
    let connect_transaction_id = TransactionId::new(1);
    let connect_request = ConnectRequest {
        transaction_id: connect_transaction_id,
    };
    client
        .send(connect_request.into())
        .await
        .expect("failed to send connect request");
    let connection_id = match client.receive().await.expect("failed to receive connect response") {
        torrust_tracker_udp_protocol::Response::Connect(resp) => resp.connection_id,
        other => panic!("expected connect response, got: {other:?}"),
    };

    // Announce
    let announce_transaction_id = TransactionId::new(2);
    let announce_request = AnnounceRequest {
        connection_id,
        action_placeholder: AnnounceActionPlaceholder::default(),
        transaction_id: announce_transaction_id,
        info_hash: torrust_tracker_udp_protocol::common::InfoHash(*info_hash),
        peer_id: torrust_peer_id::PeerId(*peer_id),
        bytes_downloaded: NumberOfBytes::new(0),
        bytes_uploaded: NumberOfBytes::new(0),
        bytes_left: NumberOfBytes::new(0),
        event: AnnounceEvent::Started.into(),
        ip_address: std::net::Ipv4Addr::UNSPECIFIED.into(),
        key: PeerKey::new(0),
        peers_wanted: NumberOfPeers::new(1),
        port: Port::new(NonZeroU16::new(port).expect("port must be non-zero")),
    };
    client
        .send(announce_request.into())
        .await
        .expect("failed to send announce request");
    client.receive().await.expect("failed to receive announce response")
}

/// Sends invalid connection IDs until the tracker bans this client's IP.
///
/// The final request must time out because the ban is enforced before it is
/// processed. The same UDP socket is retained to preserve its source address.
///
/// # Panics
///
/// Panics if the UDP client cannot be created, a request cannot be sent, an
/// expected pre-ban cookie-error response is absent, or the final request is
/// not banned.
pub async fn invalid_connection_ids_should_trigger_ban(remote_addr: SocketAddr) {
    let client = UdpTrackerClient::new(remote_addr, Duration::from_secs(1))
        .await
        .expect("failed to create UDP client");

    for transaction_id in 1..=11 {
        client
            .send(
                invalid_connection_id_announce_request(transaction_id, client.client.socket.local_addr().unwrap().port()).into(),
            )
            .await
            .expect("failed to send invalid connection ID announce request");
        client
            .receive()
            .await
            .expect("the request before the ban threshold should receive a cookie error");
    }

    client
        .send(invalid_connection_id_announce_request(12, client.client.socket.local_addr().unwrap().port()).into())
        .await
        .expect("failed to send post-threshold invalid connection ID announce request");
    assert!(
        client.receive().await.is_err(),
        "the post-threshold request should be banned without a response"
    );
}

fn invalid_connection_id_announce_request(transaction_id: i32, port: u16) -> AnnounceRequest {
    AnnounceRequest {
        connection_id: ConnectionId::new(0),
        action_placeholder: AnnounceActionPlaceholder::default(),
        transaction_id: TransactionId::new(transaction_id),
        info_hash: torrust_tracker_udp_protocol::common::InfoHash([0; 20]),
        peer_id: torrust_peer_id::PeerId([0; 20]),
        bytes_downloaded: NumberOfBytes::new(0),
        bytes_uploaded: NumberOfBytes::new(0),
        bytes_left: NumberOfBytes::new(0),
        event: AnnounceEvent::Started.into(),
        ip_address: std::net::Ipv4Addr::UNSPECIFIED.into(),
        key: PeerKey::new(0),
        peers_wanted: NumberOfPeers::new(1),
        port: Port::new(NonZeroU16::new(port).expect("UDP client port must be non-zero")),
    }
}
