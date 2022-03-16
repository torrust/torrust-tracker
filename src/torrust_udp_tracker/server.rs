use std::io::Cursor;
use std::net::{SocketAddr};
use std::sync::Arc;
use aquatic_udp_protocol::{Response};
use log::debug;
use tokio::net::UdpSocket;
use crate::{TorrentTracker, UdpTrackerConfig};
use crate::torrust_udp_tracker::{handle_packet, MAX_PACKET_SIZE};

pub struct UdpServer {
    socket: UdpSocket,
    tracker: Arc<TorrentTracker>,
}

impl UdpServer {
    pub async fn new(tracker: Arc<TorrentTracker>, config: &UdpTrackerConfig) -> Result<UdpServer, std::io::Error> {
        let srv = UdpSocket::bind(&config.bind_address).await?;

        Ok(UdpServer {
            socket: srv,
            tracker,
        })
    }

    pub async fn start(&self) {
        loop {
            let mut data = [0; MAX_PACKET_SIZE];
            if let Ok((valid_bytes, remote_addr)) = self.socket.recv_from(&mut data).await {
                let data = &data[..valid_bytes];
                debug!("Received {} bytes from {}", data.len(), remote_addr);
                debug!("{:?}", data);
                let response = handle_packet(remote_addr, data, self.tracker.clone()).await;
                self.send_response(remote_addr, response).await;
            }
        }
    }

    async fn send_response(&self, remote_addr: SocketAddr, response: Response) {
        debug!("sending response to: {:?}", &remote_addr);

        let buffer = vec![0u8; MAX_PACKET_SIZE];
        let mut cursor = Cursor::new(buffer);

        match response.write(&mut cursor) {
            Ok(_) => {
                let position = cursor.position() as usize;
                let inner = cursor.get_ref();

                debug!("{:?}", &inner[..position]);
                self.send_packet(&remote_addr, &inner[..position]).await;
            }
            Err(_) => { debug!("could not write response to bytes."); }
        }
    }

    async fn send_packet(&self, remote_addr: &SocketAddr, payload: &[u8]) {
        // doesn't matter if it reaches or not
        let _ = self.socket.send_to(payload, remote_addr).await;
    }
}
