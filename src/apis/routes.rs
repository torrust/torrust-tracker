use std::sync::Arc;

use axum::{middleware, Router};

use super::v1;
use super::v1::middlewares::auth::auth;
use crate::tracker::Tracker;

#[allow(clippy::needless_pass_by_value)]
pub fn router(tracker: Arc<Tracker>) -> Router {
    let router = Router::new();

    let router = v1::routes::add(router, tracker.clone());

    router.layer(middleware::from_fn_with_state(tracker.config.clone(), auth))
}
