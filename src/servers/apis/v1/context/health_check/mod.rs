//! API health check endpoint.
//!
//! It is used to check is the service is running. Especially for containers.
//!
//! # Endpoints
//!
//! - [Health Check](#health-check)
//!
//! # Health Check
//!
//! `GET /health_check`
//!
//! Returns the API status.
//!
//! **Example request**
//!
//! ```bash
//! curl "http://127.0.0.1:1212/health_check"
//! ```
//!
//! **Example response** `200`
//!
//! ```json
//! {
//!     "status": "Ok",
//!   }
//! ```
//!
//! **Resource**
//!
//! Refer to the API [`Stats`](crate::servers::apis::v1::context::health_check::resources::Report)
//! resource for more information about the response attributes.
pub mod handlers;
pub mod resources;
