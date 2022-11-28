use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct StatsResource {
    pub torrents: u32,
    pub seeders: u32,
    pub completed: u32,
    pub leechers: u32,
    pub tcp4_connections_handled: u32,
    pub tcp4_announces_handled: u32,
    pub tcp4_scrapes_handled: u32,
    pub tcp6_connections_handled: u32,
    pub tcp6_announces_handled: u32,
    pub tcp6_scrapes_handled: u32,
    pub udp4_connections_handled: u32,
    pub udp4_announces_handled: u32,
    pub udp4_scrapes_handled: u32,
    pub udp6_connections_handled: u32,
    pub udp6_announces_handled: u32,
    pub udp6_scrapes_handled: u32,
}
