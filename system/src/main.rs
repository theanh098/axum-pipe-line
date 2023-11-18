mod pg_triggers;
mod schedule;
mod watcher;

#[tokio::main]
async fn main() {
    dotenv::dotenv().ok();
    let db_url = std::env::var("DATABASE_URL").expect("db_url must be set");

    let db_con = sea_orm::Database::connect(&db_url).await.unwrap();

    watcher::start(&db_con).await;
    schedule::start(db_con).await;
}
