use axum::extract::FromRef;
use sqlx::PgPool;

#[derive(Clone, Debug, FromRef)]
pub struct AppState {
    pub db: PgPool,
}
