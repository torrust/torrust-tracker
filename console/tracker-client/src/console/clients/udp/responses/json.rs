use anyhow::Context;
use serde::Serialize;

use super::dto::SerializableResponse;

#[allow(clippy::module_name_repetitions)]
pub trait ToJson {
    ///
    /// Returns a string with the JSON serialized version of the response
    ///
    /// # Errors
    ///
    /// Will return an error if serialization fails.
    ///
    fn to_json_string(&self, pretty: bool) -> anyhow::Result<String>
    where
        Self: Serialize,
    {
        let json = if pretty {
            serde_json::to_string_pretty(self).context("response JSON pretty serialization")?
        } else {
            serde_json::to_string(self).context("response JSON compact serialization")?
        };

        Ok(json)
    }
}

impl ToJson for SerializableResponse {}

#[cfg(test)]
mod tests {
    use serde::Serialize;

    use super::ToJson;

    #[derive(Serialize)]
    struct SampleResponse {
        transaction_id: i32,
        seeders: i32,
    }

    impl ToJson for SampleResponse {}

    #[test]
    fn it_should_serialize_compact_json_when_pretty_is_false() {
        let response = SampleResponse {
            transaction_id: 10,
            seeders: 2,
        };

        let json = response.to_json_string(false).expect("it should serialize compact JSON");

        assert_eq!(json, "{\"transaction_id\":10,\"seeders\":2}");
    }

    #[test]
    fn it_should_serialize_pretty_json_when_pretty_is_true() {
        let response = SampleResponse {
            transaction_id: 10,
            seeders: 2,
        };

        let json = response.to_json_string(true).expect("it should serialize pretty JSON");

        assert!(json.contains('\n'));
        assert!(json.contains("  \"transaction_id\": 10"));
        assert!(json.contains("  \"seeders\": 2"));
    }
}
