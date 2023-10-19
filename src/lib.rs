use app_state::AppState;
use axum;

pub mod app_state;
pub mod controllers;
pub mod errors;
pub mod models;
pub mod routes;

use crate::routes::routes::create_routes;

pub async fn run_app(app_state: AppState) {
    let app = create_routes(app_state).await;

    let addr = std::net::SocketAddr::from(([127, 0, 0, 1], 3000));
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .expect("failed to run server")
}
