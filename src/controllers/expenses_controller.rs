use axum::debug_handler;
use axum::{extract::Path, Extension, Json};
use serde_json::{json, Value};
use sqlx::PgPool;

use crate::models::expenses::UpdateExpense;
use crate::{
    errors::error::CustomError,
    models::expenses::{Expense, ExpenseResponse},
};

#[debug_handler(state = PgPool)]
pub async fn create_expense(
    Extension(db): Extension<PgPool>,
    Json(attributes): Json<Expense>,
) -> Result<Json<Value>, CustomError> {
    sqlx::query("INSERT INTO expenses (name,amount) VALUES ($1, $2)")
        .bind(attributes.name)
        .bind(attributes.amount)
        .execute(&db)
        .await?;

    Ok(Json(json!("expense created successfully")))
}

#[debug_handler(state = PgPool)]
pub async fn get_all_expenses(
    Extension(db): Extension<PgPool>,
) -> Result<Json<Value>, CustomError> {
    let res = sqlx::query_as::<_, ExpenseResponse>("SELECT * FROM expenses")
        .fetch_all(&db)
        .await?;

    Ok(Json(json!(res)))
}

#[debug_handler(state = PgPool)]
pub async fn get_one_expense(
    Extension(db): Extension<PgPool>,
    Path(id): Path<i32>,
) -> Result<Json<Value>, CustomError> {
    let res = sqlx::query_as::<_, ExpenseResponse>("SELECT * FROM expenses WHERE id = $1")
        .bind(id)
        .fetch_one(&db)
        .await?;

    Ok(Json(json!(res)))
}

#[debug_handler(state = PgPool)]
pub async fn remove_expense(
    Extension(db): Extension<PgPool>,
    Path(id): Path<i32>,
) -> Result<Json<Value>, CustomError> {
    sqlx::query("DELETE FROM expenses WHERE id = $1")
        .bind(id)
        .execute(&db)
        .await?;

    Ok(Json(json!("deleted expense successfully")))
}

pub async fn edit_expense(
    Extension(db): Extension<PgPool>,
    Path(id): Path<i32>,
    Json(attributes): Json<UpdateExpense>,
) -> Result<String, CustomError> {
    let res = sqlx::query("SELECT * FROM expenses WHERE id = $1")
        .bind(id)
        .fetch_optional(&db)
        .await?;

    if res.is_some() {
        sqlx::query("UPDATE expenses SET name=$1, amount=$2, done=$3 WHERE id = $4")
            .bind(attributes.name)
            .bind(attributes.amount)
            .bind(attributes.done)
            .bind(id)
            .execute(&db)
            .await?;
        Ok("updated successfully".to_string())
    } else {
        Ok("No such id was found in the database".to_string())
    }
}
