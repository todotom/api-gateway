use std::net::SocketAddr;

use axum::Server;

use super::{application_state::ApplicationState, router::create_router};

pub async fn run_server(application_state: ApplicationState) {
    let router = create_router(application_state);
    let socket_address = SocketAddr::from(([0, 0, 0, 0], 3000));
    Server::bind(&socket_address)
        .serve(router.into_make_service())
        .await
        .unwrap();
}
