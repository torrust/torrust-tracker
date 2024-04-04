use std::net::{Ipv4Addr, SocketAddr};
use std::time::Duration;

use aquatic_udp_protocol::{
    AnnounceEvent, AnnounceRequest, ConnectResponse, NumberOfBytes, NumberOfPeers, PeerId, PeerKey, Port, Response,
    ScrapeRequest, TransactionId,
};
use torrust_tracker_primitives::info_hash::InfoHash;
use tracing::debug;

use super::Error;
use crate::shared::bit_torrent::tracker::udp;

/// A UDP Tracker client to make test requests (checks).
#[derive(Debug, Clone)]
pub struct Client {
    pub client: udp::Client,
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
    pub async fn bind_and_connect(&addr: &SocketAddr, &timeout: &Duration) -> Result<Self, Error> {
        let client = udp::Client::connect(addr, timeout)
            .await
            .map_err(|err| Error::UnableToBindAndConnect { addr, err })?;

        Ok(Self { client })
    }

    /// Returns the local address of this [`Client`].
    ///
    /// # Errors
    ///
    /// This function errors if the underlying call fails.
    ///
    pub fn local_addr(&self) -> Result<SocketAddr, Error> {
        self.client.local_addr().map_err(|err| Error::UnableToGetLocalAddr { err })
    }

    /// Sends a connection request to the UDP Tracker server.
    ///
    /// # Errors
    ///
    /// Will return and error if unable to get a successful response.
    ///
    pub async fn send_connection_request(&self, transaction_id: TransactionId) -> Result<ConnectResponse, Error> {
        self.client
            .do_connection_request(transaction_id)
            .await
            .map_err(|err| Error::UnexpectedConnectionResponse { err })
    }

    /// Sends an announce request to the UDP Tracker server.
    ///
    /// # Errors
    ///
    /// Will return and error if the client is not connected. You have to connect
    /// before calling this function.
    pub async fn send_announce_request(
        &self,
        ctx: &ConnectResponse,
        info_hash: InfoHash,
        client_port: Port,
    ) -> Result<Response, Error> {
        debug!("Sending announce request with transaction id: {:#?}", ctx.transaction_id);

        let announce_request = AnnounceRequest {
            connection_id: ctx.connection_id,
            transaction_id: ctx.transaction_id,
            info_hash: aquatic_udp_protocol::InfoHash(info_hash.bytes()),
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

        let _ = self
            .client
            .send_request(announce_request.into())
            .await
            .map_err(|err| Error::UnableToSendRequest { err })?;

        let response = self
            .client
            .receive_response()
            .await
            .map_err(|err| Error::UnableToReceiveResponse { err })?;

        debug!("announce request response:\n{response:#?}");

        Ok(response)
    }

    /// Sends a scrape request to the UDP Tracker server.
    ///
    /// # Errors
    ///
    /// Will return and error if the client is not connected. You have to connect
    /// before calling this function.
    pub async fn send_scrape_request(&self, ctx: &ConnectResponse, info_hashes: &[InfoHash]) -> Result<Response, Error> {
        debug!("Sending scrape request with transaction id: {:#?}", ctx.transaction_id);

        let scrape_request = ScrapeRequest {
            connection_id: ctx.connection_id,
            transaction_id: ctx.transaction_id,
            info_hashes: info_hashes
                .iter()
                .map(|torrust_info_hash| aquatic_udp_protocol::InfoHash(torrust_info_hash.bytes()))
                .collect(),
        };

        let _ = self
            .client
            .send_request(scrape_request.into())
            .await
            .map_err(|err| Error::UnableToSendRequest { err })?;

        let response = self
            .client
            .receive_response()
            .await
            .map_err(|err| Error::UnableToReceiveResponse { err })?;

        debug!("scrape request response:\n{response:#?}");

        Ok(response)
    }
}
