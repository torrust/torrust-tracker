use std::cell::RefCell;
use std::net::SocketAddr;
use std::pin::Pin;
use std::sync::Arc;
use std::task::{Context, Poll};

use futures::Stream;

use super::bound_socket::BoundSocket;
use super::RawRequest;
use crate::shared::bit_torrent::tracker::udp::MAX_PACKET_SIZE;

pub struct Receiver {
    pub bound_socket: Arc<BoundSocket>,
    data: RefCell<[u8; MAX_PACKET_SIZE]>,
}

impl Receiver {
    #[must_use]
    pub fn new(bound_socket: Arc<BoundSocket>) -> Self {
        Receiver {
            bound_socket,
            data: RefCell::new([0; MAX_PACKET_SIZE]),
        }
    }

    pub fn bound_socket_address(&self) -> SocketAddr {
        self.bound_socket.address()
    }
}

impl Stream for Receiver {
    type Item = std::io::Result<RawRequest>;

    fn poll_next(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        let mut buf = *self.data.borrow_mut();
        let mut buf = tokio::io::ReadBuf::new(&mut buf);

        let Poll::Ready(ready) = self.bound_socket.poll_recv_from(cx, &mut buf) else {
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
