//! UDP tracker job starter.
//!
//! The [`udp_tracker::start_job`](crate::bootstrap::jobs::udp_tracker::start_job)
//! function starts a new UDP tracker server.
//!
//! > **NOTICE**: that the application can launch more than one UDP tracker
//! > on different ports. Refer to the [configuration documentation](https://docs.rs/torrust-tracker-configuration)
//! > for the configuration options.
use std::net::SocketAddr;
use std::sync::Arc;

use tokio::task::JoinHandle;
use torrust_tracker_configuration::UdpTracker;
use tracing::{info, instrument};

use crate::core;
use crate::servers::registar::Form;
use crate::servers::service::Service;
use crate::servers::udp::launcher::Launcher;
use crate::servers::udp::Version;

/// It starts a new UDP server with the provided configuration.
///
/// It spawns a new asynchronous task for the new UDP server.
///
/// # Panics
///
/// It will panic if the API binding address is not a valid socket.
/// It will panic if it is unable to start the UDP service.
/// It will panic if the task did not finish successfully.
///
#[must_use]
#[allow(clippy::async_yields_async)]
#[instrument(ret)]
pub async fn start_job(config: &UdpTracker, tracker: Arc<core::Tracker>, form: Form, version: Version) -> Option<JoinHandle<()>> {
    if config.enabled {
        let addr = config.bind_address;

        match version {
            Version::V0 => Some(start_v0(addr, tracker.clone(), form).await),
        }
    } else {
        info!("Note: Not loading Udp Tracker Service, Not Enabled in Configuration.");
        None
    }
}

#[allow(clippy::async_yields_async)]
#[instrument(ret)]
async fn start_v0(socket: SocketAddr, tracker: Arc<core::Tracker>, form: Form) -> JoinHandle<()> {
    let service = Service::new(Launcher::new(tracker, socket));

    let started = service.start().expect("it should start");

    let () = started.reg_form(form).await.expect("it should register");

    let (task, _) = started.run();

    tokio::spawn(async move {
        let server = task.await.expect("it should shutdown");
        drop(server);
    })
}
