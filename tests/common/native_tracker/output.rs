//! Concurrently drains child output and owns the reader tasks until they are joined.
//!
//! This module retains raw diagnostics; it must not interpret output or make lifecycle decisions.

use std::sync::Arc;

use tokio::io::{AsyncBufReadExt as _, AsyncRead, BufReader};
use tokio::sync::Mutex;
use tokio::task::JoinHandle;

/// Concurrently drains and retains a tracker child's output for readiness and diagnostics.
pub(super) struct TrackerOutputCapture {
    output: Arc<Mutex<String>>,
    readers: Vec<JoinHandle<()>>,
}

impl TrackerOutputCapture {
    pub(super) fn new<R, S>(stdout: R, stderr: S) -> Self
    where
        R: AsyncRead + Unpin + Send + 'static,
        S: AsyncRead + Unpin + Send + 'static,
    {
        let output = Arc::new(Mutex::new(String::new()));

        Self {
            readers: vec![
                tokio::spawn(drain_output(stdout, Arc::clone(&output))),
                tokio::spawn(drain_output(stderr, Arc::clone(&output))),
            ],
            output,
        }
    }

    pub(super) async fn wait_for_readers(&mut self) {
        for reader in self.readers.drain(..) {
            reader.await.expect("output reader task must complete");
        }
    }

    pub(super) async fn contents(&self) -> String {
        self.output.lock().await.clone()
    }
}

async fn drain_output<R>(stream: R, output: Arc<Mutex<String>>)
where
    R: AsyncRead + Unpin,
{
    let mut lines = BufReader::new(stream).lines();
    while let Some(line) = lines.next_line().await.expect("read tracker child output") {
        let mut output = output.lock().await;
        output.push_str(&line);
        output.push('\n');
    }
}
