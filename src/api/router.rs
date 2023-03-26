use axum::{routing::get, Router};

use super::{application_state::ApplicationState, routes::get_root::get_root};

pub fn create_router(application_state: ApplicationState) -> Router {
    Router::new()
        .route("/", get(get_root))
        .with_state(application_state)
}
