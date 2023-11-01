use axum::{
    http::Method,
    routing::{get, post},
    Router,
};
use tower_http::cors::{Any, CorsLayer};

use crate::controllers::expenses_controller::{
    create_expense, edit_expense, get_all_expenses, get_one_expense, remove_expense,
};
use crate::controllers::income_controller::{create_income, edit, get_all, get_one, remove};
use crate::controllers::savings_controller::{
    create_savings, edit_saving, get_all_savings, get_one_saving, remove_saving,
};
use crate::controllers::total_income::{get_total_income, get_total_expenses, get_balance};
use crate::{
    app_state::AppState,
    controllers::asserts_controller::{
        create_asset, edit_asset, get_all_assets, get_one_asset, remove_asset,
    },
};

// #[debug_handler(state = PgPool)]
pub async fn create_routes(state: AppState) -> Router<()> {
    let cors = CorsLayer::new()
        .allow_methods([Method::GET, Method::POST])
        .allow_origin(Any);

    let income_router = Router::new()
            .route("/income/create", post(create_income))
            .route("/income/remove/:id", get(remove))
            .route("/income/edit/:id", post(edit))
            .route("/income/all", get(get_all))
            .route("/income/one/:id", get(get_one))
            .route("/income/total", get(get_total_income))
            .route("/income/balance", get(get_balance));

    let expense_router = Router::new()
        .route("/expense/create", post(create_expense))
        .route("/expense/all", get(get_all_expenses))
        .route("/expense/one/:id", get(get_one_expense))
        .route("/expense/remove/:id", get(remove_expense))
        .route("/expense/edit/:id", post(edit_expense))
        .route("/expense/total", get(get_total_expenses));


    let savings_router = Router::new()
        .route("/savings/create", post(create_savings))
        .route("/savings/all", get(get_all_savings))
        .route("/savings/one/:id", get(get_one_saving))
        .route("/savings/edit/:id", post(edit_saving))
        .route("/savings/remove/:id", get(remove_saving));

    let assets_router = Router::new()
        .route("/asset/create", post(create_asset))
        .route("/asset/all", get(get_all_assets))
        .route("/asset/one/:id", get(get_one_asset))
        .route("/asset/edit/:id", post(edit_asset))
        .route("/asset/remove/:id", get(remove_asset));


    Router::new()
        .nest("/api/v1",income_router)
        .nest("/api/v1", expense_router)
        .nest("/api/v1", savings_router)
        .nest("/api/v1", assets_router)

        .layer(cors)
        .with_state(state)
}
