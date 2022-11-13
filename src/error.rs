use axum::{
    extract::Json,
    http::StatusCode,
    response::{IntoResponse, Response},
};
use serde::{Deserialize, Serialize};
use serde_json::json;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error, Serialize, Deserialize)]
pub enum Error {
    #[error("'{uri}' is not a valid uri")]
    InvalidUri { uri: String },

    #[error("webfinger resource should begin with 'acct:', got '{resource}'")]
    MalformedWebfingerResource { resource: String },

    #[error("webfinger uri should be of the form 'account@domain', got '{uri}'")]
    MalformedWebfingerUri { uri: String },

    #[error("missing signature")]
    MissingSignature,

    #[error("record not found")]
    StatusNotFound { id: String },

    #[error("user not found")]
    UnknownUser { user: String },
}

impl IntoResponse for Error {
    fn into_response(self) -> Response {
        use Error::*;

        let error = self.to_string();

        let (status, data) = match self {
            InvalidUri { uri } => (
                StatusCode::UNAUTHORIZED,
                Json(json!({ "error": error, "uri": uri })),
            ),

            MalformedWebfingerResource { resource } => (
                StatusCode::BAD_REQUEST,
                Json(json!({
                    "error": error,
                    "resource": resource,
                })),
            ),

            MalformedWebfingerUri { uri } => (
                StatusCode::BAD_REQUEST,
                Json(json!({
                    "error": error,
                    "uri": uri,
                })),
            ),

            MissingSignature => (StatusCode::UNAUTHORIZED, Json(json!({ "error": error }))),

            StatusNotFound { id } => (
                StatusCode::NOT_FOUND,
                Json(json!({ "error": error, "id": id })),
            ),

            UnknownUser { user } => (
                StatusCode::NOT_FOUND,
                Json(json!({ "error": error, "user": user })),
            ),
        };

        (status, data).into_response()
    }
}
