use axum::extract::{State, Path};
use axum::Json;

use axum::debug_handler;

use serde_json::{Value, json};
use sqlx::{PgPool, FromRow};

use crate::errors::error::CustomError;
use crate::models::income::TotalAmount;
use crate::app_state::AppState;

#[debug_handler(state = PgPool)]
pub async fn get_total_income(
    State(db): State<PgPool>
) -> Result<Json<Value>, CustomError>{

    let res = sqlx::query("SELECT amount FROM income")
        .fetch_all(&db)
        .await?;

    let result = res
        .iter()
        .map(|amount| TotalAmount::from_row(amount).expect("value should be here"))
        .collect::<Vec<_>>();

    let mut sum = 0;

    for amount in result{
        sum += amount.amount;
    }
    
    Ok(Json(json!(sum)))
}

pub async fn get_total_expenses(
    State(db): State<PgPool>
) -> Result<Json<Value>, CustomError>{

    let res = sqlx::query("SELECT amount FROM expenses")
        .fetch_all(&db)
        .await?;

    let result = res
        .iter()
        .map(|amount| TotalAmount::from_row(amount).expect("value should be here"))
        .collect::<Vec<_>>();

    let mut total_expense = 0;

    for expense in result{
        total_expense += expense.amount;
    }

    Ok(Json(json!(total_expense)))
}


pub async fn get_balance(State(db): State<PgPool>) -> Result<Json<Value>, CustomError>{

     let income_res = sqlx::query("SELECT amount FROM income")
        .fetch_all(&db)
        .await?;

    let result = income_res
        .iter()
        .map(|amount| TotalAmount::from_row(amount).expect("value should be here"))
        .collect::<Vec<_>>();

    let mut sum = 0;

    for amount in result{
        sum += amount.amount;
    }
    

    let expense_res = sqlx::query("SELECT amount FROM expenses")
        .fetch_all(&db)
        .await?;

    let result = expense_res
        .iter()
        .map(|amount| TotalAmount::from_row(amount).expect("value should be here"))
        .collect::<Vec<_>>();

    let mut total_expense = 0;

    for expense in result{
        total_expense += expense.amount;
    }

    let balance = sum - total_expense;

    Ok(Json(json!(balance)))
}