//! UDP tracker test helpers.

use std::net::SocketAddr;
use std::num::NonZeroU16;
use std::time::Duration;

use torrust_tracker_client::udp::client::{UdpClient, UdpTrackerClient};
use torrust_tracker_udp_protocol::{
    AnnounceActionPlaceholder, AnnounceEvent, AnnounceRequest, ConnectRequest, ConnectionId, NumberOfBytes, NumberOfPeers,
    PeerKey, Port, Response, TransactionId,
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
pub async fn send_invalid_connection_ids_until_banned(remote_addr: SocketAddr) {
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

/// Sends invalid connection IDs through one client socket to multiple UDP
/// listeners until their shared ban service rejects the source IP.
///
/// # Panics
///
/// Panics if there is no listener, the socket cannot be created, an expected
/// pre-ban cookie-error response is absent, or the post-threshold request is
/// not rejected.
pub async fn send_invalid_connection_ids_across_listeners_until_banned(
    remote_addrs: &[SocketAddr],
    max_connection_id_errors_per_ip: u32,
) {
    assert!(!remote_addrs.is_empty(), "at least one UDP listener is required");

    let client = UdpClient::bound(
        "0.0.0.0:0".parse().expect("socket address must be valid"),
        Duration::from_secs(1),
    )
    .await
    .expect("failed to create UDP client socket");
    let source_port = client
        .socket
        .local_addr()
        .expect("UDP client must have a local address")
        .port();

    for transaction_id in 1..=max_connection_id_errors_per_ip + 1 {
        let remote_addr = remote_addrs[(transaction_id as usize - 1) % remote_addrs.len()];
        client.connect(remote_addr).await.expect("failed to select UDP listener");
        let client = UdpTrackerClient { client: client.clone() };
        let transaction_id =
            i32::try_from(transaction_id).expect("connection-ID error threshold must fit in an i32 transaction ID");
        client
            .send(invalid_connection_id_announce_request(transaction_id, source_port).into())
            .await
            .expect("failed to send invalid connection ID announce request");
        client
            .receive()
            .await
            .expect("the request before the ban threshold should receive a cookie error");
    }

    let post_threshold_transaction_id = max_connection_id_errors_per_ip
        .checked_add(2)
        .expect("connection-ID error threshold must allow a post-threshold transaction ID");
    let remote_addr = remote_addrs[(max_connection_id_errors_per_ip as usize + 1) % remote_addrs.len()];
    client.connect(remote_addr).await.expect("failed to select UDP listener");
    let client = UdpTrackerClient { client };
    client
        .send(
            invalid_connection_id_announce_request(
                i32::try_from(post_threshold_transaction_id)
                    .expect("connection-ID error threshold must fit in an i32 transaction ID"),
                source_port,
            )
            .into(),
        )
        .await
        .expect("failed to send post-threshold invalid connection ID announce request");
    assert!(
        client.receive().await.is_err(),
        "the post-threshold request should be banned without a response"
    );
}

/// Sends one UDP announce request with an invalid connection ID.
///
/// Returns the tracker response so the caller can assert its protocol contract
/// independently from any metric assertion.
///
/// # Panics
///
/// Panics if the UDP client cannot be created, the request cannot be sent, or
/// no response is received.
pub async fn send_invalid_connection_id_announce(remote_addr: SocketAddr) -> Response {
    let client = UdpTrackerClient::new(remote_addr, Duration::from_secs(1))
        .await
        .expect("failed to create UDP client");
    let request = invalid_connection_id_announce_request(1, client.client.socket.local_addr().unwrap().port());

    client
        .send(request.into())
        .await
        .expect("failed to send invalid connection ID announce request");

    client.receive().await.expect("expected a tracker response")
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
