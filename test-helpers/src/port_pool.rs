use tokio::sync::Mutex;
use lazy_static::lazy_static;

lazy_static!{
    static ref PORT_POOL_UDP: PortPool = PortPool::new(49152, 51000);
    static ref PORT_POOL_TCP: PortPool = PortPool::new(49152, 51000);
}

type Port = u16;

pub struct PortPool {
    ports: Mutex<Vec<Port>>
}

impl PortPool {
    pub fn new(start: u16, end: u16) -> Self {
        assert!(end > start);

        let mut ports: Vec<Port> = Vec::new();

        for port in start..end {
            ports.push(port);
        }

        Self {
            ports: Mutex::new(ports)
        }
    }

    pub async fn acquire(&self) -> Option<u16> {
        self.ports.lock().await.pop()
    }

    // TODO: Release ports back to the pool. This is only necessary if the number of tests that need a port exceed the amount of ports available.
}

pub async fn acquire_udp() -> u16 {
    PORT_POOL_UDP.acquire().await.expect("UDP port pool is exhausted.")
}

pub async fn acquire_tcp() -> u16 {
    PORT_POOL_TCP.acquire().await.expect("TCP port pool is exhausted.")
}
