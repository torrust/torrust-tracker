use crate::common::fixtures::invalid_info_hashes;

// When these infohashes are used in URL path params
// the response is a custom response returned in the handler
pub fn invalid_infohashes_returning_bad_request() -> Vec<String> {
    invalid_info_hashes()
}

// When these infohashes are used in URL path params
// the response is an Axum response returned in the handler
pub fn invalid_infohashes_returning_not_found() -> Vec<String> {
    [String::new(), " ".to_string()].to_vec()
}
