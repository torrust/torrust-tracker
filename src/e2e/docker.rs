//! Docker command wrapper.
use std::io;
use std::process::{Command, Output};
use std::thread::sleep;
use std::time::{Duration, Instant};

use log::debug;

/// Docker command wrapper.
pub struct Docker {}

pub struct RunningContainer {
    pub name: String,
    pub output: Output,
}

impl Drop for RunningContainer {
    /// Ensures that the temporary container is stopped and removed when the
    /// struct goes out of scope.
    fn drop(&mut self) {
        let _unused = Docker::stop(self);
        let _unused = Docker::remove(&self.name);
    }
}

impl Docker {
    /// Builds a Docker image from a given Dockerfile.
    ///
    /// # Errors
    ///
    /// Will fail if the docker build command fails.
    pub fn build(dockerfile: &str, tag: &str) -> io::Result<()> {
        let status = Command::new("docker")
            .args(["build", "-f", dockerfile, "-t", tag, "."])
            .status()?;

        if status.success() {
            Ok(())
        } else {
            Err(io::Error::new(
                io::ErrorKind::Other,
                format!("Failed to build Docker image from dockerfile {dockerfile}"),
            ))
        }
    }

    /// Runs a Docker container from a given image with multiple environment variables.
    ///
    /// # Arguments
    ///
    /// * `image` - The Docker image to run.
    /// * `container` - The name for the Docker container.
    /// * `env_vars` - A slice of tuples, each representing an environment variable as ("KEY", "value").
    ///
    /// # Errors
    ///
    /// Will fail if the docker run command fails.
    pub fn run(image: &str, container: &str, env_vars: &[(String, String)], ports: &[String]) -> io::Result<RunningContainer> {
        let initial_args = vec![
            "run".to_string(),
            "--detach".to_string(),
            "--name".to_string(),
            container.to_string(),
        ];

        // Add environment variables
        let mut env_var_args: Vec<String> = vec![];
        for (key, value) in env_vars {
            env_var_args.push("--env".to_string());
            env_var_args.push(format!("{key}={value}"));
        }

        // Add port mappings
        let mut port_args: Vec<String> = vec![];
        for port in ports {
            port_args.push("--publish".to_string());
            port_args.push(port.to_string());
        }

        let args = [initial_args, env_var_args, port_args, [image.to_string()].to_vec()].concat();

        debug!("Docker run args: {:?}", args);

        let output = Command::new("docker").args(args).output()?;

        if output.status.success() {
            Ok(RunningContainer {
                name: container.to_owned(),
                output,
            })
        } else {
            Err(io::Error::new(
                io::ErrorKind::Other,
                format!("Failed to run Docker image {image}"),
            ))
        }
    }

    /// Stops a Docker container.
    ///
    /// # Errors
    ///
    /// Will fail if the docker stop command fails.    
    pub fn stop(container: &RunningContainer) -> io::Result<()> {
        let status = Command::new("docker").args(["stop", &container.name]).status()?;

        if status.success() {
            Ok(())
        } else {
            Err(io::Error::new(
                io::ErrorKind::Other,
                format!("Failed to stop Docker container {}", container.name),
            ))
        }
    }

    /// Removes a Docker container.
    ///
    /// # Errors
    ///
    /// Will fail if the docker rm command fails.    
    pub fn remove(container: &str) -> io::Result<()> {
        let status = Command::new("docker").args(["rm", "-f", container]).status()?;

        if status.success() {
            Ok(())
        } else {
            Err(io::Error::new(
                io::ErrorKind::Other,
                format!("Failed to remove Docker container {container}"),
            ))
        }
    }

    /// Fetches logs from a Docker container.
    ///
    /// # Errors
    ///
    /// Will fail if the docker logs command fails.
    pub fn logs(container: &str) -> io::Result<String> {
        let output = Command::new("docker").args(["logs", container]).output()?;

        if output.status.success() {
            Ok(String::from_utf8_lossy(&output.stdout).to_string())
        } else {
            Err(io::Error::new(
                io::ErrorKind::Other,
                format!("Failed to fetch logs from Docker container {container}"),
            ))
        }
    }

    /// Checks if a Docker container is healthy.
    #[must_use]
    pub fn wait_until_is_healthy(name: &str, timeout: Duration) -> bool {
        let start = Instant::now();

        while start.elapsed() < timeout {
            let Ok(output) = Command::new("docker")
                .args(["ps", "-f", &format!("name={name}"), "--format", "{{.Status}}"])
                .output()
            else {
                return false;
            };

            let output_str = String::from_utf8_lossy(&output.stdout);

            if output_str.contains("(healthy)") {
                return true;
            }

            sleep(Duration::from_secs(1));
        }

        false
    }
}
