use axum;
use sqlx::postgres::PgPoolOptions;

pub mod controllers;
pub mod errors;
pub mod models;
pub mod routes;

use crate::routes::routes::create_routes;

pub async fn run_app(database_url: &str) {
    let db = PgPoolOptions::new()
        .connect(&database_url)
        .await
        .expect("failed to connect to database");

    let app = create_routes(db);

    let addr = std::net::SocketAddr::from(([127, 0, 0, 1], 3000));
    axum::Server::bind(&addr)
        .serve(app.await.into_make_service())
        .await
        .expect("failed to run server")
}
