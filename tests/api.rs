/// Integration tests for the tracker API
///
/// cargo test tracker_api -- --nocapture
extern crate rand;

mod common;

mod tracker_api {
    use core::panic;
    use std::env;
    use std::sync::atomic::{AtomicBool, Ordering};
    use std::sync::Arc;

    use tokio::task::JoinHandle;
    use tokio::time::{sleep, Duration};
    use torrust_tracker::api::resources::auth_key_resource::AuthKeyResource;
    use torrust_tracker::jobs::tracker_api;
    use torrust_tracker::tracker::key::AuthKey;
    use torrust_tracker::tracker::statistics::StatsTracker;
    use torrust_tracker::tracker::TorrentTracker;
    use torrust_tracker::{ephemeral_instance_keys, logging, static_time, Configuration};

    use crate::common::ephemeral_random_port;

    #[tokio::test]
    async fn should_generate_a_new_auth_key() {
        let configuration = tracker_configuration();
        let api_server = new_running_api_server(configuration.clone()).await;

        let bind_address = api_server.bind_address.unwrap().clone();
        let seconds_valid = 60;
        let api_token = configuration.http_api.access_tokens.get_key_value("admin").unwrap().1.clone();

        let url = format!("http://{}/api/key/{}?token={}", &bind_address, &seconds_valid, &api_token);

        let auth_key: AuthKeyResource = reqwest::Client::new().post(url).send().await.unwrap().json().await.unwrap();

        // Verify the key with the tracker
        assert!(api_server
            .tracker
            .unwrap()
            .verify_auth_key(&AuthKey::from(auth_key))
            .await
            .is_ok());
    }

    fn tracker_configuration() -> Arc<Configuration> {
        let mut config = Configuration::default();
        config.log_level = Some("off".to_owned());

        config.http_api.bind_address = format!("127.0.0.1:{}", ephemeral_random_port());

        // Temp database
        let temp_directory = env::temp_dir();
        let temp_file = temp_directory.join("data.db");
        config.db_path = temp_file.to_str().unwrap().to_owned();

        Arc::new(config)
    }

    async fn new_running_api_server(configuration: Arc<Configuration>) -> ApiServer {
        let mut api_server = ApiServer::new();
        api_server.start(configuration).await;
        api_server
    }

    pub struct ApiServer {
        pub started: AtomicBool,
        pub job: Option<JoinHandle<()>>,
        pub bind_address: Option<String>,
        pub tracker: Option<Arc<TorrentTracker>>,
    }

    impl ApiServer {
        pub fn new() -> Self {
            Self {
                started: AtomicBool::new(false),
                job: None,
                bind_address: None,
                tracker: None,
            }
        }

        pub async fn start(&mut self, configuration: Arc<Configuration>) {
            if !self.started.load(Ordering::Relaxed) {
                self.bind_address = Some(configuration.http_api.bind_address.clone());

                // Set the time of Torrust app starting
                lazy_static::initialize(&static_time::TIME_AT_APP_START);

                // Initialize the Ephemeral Instance Random Seed
                lazy_static::initialize(&ephemeral_instance_keys::RANDOM_SEED);

                // Initialize stats tracker
                let (stats_event_sender, stats_repository) = StatsTracker::new_active_instance();

                // Initialize Torrust tracker
                let tracker = match TorrentTracker::new(configuration.clone(), Some(stats_event_sender), stats_repository) {
                    Ok(tracker) => Arc::new(tracker),
                    Err(error) => {
                        panic!("{}", error)
                    }
                };
                self.tracker = Some(tracker.clone());

                // Initialize logging
                logging::setup_logging(&configuration);

                // Start the HTTP API job
                self.job = Some(tracker_api::start_job(&configuration, tracker.clone()));

                self.started.store(true, Ordering::Relaxed);

                // Wait to give time to the API server to be ready to accept requests
                sleep(Duration::from_millis(100)).await;
            }
        }
    }
}
