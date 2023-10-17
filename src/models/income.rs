use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, sqlx::FromRow)]
#[serde(rename_all = "camelCase")]
pub struct Income {
    #[serde(skip)]
    pub id: i32,
    pub name: String,
    pub amount: i64,
    #[serde(skip)]
    pub done: bool,
}

#[derive(Debug, Serialize, Deserialize, sqlx::FromRow)]
#[serde(rename_all = "camelCase")]
pub struct UpdateIncome {
    pub name: String,
    pub amount: i64,
    #[serde(skip)]
    pub done: bool,
}

#[derive(Debug, Serialize, Deserialize, sqlx::FromRow)]
#[serde(rename_all = "camelCase")]
pub struct IncomeRs {
    pub id: i32,
    pub name: String,
    pub amount: i64,
    pub done: bool,
}
