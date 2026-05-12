use std::net::{Ipv4Addr, SocketAddr};
use std::num::NonZeroU16;
use std::time::Duration;

use bittorrent_primitives::info_hash::InfoHash as TorrustInfoHash;
use bittorrent_tracker_client::peer_id::default_production_peer_id;
use bittorrent_tracker_client::udp::client::UdpTrackerClient;
use bittorrent_udp_tracker_protocol::common::InfoHash;
use bittorrent_udp_tracker_protocol::{
    AnnounceActionPlaceholder, AnnounceEvent, AnnounceRequest, ConnectRequest, ConnectionId, NumberOfBytes, NumberOfPeers,
    PeerId, PeerKey, Port, Response, ScrapeRequest, TransactionId,
};

use super::Error;

/// Optional parameters for an announce request. When a field is `None`, the
/// default announce value is used (for `port`, the socket local port is used).
#[derive(Debug, Default)]
pub struct AnnounceParams {
    pub event: Option<AnnounceEvent>,
    pub uploaded: Option<i64>,
    pub downloaded: Option<i64>,
    pub left: Option<i64>,
    pub port: Option<u16>,
    pub ip_address: Option<Ipv4Addr>,
    pub peer_id: Option<[u8; 20]>,
    pub key: Option<i32>,
    pub peers_wanted: Option<i32>,
}

/// A UDP Tracker client to make test requests (checks).
#[derive(Debug)]
pub struct Client {
    client: UdpTrackerClient,
}

impl Client {
    /// Creates a new `[Client]` for checking a UDP Tracker Service
    ///
    /// # Errors
    ///
    /// It will error if unable to bind and connect to the udp remote address.
    ///
    pub async fn new(remote_addr: SocketAddr, timeout: Duration) -> Result<Self, Error> {
        let client = UdpTrackerClient::new(remote_addr, timeout)
            .await
            .map_err(|err| Error::UnableToBindAndConnect { remote_addr, err })?;

        Ok(Self { client })
    }

    /// Returns the local addr of this [`Client`].
    ///
    /// # Errors
    ///
    /// This function will return an error if the socket is somehow not bound.
    pub fn local_addr(&self) -> std::io::Result<SocketAddr> {
        self.client.client.socket.local_addr()
    }

    /// Sends a connection request to the UDP Tracker server.
    ///
    /// # Errors
    ///
    /// Will return and error if
    ///
    /// - It can't connect to the remote UDP socket.
    /// - It can't make a connection request successfully to the remote UDP
    ///   server (after successfully connecting to the remote UDP socket).
    ///
    /// # Panics
    ///
    /// Will panic if it receives an unexpected response.
    pub async fn send_connection_request(&self, transaction_id: TransactionId) -> Result<ConnectionId, Error> {
        tracing::debug!("Sending connection request with transaction id: {transaction_id:#?}");

        let connect_request = ConnectRequest { transaction_id };

        let _ = self
            .client
            .send(connect_request.into())
            .await
            .map_err(|err| Error::UnableToSendConnectionRequest { err })?;

        let response = self
            .client
            .receive()
            .await
            .map_err(|err| Error::UnableToReceiveConnectResponse { err })?;

        match response {
            Response::Connect(connect_response) => Ok(connect_response.connection_id),
            _ => Err(Error::UnexpectedConnectionResponse { response }),
        }
    }

    /// Sends an announce request to the UDP Tracker server.
    ///
    /// # Errors
    ///
    /// Will return and error if the client is not connected. You have to connect
    /// before calling this function.
    ///
    /// # Panics
    ///
    /// It will panic if the `local_address` has a zero port.
    pub async fn send_announce_request(
        &self,
        transaction_id: TransactionId,
        connection_id: ConnectionId,
        info_hash: TorrustInfoHash,
        params: &AnnounceParams,
    ) -> Result<Response, Error> {
        tracing::debug!("Sending announce request with transaction id: {transaction_id:#?}");

        let local_port = NonZeroU16::new(
            self.client
                .client
                .socket
                .local_addr()
                .expect("it should get the local address")
                .port(),
        )
        .expect("it should not be zero");

        let port = params.port.and_then(NonZeroU16::new).unwrap_or(local_port);

        let announce_request = AnnounceRequest {
            connection_id,
            action_placeholder: AnnounceActionPlaceholder::default(),
            transaction_id,
            info_hash: InfoHash(info_hash.bytes()),
            peer_id: params.peer_id.map_or(default_production_peer_id(), PeerId),
            bytes_downloaded: NumberOfBytes::new(params.downloaded.unwrap_or(0)),
            bytes_uploaded: NumberOfBytes::new(params.uploaded.unwrap_or(0)),
            bytes_left: NumberOfBytes::new(params.left.unwrap_or(0)),
            event: params.event.unwrap_or(AnnounceEvent::Started).into(),
            ip_address: params.ip_address.unwrap_or(Ipv4Addr::UNSPECIFIED).into(),
            key: PeerKey::new(params.key.unwrap_or(0)),
            peers_wanted: NumberOfPeers::new(params.peers_wanted.unwrap_or(1)),
            port: Port::new(port),
        };

        let _ = self
            .client
            .send(announce_request.into())
            .await
            .map_err(|err| Error::UnableToSendAnnounceRequest { err })?;

        let response = self
            .client
            .receive()
            .await
            .map_err(|err| Error::UnableToReceiveAnnounceResponse { err })?;

        Ok(response)
    }

    /// Sends a scrape request to the UDP Tracker server.
    ///
    /// # Errors
    ///
    /// Will return and error if the client is not connected. You have to connect
    /// before calling this function.
    pub async fn send_scrape_request(
        &self,
        connection_id: ConnectionId,
        transaction_id: TransactionId,
        info_hashes: &[TorrustInfoHash],
    ) -> Result<Response, Error> {
        tracing::debug!("Sending scrape request with transaction id: {transaction_id:#?}");

        let scrape_request = ScrapeRequest {
            connection_id,
            transaction_id,
            info_hashes: info_hashes
                .iter()
                .map(|torrust_info_hash| InfoHash(torrust_info_hash.bytes()))
                .collect(),
        };

        let _ = self
            .client
            .send(scrape_request.into())
            .await
            .map_err(|err| Error::UnableToSendScrapeRequest { err })?;

        let response = self
            .client
            .receive()
            .await
            .map_err(|err| Error::UnableToReceiveScrapeResponse { err })?;

        Ok(response)
    }
}
