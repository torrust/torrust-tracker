use std::net::{IpAddr, Ipv4Addr, SocketAddr};
use std::sync::Arc;
use std::time::Duration;

use criterion::{Criterion, criterion_group, criterion_main};
use torrust_clock::clock::Time as _;
use torrust_net_primitives::service_binding::{Protocol, ServiceBinding};
use torrust_peer_id::PeerId;
use torrust_tracker_core::scrape_handler::ScrapeHandler;
use torrust_tracker_core::torrent::repository::in_memory::InMemoryTorrentRepository;
use torrust_tracker_core::whitelist::authorization::WhitelistAuthorization;
use torrust_tracker_core::whitelist::repository::in_memory::InMemoryWhitelist;
use torrust_tracker_events::bus::SenderStatus;
use torrust_tracker_primitives::peer::fixture::PeerBuilder;
use torrust_tracker_primitives::{ConfigurationInstanceId, ServiceRole};
use torrust_tracker_test_helpers::configuration;
use torrust_tracker_udp_core::ConnectionIdValidationPolicy;
use torrust_tracker_udp_core::connection_cookie::{gen_remote_fingerprint, make};
use torrust_tracker_udp_core::event::bus::EventBus;
use torrust_tracker_udp_core::event::sender::Broadcaster;
use torrust_tracker_udp_core::services::scrape::ScrapeService;
use torrust_tracker_udp_protocol::{InfoHash, ScrapeRequest, TransactionId};
use torrust_tracker_udp_server::handlers::CookieValidationContext;
use torrust_tracker_udp_server::handlers::scrape::handle_scrape;

const SCRAPE_TORRENT_COUNT: u8 = 74;
const BENCHMARK_COOKIE_VALIDITY_SECS: f64 = 24.0 * 60.0 * 60.0;

struct ScrapeBenchmarkContext {
    scrape_service: Arc<ScrapeService>,
    client_socket_addr: SocketAddr,
    server_service_binding: ServiceBinding,
    request: ScrapeRequest,
    cookie_validation: CookieValidationContext,
}

impl ScrapeBenchmarkContext {
    async fn new() -> Self {
        let configuration = configuration::ephemeral_public();
        let client_socket_addr = SocketAddr::new(IpAddr::V4(Ipv4Addr::LOCALHOST), 8080);
        let server_socket_addr = SocketAddr::new(IpAddr::V4(Ipv4Addr::new(203, 0, 113, 196)), 6969);
        let server_service_binding = ServiceBinding::new(Protocol::UDP, server_socket_addr).unwrap();
        let in_memory_whitelist = Arc::new(InMemoryWhitelist::default());
        let whitelist_authorization = Arc::new(WhitelistAuthorization::new(&configuration.core, &in_memory_whitelist));
        let in_memory_torrent_repository = Arc::new(InMemoryTorrentRepository::default());
        let scrape_handler = Arc::new(ScrapeHandler::new(&whitelist_authorization, &in_memory_torrent_repository));
        let event_bus = Arc::new(EventBus::new(SenderStatus::Disabled, Broadcaster::default()));
        let request_info_hashes = (1..=SCRAPE_TORRENT_COUNT)
            .map(|value| InfoHash([value; 20]))
            .collect::<Vec<_>>();

        for (peer_index, info_hash) in request_info_hashes.iter().enumerate() {
            let peer_index = u8::try_from(peer_index).expect("benchmark peer index must fit in u8");
            let peer = PeerBuilder::default()
                .with_peer_id(&torrust_tracker_primitives::PeerId(PeerId([peer_index; 20]).0))
                .with_peer_address(SocketAddr::new(IpAddr::V4(Ipv4Addr::new(192, 0, 2, peer_index)), 6881))
                .with_bytes_left_to_download(0)
                .into();

            in_memory_torrent_repository
                .handle_announcement(&info_hash.0.into(), &peer, None)
                .await;
        }

        let issue_time = torrust_clock::clock::Working::now().as_secs_f64();
        let request = ScrapeRequest {
            connection_id: make(gen_remote_fingerprint(&client_socket_addr), issue_time).unwrap(),
            transaction_id: TransactionId::new(0i32),
            info_hashes: request_info_hashes,
        };

        Self {
            scrape_service: Arc::new(ScrapeService::new(
                scrape_handler,
                event_bus.sender(),
                ConfigurationInstanceId::new(ServiceRole::UdpTracker, 0),
            )),
            client_socket_addr,
            server_service_binding,
            request,
            cookie_validation: CookieValidationContext {
                valid_range: (issue_time - BENCHMARK_COOKIE_VALIDITY_SECS)..(issue_time + BENCHMARK_COOKIE_VALIDITY_SECS),
                connection_id_validation: ConnectionIdValidationPolicy::Strict,
            },
        }
    }
}

fn bench_scrape_once(c: &mut Criterion) {
    let runtime = tokio::runtime::Runtime::new().unwrap();
    let context = runtime.block_on(ScrapeBenchmarkContext::new());
    let mut group = c.benchmark_group("udp_tracker/handle_scrape_once");
    group.warm_up_time(Duration::from_millis(500));
    group.measurement_time(Duration::from_secs(1));

    group.bench_function("74_torrents", |bench| {
        bench.to_async(&runtime).iter(|| async {
            handle_scrape(
                &context.scrape_service,
                context.client_socket_addr,
                context.server_service_binding.clone(),
                &context.request,
                &None,
                context.cookie_validation.clone(),
            )
            .await
            .unwrap();
        });
    });

    group.finish();
}

criterion_group!(benches, bench_scrape_once);
criterion_main!(benches);
