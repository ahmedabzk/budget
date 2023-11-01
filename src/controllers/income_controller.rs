use axum::debug_handler;
use axum::extract::{Path, State};
use axum::Json;
use serde_json::{json, Value};
use sqlx::{PgPool, Row};

use crate::errors::error::CustomError;
use crate::models::income::{Income, IncomeRs, UpdateIncome};
use crate::app_state::AppState;

#[debug_handler(state = PgPool)]
pub async fn create_income(
    State(db): State<PgPool>,
    Json(income): Json<Income>,
) -> Result<Json<Value>, CustomError> {

    let qry = sqlx::query("SELECT name FROM income WHERE name= $1")
        .bind(&income.name)
        .fetch_optional(&db)
        .await?;

    if qry.is_some(){
        Ok(Json(json!("income already exists")))
    }else{
        sqlx::query("INSERT INTO income (name, amount) VALUES ($1,$2)")
            .bind(income.name)
            .bind(income.amount)
            .execute(&db)
            .await?;

        Ok(Json(json!("added successfully")))
    }

   
}

#[debug_handler(state = PgPool)]
pub async fn edit(
    Path(id): Path<i32>,
    State(db): State<PgPool>,
    Json(data): Json<UpdateIncome>,
) -> Result<String, CustomError> {
    let query = sqlx::query("SELECT * FROM income WHERE id=$1")
        .bind(id)
        .fetch_optional(&db)
        .await?;

    if query.is_some() {
        sqlx::query("UPDATE income SET name=$2,amount=$3,done=$4  WHERE id = $1")
            .bind(id)
            .bind(data.name)
            .bind(data.amount)
            .bind(data.done)
            .execute(&db)
            .await?;

        Ok("updated successfully".to_string())
    } else {
        Ok("No such id was found in the database".to_string())
    }
}

#[debug_handler(state = PgPool)]
pub async fn remove(
    State(db): State<PgPool>,
    Path(id): Path<i32>,
) -> Result<Json<Value>, CustomError> {
    sqlx::query("DELETE FROM income WHERE id = $1")
        .bind(id)
        .execute(&db)
        .await?;

    Ok(Json(json!("deleted successfully")))
}

#[debug_handler (state= PgPool)]
pub async fn get_all(State(db): State<PgPool>) -> Result<Json<Value>, CustomError> {
    let res = sqlx::query_as::<_, IncomeRs>("SELECT * FROM income")
        .fetch_all(&db)
        .await?;
    // .iter()
    // .map(|income| IncomeRs::from_row(income).expect("value should be here"))
    // .collect::<Vec<_>>();

    Ok(Json(json!(res)))
}

#[debug_handler(state = PgPool)]
pub async fn get_one(
    State(db): State<PgPool>,
    Path(id): Path<i32>,
) -> Result<Json<Value>, CustomError> {
    let res = sqlx::query_as::<_, IncomeRs>("SELECT * FROM income WHERE id = $1")
        .bind(id)
        .fetch_one(&db)
        .await?;

    Ok(Json(json!(res)))
}
