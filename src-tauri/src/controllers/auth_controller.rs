use crate::{
    data::repos::implementors::user_repo::UserRepo,
    services::{authentication_service::AuthenticationService, token_service},
};
use axum::{http::StatusCode, response::IntoResponse, Json};
use serde::Serialize;

use super::dto::login_dto::LoginDTO;

#[derive(Serialize)]
struct TokenResponse {
    token: String,
}

pub async fn login(Json(payload): Json<LoginDTO>) -> impl IntoResponse {
    let user_repo = UserRepo::new().await;
    let auth_service = AuthenticationService::new();

    match user_repo.search_by_username(&payload.username).await {
        Ok(Some(mut users)) => {
            if let Some(user) = users.pop() {
                match auth_service.verify_password(&payload.password, &user.password_hash) {
                    Ok(true) => {
                        let tokenizer = token_service::Tokenizer::get_instance().await;
                        match tokenizer.generate_token(user.user_id) {
                            Ok(token) => {
                                (StatusCode::OK, Json(TokenResponse { token })).into_response()
                            }
                            Err(_) => (
                                StatusCode::INTERNAL_SERVER_ERROR,
                                "Failed to generate token",
                            )
                                .into_response(),
                        }
                    }
                    Ok(false) => (StatusCode::UNAUTHORIZED, "Invalid credentials").into_response(),
                    Err(_) => (
                        StatusCode::INTERNAL_SERVER_ERROR,
                        "Password verification failed",
                    )
                        .into_response(),
                }
            } else {
                (StatusCode::UNAUTHORIZED, "Invalid credentials").into_response()
            }
        }
        Ok(None) => (StatusCode::UNAUTHORIZED, "Invalid credentials").into_response(),
        Err(_) => (StatusCode::INTERNAL_SERVER_ERROR, "Database error").into_response(),
    }
}

// TODO: Implement logout logic here
pub fn logout() {
    todo!()
}
