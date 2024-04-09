use std::time::Duration;

use rand::distributions::Alphanumeric;
use rand::Rng;
use tracing::{debug, error, info};

use super::docker::{RunOptions, RunningContainer};
use super::trace_parser::RunningServices;
use crate::console::ci::e2e::docker::Docker;

#[derive(Debug)]
pub struct TrackerContainer {
    pub image: String,
    pub name: String,
    pub running: Option<RunningContainer>,
}

impl Drop for TrackerContainer {
    /// Ensures that the temporary container is removed when the
    /// struct goes out of scope.
    fn drop(&mut self) {
        info!("Dropping tracker container: {}", self.name);
        if Docker::container_exist(&self.name) {
            let _unused = Docker::remove(&self.name);
        }
    }
}

impl TrackerContainer {
    #[must_use]
    pub fn new(tag: &str, container_name_prefix: &str) -> Self {
        Self {
            image: tag.to_owned(),
            name: Self::generate_random_container_name(container_name_prefix),
            running: None,
        }
    }

    /// # Panics
    ///
    /// Will panic if it can't build the docker image.
    pub fn build_image(&self) {
        info!("Building tracker container image with tag: {} ...", self.image);
        Docker::build("./Containerfile", &self.image).expect("A tracker local docker image should be built");
    }

    /// # Panics
    ///
    /// Will panic if it can't run the container.
    pub fn run(&mut self, options: &RunOptions) {
        info!("Running docker tracker image: {} ...", self.name);

        let container = Docker::run(&self.image, &self.name, options).expect("A tracker local docker image should be running");

        info!("Waiting for the container {} to be healthy ...", self.name);

        let is_healthy = Docker::wait_until_is_healthy(&self.name, Duration::from_secs(10));

        assert!(is_healthy, "Unhealthy tracker container: {}", &self.name);

        info!("Container {} is healthy ...", &self.name);

        self.running = Some(container);

        self.assert_there_are_no_panics_in_traces();
    }

    /// # Panics
    ///
    /// Will panic if it can't get the traces from the running container.
    #[must_use]
    pub fn running_services(&self) -> RunningServices {
        let traces = Docker::traces(&self.name).expect("Traces should be captured from running container");

        debug!("Parsing running services from traces. Traces :\n{traces}");

        RunningServices::parse_from_traces(&traces)
    }

    /// # Panics
    ///
    /// Will panic if it can't stop the container.
    pub fn stop(&mut self) {
        match &self.running {
            Some(container) => {
                info!("Stopping docker tracker container: {} ...", self.name);

                Docker::stop(container).expect("Container should be stopped");

                self.assert_there_are_no_panics_in_traces();
            }
            None => {
                if Docker::is_container_running(&self.name) {
                    error!("Tracker container {} was started manually", self.name);
                } else {
                    info!("Docker tracker container is not running: {} ...", self.name);
                }
            }
        }

        self.running = None;
    }

    /// # Panics
    ///
    /// Will panic if it can't remove the container.
    pub fn remove(&self) {
        match &self.running {
            Some(_running_container) => {
                error!("Can't remove running container: {} ...", self.name);
            }
            None => {
                info!("Removing docker tracker container: {} ...", self.name);
                Docker::remove(&self.name).expect("Container should be removed");
            }
        }
    }

    fn generate_random_container_name(prefix: &str) -> String {
        let rand_string: String = rand::thread_rng()
            .sample_iter(&Alphanumeric)
            .take(20)
            .map(char::from)
            .collect();

        format!("{prefix}{rand_string}")
    }

    fn assert_there_are_no_panics_in_traces(&self) {
        let traces = Docker::traces(&self.name).expect("Traces should be captured from running container");

        assert!(
            !(traces.contains(" panicked at ") || traces.contains("RUST_BACKTRACE=1")),
            "{}",
            format!("Panics found is traces:\n{traces}")
        );
    }
}
