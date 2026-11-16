use std::net::{IpAddr, Ipv4Addr, SocketAddr};
use std::num::NonZeroU16;
use std::ops::Range;
use std::sync::Arc;

use torrust_net_primitives::service_binding::{Protocol, ServiceBinding};
use torrust_tracker_configuration::v3_0_0::core::Core;
use torrust_tracker_core::announce_handler::AnnounceHandler;
use torrust_tracker_core::torrent::repository::in_memory::InMemoryTorrentRepository;
use torrust_tracker_core::whitelist::authorization::WhitelistAuthorization;
use torrust_tracker_core::whitelist::repository::in_memory::InMemoryWhitelist;
use torrust_tracker_events::bus::SenderStatus;
use torrust_tracker_primitives::{AnnounceData, ConfigurationInstanceId, PeerId, ServiceRole};
use torrust_tracker_udp_core::connection_cookie::{gen_remote_fingerprint, make};
use torrust_tracker_udp_core::crypto::cookie_cipher::CookieCipher;
use torrust_tracker_udp_core::event::bus::EventBus;
use torrust_tracker_udp_core::event::sender::Broadcaster;
use torrust_tracker_udp_core::services::announce::AnnounceService;
use torrust_tracker_udp_protocol::{
    AnnounceActionPlaceholder, AnnounceEvent, AnnounceRequest, InfoHash, NumberOfBytes, NumberOfPeers, PeerKey, Port,
    TransactionId,
};

use crate::helpers::utils::{sample_ipv4_remote_addr, sample_issue_time};

/// An announce service and request built once, outside the measured loop.
///
/// Every iteration announces the same peer to the same torrent, so after the
/// first iteration each announce updates the existing peer in a one-peer swarm
/// and returns an empty peer list, as the HTTP `announce_once` benchmark does
/// (see `docs/benchmarking.md` for why). The result is a lower bound on
/// announce cost.
///
/// The tracker is public and events are disabled, so an announce measures the
/// connection-cookie check, the whitelist check, the conversion of the wire
/// request into a domain peer, and the swarm update.
pub struct AnnounceBenchmarkContext {
    announce_service: AnnounceService,
    announce_request: AnnounceRequest,
    client_socket_addr: SocketAddr,
    server_service_binding: ServiceBinding,
    cookie_valid_range: Range<f64>,
}

impl AnnounceBenchmarkContext {
    /// # Panics
    ///
    /// Panics if the connection cookie cannot be made for the fixed issue time,
    /// or if the fixed server address is not a valid UDP service binding.
    #[must_use]
    pub fn new() -> Self {
        let core = Core::default();
        let whitelist_authorization = Arc::new(WhitelistAuthorization::new(&core, &Arc::new(InMemoryWhitelist::default())));
        let torrents = Arc::new(InMemoryTorrentRepository::default());
        let announce_handler = AnnounceHandler::new_public(&core, &whitelist_authorization, &torrents);
        let cookie_cipher = CookieCipher::random();
        let event_bus = Arc::new(EventBus::new(SenderStatus::Disabled, Broadcaster::default()));

        let client_socket_addr = sample_ipv4_remote_addr();
        let connection_id = make(
            &cookie_cipher,
            gen_remote_fingerprint(&client_socket_addr),
            sample_issue_time(),
        )
        .expect("the fixed issue time should make a connection cookie");

        let announce_service = AnnounceService::new(
            Arc::new(announce_handler),
            whitelist_authorization,
            Arc::new(cookie_cipher),
            event_bus.sender(),
            ConfigurationInstanceId::new(ServiceRole::UdpTracker, 0),
            None,
        );

        let server_socket_addr = SocketAddr::new(IpAddr::V4(Ipv4Addr::new(203, 0, 113, 196)), 6969);

        Self {
            announce_service,
            announce_request: AnnounceRequest {
                connection_id,
                action_placeholder: AnnounceActionPlaceholder::default(),
                transaction_id: TransactionId(0i32.into()),
                info_hash: InfoHash([0x11; 20]),
                peer_id: PeerId([0x2d; 20]),
                bytes_downloaded: NumberOfBytes(0i64.into()),
                bytes_uploaded: NumberOfBytes(0i64.into()),
                bytes_left: NumberOfBytes(1000i64.into()),
                event: AnnounceEvent::Started.into(),
                ip_address: Ipv4Addr::UNSPECIFIED.into(),
                key: PeerKey::new(0i32),
                peers_wanted: NumberOfPeers::new(74i32),
                port: Port::new(NonZeroU16::new(6881).expect("6881 is not zero")),
            },
            client_socket_addr,
            server_service_binding: ServiceBinding::new(Protocol::UDP, server_socket_addr).unwrap(),
            cookie_valid_range: (sample_issue_time() - 60.0)..(sample_issue_time() + 60.0),
        }
    }

    /// # Panics
    ///
    /// Panics if the announce is rejected, which would mean the benchmark
    /// measures an error path instead of an announce.
    pub async fn announce_once(&self) -> AnnounceData {
        self.announce_service
            .handle_announce(
                self.client_socket_addr,
                self.server_service_binding.clone(),
                &self.announce_request,
                self.cookie_valid_range.clone(),
                true,
            )
            .await
            .expect("the benchmark announce should be accepted")
    }
}
