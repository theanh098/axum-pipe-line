use axum::{
    async_trait,
    extract::FromRequestParts,
    headers::{authorization::Bearer, Authorization},
    http::request::Parts,
    RequestPartsExt, TypedHeader,
};
use chrono::Utc;
use jsonwebtoken::errors::ErrorKind;
use jsonwebtoken::{DecodingKey, Validation};

use crate::{errors::AppError, shared::database::entities::user};
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct Claims {
    pub exp: u32,
    pub id: i32,
    pub address: String,
    pub is_admin: bool,
}

pub struct Guard(pub Claims);

#[async_trait]
impl<S> FromRequestParts<S> for Guard
where
    S: Send + Sync,
{
    type Rejection = AppError;

    async fn from_request_parts(parts: &mut Parts, _state: &S) -> Result<Self, Self::Rejection> {
        let access_secret = std::env::var("JWT_SECRET").expect("JWT_SECRET must be set.");

        let bearer = parts
            .extract::<TypedHeader<Authorization<Bearer>>>()
            .await
            .map_err(|_| AppError::AuthenticationError("Missing Authorization"))?;

        let claims = jsonwebtoken::decode::<Claims>(
            bearer.token(),
            &DecodingKey::from_secret(access_secret.as_bytes()),
            &Validation::default(),
        )
        .map_err(|err| match err.kind() {
            ErrorKind::ExpiredSignature => AppError::AuthenticationError("Expired token"),
            _ => AppError::AuthenticationError("Invalid token"),
        })
        .map(|token_data| token_data.claims)?;

        Ok(Self(claims))
    }
}

impl Claims {
    pub fn new(user: &user::Model, expired: chrono::Duration) -> Self {
        Self {
            address: user.address.to_owned(),
            id: user.id,
            is_admin: user.is_admin,
            exp: Utc::now().checked_add_signed(expired).unwrap().timestamp() as u32,
        }
    }
}
