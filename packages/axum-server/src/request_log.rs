//! Request logging that keeps secrets out of the log.
//!
//! The URI of a request can hold a secret: a peer authentication key is a path segment
//! (`/announce/<key>`, `/scrape/<key>`), a REST API key is a path segment (`/api/v1/key/<key>`) and
//! the REST API admin token can be a query parameter (`?token=<token>`). The tracker servers log
//! the URI of every request, in the `request` event and in the span that wraps the request, so
//! both go through [`redact_uri`].
//!
//! Names are compared after percent-decoding, as the server reads them (`%74oken` is `token`).
//!
//! The rule redacts what follows `/announce/`, `/scrape/` and `/key/` whatever its shape, because
//! a malformed key (a stray character, a wrong length) is still the user's secret. The seconds
//! value of `POST /api/v1/key/<seconds>` is lost in the log with it; that is accepted.
// cspell:ignore nnounce oken zzoken rposition
use http::{Request, Uri};
use tower_http::trace::MakeSpan;
use tracing::field::Empty;
use tracing::{Level, Span};

/// What replaces a secret in a logged URI. It is the marker the configuration redaction uses.
pub const REDACTED: &str = "***";

/// Path segments after which the rest of the path is a secret.
const SECRET_PATH_PREFIXES: [&str; 3] = ["announce", "scrape", "key"];

/// Query parameters whose value is a secret.
const SECRET_QUERY_PARAMS: [&str; 1] = ["token"];

/// The path and query of `uri`, with the secrets replaced by [`REDACTED`].
///
/// A URI in absolute form loses its scheme and authority: the log shows what was requested, not
/// where the client sent it.
#[must_use]
pub fn redact_uri(uri: &Uri) -> String {
    redact_path_and_query(uri.path(), uri.query())
}

/// Replaces the secrets in a path and an optional query. See the [module](self) documentation.
#[must_use]
pub fn redact_path_and_query(path: &str, query: Option<&str>) -> String {
    let mut redacted = redact_path(path);

    if let Some(query) = query {
        redacted.push('?');
        redacted.push_str(&redact_query(query));
    }

    redacted
}

fn redact_path(path: &str) -> String {
    let segments: Vec<&str> = path.split('/').collect();

    // Anything after a prefix is the secret, also after empty segments (`/announce//<key>`), so a
    // prefix matters only when it comes before the last segment that is not empty. That segment
    // is found once: looking at the rest of the path for every segment would make a path of
    // many slashes, which any client can send, cost a quadratic number of comparisons.
    let Some(last_not_empty) = segments.iter().rposition(|segment| !segment.is_empty()) else {
        return path.to_string();
    };

    for (index, segment) in segments[..last_not_empty].iter().enumerate() {
        if SECRET_PATH_PREFIXES.contains(&percent_decoded(segment).as_str()) {
            return format!("{}/{REDACTED}", segments[..=index].join("/"));
        }
    }

    path.to_string()
}

fn redact_query(query: &str) -> String {
    query
        .split('&')
        .map(|parameter| match parameter.split_once('=') {
            Some((name, _value)) if is_a_secret_query_param(name) => {
                format!("{name}={REDACTED}")
            }
            _ => parameter.to_string(),
        })
        .collect::<Vec<_>>()
        .join("&")
}

/// Whether the query parameter `name`, once decoded, names a secret. The server decodes names
/// before it reads them (`%74oken` is `token`), so the comparison must too.
fn is_a_secret_query_param(name: &str) -> bool {
    let decoded = percent_decoded(name);

    SECRET_QUERY_PARAMS.iter().any(|secret| decoded.eq_ignore_ascii_case(secret))
}

/// Decodes the `%XX` escapes of `text`, where `XX` is two hexadecimal digits. Anything else after a
/// `%` (one digit, a sign, another letter) is not an escape and stays as written, as the server's
/// form decoder leaves it. Bytes that are not UTF-8 become the replacement character: the result
/// is only compared with known names.
fn percent_decoded(text: &str) -> String {
    let bytes = text.as_bytes();
    let mut decoded = Vec::with_capacity(bytes.len());
    let mut index = 0;

    while index < bytes.len() {
        let escaped = (bytes[index] == b'%')
            .then(|| bytes.get(index + 1..index + 3))
            .flatten()
            // `from_str_radix` alone would accept a sign (`+4`), which is not an escape.
            .filter(|hex| hex.iter().all(u8::is_ascii_hexdigit))
            .and_then(|hex| std::str::from_utf8(hex).ok())
            .and_then(|hex| u8::from_str_radix(hex, 16).ok());

        if let Some(byte) = escaped {
            decoded.push(byte);
            index += 3;
        } else {
            decoded.push(bytes[index]);
            index += 1;
        }
    }

    String::from_utf8_lossy(&decoded).into_owned()
}

/// Makes the span of a request like `tower_http`'s `DefaultMakeSpan` at the `INFO` level, with
/// the redacted URI instead of the raw one and an empty `request_id` field for request layers to
/// record.
#[derive(Clone, Copy, Debug, Default)]
pub struct RedactedMakeSpan;

impl<B> MakeSpan<B> for RedactedMakeSpan {
    fn make_span(&mut self, request: &Request<B>) -> Span {
        tracing::span!(
            Level::INFO,
            "request",
            method = %request.method(),
            uri = %redact_uri(request.uri()),
            version = ?request.version(),
            request_id = Empty,
        )
    }
}

#[cfg(test)]
mod tests {
    use http::Request;
    use rstest::rstest;
    use torrust_tracker_test_helpers::logging::{capture_logs_in_scope, capture_logs_while};
    use tower_http::trace::MakeSpan as _;

    use super::{RedactedMakeSpan, percent_decoded, redact_path_and_query};

    // The tests use made-up keys and tokens. Each case states the whole expected text.
    #[rstest]
    #[case::an_announce_with_a_key(
        "/announce/TestPeerKey000000000000000000001",
        Some("info_hash=%00%00&peer_id=-qB00000000000000001&port=17548&left=0"),
        "/announce/***?info_hash=%00%00&peer_id=-qB00000000000000001&port=17548&left=0"
    )]
    #[case::a_scrape_with_a_key(
        "/scrape/TestPeerKey000000000000000000001",
        Some("info_hash=%00%00"),
        "/scrape/***?info_hash=%00%00"
    )]
    #[case::a_key_delete_with_a_token_in_the_query(
        "/api/v1/key/TestPeerKey000000000000000000001",
        Some("token=TestAdminToken0001"),
        "/api/v1/key/***?token=***"
    )]
    #[case::a_token_in_the_query_only("/api/v1/stats", Some("token=TestAdminToken0001"), "/api/v1/stats?token=***")]
    #[case::a_token_among_other_parameters(
        "/api/v1/torrents",
        Some("limit=10&token=TestAdminToken0001&offset=5"),
        "/api/v1/torrents?limit=10&token=***&offset=5"
    )]
    #[case::a_token_with_another_case("/api/v1/stats", Some("Token=TestAdminToken0001"), "/api/v1/stats?Token=***")]
    #[case::an_empty_token_value("/api/v1/stats", Some("token="), "/api/v1/stats?token=***")]
    #[case::a_token_name_with_an_encoded_letter("/api/v1/stats", Some("%74oken=TestAdminToken0001"), "/api/v1/stats?%74oken=***")]
    #[case::a_token_name_fully_encoded(
        "/api/v1/stats",
        Some("%74%6F%6B%65%6E=TestAdminToken0001"),
        "/api/v1/stats?%74%6F%6B%65%6E=***"
    )]
    #[case::an_encoded_token_name_with_lower_case_hex(
        "/api/v1/stats",
        Some("to%6ben=TestAdminToken0001"),
        "/api/v1/stats?to%6ben=***"
    )]
    #[case::names_with_a_malformed_escape_stay_as_written("/api/v1/stats", Some("%7=1&%+4=2"), "/api/v1/stats?%7=1&%+4=2")]
    #[case::a_name_that_is_not_valid_utf8_after_decoding("/api/v1/stats", Some("%FF=1&limit=2"), "/api/v1/stats?%FF=1&limit=2")]
    #[case::an_encoded_path_segment_name("/%61nnounce/TestPeerKey000000000000000000001", None, "/%61nnounce/***")]
    #[case::a_uri_without_a_secret("/api/v1/stats", Some("limit=10"), "/api/v1/stats?limit=10")]
    #[case::a_path_without_a_query("/health_check", None, "/health_check")]
    #[case::an_announce_without_a_key("/announce", Some("info_hash=%00%00"), "/announce?info_hash=%00%00")]
    #[case::an_announce_with_an_empty_key_segment("/announce/", None, "/announce/")]
    #[case::an_announce_with_only_empty_segments("/announce//", None, "/announce//")]
    #[case::a_key_after_an_empty_segment("/announce//TestPeerKey000000000000000000001", Some("port=1"), "/announce/***?port=1")]
    #[case::a_key_after_two_empty_segments("/scrape///TestPeerKey000000000000000000001", None, "/scrape/***")]
    #[case::a_rest_key_after_an_empty_segment("/api/v1/key//TestPeerKey000000000000000000001", None, "/api/v1/key/***")]
    #[case::a_percent_encoded_key("/announce/Test%50eerKey000000000000000000001", None, "/announce/***")]
    #[case::a_key_with_a_stray_character("/announce/TestPeerKey-0000000000000000001", None, "/announce/***")]
    #[case::a_key_of_the_wrong_length("/announce/ShortTestKey01", Some("port=1"), "/announce/***?port=1")]
    #[case::a_key_delete_with_a_key_of_the_wrong_length("/api/v1/key/ShortTestKey01", None, "/api/v1/key/***")]
    #[case::a_key_posted_where_the_seconds_go("/api/v1/key/TestPeerKeyPostedByMistake000001", None, "/api/v1/key/***")]
    #[case::more_segments_after_the_key("/announce/TestPeerKey000000000000000000001/extra", None, "/announce/***")]
    #[case::the_keys_collection_has_no_secret("/api/v1/keys", None, "/api/v1/keys")]
    #[case::a_rest_prefix_other_than_the_default("/custom/key/TestPeerKey000000000000000000001", None, "/custom/key/***")]
    fn it_should_replace_the_secrets_of_a_uri_with_a_marker(
        #[case] path: &str,
        #[case] query: Option<&str>,
        #[case] expected: &str,
    ) {
        let redacted = redact_path_and_query(path, query);

        assert_eq!(redacted, expected);
    }

    #[test]
    fn it_should_redact_a_key_after_many_empty_segments() {
        // The result for a path of the size a client can send (tens of thousands of slashes).
        // The test checks the result only, not the cost: the cost of such a path (a scan of the
        // rest of the path for every segment took 0.66 s) is recorded as a measurement in the
        // evidence of issue 2999, section R5.
        let key = "TestPeerKey000000000000000000001";
        let slashes = "/".repeat(60_000);
        let path = format!("/announce{slashes}{key}");

        let redacted = redact_path_and_query(&path, None);

        assert_eq!(redacted, "/announce/***");
    }

    #[test]
    fn it_should_leave_a_path_of_only_slashes_as_it_is() {
        let path = "/".repeat(60_000);

        let redacted = redact_path_and_query(&path, None);

        assert_eq!(redacted, path);
    }

    #[rstest]
    #[case::a_valid_escape("%74oken", "token")]
    #[case::a_valid_escape_with_lower_case_hex("to%6ben", "token")]
    #[case::a_percent_with_one_hex_digit_at_the_end("to%7", "to%7")]
    #[case::a_percent_at_the_end("token%", "token%")]
    #[case::a_percent_followed_by_a_sign("%+4oken", "%+4oken")]
    #[case::a_percent_followed_by_a_letter_that_is_not_hex("%zzoken", "%zzoken")]
    fn it_should_decode_only_a_percent_followed_by_two_hex_digits(#[case] text: &str, #[case] expected: &str) {
        assert_eq!(percent_decoded(text), expected);
    }

    #[test]
    fn it_should_record_the_redacted_uri_and_not_the_raw_one_in_the_span_of_a_request() {
        // Arrange
        let peer_key = "TestPeerKeyInTheSpan000000000001";
        let admin_token = "TestAdminTokenInTheSpan0001";
        let request = Request::builder()
            .uri(format!("/announce/{peer_key}?port=17548&token={admin_token}"))
            .body(())
            .expect("the test request should be valid");

        // Act
        let ((), logs) = capture_logs_while(|| {
            let span = RedactedMakeSpan.make_span(&request);
            let _entered = span.enter();
            tracing::info!("an event inside the request span");
        });

        // Assert
        assert!(
            logs.contains("uri=/announce/***?port=17548&token=***"),
            "the span should record the redacted URI, got: {logs}"
        );
        assert!(!logs.contains(peer_key), "the span holds the test peer key");
        assert!(!logs.contains(admin_token), "the span holds the test token");
    }

    #[test]
    fn it_should_record_the_request_id_in_the_span_of_a_request() {
        // Arrange
        let request_id = "test-request-id-0001";
        let request = Request::builder()
            .uri("/health_check")
            .body(())
            .expect("the test request should be valid");
        let capture = capture_logs_in_scope();

        // Act
        {
            let span = RedactedMakeSpan.make_span(&request);
            let _entered = span.enter();
            span.record("request_id", request_id);
            tracing::info!("an event inside the request span");
        }

        // Assert
        let lines = capture.lines();
        let event_line = lines
            .iter()
            .find(|line| line.contains("an event inside the request span"))
            .expect("the inner event inside the request span should be captured");
        let Some((span_fields, _)) = event_line.split_once("request{").and_then(|(_, rest)| rest.split_once('}')) else {
            panic!("expected a request{{...}} span on the event line, got: {event_line}");
        };
        let expected_field = format!(r#"request_id="{request_id}""#);
        assert!(
            span_fields.contains(&expected_field),
            "expected the request span to contain {expected_field}, got span fields `{span_fields}` in line: {event_line}"
        );
    }
}
