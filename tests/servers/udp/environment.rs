use std::net::SocketAddr;
use std::sync::Arc;

use torrust_tracker::bootstrap::app::initialize_with_configuration;
use torrust_tracker::core::Tracker;
use torrust_tracker::servers::registar::Registar;
use torrust_tracker::servers::udp::server::{Launcher, Running, Stopped, UdpServer};
use torrust_tracker_configuration::{Configuration, UdpTracker};
use torrust_tracker_primitives::info_hash::InfoHash;
use torrust_tracker_primitives::peer;

pub struct Environment<S> {
    pub config: Arc<UdpTracker>,
    pub tracker: Arc<Tracker>,
    pub registar: Registar,
    pub server: UdpServer<S>,
}

impl<S> Environment<S> {
    /// Add a torrent to the tracker
    #[allow(dead_code)]
    pub async fn add_torrent(&self, info_hash: &InfoHash, peer: &peer::Peer) {
        self.tracker.update_torrent_with_peer_and_get_stats(info_hash, peer).await;
    }
}

impl Environment<Stopped> {
    #[allow(dead_code)]
    pub fn new(configuration: &Arc<Configuration>) -> Self {
        let tracker = initialize_with_configuration(configuration);

        let config = Arc::new(configuration.udp_trackers[0].clone());

        let bind_to = config
            .bind_address
            .parse::<std::net::SocketAddr>()
            .expect("Tracker API bind_address invalid.");

        let server = UdpServer::new(Launcher::new(bind_to));

        Self {
            config,
            tracker,
            registar: Registar::default(),
            server,
        }
    }

    #[allow(dead_code)]
    pub async fn start(self) -> Environment<Running> {
        Environment {
            config: self.config,
            tracker: self.tracker.clone(),
            registar: self.registar.clone(),
            server: self.server.start(self.tracker, self.registar.give_form()).await.unwrap(),
        }
    }
}

impl Environment<Running> {
    pub async fn new(configuration: &Arc<Configuration>) -> Self {
        Environment::<Stopped>::new(configuration).start().await
    }

    #[allow(dead_code)]
    pub async fn stop(self) -> Environment<Stopped> {
        Environment {
            config: self.config,
            tracker: self.tracker,
            registar: Registar::default(),
            server: self.server.stop().await.expect("it stop the udp tracker service"),
        }
    }

    pub fn bind_address(&self) -> SocketAddr {
        self.server.state.binding
    }
}

#[cfg(test)]
mod tests {
    use std::time::Duration;

    use tokio::time::sleep;
    use torrust_tracker_test_helpers::configuration;

    use crate::servers::udp::Started;

    #[tokio::test]
    async fn it_should_make_and_stop_udp_server() {
        let env = Started::new(&configuration::ephemeral().into()).await;
        sleep(Duration::from_secs(1)).await;
        env.stop().await;
        sleep(Duration::from_secs(1)).await;
    }
}
