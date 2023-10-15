use axum::{routing::get, Router};

mod handlers;
mod shared;

pub async fn bootstrap() {
  let app = Router::new().route("/", get(root));

  axum::Server::bind(&"0.0.0.0:8080".parse().unwrap())
    .serve(app.into_make_service())
    .await
    .unwrap();
}

async fn root() -> &'static str {
  dbg!("what's up");
  "Hello Kitty"
}
