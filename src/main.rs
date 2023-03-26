use api_gateway::api::{application_state::ApplicationState, server::run_server};

#[tokio::main]
async fn main() {
    let application_state = ApplicationState {
        default_message: "Hello world!".to_owned(),
    };
    run_server(application_state).await;
}
