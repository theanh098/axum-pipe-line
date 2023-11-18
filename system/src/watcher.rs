mod compute_block;
mod update_position;

use crate::pg_triggers;
use sea_orm::DatabaseConnection;
use serde::de::DeserializeOwned;
use sqlx::postgres::PgListener;
use std::collections::HashMap;
use std::fmt::Debug;
use std::future::Future;

// P : Payload pass to callback
// F : Callback
async fn listen_postgres_trigger<'a, P, F, Fut>(
    db: &'a DatabaseConnection,
    workers: HashMap<&'a str, F>,
) -> Result<(), sqlx::Error>
where
    P: DeserializeOwned + Sized + Debug,
    Fut: Future<Output = ()> + 'a,
    F: Fn(P, &'a DatabaseConnection) -> Fut,
{
    let pool = db.get_postgres_connection_pool();
    let mut listener = PgListener::connect_with(pool).await.unwrap();

    let channels: Vec<&str> = workers.keys().into_iter().map(|key| *key).collect();
    listener.listen_all(channels).await?;

    loop {
        while let Some(notification) = listener.try_recv().await? {
            let chanel = notification.channel();
            let call_back = workers.get(chanel).unwrap();

            let payload_string = notification.payload().to_owned();
            let payload = serde_json::from_str::<P>(&payload_string).unwrap();

            call_back(payload, db).await;
        }
    }
}

pub async fn start(db_con: &DatabaseConnection) {
    pg_triggers::nfts_change::create_nfts_change_event(db_con)
        .await
        .unwrap();

    let mut workers = HashMap::new();

    workers.insert("nfts_change", |payload: serde_json::Value, db| async move {
        println!("payload from pg_notify: {}", payload);

        update_position::update_nfts_position(db)
            .await
            .unwrap_or_else(|e| eprint!("update_nfts_position  fail with err {}", e));

        compute_block::generate_9_block_images(db).await;
    });

    listen_postgres_trigger(db_con, workers)
        .await
        .unwrap_or_else(|e| eprint!("listen_postgres_trigger  fail with err {}", e));
}
