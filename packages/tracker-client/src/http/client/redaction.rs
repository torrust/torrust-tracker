//! Masking of the credentials an HTTP tracker URL can carry, for text that is written to logs,
//! diagnostics or results.
//!
//! A private HTTP tracker authenticates a peer with a key in the URL path: `/announce/<key>` and
//! `/scrape/<key>`. Section 7 of the global CLI output contract
//! (`docs/adrs/20260519000000_define_global_cli_output_contract.md`) says secrets are masked with a
//! fixed string, so these functions replace the key with [`MASK`] and keep the rest of the URL, so a
//! reader can still tell which tracker it was.
use std::error::Error as _;
use std::iter;

use reqwest::Url;

/// The fixed string that replaces a secret.
pub const MASK: &str = "****";

/// The path segments a tracker key follows: `/announce/<key>` and `/scrape/<key>`. They are
/// compared without regard to ASCII case.
const KEYED_ENDPOINTS: [&str; 2] = ["announce", "scrape"];

/// Returns `url` with its tracker key and its password replaced by [`MASK`].
///
/// The tracker key is the path segment after `announce` or `scrape`; the password is the one of
/// the user info, if any. The rest of the URL (scheme, host, port, the other path segments and the
/// query) is kept.
#[must_use]
pub fn mask_tracker_key(url: &Url) -> Url {
    mask_path_segments(url, |previous, segment| {
        !segment.is_empty() && previous.is_some_and(is_keyed_endpoint)
    })
}

/// Returns `url` with its password and every path segment equal to `key` replaced by [`MASK`].
///
/// A client that knows its own key uses it to mask URLs where the key is not after `announce` or
/// `scrape`, such as a custom endpoint path or the health-check path.
#[must_use]
pub fn mask_key_segments(url: &Url, key: &str) -> Url {
    mask_path_segments(url, |_, segment| !key.is_empty() && segment == key)
}

fn mask_path_segments(url: &Url, is_secret: impl Fn(Option<&str>, &str) -> bool) -> Url {
    let mut masked = url.clone();

    if masked.password().is_some() {
        // It fails only for a URL that cannot have a password, and this one has one.
        let _ = masked.set_password(Some(MASK));
    }

    if let Some(segments) = url.path_segments() {
        let segments: Vec<&str> = segments.collect();

        let masked_segments: Vec<&str> = segments
            .iter()
            .enumerate()
            .map(|(index, segment)| {
                let previous = index.checked_sub(1).map(|previous| segments[previous]);

                if is_secret(previous, segment) { MASK } else { segment }
            })
            .collect();

        if masked_segments != segments {
            masked.set_path(&format!("/{}", masked_segments.join("/")));
        }
    }

    masked
}

fn is_keyed_endpoint(segment: &str) -> bool {
    KEYED_ENDPOINTS.iter().any(|endpoint| segment.eq_ignore_ascii_case(endpoint))
}

/// Returns `text` with the tracker key and the user-info password of every URL in it replaced by
/// [`MASK`].
///
/// It is meant for text built by a third-party formatter that echoes input, such as a parse error
/// or a command-line usage error, where the URL may be malformed. It does not parse URLs:
///
/// - after every `/announce/` or `/scrape/`, compared in any ASCII case and with `\` taken as
///   `/`, as URL parsers do for HTTP (so an escaped `\\` is a separator too), the rest of the path
///   is masked: from its first non-empty segment to the query (`?`), the fragment (`#`), a
///   character that cannot appear in a URL (whitespace, `"`, `<`, `>`, a backtick, `{`, `}`, `|`,
///   `^`), or an apostrophe followed by whitespace or the end of the text (the closing quote of a
///   quoted value). Masking the whole rest, not one segment, also covers the forms a URL parser
///   normalizes to the same key (`./<key>`, `%2e/<key>`, `../x/<key>`, a nested endpoint);
/// - a password is the text between the first `:` and the last `@` of each authority that follows
///   a `://`.
#[must_use]
pub fn mask_tracker_keys_in_text(text: &str) -> String {
    mask_keys_after_endpoints(&mask_user_info_passwords(text))
}

fn mask_keys_after_endpoints(text: &str) -> String {
    // ASCII lowercasing and turning `\` into `/` keep every byte offset, so offsets found in
    // `normalized` index `text` too.
    let normalized: String = text
        .chars()
        .map(|c| if c == '\\' { '/' } else { c.to_ascii_lowercase() })
        .collect();

    let mut keys: Vec<(usize, usize)> = Vec::new();

    for endpoint in KEYED_ENDPOINTS {
        let marker = format!("/{endpoint}/");
        let mut from = 0;

        // A marker can start at the `/` that ends the previous one (`/announce/scrape/<key>`), so
        // the search goes on from the next byte, not from the end of the key.
        while let Some(offset) = normalized[from..].find(&marker) {
            // More separators can follow the marker: an escaped backslash (`\\`, as `serde_json`
            // echoes one) or an empty segment. The key is the first segment after them.
            let after_marker = from + offset + marker.len();
            let key_start = after_marker
                + text[after_marker..]
                    .find(|c: char| c != '/' && c != '\\')
                    .unwrap_or(text.len() - after_marker);
            let key_end = path_end(text, key_start);

            if key_end > key_start {
                keys.push((key_start, key_end));
            }

            from += offset + 1;
        }
    }

    keys.sort_unstable();

    let mut masked = String::with_capacity(text.len());
    let mut position = 0;

    for (key_start, key_end) in keys {
        if key_start < position {
            // Overlaps a key already masked.
            position = position.max(key_end);
            continue;
        }

        masked.push_str(&text[position..key_start]);
        masked.push_str(MASK);
        position = key_end;
    }

    masked.push_str(&text[position..]);
    masked
}

/// The offset where the path that continues at `key_start` ends.
fn path_end(text: &str, key_start: usize) -> usize {
    let mut chars = text[key_start..].char_indices().peekable();

    while let Some((offset, c)) = chars.next() {
        let closes_a_quote = c == '\'' && chars.peek().is_none_or(|(_, next)| next.is_whitespace());

        if ends_path(c) || closes_a_quote {
            return key_start + offset;
        }
    }

    text.len()
}

const fn ends_path(c: char) -> bool {
    c.is_whitespace() || matches!(c, '?' | '#' | '"' | '<' | '>' | '`' | '{' | '}' | '|' | '^')
}

fn mask_user_info_passwords(text: &str) -> String {
    let mut masked = String::with_capacity(text.len());
    let mut copied = 0;
    let mut search_from = 0;

    while let Some(offset) = text[search_from..].find("://") {
        let authority_start = search_from + offset + "://".len();
        let authority_length = text[authority_start..]
            .find(|c: char| c.is_whitespace() || matches!(c, '/' | '\\' | '?' | '#'))
            .unwrap_or(text.len() - authority_start);
        let authority = &text[authority_start..authority_start + authority_length];

        masked.push_str(&text[copied..authority_start]);

        let password = authority
            .rfind('@')
            .and_then(|at| authority[..at].find(':').map(|colon| (colon, at)));

        match password {
            Some((colon, at)) => {
                masked.push_str(&authority[..=colon]);
                masked.push_str(MASK);
                masked.push_str(&authority[at..]);
            }
            None => masked.push_str(authority),
        }

        copied = authority_start + authority_length;
        // The authority can end with the scheme of the next URL (`...@a.example,http://...`), so
        // the next search starts inside it.
        search_from = authority_start;
    }

    masked.push_str(&text[copied..]);
    masked
}

/// A short description of a failed HTTP request: the kind of failure, the masked request URL and
/// the chain of causes.
///
/// It replaces the `Display` and `Debug` texts of [`reqwest::Error`], which both carry the full
/// request URL, tracker key included. The causes (from `hyper`, `hyper-util` and `std::io`) carry
/// no URL; they are masked with [`mask_tracker_keys_in_text`] all the same.
#[must_use]
pub fn request_error_text(error: &reqwest::Error) -> String {
    let kind = if error.is_timeout() {
        "timeout"
    } else if error.is_connect() {
        "connection"
    } else if error.is_status() {
        "status"
    } else if error.is_decode() {
        "decode"
    } else if error.is_body() {
        "body"
    } else if error.is_redirect() {
        "redirect"
    } else if error.is_builder() {
        "builder"
    } else if error.is_request() {
        "request"
    } else {
        "unknown"
    };

    let summary = error.url().map_or_else(
        || format!("{kind} error"),
        |url| format!("{kind} error for url ({})", mask_tracker_key(url)),
    );

    let causes: Vec<String> = iter::successors(error.source(), |&cause| cause.source())
        .map(|cause| mask_tracker_keys_in_text(&cause.to_string()))
        .collect();

    if causes.is_empty() {
        summary
    } else {
        format!("{summary}: {}", causes.join(": "))
    }
}

#[cfg(test)]
mod tests {
    use reqwest::Url;

    use super::{mask_tracker_key, mask_tracker_keys_in_text};

    fn masked(url: &str) -> String {
        mask_tracker_key(&Url::parse(url).expect("test URL should parse")).to_string()
    }

    #[test]
    fn it_should_mask_the_key_of_an_announce_url() {
        assert_eq!(
            masked("http://tracker.example.com:7070/announce/SECRET"),
            "http://tracker.example.com:7070/announce/****"
        );
    }

    #[test]
    fn it_should_mask_the_key_of_a_scrape_url() {
        assert_eq!(
            masked("http://tracker.example.com:7070/scrape/SECRET"),
            "http://tracker.example.com:7070/scrape/****"
        );
    }

    #[test]
    fn it_should_keep_the_trailing_slash_and_the_query_when_masking_the_key() {
        assert_eq!(
            masked("http://tracker.example.com/announce/SECRET/?info_hash=%9C8B"),
            "http://tracker.example.com/announce/****/?info_hash=%9C8B"
        );
    }

    #[test]
    fn it_should_keep_a_path_prefix_before_the_endpoint_when_masking_the_key() {
        assert_eq!(
            masked("https://tracker.example.com/tracker/announce/SECRET"),
            "https://tracker.example.com/tracker/announce/****"
        );
    }

    #[test]
    fn it_should_mask_the_password_of_the_url_user_info() {
        assert_eq!(
            masked("http://user:SECRET@tracker.example.com/announce"),
            "http://user:****@tracker.example.com/announce"
        );
    }

    #[test]
    fn it_should_not_change_an_announce_or_scrape_url_without_a_key() {
        assert_eq!(
            masked("http://tracker.example.com/announce"),
            "http://tracker.example.com/announce"
        );
        assert_eq!(
            masked("http://tracker.example.com/scrape/"),
            "http://tracker.example.com/scrape/"
        );
    }

    #[test]
    fn it_should_not_change_a_url_that_is_not_an_announce_or_scrape_url() {
        assert_eq!(
            masked("http://tracker.example.com/health_check"),
            "http://tracker.example.com/health_check"
        );
    }

    #[test]
    fn it_should_mask_the_key_of_every_tracker_url_in_a_text() {
        let text =
            "unexpected argument 'http://a.example.com/announce/SECRET1' found, also \"https://b.example.com/scrape/SECRET2\"";

        assert_eq!(
            mask_tracker_keys_in_text(text),
            "unexpected argument 'http://a.example.com/announce/****' found, also \"https://b.example.com/scrape/****\""
        );
    }

    #[test]
    fn it_should_not_change_a_text_without_a_keyed_tracker_url() {
        let text = "invalid type: string \"http://tracker.example.com/announce\", expected a sequence at line 1 column 80";

        assert_eq!(mask_tracker_keys_in_text(text), text);
    }

    #[test]
    fn it_should_mask_the_key_of_a_text_that_is_one_keyed_url() {
        assert_eq!(
            mask_tracker_keys_in_text("http://tracker.example.com/announce/SECRET"),
            "http://tracker.example.com/announce/****"
        );
    }

    #[test]
    fn it_should_not_change_a_text_without_a_url() {
        let text = "JSON parse error: expected value at line 1 column 1";

        assert_eq!(mask_tracker_keys_in_text(text), text);
    }

    #[test]
    fn it_should_mask_the_key_of_a_url_whose_scheme_is_not_lowercase() {
        let text = "unexpected argument 'HTTP://a.example.com/announce/SECRET1' and \"Https://b.example.com/scrape/SECRET2\"";

        assert_eq!(
            mask_tracker_keys_in_text(text),
            "unexpected argument 'HTTP://a.example.com/announce/****' and \"Https://b.example.com/scrape/****\""
        );
    }

    #[test]
    fn it_should_mask_the_key_of_a_url_with_a_closing_parenthesis_before_the_endpoint() {
        assert_eq!(
            mask_tracker_keys_in_text("'http://tracker.example.com/prefix)/announce/SECRET' found"),
            "'http://tracker.example.com/prefix)/announce/****' found"
        );
    }

    #[test]
    fn it_should_mask_the_key_of_a_url_that_does_not_parse() {
        assert_eq!(
            mask_tracker_keys_in_text("invalid type: string \"http://127.0.0.1:bad/announce/SECRET\", expected a sequence"),
            "invalid type: string \"http://127.0.0.1:bad/announce/****\", expected a sequence"
        );
    }

    #[test]
    fn it_should_mask_the_password_and_the_key_of_a_url_with_an_apostrophe_in_the_user_info() {
        assert_eq!(
            mask_tracker_keys_in_text("unexpected argument 'http://o'neil:SECRET1@tracker.example.com/announce/SECRET2' found"),
            "unexpected argument 'http://o'neil:****@tracker.example.com/announce/****' found"
        );
    }

    #[test]
    fn it_should_mask_the_key_after_a_backslash_separator() {
        assert_eq!(
            mask_tracker_keys_in_text("unexpected argument 'http://tracker.example.com/announce\\SECRET' found"),
            "unexpected argument 'http://tracker.example.com/announce\\****' found"
        );
    }

    #[test]
    fn it_should_mask_a_key_segment_that_starts_with_or_contains_a_delimiter() {
        assert_eq!(
            mask_tracker_keys_in_text("http://tracker.example.com/announce/(SECRET"),
            "http://tracker.example.com/announce/****"
        );
        assert_eq!(
            mask_tracker_keys_in_text("http://tracker.example.com/announce/abc)SECRET"),
            "http://tracker.example.com/announce/****"
        );
    }

    #[test]
    fn it_should_mask_the_key_after_an_endpoint_that_follows_another_endpoint() {
        assert_eq!(
            mask_tracker_keys_in_text("http://tracker.example.com/announce/scrape/SECRET"),
            "http://tracker.example.com/announce/****"
        );
    }

    #[test]
    fn it_should_mask_the_password_of_every_url_in_a_text() {
        assert_eq!(
            mask_tracker_keys_in_text("http://u:SECRET1@a.example.com,http://v:SECRET2@b.example.com"),
            "http://u:****@a.example.com,http://v:****@b.example.com"
        );
    }

    #[test]
    fn it_should_mask_the_key_after_an_escaped_backslash_separator() {
        // `serde_json` escapes a backslash as `\\` when it echoes a string value.
        assert_eq!(
            mask_tracker_keys_in_text(
                r#"invalid type: string "http://tracker.example.com/announce\\SECRET", expected a sequence"#
            ),
            r#"invalid type: string "http://tracker.example.com/announce\\****", expected a sequence"#
        );
    }

    #[test]
    fn it_should_keep_the_percent_encoding_of_the_other_segments_and_the_query() {
        assert_eq!(
            masked("http://tracker.example.com/a%7Eb/announce/SECRET?info_hash=%9C8B&x=%2F"),
            "http://tracker.example.com/a%7Eb/announce/****?info_hash=%9C8B&x=%2F"
        );
    }

    #[test]
    fn it_should_mask_the_key_after_a_dot_segment() {
        assert_eq!(
            mask_tracker_keys_in_text("unexpected argument 'http://tracker.example.com/announce/./SECRET' found"),
            "unexpected argument 'http://tracker.example.com/announce/****' found"
        );
        assert_eq!(
            mask_tracker_keys_in_text("unexpected argument 'http://tracker.example.com/announce/%2E/SECRET' found"),
            "unexpected argument 'http://tracker.example.com/announce/****' found"
        );
    }

    #[test]
    fn it_should_mask_the_whole_path_after_the_endpoint_but_keep_the_query() {
        assert_eq!(
            mask_tracker_keys_in_text("\"http://tracker.example.com/announce/../x/SECRET?info_hash=%9C8B\", expected"),
            "\"http://tracker.example.com/announce/****?info_hash=%9C8B\", expected"
        );
    }
}
