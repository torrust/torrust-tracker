//! API handlers for the [`torrent`](crate::v1::context::torrent)
//! API context.
use std::fmt;
use std::str::FromStr;
use std::sync::Arc;

use axum::extract::{Path, State};
use axum::response::{IntoResponse, Response};
use axum_extra::extract::Query;
use serde::{Deserialize, Deserializer, de};
use thiserror::Error;
use torrust_info_hash::InfoHash;
use torrust_tracker_primitives::pagination::Pagination;
use torrust_tracker_rest_api_application::v1::use_cases::torrent::TorrentApiService;

use super::responses::{torrent_info_response, torrent_list_response, torrent_not_known_response};
use crate::InfoHashParam;
use crate::v1::responses::invalid_info_hash_param_response;

/// It handles the request to get the torrent data.
///
/// It returns:
///
/// - `200` response with a json [`Torrent`](torrust_tracker_rest_api_protocol::v1::context::torrent::resources::torrent::Torrent).
/// - `500` with serialized error in debug format if the torrent is not known.
///
/// Refer to the [API endpoint documentation](crate::v1::context::torrent#get-a-torrent)
/// for more information about this endpoint.
pub async fn get_torrent_handler(
    State(service): State<Arc<TorrentApiService>>,
    Path(info_hash): Path<InfoHashParam>,
) -> Response {
    match InfoHash::from_str(&info_hash.0) {
        Err(_) => invalid_info_hash_param_response(&info_hash.0),
        Ok(info_hash) => service
            .get_torrent(&info_hash)
            .await
            .map_or_else(torrent_not_known_response, |torrent| {
                torrent_info_response(torrent).into_response()
            }),
    }
}

/// A container for the URL query parameters.
///
/// Pagination: `offset` and `limit`.
/// Array of infohashes: `info_hash`.
///
/// You can either get all torrents with pagination or get a list of torrents
/// providing a list of infohashes. For example:
///
/// First page of torrents:
///
/// <http://127.0.0.1:1212/api/v1/torrents?token=MyAccessToken>
///
///
/// Only two torrents:
///
/// <http://127.0.0.1:1212/api/v1/torrents?token=MyAccessToken&info_hash=9c38422213e30bff212b30c360d26f9a02136422&info_hash=2b66980093bc11806fab50cb3cb41835b95a0362>
///
///
/// NOTICE: Pagination is ignored if array of infohashes is provided.
#[derive(Deserialize, Debug)]
pub struct QueryParams {
    /// The offset of the first page to return. Starts at 0.
    #[serde(default, deserialize_with = "empty_string_as_none")]
    pub offset: Option<u32>,
    /// The maximum number of items to return per page.
    #[serde(default, deserialize_with = "empty_string_as_none")]
    pub limit: Option<u32>,
    /// A list of infohashes to retrieve.
    #[serde(default, rename = "info_hash")]
    pub info_hashes: Vec<String>,
}

/// It handles the request to get a list of torrents.
///
/// It returns a `200` response with a json array with [`ListItem`](torrust_tracker_rest_api_protocol::v1::context::torrent::resources::torrent::ListItem) resources.
///
/// Refer to the [API endpoint documentation](crate::v1::context::torrent#list-torrents)
/// for more information about this endpoint.
pub async fn get_torrents_handler(State(service): State<Arc<TorrentApiService>>, pagination: Query<QueryParams>) -> Response {
    tracing::debug!("pagination: {:?}", pagination);

    if pagination.0.info_hashes.is_empty() {
        torrent_list_response(
            service
                .get_torrents_page(&Pagination::new_with_options(pagination.0.offset, pagination.0.limit))
                .await,
        )
        .into_response()
    } else {
        match parse_info_hashes(pagination.0.info_hashes) {
            Ok(info_hashes) => torrent_list_response(service.get_torrents(&info_hashes).await).into_response(),
            Err(err) => match err {
                QueryParamError::InvalidInfoHash { info_hash } => invalid_info_hash_param_response(&info_hash),
            },
        }
    }
}

#[derive(Error, Debug)]
#[non_exhaustive]
pub enum QueryParamError {
    #[error("invalid infohash {info_hash}")]
    InvalidInfoHash { info_hash: String },
}

fn parse_info_hashes(info_hashes_str: Vec<String>) -> Result<Vec<InfoHash>, QueryParamError> {
    let mut info_hashes: Vec<InfoHash> = Vec::new();

    for info_hash_str in info_hashes_str {
        match InfoHash::from_str(&info_hash_str) {
            Ok(info_hash) => info_hashes.push(info_hash),
            Err(_err) => {
                return Err(QueryParamError::InvalidInfoHash {
                    info_hash: info_hash_str,
                });
            }
        }
    }

    Ok(info_hashes)
}

/// Serde deserialization decorator to map empty Strings to None,
fn empty_string_as_none<'de, D, T>(de: D) -> Result<Option<T>, D::Error>
where
    D: Deserializer<'de>,
    T: FromStr,
    T::Err: fmt::Display,
{
    let opt = Option::<String>::deserialize(de)?;
    match opt.as_deref() {
        None | Some("") => Ok(None),
        Some(s) => FromStr::from_str(s).map_err(de::Error::custom).map(Some),
    }
}

#[cfg(test)]
mod tests {
    use std::sync::{Arc, Mutex};

    use async_trait::async_trait;
    use axum::extract::{Path, State};
    use axum::http::{StatusCode, Uri};
    use axum_extra::extract::Query;
    use rstest::rstest;
    use serde::Deserialize;
    use torrust_info_hash::InfoHash;
    use torrust_tracker_primitives::pagination::Pagination;
    use torrust_tracker_rest_api_application::v1::ports::torrent::TorrentQueryPort;
    use torrust_tracker_rest_api_application::v1::use_cases::torrent::TorrentApiService;
    use torrust_tracker_rest_api_protocol::v1::context::torrent::resources::torrent::{ListItem, Torrent};

    use super::{QueryParams, empty_string_as_none, get_torrent_handler, get_torrents_handler};
    use crate::InfoHashParam;
    use crate::test_support::{Observed, observed};

    type TorrentOutcome = fn() -> Option<Torrent>;
    type ItemsOutcome = fn() -> Vec<ListItem>;
    type CallLog = Arc<Mutex<Vec<Call>>>;

    const FIRST_HASH: &str = "443c7602b4fde83d1154d6d9da48808418b181b6";
    const SECOND_HASH: &str = "5452869be36f9f3350ccee6b4544e7e76caaadab";
    const JSON: &str = "application/json";
    const TEXT: &str = "text/plain; charset=utf-8";
    const TORRENT_JSON: &str =
        r#"{"info_hash":"443c7602b4fde83d1154d6d9da48808418b181b6","seeders":1,"completed":2,"leechers":3}"#;
    const ITEMS_JSON: &str = concat!(
        r#"[{"info_hash":"443c7602b4fde83d1154d6d9da48808418b181b6","seeders":1,"completed":2,"leechers":3},"#,
        r#"{"info_hash":"5452869be36f9f3350ccee6b4544e7e76caaadab","seeders":4,"completed":5,"leechers":6}]"#
    );

    /// A call that the fake port received, with the inputs the handler forwarded.
    #[derive(Clone, Debug, PartialEq, Eq)]
    enum Call {
        TorrentInfo(InfoHash),
        TorrentsPage(Pagination),
        Torrents(Vec<InfoHash>),
    }

    fn info_hash(value: &str) -> InfoHash {
        value.parse().expect("the test info hash is valid")
    }

    fn invalid_info_hash(value: &str) -> Observed {
        Observed::new(
            StatusCode::BAD_REQUEST,
            TEXT,
            &format!(r#"Invalid URL: invalid infohash param: string "{value}", expected a 40 character long string"#),
        )
    }

    #[allow(clippy::unnecessary_wraps, reason = "it must have the outcome type of the fake port")]
    fn a_torrent() -> Option<Torrent> {
        Some(Torrent {
            info_hash: FIRST_HASH.to_string(),
            seeders: 1,
            completed: 2,
            leechers: 3,
            peers: None,
        })
    }

    fn no_torrent() -> Option<Torrent> {
        None
    }

    fn two_items() -> Vec<ListItem> {
        vec![
            ListItem {
                info_hash: FIRST_HASH.to_string(),
                seeders: 1,
                completed: 2,
                leechers: 3,
            },
            ListItem {
                info_hash: SECOND_HASH.to_string(),
                seeders: 4,
                completed: 5,
                leechers: 6,
            },
        ]
    }

    /// A `TorrentQueryPort` that answers each call with a scripted outcome and records the call.
    struct FakeTorrentQueryPort {
        torrent: TorrentOutcome,
        items: ItemsOutcome,
        calls: CallLog,
    }

    impl FakeTorrentQueryPort {
        fn answering(torrent: TorrentOutcome, items: ItemsOutcome) -> Self {
            Self {
                torrent,
                items,
                calls: Arc::new(Mutex::new(Vec::new())),
            }
        }

        fn call_log(&self) -> CallLog {
            self.calls.clone()
        }

        fn record(&self, call: Call) {
            self.calls.lock().expect("the call log is not poisoned").push(call);
        }
    }

    #[async_trait]
    impl TorrentQueryPort for FakeTorrentQueryPort {
        async fn get_torrent_info(&self, info_hash: &InfoHash) -> Option<Torrent> {
            self.record(Call::TorrentInfo(*info_hash));
            (self.torrent)()
        }

        async fn get_torrents_page(&self, pagination: &Pagination) -> Vec<ListItem> {
            self.record(Call::TorrentsPage(*pagination));
            (self.items)()
        }

        async fn get_torrents(&self, info_hashes: &[InfoHash]) -> Vec<ListItem> {
            self.record(Call::Torrents(info_hashes.to_vec()));
            (self.items)()
        }
    }

    fn state_with(fake: FakeTorrentQueryPort) -> State<Arc<TorrentApiService>> {
        State(Arc::new(TorrentApiService::new(Box::new(fake))))
    }

    fn param(value: &str) -> Path<InfoHashParam> {
        Path(InfoHashParam(value.to_string()))
    }

    fn query(query_string: &str) -> Query<QueryParams> {
        let uri: Uri = format!("/api/v1/torrents?{query_string}")
            .parse()
            .expect("the test URI is valid");

        Query::try_from_uri(&uri).expect("the query string is valid")
    }

    fn recorded(calls: &CallLog) -> Vec<Call> {
        calls.lock().expect("the call log is not poisoned").clone()
    }

    #[tokio::test]
    async fn it_should_reject_an_invalid_info_hash_without_calling_the_service_when_getting_a_torrent() {
        let fake = FakeTorrentQueryPort::answering(a_torrent, two_items);
        let calls = fake.call_log();

        let response = get_torrent_handler(state_with(fake), param("abc")).await;

        assert_eq!(observed(response).await, invalid_info_hash("abc"));
        assert_eq!(recorded(&calls), Vec::<Call>::new());
    }

    #[tokio::test]
    async fn it_should_answer_the_torrent_and_forward_the_parsed_info_hash_to_the_service_when_getting_a_torrent() {
        let fake = FakeTorrentQueryPort::answering(a_torrent, two_items);
        let calls = fake.call_log();

        let response = get_torrent_handler(state_with(fake), param(FIRST_HASH)).await;

        assert_eq!(
            (observed(response).await, recorded(&calls)),
            (
                Observed::new(StatusCode::OK, JSON, TORRENT_JSON),
                vec![Call::TorrentInfo(info_hash(FIRST_HASH))]
            )
        );
    }

    /// The legacy answer for an unknown torrent is a `200` with a JSON string, as the module documentation says.
    #[tokio::test]
    async fn it_should_answer_the_documented_not_known_text_with_ok_when_the_torrent_is_not_known() {
        let fake = FakeTorrentQueryPort::answering(no_torrent, two_items);
        let calls = fake.call_log();

        let response = get_torrent_handler(state_with(fake), param(FIRST_HASH)).await;

        assert_eq!(
            (observed(response).await, recorded(&calls)),
            (
                Observed::new(StatusCode::OK, JSON, r#""torrent not known""#),
                vec![Call::TorrentInfo(info_hash(FIRST_HASH))]
            )
        );
    }

    #[rstest]
    #[case::no_parameters("", Pagination::default())]
    #[case::empty_parameters("offset=&limit=", Pagination::default())]
    #[case::only_the_offset("offset=5", Pagination::new(5, Pagination::default_limit()))]
    #[case::only_the_limit("limit=2", Pagination::new(Pagination::default_offset(), 2))]
    #[case::the_offset_and_the_limit("offset=5&limit=2", Pagination::new(5, 2))]
    #[tokio::test]
    async fn it_should_serve_a_page_of_torrents_when_no_info_hash_is_given(
        #[case] query_string: &str,
        #[case] expected: Pagination,
    ) {
        let fake = FakeTorrentQueryPort::answering(a_torrent, two_items);
        let calls = fake.call_log();

        let response = get_torrents_handler(state_with(fake), query(query_string)).await;

        assert_eq!(
            (observed(response).await, recorded(&calls)),
            (
                Observed::new(StatusCode::OK, JSON, ITEMS_JSON),
                vec![Call::TorrentsPage(expected)]
            )
        );
    }

    #[tokio::test]
    async fn it_should_serve_the_given_torrents_in_order_and_ignore_the_pagination_when_info_hashes_are_given() {
        let fake = FakeTorrentQueryPort::answering(a_torrent, two_items);
        let calls = fake.call_log();
        let query_string = format!("info_hash={SECOND_HASH}&info_hash={FIRST_HASH}&info_hash={FIRST_HASH}&offset=1&limit=1");

        let response = get_torrents_handler(state_with(fake), query(&query_string)).await;

        assert_eq!(
            (observed(response).await, recorded(&calls)),
            (
                Observed::new(StatusCode::OK, JSON, ITEMS_JSON),
                vec![Call::Torrents(vec![
                    info_hash(SECOND_HASH),
                    info_hash(FIRST_HASH),
                    info_hash(FIRST_HASH)
                ])]
            )
        );
    }

    #[rstest]
    #[case::an_invalid_one_after_a_valid_one(format!("info_hash={FIRST_HASH}&info_hash=bad"), "bad")]
    #[case::two_invalid_ones("info_hash=bad&info_hash=worse".to_string(), "bad")]
    #[case::an_empty_one("info_hash=".to_string(), "")]
    #[tokio::test]
    async fn it_should_reject_the_first_invalid_info_hash_without_calling_the_service_when_listing_torrents(
        #[case] query_string: String,
        #[case] rejected: &str,
    ) {
        let fake = FakeTorrentQueryPort::answering(a_torrent, two_items);
        let calls = fake.call_log();

        let response = get_torrents_handler(state_with(fake), query(&query_string)).await;

        assert_eq!(observed(response).await, invalid_info_hash(rejected));
        assert_eq!(recorded(&calls), Vec::<Call>::new());
    }

    #[rstest]
    #[case::no_parameters("", None, None)]
    #[case::empty_values("offset=&limit=", None, None)]
    #[case::zero("offset=0&limit=0", Some(0), Some(0))]
    #[case::normal_values("offset=5&limit=2", Some(5), Some(2))]
    #[case::the_largest_values("offset=4294967295&limit=4294967295", Some(u32::MAX), Some(u32::MAX))]
    fn it_should_read_an_empty_or_absent_pagination_value_as_not_set(
        #[case] query_string: &str,
        #[case] offset: Option<u32>,
        #[case] limit: Option<u32>,
    ) {
        let params = query(query_string).0;

        assert_eq!((params.offset, params.limit), (offset, limit));
    }

    #[rstest]
    #[case::a_negative_offset("offset=-1")]
    #[case::a_decimal_offset("offset=1.1")]
    #[case::a_word_as_the_offset("offset=abc")]
    #[case::a_space_as_the_offset("offset=%20")]
    #[case::an_offset_over_the_maximum("offset=4294967296")]
    #[case::a_negative_limit("limit=-1")]
    #[case::a_limit_over_the_maximum("limit=4294967296")]
    fn it_should_reject_a_pagination_value_that_is_not_an_unsigned_32_bit_number(#[case] query_string: &str) {
        let uri: Uri = format!("/api/v1/torrents?{query_string}")
            .parse()
            .expect("the test URI is valid");

        let result = Query::<QueryParams>::try_from_uri(&uri);

        assert!(result.is_err(), "the value was accepted: {result:?}");
    }

    #[rstest]
    #[case::two_hashes("info_hash=a&info_hash=b", vec!["a", "b"])]
    #[case::an_empty_hash("info_hash=", vec![""])]
    #[case::no_hashes("offset=1", vec![])]
    fn it_should_collect_every_info_hash_parameter_in_order(#[case] query_string: &str, #[case] expected: Vec<&str>) {
        let params = query(query_string).0;

        assert_eq!(params.info_hashes, expected);
    }

    /// The value of the field read by `empty_string_as_none`, to test the function apart from the query extractor.
    #[derive(Deserialize)]
    struct Probe {
        #[serde(default, deserialize_with = "empty_string_as_none")]
        value: Option<u32>,
    }

    /// The query extractor already reads an empty `offset=` as absent, so the empty-string arm is only reachable
    /// through other deserializers; it is tested here with JSON.
    #[rstest]
    #[case::an_empty_string(r#"{"value":""}"#, Ok(None))]
    #[case::a_number_in_a_string(r#"{"value":"7"}"#, Ok(Some(7)))]
    #[case::null(r#"{"value":null}"#, Ok(None))]
    #[case::an_absent_field("{}", Ok(None))]
    #[case::a_number_instead_of_a_string(r#"{"value":7}"#, Err(()))]
    #[case::a_word(r#"{"value":"abc"}"#, Err(()))]
    #[case::a_number_over_the_maximum(r#"{"value":"4294967296"}"#, Err(()))]
    fn it_should_read_an_empty_string_as_not_set_and_reject_what_is_not_a_number(
        #[case] json: &str,
        #[case] expected: Result<Option<u32>, ()>,
    ) {
        let probe = serde_json::from_str::<Probe>(json);

        assert_eq!(probe.map(|probe| probe.value).map_err(|_| ()), expected);
    }
}
