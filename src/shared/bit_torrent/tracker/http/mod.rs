pub mod client;

use percent_encoding::NON_ALPHANUMERIC;
use torrust_tracker_primitives::info_hash::ByteArray20;

#[must_use]
pub fn percent_encode_byte_array(bytes: &ByteArray20) -> String {
    percent_encoding::percent_encode(bytes, NON_ALPHANUMERIC).to_string()
}
