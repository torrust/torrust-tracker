    use std::net::{IpAddr, Ipv4Addr,  SocketAddr};

    use futures::future::BoxFuture;
    use mockall::mock;
    use tokio::sync::mpsc::error::SendError;


    use bittorrent_udp_tracker_core::statistics;

    pub(crate) fn sample_ipv4_remote_addr() -> SocketAddr {
        sample_ipv4_socket_address()
    }

  /*  fn sample_ipv4_remote_addr_fingerprint() -> u64 {
        gen_remote_fingerprint(&sample_ipv4_socket_address())
    }

    fn sample_ipv6_remote_addr() -> SocketAddr {
        sample_ipv6_socket_address()
    }

    fn sample_ipv6_remote_addr_fingerprint() -> u64 {
        gen_remote_fingerprint(&sample_ipv6_socket_address())
    }
    */

    pub(crate) fn sample_ipv4_socket_address() -> SocketAddr {
        SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), 8080)
    }

/*    fn sample_ipv6_socket_address() -> SocketAddr {
        SocketAddr::new(IpAddr::V6(Ipv6Addr::new(0, 0, 0, 0, 0, 0, 0, 1)), 8080)
    }
*/
    pub(crate) fn sample_issue_time() -> f64 {
        1_000_000_000_f64
    }

    mock! {
        pub(crate) UdpCoreStatsEventSender {}
        impl statistics::event::sender::Sender for UdpCoreStatsEventSender {
             fn send_event(&self, event: statistics::event::Event) -> BoxFuture<'static,Option<Result<(),SendError<statistics::event::Event> > > > ;
        }
    }

