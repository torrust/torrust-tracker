use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use serde::{self, Serialize};

#[derive(Serialize)]
pub struct Error {
    #[serde(rename = "failure reason")]
    pub failure_reason: String,
}

impl Error {
    /// # Panics
    ///
    /// It would panic if the `Error` struct contained an inappropriate type.
    #[must_use]
    pub fn write(&self) -> String {
        serde_bencode::to_string(&self).unwrap()
    }
}

impl IntoResponse for Error {
    fn into_response(self) -> Response {
        (StatusCode::OK, self.write()).into_response()
    }
}

#[cfg(test)]
mod tests {

    use super::Error;

    #[test]
    fn http_tracker_errors_can_be_bencoded() {
        let err = Error {
            failure_reason: "error message".to_owned(),
        };

        assert_eq!(err.write(), "d14:failure reason13:error messagee"); // cspell:disable-line
    }
}
