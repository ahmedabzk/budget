use axum::debug_handler;
use axum::extract::State;
use axum::{extract::Path, Json};
use serde_json::{json, Value};
use sqlx::PgPool;

use crate::app_state::AppState;
use crate::{
    errors::error::CustomError,
    models::asserts::{Asset, AssetRs, UpdateAsset},
};

#[debug_handler(state = PgPool)]
pub async fn create_asset(
    State(db): State<PgPool>,
    Json(data): Json<Asset>,
) -> Result<Json<Value>, CustomError> {
    sqlx::query("INSERT INTO assets (name,value,yld) VALUES ($1, $2, $3)")
        .bind(data.name)
        .bind(data.value)
        .bind(data.yld)
        .execute(&db)
        .await?;

    Ok(Json(json!("assert created successfully")))
}

#[debug_handler(state = PgPool)]
pub async fn get_all_assets(State(db): State<PgPool>) -> Result<Json<Value>, CustomError> {
    let res = sqlx::query_as::<_, AssetRs>("SELECT * FROM assets")
        .fetch_all(&db)
        .await?;

    Ok(Json(json!(res)))
}

#[debug_handler(state = PgPool)]
pub async fn get_one_asset(
    State(db): State<PgPool>,
    Path(id): Path<i32>,
) -> Result<Json<Value>, CustomError> {
    let res = sqlx::query_as::<_, AssetRs>("SELECT * FROM assets WHERE id = $1")
        .bind(id)
        .fetch_one(&db)
        .await?;

    Ok(Json(json!(res)))
}

#[debug_handler(state = PgPool)]
pub async fn remove_asset(
    State(db): State<PgPool>,
    Path(id): Path<i32>,
) -> Result<Json<Value>, CustomError> {
    sqlx::query("DELETE FROM assets WHERE id = $1")
        .bind(id)
        .execute(&db)
        .await?;

    Ok(Json(json!("Deleted successfully")))
}

#[debug_handler(state = AppState)]
pub async fn edit_asset(
    State(db): State<PgPool>,
    Path(id): Path<i32>,
    Json(data): Json<UpdateAsset>,
) -> Result<Json<Value>, CustomError> {
    let res = sqlx::query("SELECT * FROM assets WHERE id = $1")
        .bind(id)
        .fetch_optional(&db)
        .await?;

    if res.is_some() {
        sqlx::query("UPDATE assets SET name=$1, value=$2, yld=$3 WHERE id=$4")
            .bind(data.name)
            .bind(data.value)
            .bind(data.yld)
            .bind(id)
            .execute(&db)
            .await?;

        Ok(Json(json!("Updated successfully")))
    } else {
        Ok(Json(json!("No element with such id in the database")))
    }
}
