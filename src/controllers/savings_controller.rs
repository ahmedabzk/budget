use axum::debug_handler;
use axum::extract::{State,Path};
use axum::Json;
use serde_json::{json, Value};
use sqlx::PgPool;

use crate::models::savings::UpdateSavings;
use crate::{
    errors::error::CustomError,
    models::savings::{Savings, SavingsRs},
};

#[debug_handler(state = PgPool)]
pub async fn create_savings(
    State(db): State<PgPool>,
    Json(attributes): Json<Savings>,
) -> Result<Json<Value>, CustomError> {
    sqlx::query("INSERT INTO savings (name, value, yld) VALUES ($1, $2, $3)")
        .bind(attributes.name)
        .bind(attributes.value)
        .bind(attributes.yld)
        .execute(&db)
        .await?;

    Ok(Json(json!("created savings successfully")))
}

#[debug_handler(state = PgPool)]
pub async fn get_all_savings(State(db): State<PgPool>) -> Result<Json<Value>, CustomError> {
    let res = sqlx::query_as::<_, SavingsRs>("SELECT * FROM savings")
        .fetch_all(&db)
        .await?;

    Ok(Json(json!(res)))
}

#[debug_handler(state = PgPool)]
pub async fn get_one_saving(
    State(db): State<PgPool>,
    Path(id): Path<i32>,
) -> Result<Json<Value>, CustomError> {
    let res = sqlx::query_as::<_, SavingsRs>("SELECT * FROM savings WHERE id = $1")
        .bind(id)
        .fetch_one(&db)
        .await?;

    Ok(Json(json!(res)))
}

#[debug_handler(state = PgPool)]
pub async fn remove_saving(
    State(db): State<PgPool>,
    Path(id): Path<i32>,
) -> Result<Json<Value>, CustomError> {
    sqlx::query("DELETE FROM savings WHERE id = $1")
        .bind(id)
        .execute(&db)
        .await?;

    Ok(Json(json!("Deleted successfully")))
}

#[debug_handler(state = PgPool)]
pub async fn edit_saving(
    State(db): State<PgPool>,
    Path(id): Path<i32>,
    Json(data): Json<UpdateSavings>,
) -> Result<Json<Value>, CustomError> {
    let res = sqlx::query("SELECT * FROM savings WHERE id = $1")
        .bind(id)
        .fetch_optional(&db)
        .await?;

    if res.is_some() {
        sqlx::query("UPDATE savings SET name=$1, value=$2, yld=$3 WHERE id = $4")
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
