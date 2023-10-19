use axum::{
  extract::State,
  routing::{get, post},
  Router,
};

pub mod errors;
pub mod shared;

mod extract;
mod handlers;

use extract::state::AppState;

#[tokio::main]
pub async fn start() {
  dotenv::dotenv().ok();
  let db_url = std::env::var("DATABASE_URL").expect("db_url must be set");

  let app = Router::new()
    .route("/", get(root))
    .route("/auth", post(handlers::sign_in))
    .with_state(AppState::new(&db_url).await.unwrap());

  axum::Server::bind(&"0.0.0.0:8080".parse().unwrap())
    .serve(app.into_make_service())
    .await
    .unwrap();
}

async fn root(_state: State<AppState>) -> &'static str {
  dbg!("what's up");
  "Hello Kitty"
}
