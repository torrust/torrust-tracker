use std::panic::Location;

use axum::async_trait;
use axum::extract::{FromRequestParts, Path};
use axum::http::request::Parts;
use axum::response::{IntoResponse, Response};

use crate::http::axum_implementation::handlers::auth::{self, KeyParam};
use crate::http::axum_implementation::responses;
use crate::tracker::auth::Key;

pub struct Extract(pub Key);

#[async_trait]
impl<S> FromRequestParts<S> for Extract
where
    S: Send + Sync,
{
    type Rejection = Response;

    async fn from_request_parts(parts: &mut Parts, state: &S) -> Result<Self, Self::Rejection> {
        match Path::<KeyParam>::from_request_parts(parts, state).await {
            Ok(key_param) => {
                let Ok(key) = key_param.0.value().parse::<Key>() else {
                    return Err(responses::error::Error::from(
                        auth::Error::InvalidKeyFormat {
                            location: Location::caller()
                        })
                    .into_response())
                };
                Ok(Extract(key))
            }
            Err(rejection) => match rejection {
                axum::extract::rejection::PathRejection::FailedToDeserializePathParams(_) => {
                    return Err(responses::error::Error::from(auth::Error::InvalidKeyFormat {
                        location: Location::caller(),
                    })
                    .into_response())
                }
                axum::extract::rejection::PathRejection::MissingPathParams(_) => {
                    return Err(responses::error::Error::from(auth::Error::MissingAuthKey {
                        location: Location::caller(),
                    })
                    .into_response())
                }
                _ => {
                    return Err(responses::error::Error::from(auth::Error::CannotExtractKeyParam {
                        location: Location::caller(),
                    })
                    .into_response())
                }
            },
        }
    }
}
