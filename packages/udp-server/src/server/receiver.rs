use std::cell::RefCell;
use std::net::SocketAddr;
use std::pin::Pin;
use std::sync::Arc;
use std::task::{Context, Poll};

use futures::Stream;
use torrust_tracker_udp_protocol::MAX_PACKET_SIZE;

use super::bound_socket::BoundSocket;
use crate::RawRequest;

pub struct Receiver {
    pub socket: Arc<BoundSocket>,
    data: RefCell<[u8; MAX_PACKET_SIZE]>,
}

impl Receiver {
    #[must_use]
    pub const fn new(bound_socket: Arc<BoundSocket>) -> Self {
        Self {
            socket: bound_socket,
            data: RefCell::new([0; MAX_PACKET_SIZE]),
        }
    }

    pub fn bound_socket_address(&self) -> SocketAddr {
        self.socket.address()
    }
}

impl Stream for Receiver {
    type Item = std::io::Result<RawRequest>;

    fn poll_next(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        let mut buf = *self.data.borrow_mut();
        let mut buf = tokio::io::ReadBuf::new(&mut buf);

        let Poll::Ready(ready) = self.socket.poll_recv_from(cx, &mut buf) else {
            return Poll::Pending;
        };

        let res = match ready {
            Ok(from) => {
                let payload = buf.filled().to_vec();
                let request = RawRequest { payload, from };
                Some(Ok(request))
            }
            Err(err) => Some(Err(err)),
        };

        Poll::Ready(res)
    }
}

#[cfg(test)]
mod tests {
    use std::net::{IpAddr, Ipv4Addr, SocketAddr};
    use std::sync::Arc;
    use std::time::Duration;

    use futures::StreamExt;
    use tokio::net::UdpSocket;

    use super::Receiver;
    use crate::RawRequest;
    use crate::server::bound_socket::BoundSocket;

    const RECEIVE_TIMEOUT: Duration = Duration::from_secs(1);

    struct ReceiverWithQueuedLoopbackDatagram {
        receiver: Receiver,
        expected_request: RawRequest,
    }

    impl ReceiverWithQueuedLoopbackDatagram {
        async fn new(payload: Vec<u8>) -> Self {
            let bound_socket = Arc::new(
                BoundSocket::bind(SocketAddr::new(IpAddr::V4(Ipv4Addr::LOCALHOST), 0), false)
                    .expect("UDP receiver socket should bind"),
            );

            let client_socket = UdpSocket::bind(SocketAddr::new(IpAddr::V4(Ipv4Addr::LOCALHOST), 0))
                .await
                .expect("UDP client socket should bind");

            let expected_request = RawRequest {
                payload: payload.clone(),
                from: client_socket
                    .local_addr()
                    .expect("UDP client socket should have a local address"),
            };

            client_socket
                .send_to(&payload, bound_socket.address())
                .await
                .expect("UDP client should send the datagram");

            Self {
                receiver: Receiver::new(bound_socket),
                expected_request,
            }
        }
    }

    #[tokio::test]
    async fn should_yield_a_raw_request_with_the_received_datagram_and_sender_address() {
        // Arrange
        let mut scenario = ReceiverWithQueuedLoopbackDatagram::new(vec![1, 2, 3]).await;

        // Act
        let request = tokio::time::timeout(RECEIVE_TIMEOUT, scenario.receiver.next())
            .await
            .expect("receiver should yield the queued loopback datagram before the test deadline")
            .expect("UDP receiver stream should not end")
            .expect("loopback receive should succeed");

        // Assert
        assert_eq!(request, scenario.expected_request);
    }
}
