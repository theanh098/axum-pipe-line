use self::io::{AuthPayload, Tokens};
use crate::{
  errors::AppError,
  extract::{
    security::Claims,
    state::{Postgres, Redis, RedisConnection},
  },
  shared::database::{entities::user, repositories::UserRepository},
};
use axum::Json;
use ethers::types::Signature;
use jsonwebtoken::{encode, EncodingKey, Header};

mod io;

pub async fn handler(
  Postgres(pg_conn): Postgres,
  Redis(mut redis_conn): Redis,
  Json(AuthPayload { message, signature }): Json<AuthPayload>,
) -> Result<Json<Tokens>, AppError> {
  let siwe_message = message
    .parse::<siwe::Message>()
    .map_err(|_| AppError::AuthenticationError("Invalid message"))?;

  signature
    .as_str()
    .parse::<Signature>()
    .map_err(|_| AppError::AuthenticationError("Invalid signature"))?;

  let address = siwe::eip55(&siwe_message.address);

  let user = UserRepository::new(&pg_conn).upsert_user(address).await?;

  generate_tokens(&user, &mut redis_conn)
    .await
    .map(|tokens| Json(tokens))
}

async fn generate_tokens(
  user: &user::Model,
  redis_conn: &mut RedisConnection,
) -> Result<Tokens, AppError> {
  let access_secret = std::env::var("JWT_SECRET")?;
  let refresh_secret = std::env::var("JWT_REFRESH_SECRET")?;

  let access_token = encode(
    &Header::default(),
    &Claims::new(user, chrono::Duration::days(3)),
    &EncodingKey::from_secret(access_secret.as_bytes()),
  )?;

  let refresh_token = encode(
    &Header::default(),
    &Claims::new(user, chrono::Duration::days(180)),
    &EncodingKey::from_secret(refresh_secret.as_bytes()),
  )?;

  deadpool_redis::redis::cmd("SET")
    .arg(format!("refresh_token_on_user_{}", user.id).as_str())
    .arg(&refresh_token)
    .query_async(redis_conn)
    .await?;

  Ok(Tokens {
    access_token,
    refresh_token,
  })
}
