use budget::run_app;
use dotenvy::dotenv;

#[tokio::main]
async fn main() {
    dotenv().ok();

    let database_url = dotenvy::var("DATABASE_URL").unwrap();
    // println!("{:?}",database_url);

    run_app(&database_url).await;
}
