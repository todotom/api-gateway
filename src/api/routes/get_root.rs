use axum::extract::State;

use crate::api::application_state::ApplicationState;

pub async fn get_root(State(application_state): State<ApplicationState>) -> String {
    application_state.default_message
}
