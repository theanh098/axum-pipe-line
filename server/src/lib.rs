use axum::{routing::get, Router};

mod errors;
mod extract;
mod handlers;
mod shared;

use extract::state::AppState;

#[tokio::main]
pub async fn start() {
  let app = Router::new()
    .route("/", get(root))
    .with_state(AppState::new("afs").await.unwrap());

  axum::Server::bind(&"0.0.0.0:8080".parse().unwrap())
    .serve(app.into_make_service())
    .await
    .unwrap();
}

async fn root() -> &'static str {
  dbg!("what's up");
  "Hello Kitty"
}
