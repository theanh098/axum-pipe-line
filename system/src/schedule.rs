mod inactive_nfts;

use sea_orm::DatabaseConnection;
use system::Scheduler;

use inactive_nfts::inactive_nfts;

pub async fn start(db_conn: DatabaseConnection) {
  Scheduler::new()
    .set_context(db_conn)
    .job("* * * * * *", &|db| {
      Box::pin(async move {
        println!("Every second!");
        inactive_nfts(&db).await.unwrap();
      })
    })
    .start()
    .await
    .unwrap_or_else(|err| {
      eprintln!("An error occurred when starting jobs with reason: {}", err);
    });
}
