//! Installs the logging of the tracker process and applies its configuration.
//!
//! The logging is installed first thing in `main`, before the arguments are parsed and the
//! configuration is loaded, so that every record of the process has one writer, including the
//! records of the configuration loading. When the configuration is loaded, [`configure`] sets the
//! level (`trace_filter`) and the style (`trace_style`) of the records.
//!
//! The decision is recorded in
//! `docs/adrs/20261127063102_write_the_daemon_logs_as_json_on_stderr_by_default.md`.
//!
//! # How a style change stays safe
//!
//! `tracing-subscriber` stores the fields of every span in the span itself, in a form that depends on
//! the formatter, and the `pretty` and `json` formatters panic when a span that is still open was
//! created without it. The configuration is applied inside the spans of the application setup, so
//! replacing the formatter at that point would panic for those two styles. Instead, one layer
//! ([`StyleLayers`]) holds a formatter for every style, tells all of them about every span, and sends
//! each event to the formatter of the active style. Each formatter has its own field formatter, so
//! the fields of a span are neither missing nor repeated.
use std::sync::atomic::{AtomicU8, Ordering};
use std::sync::{Arc, OnceLock};

use torrust_tracker_configuration::v3_0_0::logging::{Logging, Threshold, TraceStyle};
use tracing::level_filters::LevelFilter;
use tracing::span::{Attributes, Id, Record};
use tracing::{Event, Subscriber};
use tracing_subscriber::field::RecordFields;
use tracing_subscriber::fmt::format::{DefaultFields, FormatFields, Writer};
use tracing_subscriber::fmt::writer::BoxMakeWriter;
use tracing_subscriber::fmt::{self, TestWriter};
use tracing_subscriber::layer::{Context, SubscriberExt as _};
use tracing_subscriber::registry::LookupSpan;
use tracing_subscriber::util::SubscriberInitExt as _;
use tracing_subscriber::{Layer, Registry, reload};

/// The position of every style in [`StyleLayers`].
const FULL: u8 = 0;
const PRETTY: u8 = 1;
const COMPACT: u8 = 2;
const JSON: u8 = 3;

const fn style_position(trace_style: &TraceStyle) -> u8 {
    match trace_style {
        TraceStyle::Full => FULL,
        TraceStyle::Pretty => PRETTY,
        TraceStyle::Compact => COMPACT,
        TraceStyle::Json => JSON,
    }
}

/// Formats the fields of a span for the `compact` style.
///
/// It formats like [`DefaultFields`], which the `full` style uses too, but it is another type, so
/// the fields of a span are stored once for each of the two styles instead of being shared and
/// appended twice when a span records a value.
#[derive(Default)]
struct CompactFields(DefaultFields);

impl<'writer> FormatFields<'writer> for CompactFields {
    fn format_fields<R: RecordFields>(&self, writer: Writer<'writer>, fields: R) -> std::fmt::Result {
        self.0.format_fields(writer, fields)
    }
}

type BoxedLayer<S> = Box<dyn Layer<S> + Send + Sync + 'static>;

/// A layer with a formatter for every style that writes each event in the active style.
struct StyleLayers<S> {
    layers: [BoxedLayer<S>; 4],
    active: Arc<AtomicU8>,
}

impl<S> StyleLayers<S>
where
    S: Subscriber + for<'span> LookupSpan<'span>,
{
    fn new(make_writer: &impl Fn() -> BoxMakeWriter, active: Arc<AtomicU8>) -> Self {
        let full = fmt::layer().with_ansi(true).with_writer(make_writer()).boxed();
        let pretty = fmt::layer()
            .with_ansi(true)
            .with_writer(make_writer())
            .pretty()
            .with_file(false)
            .boxed();
        let compact = fmt::layer()
            .fmt_fields(CompactFields::default())
            .with_ansi(true)
            .with_writer(make_writer())
            .compact()
            .boxed();
        let json = fmt::layer().with_ansi(true).with_writer(make_writer()).json().boxed();

        Self {
            layers: [full, pretty, compact, json],
            active,
        }
    }
}

impl<S> Layer<S> for StyleLayers<S>
where
    S: Subscriber + for<'span> LookupSpan<'span>,
{
    fn on_new_span(&self, attributes: &Attributes<'_>, id: &Id, context: Context<'_, S>) {
        for layer in &self.layers {
            layer.on_new_span(attributes, id, context.clone());
        }
    }

    fn on_record(&self, id: &Id, values: &Record<'_>, context: Context<'_, S>) {
        for layer in &self.layers {
            layer.on_record(id, values, context.clone());
        }
    }

    fn on_enter(&self, id: &Id, context: Context<'_, S>) {
        for layer in &self.layers {
            layer.on_enter(id, context.clone());
        }
    }

    fn on_exit(&self, id: &Id, context: Context<'_, S>) {
        for layer in &self.layers {
            layer.on_exit(id, context.clone());
        }
    }

    fn on_close(&self, id: Id, context: Context<'_, S>) {
        for layer in &self.layers {
            layer.on_close(id.clone(), context.clone());
        }
    }

    fn on_event(&self, event: &Event<'_>, context: Context<'_, S>) {
        if let Some(layer) = self.layers.get(usize::from(self.active.load(Ordering::Relaxed))) {
            layer.on_event(event, context);
        }
    }
}

/// The handles that change the logging of the installed subscriber.
pub struct Controls {
    level: reload::Handle<LevelFilter, Registry>,
    style: Arc<AtomicU8>,
}

impl Controls {
    /// Sets the level and the style of the records from the configuration.
    pub fn apply(&self, logging: &Logging) {
        self.style.store(style_position(&logging.trace_style), Ordering::Relaxed);

        if let Err(error) = self.level.reload(map_to_tracing_level_filter(&logging.trace_filter)) {
            tracing::warn!(%error, "Could not change the trace filter");
        }

        tracing::info!("Logging initialized");
    }
}

/// Builds the subscriber of the tracker process and the controls that change it.
///
/// It starts at level `info`, in the `full` style. `make_writer` is called once for every style,
/// and every record is written to the writer of its style.
pub fn build(make_writer: impl Fn() -> BoxMakeWriter) -> (impl Subscriber + Send + Sync + 'static, Controls) {
    let (level_layer, level) = reload::Layer::new(LevelFilter::INFO);
    let style = Arc::new(AtomicU8::new(FULL));
    let style_layers = StyleLayers::new(&make_writer, Arc::clone(&style));

    let subscriber = tracing_subscriber::registry().with(level_layer).with(style_layers);

    (subscriber, Controls { level, style })
}

static INSTALLED: OnceLock<Controls> = OnceLock::new();

/// Installs the logging of the process, writing to the same stream as the tracker always has
/// (standard output). It does nothing when a subscriber is already installed.
pub fn install() {
    let (subscriber, controls) = build(|| BoxMakeWriter::new(TestWriter::default));

    if subscriber.try_init().is_ok() {
        drop(INSTALLED.set(controls));
    }
}

/// Applies the logging configuration. It does nothing when [`install`] did not install the logging,
/// as in the tests that start the application in the same process.
pub fn configure(logging: &Logging) {
    if let Some(controls) = INSTALLED.get() {
        controls.apply(logging);
    }
}

const fn map_to_tracing_level_filter(trace_filter: &Threshold) -> LevelFilter {
    match trace_filter {
        Threshold::Off => LevelFilter::OFF,
        Threshold::Error => LevelFilter::ERROR,
        Threshold::Warn => LevelFilter::WARN,
        Threshold::Info => LevelFilter::INFO,
        Threshold::Debug => LevelFilter::DEBUG,
        Threshold::Trace => LevelFilter::TRACE,
    }
}

#[cfg(test)]
mod tests {
    use std::io;
    use std::sync::{Arc, Mutex};

    use torrust_tracker_configuration::v3_0_0::logging::{Logging, Threshold, TraceStyle};
    use tracing::level_filters::LevelFilter;
    use tracing_subscriber::fmt::writer::BoxMakeWriter;

    use super::{Controls, build, configure, map_to_tracing_level_filter};

    /// The bytes that the subscriber of a test writes.
    #[derive(Clone, Default)]
    struct Captured(Arc<Mutex<Vec<u8>>>);

    impl io::Write for Captured {
        fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
            self.0.lock().expect("the capture is not poisoned").extend_from_slice(buf);
            Ok(buf.len())
        }

        fn flush(&mut self) -> io::Result<()> {
            Ok(())
        }
    }

    impl Captured {
        fn make_writer(&self) -> BoxMakeWriter {
            let sink = self.clone();
            BoxMakeWriter::new(move || sink.clone())
        }

        /// The text written, without the ANSI colour codes that the text styles add.
        fn text(&self) -> String {
            let written =
                String::from_utf8(self.0.lock().expect("the capture is not poisoned").clone()).expect("the capture is UTF-8");

            let mut text = String::new();
            let mut characters = written.chars();
            while let Some(character) = characters.next() {
                if character == '\u{1b}' {
                    // An escape sequence ends with the letter that follows `ESC [` and its parameters.
                    for escaped in characters.by_ref() {
                        if escaped.is_ascii_alphabetic() {
                            break;
                        }
                    }
                } else {
                    text.push(character);
                }
            }
            text
        }

        fn lines(&self) -> Vec<String> {
            self.text().lines().map(str::to_owned).collect()
        }
    }

    fn logging(trace_filter: Threshold, trace_style: TraceStyle) -> Logging {
        Logging {
            trace_filter,
            trace_style,
        }
    }

    /// Runs `scenario` with a fresh subscriber of the tracker and returns what it wrote.
    fn written_by(scenario: impl FnOnce(&Controls)) -> Captured {
        let captured = Captured::default();
        let (subscriber, controls) = build(|| captured.make_writer());

        tracing::subscriber::with_default(subscriber, || scenario(&controls));

        captured
    }

    #[test]
    fn it_should_write_the_info_records_before_the_configuration_is_applied() {
        // Arrange: nothing applied yet
        // Act
        let captured = written_by(|_controls| tracing::info!("a record before the configuration"));

        // Assert
        assert!(
            captured.text().contains("a record before the configuration"),
            "written: {}",
            captured.text()
        );
    }

    #[test]
    fn it_should_not_write_the_debug_records_before_the_configuration_is_applied() {
        // Arrange: nothing applied yet
        // Act
        let captured = written_by(|_controls| tracing::debug!("a debug record before the configuration"));

        // Assert
        assert!(captured.text().is_empty(), "written: {}", captured.text());
    }

    #[test]
    fn it_should_apply_the_trace_filter_of_the_configuration() {
        // Arrange
        let configuration = logging(Threshold::Error, TraceStyle::Full);

        // Act
        let captured = written_by(|controls| {
            controls.apply(&configuration);
            tracing::info!("an info record");
            tracing::error!("an error record");
        });

        // Assert
        let text = captured.text();
        assert!(text.contains("an error record"), "written: {text}");
        assert!(!text.contains("an info record"), "written: {text}");
    }

    #[test]
    fn it_should_write_the_debug_records_when_the_configuration_asks_for_them() {
        // Arrange
        let configuration = logging(Threshold::Debug, TraceStyle::Full);

        // Act
        let captured = written_by(|controls| {
            controls.apply(&configuration);
            tracing::debug!("a debug record");
        });

        // Assert
        assert!(captured.text().contains("a debug record"), "written: {}", captured.text());
    }

    #[test]
    fn it_should_write_nothing_when_the_trace_filter_is_off() {
        // Arrange
        let configuration = logging(Threshold::Off, TraceStyle::Full);

        // Act
        let captured = written_by(|controls| {
            controls.apply(&configuration);
            tracing::error!("an error record");
        });

        // Assert
        assert!(captured.text().is_empty(), "written: {}", captured.text());
    }

    #[test]
    fn it_should_write_text_records_in_the_full_style() {
        // Arrange
        let configuration = logging(Threshold::Info, TraceStyle::Full);

        // Act
        let captured = written_by(|controls| {
            controls.apply(&configuration);
            tracing::info!("a text record");
        });

        // Assert
        let lines = captured.lines();
        let line = lines
            .iter()
            .find(|line| line.contains("a text record"))
            .expect("the record is written");
        assert!(!line.starts_with('{'), "written: {line}");
    }

    #[test]
    fn it_should_write_json_records_when_the_configuration_asks_for_the_json_style() {
        // Arrange
        let configuration = logging(Threshold::Info, TraceStyle::Json);

        // Act
        let captured = written_by(|controls| {
            controls.apply(&configuration);
            tracing::info!("a json record");
        });

        // Assert
        let lines = captured.lines();
        let line = lines
            .iter()
            .find(|line| line.contains("a json record"))
            .expect("the record is written");
        let record: serde_json::Value = serde_json::from_str(line).expect("the record is a JSON object");
        assert_eq!(record["fields"]["message"], "a json record");
    }

    /// The style changes while a span that was created before the change is open.
    fn written_when_the_style_changes_inside_an_open_span(trace_style: TraceStyle) -> Captured {
        let configuration = logging(Threshold::Info, trace_style);

        written_by(|controls| {
            let span = tracing::info_span!("outer");
            let _entered = span.enter();

            controls.apply(&configuration);
            tracing::info!("a record inside the span");
        })
    }

    #[test]
    fn it_should_keep_writing_when_the_style_changes_to_pretty_inside_an_open_span() {
        // Arrange and Act
        let captured = written_when_the_style_changes_inside_an_open_span(TraceStyle::Pretty);

        // Assert
        assert!(
            captured.text().contains("a record inside the span"),
            "written: {}",
            captured.text()
        );
    }

    #[test]
    fn it_should_keep_writing_when_the_style_changes_to_json_inside_an_open_span() {
        // Arrange and Act
        let captured = written_when_the_style_changes_inside_an_open_span(TraceStyle::Json);

        // Assert
        assert!(
            captured.text().contains("a record inside the span"),
            "written: {}",
            captured.text()
        );
    }

    #[test]
    fn it_should_keep_writing_when_the_style_changes_to_compact_inside_an_open_span() {
        // Arrange and Act
        let captured = written_when_the_style_changes_inside_an_open_span(TraceStyle::Compact);

        // Assert
        assert!(
            captured.text().contains("a record inside the span"),
            "written: {}",
            captured.text()
        );
    }

    #[test]
    fn it_should_write_the_fields_that_a_span_recorded_after_its_creation_once() {
        // Arrange
        let configuration = logging(Threshold::Info, TraceStyle::Compact);

        // Act
        let captured = written_by(|controls| {
            controls.apply(&configuration);
            let span = tracing::info_span!("outer", answer = tracing::field::Empty);
            span.record("answer", 42);
            let _entered = span.enter();
            tracing::info!("a record inside the span");
        });

        // Assert
        let lines = captured.lines();
        let line = lines
            .iter()
            .find(|line| line.contains("a record inside the span"))
            .expect("the record is written");
        assert_eq!(line.matches("answer=42").count(), 1, "written: {line}");
    }

    #[test]
    fn it_should_map_every_threshold_to_its_tracing_level() {
        // Arrange
        let expected = [
            (Threshold::Off, LevelFilter::OFF),
            (Threshold::Error, LevelFilter::ERROR),
            (Threshold::Warn, LevelFilter::WARN),
            (Threshold::Info, LevelFilter::INFO),
            (Threshold::Debug, LevelFilter::DEBUG),
            (Threshold::Trace, LevelFilter::TRACE),
        ];

        for (threshold, level) in expected {
            // Act
            let mapped = map_to_tracing_level_filter(&threshold);

            // Assert
            assert_eq!(mapped, level, "threshold {threshold:?}");
        }
    }

    #[test]
    fn it_should_ignore_the_configuration_when_the_logging_was_not_installed() {
        // Arrange: the unit tests never install the global subscriber
        let configuration = logging(Threshold::Trace, TraceStyle::Json);

        // Act and Assert: nothing to change, and no panic
        configure(&configuration);
    }
}
