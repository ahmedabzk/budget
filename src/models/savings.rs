use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Debug, Serialize, Deserialize, FromRow)]
#[serde(rename_all = "camelCase")]
pub struct Savings {
    #[serde(skip)]
    pub id: i32,
    pub name: String,
    pub value: i64,
    pub yld: i64,
}

#[derive(Debug, Serialize, Deserialize, FromRow)]
#[serde(rename_all = "camelCase")]
pub struct SavingsRs {
    pub id: i32,
    pub name: String,
    pub value: i64,
    pub yld: i64,
}

#[derive(Debug, Serialize, Deserialize, FromRow)]
#[serde(rename_all = "camelCase")]
pub struct UpdateSavings {
    pub name: String,
    pub value: i64,
    pub yld: i64,
}
