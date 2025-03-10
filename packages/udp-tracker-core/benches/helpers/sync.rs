use std::time::{Instant, Duration};
use std::sync::Arc;
use bittorrent_udp_tracker_core::statistics;
use bittorrent_udp_tracker_core::services::connect::ConnectService;

use crate::helpers::utils::{sample_ipv4_remote_addr, sample_issue_time};
pub async fn connect_once(samples: u64) -> Duration {
            let (udp_core_stats_event_sender, _udp_core_stats_repository) = statistics::setup::factory(false);
            let udp_core_stats_event_sender = Arc::new(udp_core_stats_event_sender);
            let connect_service = Arc::new(ConnectService::new(udp_core_stats_event_sender));
let start = Instant::now();
    
    for _ in 0..samples {
            let _response = connect_service
                .handle_connect(sample_ipv4_remote_addr(), sample_issue_time())
                .await;
        
    }
    start.elapsed()

}
