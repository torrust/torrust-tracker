//! Docker compose command wrapper.
use std::io;
use std::path::{Path, PathBuf};
use std::process::{Command, Output};

#[derive(Clone, Debug)]
pub struct DockerCompose {
    file: PathBuf,
    project: String,
    env_vars: Vec<(String, String)>,
}

#[derive(Debug)]
pub struct RunningCompose {
    compose: DockerCompose,
    is_active: bool,
}

impl Drop for RunningCompose {
    fn drop(&mut self) {
        if !self.is_active {
            return;
        }

        if let Err(error) = self.compose.down() {
            tracing::error!(
                "Failed to stop compose project '{}' from '{}': {error}",
                self.compose.project,
                self.compose.file.display()
            );
        }
    }
}

impl RunningCompose {
    /// Returns the compose project name for this running stack.
    #[must_use]
    pub fn project(&self) -> &str {
        &self.compose.project
    }

    /// Disables the automatic teardown so containers are left running after this
    /// guard is dropped.  Useful for post-run debugging.
    pub fn keep(&mut self) {
        self.is_active = false;
    }
}

impl DockerCompose {
    #[must_use]
    pub fn new(file: &Path, project: &str) -> Self {
        Self {
            file: file.to_path_buf(),
            project: project.to_string(),
            env_vars: vec![],
        }
    }

    #[must_use]
    pub fn with_env(mut self, key: &str, value: &str) -> Self {
        self.env_vars.push((key.to_string(), value.to_string()));
        self
    }

    /// Runs docker compose up and returns a guard that will always run `down --volumes` on drop.
    ///
    /// # Errors
    ///
    /// Returns an error when docker compose fails to start all services.
    pub fn up(&self) -> io::Result<RunningCompose> {
        let output = self.run_compose(&["up", "--wait", "--detach"])?;

        if output.status.success() {
            Ok(RunningCompose {
                compose: self.clone(),
                is_active: true,
            })
        } else {
            Err(io::Error::new(
                io::ErrorKind::Other,
                format!(
                    "docker compose up failed for file '{}' and project '{}': {}",
                    self.file.display(),
                    self.project,
                    String::from_utf8_lossy(&output.stderr)
                ),
            ))
        }
    }

    /// Runs docker compose down --volumes.
    ///
    /// # Errors
    ///
    /// Returns an error when docker compose cannot stop and remove resources.
    pub fn down(&self) -> io::Result<()> {
        let output = self.run_compose(&["down", "--volumes"])?;

        if output.status.success() {
            Ok(())
        } else {
            Err(io::Error::new(
                io::ErrorKind::Other,
                format!(
                    "docker compose down failed for file '{}' and project '{}': {}",
                    self.file.display(),
                    self.project,
                    String::from_utf8_lossy(&output.stderr)
                ),
            ))
        }
    }

    /// Resolves an ephemeral host port from a service published container port.
    ///
    /// # Errors
    ///
    /// Returns an error when the compose command fails or port parsing fails.
    pub fn port(&self, service: &str, container_port: u16) -> io::Result<u16> {
        let output = self.run_compose(&["port", service, &container_port.to_string()])?;

        if !output.status.success() {
            return Err(io::Error::new(
                io::ErrorKind::Other,
                format!(
                    "docker compose port failed for file '{}' and project '{}', service '{}' and port '{}': stderr: {} stdout: {}",
                    self.file.display(),
                    self.project,
                    service,
                    container_port,
                    String::from_utf8_lossy(&output.stderr),
                    String::from_utf8_lossy(&output.stdout)
                ),
            ));
        }

        let stdout = String::from_utf8_lossy(&output.stdout);
        let first_line = stdout
            .lines()
            .next()
            .ok_or_else(|| io::Error::new(io::ErrorKind::Other, "docker compose port returned no output"))?;

        let host_port = first_line
            .rsplit(':')
            .next()
            .ok_or_else(|| io::Error::new(io::ErrorKind::Other, "docker compose port output has no ':' separator"))?
            .parse::<u16>()
            .map_err(|_| io::Error::new(io::ErrorKind::Other, format!("invalid host port in output: '{first_line}'")))?;

        Ok(host_port)
    }

    /// Runs `docker compose exec` in non-interactive mode for scripted commands.
    ///
    /// # Errors
    ///
    /// Returns an error when command execution fails.
    pub fn exec(&self, service: &str, cmd: &[&str]) -> io::Result<Output> {
        let mut args = vec!["exec".to_string(), "-T".to_string(), service.to_string()];
        args.extend(cmd.iter().map(|value| (*value).to_string()));

        self.run_compose_strings(&args)
    }

    /// Runs `docker compose ps -a` and returns stdout.
    ///
    /// # Errors
    ///
    /// Returns an error when the compose command fails.
    pub fn ps(&self) -> io::Result<String> {
        let output = self.run_compose(&["ps", "-a"])?;

        if output.status.success() {
            Ok(String::from_utf8_lossy(&output.stdout).to_string())
        } else {
            Err(io::Error::new(
                io::ErrorKind::Other,
                format!(
                    "docker compose ps failed for file '{}' and project '{}': {}",
                    self.file.display(),
                    self.project,
                    String::from_utf8_lossy(&output.stderr)
                ),
            ))
        }
    }

    /// Runs `docker compose logs --no-color <services...>` and returns stdout.
    ///
    /// # Errors
    ///
    /// Returns an error when the compose command fails.
    pub fn logs(&self, services: &[&str]) -> io::Result<String> {
        let mut args = vec!["logs".to_string(), "--no-color".to_string()];
        args.extend(services.iter().map(|service| (*service).to_string()));

        let output = self.run_compose_strings(&args)?;

        if output.status.success() {
            Ok(String::from_utf8_lossy(&output.stdout).to_string())
        } else {
            Err(io::Error::new(
                io::ErrorKind::Other,
                format!(
                    "docker compose logs failed for file '{}' and project '{}': {}",
                    self.file.display(),
                    self.project,
                    String::from_utf8_lossy(&output.stderr)
                ),
            ))
        }
    }

    fn run_compose(&self, args: &[&str]) -> io::Result<Output> {
        let args_as_strings: Vec<String> = args.iter().map(|value| (*value).to_string()).collect();
        self.run_compose_strings(&args_as_strings)
    }

    fn run_compose_strings(&self, args: &[String]) -> io::Result<Output> {
        let mut command = Command::new("docker");
        command.envs(self.env_vars.iter().map(|(key, value)| (key, value)));
        command.arg("compose");
        command.arg("-f").arg(&self.file);
        command.arg("-p").arg(&self.project);
        command.args(args);

        tracing::info!("Running docker compose command: {:?}", command);

        command.output()
    }
}
