use std::net::{Ipv4Addr, SocketAddr};

use anyhow::Context;
use aquatic_udp_protocol::common::InfoHash;
use aquatic_udp_protocol::{
    AnnounceEvent, AnnounceRequest, ConnectRequest, ConnectionId, NumberOfBytes, NumberOfPeers, PeerId, PeerKey, Port, Response,
    ScrapeRequest, TransactionId,
};
use thiserror::Error;
use torrust_tracker_primitives::info_hash::InfoHash as TorrustInfoHash;
use tracing::debug;

use crate::shared::bit_torrent::tracker::udp::client::{UdpClient, UdpTrackerClient};

#[derive(Error, Debug)]
pub enum ClientError {
    #[error("Local socket address is not bound yet. Try binding before connecting.")]
    NotBound,
    #[error("Not connected to remote tracker UDP socket. Try connecting before making requests.")]
    NotConnected,
    #[error("Unexpected response while connecting the the remote server.")]
    UnexpectedConnectionResponse,
}

/// A UDP Tracker client to make test requests (checks).
#[derive(Debug, Default)]
pub struct Client {
    /// Local UDP socket. It could be 0 to assign a free port.
    local_binding_address: Option<SocketAddr>,

    /// Local UDP socket after binding. It's equals to binding address if a
    /// non- zero port was used.
    local_bound_address: Option<SocketAddr>,

    /// Remote UDP tracker socket
    remote_socket: Option<SocketAddr>,

    /// The client used to make UDP requests to the tracker.
    udp_tracker_client: Option<UdpTrackerClient>,
}

impl Client {
    /// Binds to the local socket and connects to the remote one.
    ///
    /// # Errors
    ///
    /// Will return an error if
    ///
    /// - It can't bound to the local socket address.
    /// - It can't make a connection request successfully to the remote UDP server.
    pub async fn bind_and_connect(&mut self, local_port: u16, remote_socket_addr: &SocketAddr) -> anyhow::Result<SocketAddr> {
        let bound_to = self.bind(local_port).await?;
        self.connect(remote_socket_addr).await?;
        Ok(bound_to)
    }

    /// Binds local client socket.
    ///
    /// # Errors
    ///
    /// Will return an error if it can't bound to the local address.
    async fn bind(&mut self, local_port: u16) -> anyhow::Result<SocketAddr> {
        let local_bind_to = format!("0.0.0.0:{local_port}");
        let binding_address = local_bind_to.parse().context("binding local address")?;

        debug!("Binding to: {local_bind_to}");
        let udp_client = UdpClient::bind(&local_bind_to).await;

        let bound_to = udp_client.socket.local_addr().context("bound local address")?;
        debug!("Bound to: {bound_to}");

        self.local_binding_address = Some(binding_address);
        self.local_bound_address = Some(bound_to);

        self.udp_tracker_client = Some(UdpTrackerClient { udp_client });

        Ok(bound_to)
    }

    /// Connects to the remote server socket.
    ///
    /// # Errors
    ///
    /// Will return and error if it can't make a connection request successfully
    /// to the remote UDP server.
    async fn connect(&mut self, tracker_socket_addr: &SocketAddr) -> anyhow::Result<()> {
        debug!("Connecting to tracker: udp://{tracker_socket_addr}");

        match &self.udp_tracker_client {
            Some(client) => {
                client.udp_client.connect(&tracker_socket_addr.to_string()).await;
                self.remote_socket = Some(*tracker_socket_addr);
                Ok(())
            }
            None => Err(ClientError::NotBound.into()),
        }
    }

    /// Sends a connection request to the UDP Tracker server.
    ///
    /// # Errors
    ///
    /// Will return and error if
    ///
    /// - It can't connect to the remote UDP socket.
    /// - It can't make a connection request successfully to the remote UDP
    /// server (after successfully connecting to the remote UDP socket).
    ///
    /// # Panics
    ///
    /// Will panic if it receives an unexpected response.
    pub async fn send_connection_request(&self, transaction_id: TransactionId) -> anyhow::Result<ConnectionId> {
        debug!("Sending connection request with transaction id: {transaction_id:#?}");

        let connect_request = ConnectRequest { transaction_id };

        match &self.udp_tracker_client {
            Some(client) => {
                client.send(connect_request.into()).await;

                let response = client.receive().await;

                debug!("connection request response:\n{response:#?}");

                match response {
                    Response::Connect(connect_response) => Ok(connect_response.connection_id),
                    _ => Err(ClientError::UnexpectedConnectionResponse.into()),
                }
            }
            None => Err(ClientError::NotConnected.into()),
        }
    }

    /// Sends an announce request to the UDP Tracker server.
    ///
    /// # Errors
    ///
    /// Will return and error if the client is not connected. You have to connect
    /// before calling this function.
    pub async fn send_announce_request(
        &self,
        connection_id: ConnectionId,
        transaction_id: TransactionId,
        info_hash: TorrustInfoHash,
        client_port: Port,
    ) -> anyhow::Result<Response> {
        debug!("Sending announce request with transaction id: {transaction_id:#?}");

        let announce_request = AnnounceRequest {
            connection_id,
            transaction_id,
            info_hash: InfoHash(info_hash.bytes()),
            peer_id: PeerId(*b"-qB00000000000000001"),
            bytes_downloaded: NumberOfBytes(0i64),
            bytes_uploaded: NumberOfBytes(0i64),
            bytes_left: NumberOfBytes(0i64),
            event: AnnounceEvent::Started,
            ip_address: Some(Ipv4Addr::new(0, 0, 0, 0)),
            key: PeerKey(0u32),
            peers_wanted: NumberOfPeers(1i32),
            port: client_port,
        };

        match &self.udp_tracker_client {
            Some(client) => {
                client.send(announce_request.into()).await;

                let response = client.receive().await;

                debug!("announce request response:\n{response:#?}");

                Ok(response)
            }
            None => Err(ClientError::NotConnected.into()),
        }
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
        info_hashes: Vec<TorrustInfoHash>,
    ) -> anyhow::Result<Response> {
        debug!("Sending scrape request with transaction id: {transaction_id:#?}");

        let scrape_request = ScrapeRequest {
            connection_id,
            transaction_id,
            info_hashes: info_hashes
                .iter()
                .map(|torrust_info_hash| InfoHash(torrust_info_hash.bytes()))
                .collect(),
        };

        match &self.udp_tracker_client {
            Some(client) => {
                client.send(scrape_request.into()).await;

                let response = client.receive().await;

                debug!("scrape request response:\n{response:#?}");

                Ok(response)
            }
            None => Err(ClientError::NotConnected.into()),
        }
    }
}
