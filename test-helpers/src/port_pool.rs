use tokio::sync::Mutex;
use lazy_static::lazy_static;

lazy_static!{
    static ref PORT_POOL_UDP: PortPool = PortPool::new(49152, 51000);
    static ref PORT_POOL_TCP: PortPool = PortPool::new(49152, 51000);
}

type Port = u16;

/// A resource pool of ports.
/// Can be used to avoid address already in use errors.
pub struct PortPool {
    ports: Mutex<Vec<Port>>
}

impl PortPool {
    /// Returns a new `PortPool`.
    ///
    /// # Arguments
    ///
    /// * `start` - u16 inclusive.
    /// * `end` - u16 exclusive.
    ///
    /// # Example
    ///
    /// ```
    /// let port_pool = PortPool::new(0, 2);
    ///
    /// let x = port_pool.acquire(); // x = Some(1)
    /// let y = port_pool.acquire(); // y = Some(0)
    /// let z = port_pool.acquire(); // z = None
    /// ```
    ///
    /// # Panics
    ///
    /// Will panic if end is not higher than start.
    #[must_use]
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
