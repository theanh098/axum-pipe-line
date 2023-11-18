use std::env::VarError;

use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use sea_orm::DbErr;

#[derive(Debug)]
pub enum AppError {
    ExecutionError(ErrorTag, anyhow::Error),
    RecordNotFoundError {
        table: &'static str,
        col: &'static str,
        value: &'static str,
    },
    AuthenticationError(&'static str),
    SurfError(String),
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        use AppError::*;
        match self {
            ExecutionError(tag, err) => {
                (StatusCode::INTERNAL_SERVER_ERROR, specific_err(tag, err)).into_response()
            }

            RecordNotFoundError { table, col, value } => (
                StatusCode::INTERNAL_SERVER_ERROR,
                specific_err(
                    "RecordNotFoundError",
                    format!(
                        "Not found record on table {} with {} is {}",
                        table, col, value
                    ),
                ),
            )
                .into_response(),

            AuthenticationError(reason) => (
                StatusCode::UNAUTHORIZED,
                specific_err("UnAuthorized", reason),
            )
                .into_response(),

            SurfError(reason) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                specific_err("SurfError", reason),
            )
                .into_response(),
        }
    }
}

impl<E> From<E> for AppError
where
    E: Into<anyhow::Error>,
{
    fn from(err: E) -> Self {
        let e: anyhow::Error = err.into();

        if e.is::<DbErr>() {
            Self::ExecutionError(ErrorTag::DatabaseQueryError, e)
        } else if e.is::<VarError>() {
            Self::ExecutionError(ErrorTag::MissingEnvironment, e)
        } else {
            Self::ExecutionError(ErrorTag::UnSpecificError, e)
        }
    }
}

#[derive(thiserror::Error, Debug)]
pub enum ErrorTag {
    #[error("DatabaseQueryError")]
    DatabaseQueryError,

    #[error("UnSpecificError")]
    UnSpecificError,

    #[error("MissingEnvironment")]
    MissingEnvironment,
}

fn specific_err(tag: impl ToString, reason: impl ToString) -> Json<serde_json::Value> {
    Json(serde_json::json!({
        "tag": tag.to_string(),
        "description": reason.to_string()
    }))
}
