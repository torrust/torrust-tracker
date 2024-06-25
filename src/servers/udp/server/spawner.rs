//! A thin wrapper for tokio spawn to launch the UDP server launcher as a new task.
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
    /// It spawns a new task to run the UDP server instance.
    ///
    /// # Panics
    ///
    /// It would panic if unable to resolve the `local_addr` from the supplied ´socket´.
    pub fn spawn_launcher(
        &self,
        tracker: Arc<Tracker>,
        tx_start: oneshot::Sender<Started>,
        rx_halt: oneshot::Receiver<Halted>,
    ) -> JoinHandle<Spawner> {
        let spawner = Self::new(self.bind_to);

        tokio::spawn(async move {
            Launcher::run_with_graceful_shutdown(tracker, spawner.bind_to, tx_start, rx_halt).await;
            spawner
        })
    }
}
