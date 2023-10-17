use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Debug, Serialize, Deserialize, FromRow)]
#[serde(rename_all = "camelCase")]
pub struct Expense {
    #[serde(skip)]
    pub id: i32,
    pub name: String,
    pub amount: i64,
    #[serde(skip)]
    pub done: bool,
}

#[derive(Debug, Serialize, Deserialize, FromRow)]
#[serde(rename_all = "camelCase")]
pub struct ExpenseResponse {
    pub id: i32,
    pub name: String,
    pub amount: i64,
    pub done: bool,
}

#[derive(Debug, Serialize, Deserialize, FromRow)]
#[serde(rename_all = "camelCase")]
pub struct UpdateExpense {
    pub name: String,
    pub amount: i64,
    pub done: bool,
}
