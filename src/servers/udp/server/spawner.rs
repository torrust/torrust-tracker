use std::net::SocketAddr;
use std::sync::Arc;

use derive_more::Constructor;
use tokio::sync::oneshot;
use tokio::task::JoinHandle;

use super::launcher::Launcher;
use crate::bootstrap::jobs::Started;
use crate::core::Tracker;
use crate::servers::signals::Halted;

#[derive(Constructor, Copy, Clone, Debug)]
pub struct Spawner {
    pub bind_to: SocketAddr,
}

impl Spawner {
    /// It spawns a new tasks to run the UDP server instance.
    ///
    /// # Panics
    ///
    /// It would panic if unable to resolve the `local_addr` from the supplied ´socket´.
    pub fn start(
        &self,
        tracker: Arc<Tracker>,
        tx_start: oneshot::Sender<Started>,
        rx_halt: oneshot::Receiver<Halted>,
    ) -> JoinHandle<Spawner> {
        let launcher = Spawner::new(self.bind_to);
        tokio::spawn(async move {
            Launcher::run_with_graceful_shutdown(tracker, launcher.bind_to, tx_start, rx_halt).await;
            launcher
        })
    }
}
