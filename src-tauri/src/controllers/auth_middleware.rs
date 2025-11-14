use axum::{
    extract::FromRequestParts,
    http::{request::Parts, StatusCode},
    response::{IntoResponse, Response},
    RequestPartsExt,
};
use axum_extra::{
    headers::{authorization::Bearer, Authorization},
    TypedHeader,
};
use serde::{Deserialize, Serialize};

use crate::services::token_service;

#[derive(Debug, Serialize, Deserialize)]
pub struct AuthUser {
    pub id: i32,
}

impl<S> FromRequestParts<S> for AuthUser
where
    S: Send + Sync,
{
    type Rejection = AuthError;

    async fn from_request_parts(parts: &mut Parts, _state: &S) -> Result<Self, Self::Rejection> {
        let TypedHeader(Authorization(bearer)) = parts
            .extract::<TypedHeader<Authorization<Bearer>>>()
            .await
            .map_err(|_| AuthError::InvalidToken)?;

        let tokenizer = token_service::Tokenizer::get_instance().await;
        let claims = tokenizer
            .decode_token(bearer.token())
            .map_err(|_| AuthError::InvalidToken)?;

        Ok(AuthUser { id: claims.sub })
    }
}

pub enum AuthError {
    InvalidToken,
}

impl IntoResponse for AuthError {
    fn into_response(self) -> Response {
        let (status, error_message) = match self {
            AuthError::InvalidToken => (StatusCode::UNAUTHORIZED, "Invalid or missing token"),
        };
        (status, error_message).into_response()
    }
}
