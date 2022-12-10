use std::io::Cursor;
use std::net::SocketAddr;
use std::sync::Arc;

use aquatic_udp_protocol::Response;
use log::{debug, error, info};
use tokio::net::UdpSocket;

use crate::tracker;
use crate::udp::handlers::handle_packet;
use crate::udp::MAX_PACKET_SIZE;

pub struct Udp {
    socket: Arc<UdpSocket>,
    tracker: Arc<tracker::Tracker>,
}

impl Udp {
    /// # Errors
    ///
    /// Will return `Err` unable to bind to the supplied `bind_address`.
    pub async fn new(tracker: Arc<tracker::Tracker>, bind_address: &str) -> tokio::io::Result<Udp> {
        let socket = UdpSocket::bind(bind_address).await?;

        Ok(Udp {
            socket: Arc::new(socket),
            tracker,
        })
    }

    /// # Panics
    ///
    /// It would panic if unable to resolve the `local_addr` from the supplied ´socket´.
    pub async fn start(&self) {
        loop {
            let mut data = [0; MAX_PACKET_SIZE];
            let socket = self.socket.clone();
            let tracker = self.tracker.clone();

            tokio::select! {
                _ = tokio::signal::ctrl_c() => {
                    info!("Stopping UDP server: {}..", socket.local_addr().unwrap());
                    break;
                }
                Ok((valid_bytes, remote_addr)) = socket.recv_from(&mut data) => {
                    let payload = data[..valid_bytes].to_vec();

                    info!("Received {} bytes from {}", payload.len(), remote_addr);
                    info!("{:?}", payload);

                    let response = handle_packet(remote_addr, payload, tracker).await;
                    Udp::send_response(socket, remote_addr, response).await;
                }
            }
        }
    }

    async fn send_response(socket: Arc<UdpSocket>, remote_addr: SocketAddr, response: Response) {
        info!("sending response to: {:?}", &remote_addr);

        let buffer = vec![0u8; MAX_PACKET_SIZE];
        let mut cursor = Cursor::new(buffer);

        match response.write(&mut cursor) {
            Ok(_) => {
                #[allow(clippy::cast_possible_truncation)]
                let position = cursor.position() as usize;
                let inner = cursor.get_ref();

                debug!("{:?}", &inner[..position]);
                Udp::send_packet(socket, &remote_addr, &inner[..position]).await;
            }
            Err(_) => {
                error!("could not write response to bytes.");
            }
        }
    }

    async fn send_packet(socket: Arc<UdpSocket>, remote_addr: &SocketAddr, payload: &[u8]) {
        // doesn't matter if it reaches or not
        drop(socket.send_to(payload, remote_addr).await);
    }
}
