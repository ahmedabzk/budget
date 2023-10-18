use axum::{
    http::Method,
    routing::{get, post},
    Extension, Router,
};
use sqlx::PgPool;

use tower_http::cors::{Any, CorsLayer};

use crate::controllers::asserts_controller::{
    create_asset, edit_asset, get_all_assets, get_one_asset, remove_asset,
};
use crate::controllers::expenses_controller::{
    create_expense, edit_expense, get_all_expenses, get_one_expense, remove_expense,
};
use crate::controllers::income_controller::{create_income, edit, get_all, get_one, remove};
use crate::controllers::savings_controller::{
    create_savings, edit_saving, get_all_savings, get_one_saving, remove_saving,
};

// #[debug_handler(state = PgPool)]
pub async fn create_routes(db: PgPool) -> Router<()> {
    let cors = CorsLayer::new()
        .allow_methods([Method::GET, Method::POST])
        .allow_origin(Any);

    Router::new()
        .route("/api/v1/income/create", post(create_income))
        .route("/api/v1/income/remove/:id", get(remove))
        .route("/api/v1/income/edit/:id", post(edit))
        .route("/api/v1/income/all", get(get_all))
        .route("/api/v1/income/one/:id", get(get_one))
        .route("/api/v1/expense/create", post(create_expense))
        .route("/api/v1/expense/all", get(get_all_expenses))
        .route("/api/v1/expense/one/:id", get(get_one_expense))
        .route("/api/v1/expense/remove/:id", get(remove_expense))
        .route("/api/v1/expense/edit/:id", post(edit_expense))
        .route("/api/v1/savings/create", post(create_savings))
        .route("/api/v1/savings/all", get(get_all_savings))
        .route("/api/v1/savings/one/:id", get(get_one_saving))
        .route("/api/v1/savings/edit/:id", post(edit_saving))
        .route("/api/v1/savings/remove/:id", get(remove_saving))
        .route("/api/v1/asset/create", post(create_asset))
        .route("/api/v1/asset/all", get(get_all_assets))
        .route("/api/v1/asset/one/:id", get(get_one_asset))
        .route("/api/v1/asset/edit/:id", post(edit_asset))
        .route("/api/v1/asset/remove/:id", get(remove_asset))
        .layer(cors)
        .layer(Extension(db))
}
