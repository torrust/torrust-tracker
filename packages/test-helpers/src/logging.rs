//! Setup for logging in tests.
use std::collections::VecDeque;
use std::io;
use std::sync::{Arc, Mutex, MutexGuard, Once, OnceLock};

use torrust_tracker_configuration::v3_0_0::logging::TraceStyle;
use tracing::Dispatch;
use tracing::level_filters::LevelFilter;
use tracing::subscriber::{DefaultGuard, NoSubscriber, set_default};
use tracing_subscriber::fmt::MakeWriter;

static INIT: Once = Once::new();

/// A global buffer containing the latest lines captured from logs.
#[doc(hidden)]
pub fn captured_logs_buffer() -> &'static Mutex<CircularBuffer> {
    static CAPTURED_LOGS_GLOBAL_BUFFER: OnceLock<Mutex<CircularBuffer>> = OnceLock::new();
    CAPTURED_LOGS_GLOBAL_BUFFER.get_or_init(|| Mutex::new(CircularBuffer::new(10000, 200)))
}

pub fn setup() {
    INIT.call_once(|| {
        tracing_init(LevelFilter::WARN, &TraceStyle::Full);
    });
}

fn tracing_init(level_filter: LevelFilter, style: &TraceStyle) {
    let mock_writer = LogCapturer::new(captured_logs_buffer());

    let builder = tracing_subscriber::fmt()
        .with_max_level(level_filter)
        .with_ansi(true)
        .with_test_writer()
        .with_writer(mock_writer);

    let () = match style {
        TraceStyle::Full => builder.init(),
        TraceStyle::Pretty => builder.pretty().with_file(false).init(),
        TraceStyle::Compact => builder.compact().init(),
        TraceStyle::Json => builder.json().init(),
    };

    tracing::info!("Logging initialized");
}

static INIT_EVERY_LEVEL: Once = Once::new();

/// Installs a global subscriber that captures every level, `TRACE` included, without colors.
///
/// Use it in a test binary of its own, to read what a spawned server wrote, with
/// [`captured_logs`]: [`setup`] captures `WARN` and above only, and only one of the two can be the
/// global subscriber of a binary.
pub fn setup_capturing_every_level() {
    INIT_EVERY_LEVEL.call_once(|| {
        tracing_subscriber::fmt()
            .with_max_level(LevelFilter::TRACE)
            .with_ansi(false)
            .with_test_writer()
            .with_writer(LogCapturer::new(captured_logs_buffer()))
            .init();
    });
}

/// The text of the global buffer: the latest lines the global subscriber captured.
///
/// # Panics
///
/// Will panic if it can't get the lock for the global buffer or the buffer is not UTF-8.
#[must_use]
pub fn captured_logs() -> String {
    String::from_utf8(captured_logs_buffer().lock().unwrap().as_vec()).unwrap()
}

/// Runs `f` and returns its result with the log lines the current thread wrote while it ran.
///
/// It captures every level, without colors. The subscriber is local to the thread, so the code
/// under test must run on it: use a `current_thread` runtime for asynchronous code.
///
/// # Panics
///
/// Will panic if the captured text is not UTF-8.
pub fn capture_logs_while<R>(f: impl FnOnce() -> R) -> (R, String) {
    #[derive(Clone)]
    struct SharedWriter(Arc<Mutex<Vec<u8>>>);

    impl io::Write for SharedWriter {
        fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
            self.0
                .lock()
                .map_err(|_| io::Error::from(io::ErrorKind::Other))?
                .extend_from_slice(buf);
            Ok(buf.len())
        }

        fn flush(&mut self) -> io::Result<()> {
            Ok(())
        }
    }

    let captured = Arc::new(Mutex::new(Vec::new()));
    let writer = SharedWriter(captured.clone());

    let subscriber = tracing_subscriber::fmt()
        .with_max_level(LevelFilter::TRACE)
        .with_ansi(false)
        .with_writer(move || writer.clone())
        .finish();

    let result = tracing::subscriber::with_default(subscriber, f);

    let text = String::from_utf8(captured.lock().unwrap().clone()).unwrap();

    (result, text)
}

/// It returns true is there is a log line containing all the texts passed.
///
/// # Panics
///
/// Will panic if it can't get the lock for the global buffer or convert it into
/// a vec.
#[must_use]
#[allow(dead_code)]
pub fn logs_contains_a_line_with(texts: &[&str]) -> bool {
    // code-review: we can search directly in the buffer instead of converting
    // the buffer into a string but that would slow down the tests because
    // cloning should be faster that locking the buffer for searching.
    // Because the buffer is not big.
    let logs = String::from_utf8(captured_logs_buffer().lock().unwrap().as_vec()).unwrap();

    for line in logs.split('\n') {
        if contains(line, texts) {
            return true;
        }
    }

    false
}

#[allow(dead_code)]
fn contains(text: &str, texts: &[&str]) -> bool {
    texts.iter().all(|&word| text.contains(word))
}

/// A tracing writer which captures the latests logs lines into a buffer.
/// It's used to capture the logs in the tests.
#[derive(Debug)]
pub struct LogCapturer<'a> {
    logs: &'a Mutex<CircularBuffer>,
}

impl<'a> LogCapturer<'a> {
    pub const fn new(buf: &'a Mutex<CircularBuffer>) -> Self {
        Self { logs: buf }
    }

    fn buf(&self) -> io::Result<MutexGuard<'a, CircularBuffer>> {
        self.logs.lock().map_err(|_| io::Error::from(io::ErrorKind::Other))
    }
}

impl io::Write for LogCapturer<'_> {
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        std::io::stdout().lock().write_all(buf)?;

        let mut target = self.buf()?;

        target.write(buf)
    }

    fn flush(&mut self) -> io::Result<()> {
        self.buf()?.flush()
    }
}

impl MakeWriter<'_> for LogCapturer<'_> {
    type Writer = Self;

    fn make_writer(&self) -> Self::Writer {
        LogCapturer::new(self.logs)
    }
}

#[derive(Debug)]
pub struct CircularBuffer {
    max_size: usize,
    buffer: VecDeque<u8>,
}

impl CircularBuffer {
    #[must_use]
    pub fn new(max_lines: usize, average_line_size: usize) -> Self {
        Self {
            max_size: max_lines * average_line_size,
            buffer: VecDeque::with_capacity(max_lines * average_line_size),
        }
    }

    /// # Errors
    ///
    /// Won't return any error.
    #[allow(clippy::unnecessary_wraps, reason = "standard Write trait requires io::Result from write")]
    pub fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        for &byte in buf {
            if self.buffer.len() == self.max_size {
                // Remove oldest byte to make space
                self.buffer.pop_front();
            }
            self.buffer.push_back(byte);
        }

        Ok(buf.len())
    }

    /// # Errors
    ///
    /// Won't return any error.
    #[allow(clippy::unnecessary_wraps, reason = "standard Write trait requires io::Result from flush")]
    #[allow(clippy::unused_self, reason = "standard Write trait requires a mutable receiver for flush")]
    pub const fn flush(&mut self) -> io::Result<()> {
        Ok(())
    }

    #[must_use]
    pub fn as_vec(&self) -> Vec<u8> {
        self.buffer.iter().copied().collect()
    }
}

/// Log records captured on the current thread for the lifetime of this guard.
///
/// Unlike [`captured_logs_buffer`], which is global and shared by every test of a binary, the
/// capture sees only the records the current thread emits while the guard lives, at every level.
/// Use it with a single-threaded runtime (`#[tokio::test]`), where the code under test runs on the
/// test thread.
#[derive(Debug)]
pub struct ScopedLogCapture {
    records: Arc<Mutex<Vec<u8>>>,
    _guard: DefaultGuard,
}

/// Starts capturing, as plain text without color codes, every record this thread emits until the
/// returned guard is dropped.
#[must_use]
pub fn capture_logs_in_scope() -> ScopedLogCapture {
    keep_a_second_dispatcher_alive();

    let records = Arc::new(Mutex::new(Vec::new()));
    let sink = records.clone();
    let subscriber = tracing_subscriber::fmt()
        .with_max_level(LevelFilter::TRACE)
        .with_ansi(false)
        .with_writer(move || SharedSink(sink.clone()))
        .finish();

    ScopedLogCapture {
        records,
        _guard: set_default(subscriber),
    }
}

/// Keeps one more dispatcher registered for the life of the process.
///
/// While `tracing` knows a single dispatcher, a callsite reached for the first time from a thread
/// that has no scoped subscriber caches "never interested", and a capture started on another
/// thread then misses that callsite's records. With two live dispatchers `tracing` asks all of
/// them, so tests that run in parallel see their own records.
fn keep_a_second_dispatcher_alive() {
    static SECOND_DISPATCHER: OnceLock<Dispatch> = OnceLock::new();

    SECOND_DISPATCHER.get_or_init(|| Dispatch::new(NoSubscriber::default()));
}

impl ScopedLogCapture {
    /// The records captured so far, one per line, oldest first.
    ///
    /// # Panics
    ///
    /// Panics if the capture buffer is poisoned or holds invalid UTF-8.
    #[must_use]
    pub fn lines(&self) -> Vec<String> {
        let records = self.records.lock().expect("the capture buffer is not poisoned");

        String::from_utf8(records.clone())
            .expect("captured records are UTF-8")
            .lines()
            .map(str::to_owned)
            .collect()
    }
}

struct SharedSink(Arc<Mutex<Vec<u8>>>);

impl io::Write for SharedSink {
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        self.0
            .lock()
            .map_err(|_| io::Error::from(io::ErrorKind::Other))?
            .extend_from_slice(buf);
        Ok(buf.len())
    }

    fn flush(&mut self) -> io::Result<()> {
        Ok(())
    }
}
