//! Owns tracker children expected to fail startup, including their workspaces and permission guards.
//!
//! `wait_for_exit` performs deadline-bounded reaping and restoration; `Drop` is a non-panicking,
//! best-effort fallback. This module must not participate in normal tracker readiness.

use std::io::Write as _;
use std::os::unix::fs::PermissionsExt as _;
use std::path::PathBuf;
use std::time::Duration;

use tokio::process::{Child, Command};

use super::RETRY_INTERVAL;
use super::command::{
    configure_tracker_command, tracker_binary, tracker_command, write_configuration, write_configuration_in_directory,
};
use super::output::TrackerOutputCapture;

const FAILURE_DEADLINE: Duration = Duration::from_secs(10);
const USAGE_ERROR_EXIT_CODE: i32 = 2;
const STARTUP_FAILURE_EXIT_CODE: i32 = 1;

/// Invalid CLI configuration sources supported by the failed-start fixture.
#[derive(Clone, Copy)]
pub enum NativeTrackerInvalidCliSource {
    MissingOptionValue,
    EmptyOptionValue,
    MissingFile,
    Directory,
    MalformedToml,
    ParentOnlyRelativeFile,
}

/// Result of preparing an unreadable regular-file CLI source.
pub enum NativeTrackerUnreadableCliSource {
    Enforced(Box<NativeTrackerStartAttempt>),
    NotEnforced { reason: String },
}

impl NativeTrackerUnreadableCliSource {
    pub fn enforced_or_report_skip(self) -> Option<NativeTrackerStartAttempt> {
        match self {
            Self::Enforced(failed_start) => Some(*failed_start),
            Self::NotEnforced { reason } => {
                drop(writeln!(
                    std::io::stderr(),
                    "skipping unreadable regular-file assertion: {reason}"
                ));
                None
            }
        }
    }
}

/// A prepared tracker process startup that has not yet launched the executable.
pub struct NativeTrackerStartAttempt {
    command: Command,
    permission_restore: Option<NativeTrackerPermissionRestore>,
    workspace: Option<tempfile::TempDir>,
    source_path: Option<PathBuf>,
}

impl NativeTrackerStartAttempt {
    pub fn with_invalid_cli_source(source: NativeTrackerInvalidCliSource) -> Self {
        let workspace = tempfile::tempdir().expect("create temporary invalid-source workspace");
        let (command, source_path) = invalid_source_command(&workspace, source);

        Self {
            command,
            permission_restore: None,
            workspace: Some(workspace),
            source_path,
        }
    }

    pub fn with_unreadable_regular_file() -> NativeTrackerUnreadableCliSource {
        let workspace = tempfile::tempdir().expect("create temporary unreadable-source workspace");
        let (source_path, _) = write_configuration(&workspace, "unreadable", 0);
        let original_mode = std::fs::metadata(&source_path)
            .expect("read fixture-owned configuration file metadata")
            .permissions()
            .mode();
        std::fs::set_permissions(&source_path, std::fs::Permissions::from_mode(0o000))
            .expect("make fixture-owned configuration file unreadable");
        let permission_restore = NativeTrackerPermissionRestore {
            path: source_path.clone(),
            mode: Some(original_mode),
        };

        match std::fs::read_to_string(&source_path) {
            Err(error) if error.kind() == std::io::ErrorKind::PermissionDenied => {
                NativeTrackerUnreadableCliSource::Enforced(Box::new(Self {
                    command: tracker_command(&source_path, None, None, None),
                    permission_restore: Some(permission_restore),
                    workspace: Some(workspace),
                    source_path: Some(source_path),
                }))
            }
            Ok(_) => NativeTrackerUnreadableCliSource::NotEnforced {
                reason: "the current process can read a mode-000 regular file (for example, it is privileged or has a filesystem capability)"
                    .to_owned(),
            },
            Err(error) => panic!("probe fixture-owned unreadable configuration file: {error}"),
        }
    }

    pub fn start(mut self) -> NativeTrackerFailedStart {
        let mut child = self.command.spawn().expect("spawn Cargo-built tracker executable");
        let stdout = child.stdout.take().expect("tracker child stdout is piped");
        let stderr = child.stderr.take().expect("tracker child stderr is piped");

        NativeTrackerFailedStart {
            child: Some(child),
            output: Some(TrackerOutputCapture::new(stdout, stderr)),
            permission_restore: self.permission_restore.take(),
            workspace: self.workspace.take(),
            source_path: self.source_path.take(),
        }
    }
}

/// A tracker child process expected to fail before completing startup.
pub struct NativeTrackerFailedStart {
    child: Option<Child>,
    output: Option<TrackerOutputCapture>,
    permission_restore: Option<NativeTrackerPermissionRestore>,
    workspace: Option<tempfile::TempDir>,
    source_path: Option<PathBuf>,
}

struct NativeTrackerPermissionRestore {
    path: PathBuf,
    mode: Option<u32>,
}

impl NativeTrackerPermissionRestore {
    fn restore(&mut self) -> std::io::Result<()> {
        let Some(mode) = self.mode else {
            return Ok(());
        };
        std::fs::set_permissions(&self.path, std::fs::Permissions::from_mode(mode))?;
        self.mode = None;
        Ok(())
    }
}

impl Drop for NativeTrackerPermissionRestore {
    fn drop(&mut self) {
        drop(self.restore());
    }
}

/// Stable evidence retained after a failed-start child has been reaped.
pub struct NativeTrackerFailedStartResult {
    exit_code: i32,
    output: String,
    source_path: Option<PathBuf>,
    source_mode: Option<u32>,
    _workspace: Option<tempfile::TempDir>,
}

impl NativeTrackerFailedStartResult {
    pub const fn exit_code(&self) -> i32 {
        self.exit_code
    }

    pub fn output(&self) -> &str {
        &self.output
    }

    pub fn source_path(&self) -> Option<&std::path::Path> {
        self.source_path.as_deref()
    }

    pub const fn source_mode(&self) -> Option<u32> {
        self.source_mode
    }

    pub fn assert_usage_error(&self, expected_diagnostic: &str) {
        self.assert_exit_code(USAGE_ERROR_EXIT_CODE);
        self.assert_output_contains(expected_diagnostic);
    }

    pub fn assert_startup_failure(&self, expected_diagnostic: &str) {
        self.assert_exit_code(STARTUP_FAILURE_EXIT_CODE);
        self.assert_output_contains(expected_diagnostic);
    }

    pub fn assert_diagnostic_names_source_path(&self) {
        let path = self
            .source_path()
            .expect("this failure source has no fixture-owned source path")
            .to_string_lossy()
            .into_owned();
        self.assert_output_contains(&path);
    }

    pub fn assert_explicit_configuration_file_load_failure(&self) {
        self.assert_startup_failure("Unable to load explicit configuration file");
        self.assert_diagnostic_names_source_path();
    }

    fn assert_exit_code(&self, expected: i32) {
        assert_eq!(
            self.exit_code, expected,
            "unexpected exit code\ntracker output:\n{}",
            self.output
        );
    }

    fn assert_output_contains(&self, expected_fragment: &str) {
        assert!(
            self.output.contains(expected_fragment),
            "tracker output does not contain {expected_fragment:?}\ntracker output:\n{}",
            self.output
        );
    }
}

impl NativeTrackerFailedStart {
    pub fn source_path(&self) -> Option<PathBuf> {
        self.source_path.clone()
    }

    pub async fn wait_for_exit(mut self) -> Result<NativeTrackerFailedStartResult, String> {
        let mut child = self.child.take().expect("invalid-source tracker child must be available");
        let mut output_capture = self.output.take().expect("invalid-source output capture must be available");
        let status = match tokio::time::timeout(FAILURE_DEADLINE, child.wait()).await {
            Ok(Ok(status)) => Ok(status),
            Ok(Err(error)) => Err(format!("wait for invalid-source tracker child: {error}")),
            Err(_) => Self::kill_and_reap_after_timeout(&mut child).await,
        };
        let reader_result = Self::wait_for_output_readers(&mut output_capture).await;
        let output = output_capture.contents().await;
        let source_mode = self.permission_restore.as_ref().and_then(|restore| restore.mode);
        let restore_result = self
            .permission_restore
            .as_mut()
            .map_or(Ok(()), NativeTrackerPermissionRestore::restore)
            .map_err(|error| format!("restore fixture-owned configuration file permissions: {error}"));
        restore_result.map_err(|message| format!("{message}\ntracker output:\n{output}"))?;
        let status = status.map_err(|message| format!("{message}\ntracker output:\n{output}"))?;
        reader_result.map_err(|message| format!("{message}\ntracker output:\n{output}"))?;
        let exit_code = status
            .code()
            .ok_or_else(|| format!("invalid-source tracker child exited without a code: {status}\ntracker output:\n{output}"))?;

        Ok(NativeTrackerFailedStartResult {
            exit_code,
            output,
            source_path: self.source_path.take(),
            source_mode,
            _workspace: self.workspace.take(),
        })
    }

    async fn kill_and_reap_after_timeout(child: &mut Child) -> Result<std::process::ExitStatus, String> {
        let kill_result = child.start_kill();

        match tokio::time::timeout(FAILURE_DEADLINE, child.wait()).await {
            Ok(Ok(status)) => match kill_result {
                Ok(()) => Err(format!(
                    "invalid-source tracker child did not exit within {FAILURE_DEADLINE:?}; force-killed and reaped with {status}"
                )),
                Err(error) => Err(format!(
                    "invalid-source tracker child did not exit within {FAILURE_DEADLINE:?}; force-kill failed: {error}; reaped with {status}"
                )),
            },
            Ok(Err(error)) => Err(format!("reap force-killed invalid-source tracker child: {error}")),
            Err(_) => match kill_result {
                Ok(()) => Err(format!(
                    "invalid-source tracker child did not exit within {FAILURE_DEADLINE:?}, and did not reap within an additional {FAILURE_DEADLINE:?} after force-kill"
                )),
                Err(error) => Err(format!(
                    "invalid-source tracker child did not exit within {FAILURE_DEADLINE:?}; force-kill failed: {error}; and it did not reap within an additional {FAILURE_DEADLINE:?}"
                )),
            },
        }
    }

    async fn wait_for_output_readers(output_capture: &mut TrackerOutputCapture) -> Result<(), String> {
        tokio::time::timeout(FAILURE_DEADLINE, output_capture.wait_for_readers())
            .await
            .map_err(|_| format!("timed out waiting {FAILURE_DEADLINE:?} for invalid-source tracker output readers"))
    }
}

impl Drop for NativeTrackerFailedStart {
    fn drop(&mut self) {
        let Some(mut child) = self.child.take() else {
            return;
        };
        let output = self.output.take();
        if let Some(mut permission_restore) = self.permission_restore.take() {
            drop(permission_restore.restore());
        }

        let Ok(runtime) = tokio::runtime::Handle::try_current() else {
            drop(child.start_kill());
            let deadline = std::time::Instant::now() + FAILURE_DEADLINE;
            while std::time::Instant::now() < deadline {
                match child.try_wait() {
                    Ok(Some(_)) | Err(_) => break,
                    Ok(None) => std::thread::sleep(RETRY_INTERVAL),
                }
            }
            drop(output);
            return;
        };
        let workspace = self.workspace.take();
        drop(runtime.spawn(async move {
            drop(child.start_kill());
            drop(tokio::time::timeout(FAILURE_DEADLINE, child.wait()).await);
            if let Some(mut output) = output {
                drop(Self::wait_for_output_readers(&mut output).await);
            }
            drop(workspace);
        }));
    }
}

fn invalid_source_command(workspace: &tempfile::TempDir, source: NativeTrackerInvalidCliSource) -> (Command, Option<PathBuf>) {
    let mut command = Command::new(tracker_binary());
    configure_tracker_command(&mut command);

    match source {
        NativeTrackerInvalidCliSource::MissingOptionValue => {
            command.arg("--config-toml-path");
            (command, None)
        }
        NativeTrackerInvalidCliSource::EmptyOptionValue => {
            command.arg("--config-toml-path").arg("");
            (command, None)
        }
        NativeTrackerInvalidCliSource::MissingFile => {
            let path = workspace.path().join("does-not-exist.toml");
            command.arg("--config-toml-path").arg(&path);
            (command, Some(path))
        }
        NativeTrackerInvalidCliSource::Directory => {
            let path = workspace.path().to_path_buf();
            command.arg("--config-toml-path").arg(&path);
            (command, Some(path))
        }
        NativeTrackerInvalidCliSource::MalformedToml => {
            let (path, _) = write_configuration(workspace, "malformed", 0);
            std::fs::write(
                &path,
                format!(
                    "{}\nmalformed_key = [",
                    std::fs::read_to_string(&path).expect("read configuration")
                ),
            )
            .expect("write malformed tracker configuration");
            command.arg("--config-toml-path").arg(&path);
            (command, Some(path))
        }
        NativeTrackerInvalidCliSource::ParentOnlyRelativeFile => {
            let parent = workspace.path().join("parent");
            let child = parent.join("child");
            std::fs::create_dir_all(&child).expect("create child working directory");
            let (parent_configuration, _) = write_configuration_in_directory(&parent, 0);
            assert_eq!(parent_configuration.file_name(), Some(std::ffi::OsStr::new("tracker.toml")));
            command.current_dir(child).arg("--config-toml-path").arg("tracker.toml");
            (command, Some(parent_configuration))
        }
    }
}

#[cfg(test)]
mod tests {
    use std::ffi::OsStr;
    use std::os::unix::fs::PermissionsExt as _;
    use std::process::Stdio;

    use nix::sys::signal::kill;
    use nix::unistd::Pid;
    use tokio::process::Command;

    use super::{
        NativeTrackerFailedStart, NativeTrackerInvalidCliSource, NativeTrackerPermissionRestore, NativeTrackerStartAttempt,
        TrackerOutputCapture, invalid_source_command,
    };

    #[test]
    fn it_should_restore_permissions_when_the_restore_guard_is_dropped_without_a_tokio_runtime() {
        // Arrange
        let workspace = tempfile::tempdir().expect("create temporary permission-restore workspace");
        let path = workspace.path().join("configuration.toml");
        std::fs::write(&path, "configuration").expect("write fixture-owned configuration file");
        let original_mode = std::fs::metadata(&path)
            .expect("read fixture-owned configuration file metadata")
            .permissions()
            .mode();
        std::fs::set_permissions(&path, std::fs::Permissions::from_mode(0o000))
            .expect("make fixture-owned configuration file unreadable");
        let restore = NativeTrackerPermissionRestore {
            path: path.clone(),
            mode: Some(original_mode),
        };

        // Act
        let thread_result = std::thread::spawn(move || drop(restore)).join();

        // Assert
        assert!(
            thread_result.is_ok(),
            "dropping the restoration guard outside Tokio must not panic"
        );
        assert_eq!(
            std::fs::metadata(path)
                .expect("read restored fixture-owned configuration file metadata")
                .permissions()
                .mode(),
            original_mode
        );
    }

    #[test]
    fn it_should_construct_a_parent_only_relative_source_from_the_child_working_directory() {
        // Arrange
        let workspace = tempfile::tempdir().expect("create temporary tracker workspace");

        // Act
        let (command, source_path) = invalid_source_command(&workspace, NativeTrackerInvalidCliSource::ParentOnlyRelativeFile);

        // Assert
        let source_path = source_path.expect("parent-only source should retain its parent file path");
        assert_eq!(source_path.file_name(), Some(OsStr::new("tracker.toml")));
        assert_eq!(
            source_path.parent().map(|parent| parent.join("child")).as_deref(),
            command.as_std().get_current_dir()
        );
        assert_eq!(command.as_std().get_args().last(), Some(OsStr::new("tracker.toml")));
    }

    #[tokio::test]
    async fn it_should_reap_a_failed_start_dropped_without_a_tokio_runtime() {
        // Arrange: spawning needs a runtime; the drop below happens outside one.
        let mut child = Command::new("sleep")
            .arg("60")
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .spawn()
            .expect("spawn long-running tracker substitute");
        let pid = Pid::from_raw(child.id().expect("child should have a PID").cast_signed());
        let stdout = child.stdout.take().expect("child stdout is piped");
        let stderr = child.stderr.take().expect("child stderr is piped");
        let failed_start = NativeTrackerFailedStart {
            child: Some(child),
            output: Some(TrackerOutputCapture::new(stdout, stderr)),
            permission_restore: None,
            workspace: Some(tempfile::tempdir().expect("create fixture workspace")),
            source_path: None,
        };

        // Act
        let result = std::thread::spawn(move || drop(failed_start)).join();

        // Assert
        assert!(result.is_ok(), "dropping a failed start outside Tokio must not panic");
        assert_eq!(kill(pid, None), Err(nix::errno::Errno::ESRCH));
    }

    #[tokio::test]
    async fn it_should_restore_the_unreadable_source_permissions_after_waiting_for_exit() {
        // Arrange
        let Some(start_attempt) = NativeTrackerStartAttempt::with_unreadable_regular_file().enforced_or_report_skip() else {
            return;
        };

        // Act
        let failure = start_attempt
            .start()
            .wait_for_exit()
            .await
            .expect("tracker should exit for an unreadable regular file");

        // Assert
        let restored_mode = std::fs::metadata(failure.source_path().expect("unreadable-file result should retain its path"))
            .expect("read restored unreadable-file metadata")
            .permissions()
            .mode();
        let original_mode = failure
            .source_mode()
            .expect("unreadable-file result should retain the original source mode");
        assert_eq!(restored_mode & 0o777, original_mode & 0o777);
    }
}
