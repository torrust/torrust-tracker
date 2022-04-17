use std::io::Cursor;
use std::net::SocketAddr;
use std::sync::Arc;

use aquatic_udp_protocol::Response;
use log::{debug, info};
use tokio::net::UdpSocket;

use crate::TorrentTracker;
use crate::torrust_udp_tracker::{handle_packet, MAX_PACKET_SIZE};

pub struct UdpServer {
    socket: Arc<UdpSocket>,
    tracker: Arc<TorrentTracker>,
}

impl UdpServer {
    pub async fn new(tracker: Arc<TorrentTracker>, bind_address: &str) -> tokio::io::Result<UdpServer> {
        let socket = UdpSocket::bind(bind_address).await?;

        Ok(UdpServer {
            socket: Arc::new(socket),
            tracker,
        })
    }

    pub async fn start(&self, rx: tokio::sync::watch::Receiver<bool>) {
        loop {
            let mut rx = rx.clone();
            let mut data = [0; MAX_PACKET_SIZE];
            let socket = self.socket.clone();
            let tracker = self.tracker.clone();

            tokio::select! {
                _ = rx.changed() => {
                    info!("Stopping UDP server: {}...", socket.local_addr().unwrap());
                    break;
                }
                Ok((valid_bytes, remote_addr)) = socket.recv_from(&mut data) => {
                    let payload = data[..valid_bytes].to_vec();

                    debug!("Received {} bytes from {}", payload.len(), remote_addr);
                    debug!("{:?}", payload);

                    let response = handle_packet(remote_addr, payload, tracker).await;
                    UdpServer::send_response(socket, remote_addr, response).await;
                }
            }
        }
    }

    async fn send_response(socket: Arc<UdpSocket>, remote_addr: SocketAddr, response: Response) {
        debug!("sending response to: {:?}", &remote_addr);

        let buffer = vec![0u8; MAX_PACKET_SIZE];
        let mut cursor = Cursor::new(buffer);

        match response.write(&mut cursor) {
            Ok(_) => {
                let position = cursor.position() as usize;
                let inner = cursor.get_ref();

                debug!("{:?}", &inner[..position]);
                UdpServer::send_packet(socket, &remote_addr, &inner[..position]).await;
            }
            Err(_) => { debug!("could not write response to bytes."); }
        }
    }

    async fn send_packet(socket: Arc<UdpSocket>, remote_addr: &SocketAddr, payload: &[u8]) {
        // doesn't matter if it reaches or not
        let _ = socket.send_to(payload, remote_addr).await;
    }
}
