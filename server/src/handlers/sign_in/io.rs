use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
pub struct AuthPayload {
  pub message: String,
  pub signature: String,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Tokens {
  pub access_token: String,
  pub refresh_token: String,
}
