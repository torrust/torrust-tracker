use std::fmt::Debug;
use std::net::SocketAddr;
use std::sync::Arc;

use torrust_tracker::bootstrap::app::tracker;
use torrust_tracker::core::Tracker;
use torrust_tracker::servers::registar::Registar;
use torrust_tracker::servers::service::{Service, Started, Stopped};
use torrust_tracker::servers::udp::handle::Handle;
use torrust_tracker::servers::udp::launcher::Launcher;
use torrust_tracker_configuration::{Configuration, UdpTracker};
use torrust_tracker_primitives::info_hash::InfoHash;
use torrust_tracker_primitives::peer;

pub struct Environment<S: Debug> {
    pub config: Arc<UdpTracker>,
    pub tracker: Arc<Tracker>,
    pub registar: Registar,
    pub server: Service<S, Launcher, Handle>,
    pub addr: Option<SocketAddr>,
}

impl<S: Debug> Environment<S> {
    /// Add a torrent to the tracker
    #[allow(dead_code)]
    pub async fn add_torrent_peer(&self, info_hash: &InfoHash, peer: &peer::Peer) {
        self.tracker.upsert_peer_and_get_stats(info_hash, peer).await;
    }
}

impl Environment<Stopped> {
    #[allow(dead_code)]
    pub fn new(configuration: &Arc<Configuration>) -> Self {
        let tracker = tracker(configuration);

        let config = Arc::new(configuration.udp_trackers[0].clone());

        let addr = config.bind_address;

        let server = Service::new(Launcher::new(tracker.clone(), addr));

        Self {
            config,
            tracker,
            server,
            registar: Registar::default(),
            addr: None,
        }
    }

    #[allow(dead_code)]
    pub async fn start(self) -> Environment<Started<Handle>> {
        let server = self.server.start().unwrap();

        // reg_form wait for the service to be ready before proceeding
        let () = server
            .reg_form(self.registar.form())
            .await
            .expect("it should register a form");

        let addr = server.listening().await.expect("it should get address");

        Environment {
            config: self.config,
            tracker: self.tracker.clone(),
            registar: self.registar.clone(),
            server,
            addr: Some(addr),
        }
    }
}

impl Environment<Started<Handle>> {
    pub async fn new(configuration: &Arc<Configuration>) -> Self {
        Environment::<Stopped>::new(configuration).start().await
    }

    pub async fn stop(self) -> Environment<Stopped> {
        Environment {
            config: self.config,
            tracker: self.tracker,
            registar: Registar::default(),
            server: self.server.stop().await.unwrap(),
            addr: None,
        }
    }

    pub fn bind_address(&self) -> std::net::SocketAddr {
        self.addr.expect("it should get the listening address")
    }
}
