use budget::{app_state::AppState, run_app};
use dotenvy::dotenv;
use sqlx::postgres::PgPoolOptions;

#[tokio::main]
async fn main() {
    dotenv().ok();

    let database_url = dotenvy::var("DATABASE_URL").unwrap();
    // println!("{:?}",database_url);

    let conn = PgPoolOptions::new()
        .connect(&database_url)
        .await
        .expect("failed to connect to database");

    let app_state = AppState { db: conn };

    run_app(app_state).await;
}
