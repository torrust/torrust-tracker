//! Masking of the credentials an HTTP tracker URL can carry, for text that is written to logs,
//! diagnostics or results.
//!
//! A private HTTP tracker authenticates a peer with a key in the URL path: `/announce/<key>` and
//! `/scrape/<key>`. Section 7 of the global CLI output contract
//! (`docs/adrs/20260519000000_define_global_cli_output_contract.md`) says secrets are masked with a
//! fixed string, so these functions replace the key with [`MASK`] and keep the rest of the URL, so a
//! reader can still tell which tracker it was.
use reqwest::Url;

/// The fixed string that replaces a secret.
pub const MASK: &str = "****";

/// The path segments a tracker key follows: `/announce/<key>` and `/scrape/<key>`.
const KEYED_ENDPOINTS: [&str; 2] = ["announce", "scrape"];

/// Returns `url` with its tracker key and its password replaced by [`MASK`].
///
/// The tracker key is the path segment after `announce` or `scrape`; the password is the one of
/// the user info, if any. The rest of the URL (scheme, host, port, the other path segments and the
/// query) is kept.
#[must_use]
pub fn mask_tracker_key(url: &Url) -> Url {
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
                let follows_keyed_endpoint = index > 0 && KEYED_ENDPOINTS.contains(&segments[index - 1]);

                if follows_keyed_endpoint && !segment.is_empty() {
                    MASK
                } else {
                    segment
                }
            })
            .collect();

        if masked_segments != segments {
            masked.set_path(&format!("/{}", masked_segments.join("/")));
        }
    }

    masked
}

/// Returns `text` with the secrets of every HTTP tracker URL in it masked by
/// [`mask_tracker_key`].
///
/// It is meant for text built by a third-party formatter that echoes input, such as a parse error
/// or a command-line usage error. A URL ends at whitespace, a quote, a backtick, an angle bracket
/// or a closing parenthesis. A URL that does not parse is left as it is.
#[must_use]
pub fn mask_tracker_keys_in_text(text: &str) -> String {
    let mut masked = String::with_capacity(text.len());
    let mut rest = text;

    while let Some(start) = find_http_url_start(rest) {
        masked.push_str(&rest[..start]);

        let candidate = &rest[start..];
        let end = candidate
            .find(|c: char| c.is_whitespace() || matches!(c, '\'' | '"' | '`' | '<' | '>' | ')'))
            .unwrap_or(candidate.len());
        let token = &candidate[..end];

        match Url::parse(token) {
            Ok(url) if mask_tracker_key(&url) != url => masked.push_str(mask_tracker_key(&url).as_str()),
            _ => masked.push_str(token),
        }

        rest = &candidate[end..];
    }

    masked.push_str(rest);
    masked
}

fn find_http_url_start(text: &str) -> Option<usize> {
    ["http://", "https://"].iter().filter_map(|scheme| text.find(scheme)).min() // DevSkim: ignore DS137138
}

/// A short description of a failed HTTP request: the kind of failure and the masked request URL.
///
/// It replaces the `Display` and `Debug` texts of [`reqwest::Error`], which both carry the full
/// request URL, tracker key included.
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

    error.url().map_or_else(
        || format!("{kind} error"),
        |url| format!("{kind} error for url ({})", mask_tracker_key(url)),
    )
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
}
