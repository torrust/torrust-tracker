use std::net::SocketAddr;
use std::sync::Arc;

use tokio::net::UdpSocket;

/// A generic UDP client
pub struct Client {
    pub socket: Arc<UdpSocket>,
}

impl Client {
    #[allow(dead_code)]
    pub async fn connected(remote_socket_addr: &SocketAddr, local_socket_addr: &SocketAddr) -> Client {
        let client = Client::bind(local_socket_addr).await;
        client.connect(remote_socket_addr).await;
        client
    }

    pub async fn bind(local_socket_addr: &SocketAddr) -> Self {
        let socket = UdpSocket::bind(local_socket_addr).await.unwrap();
        Self {
            socket: Arc::new(socket),
        }
    }

    pub async fn connect(&self, remote_address: &SocketAddr) {
        self.socket.connect(remote_address).await.unwrap();
    }

    #[allow(dead_code)]
    pub async fn send(&self, bytes: &[u8]) -> usize {
        self.socket.writable().await.unwrap();
        self.socket.send(bytes).await.unwrap()
    }

    #[allow(dead_code)]
    pub async fn receive(&self, bytes: &mut [u8]) -> usize {
        self.socket.readable().await.unwrap();
        self.socket.recv(bytes).await.unwrap()
    }
}
