use axum::{
    body::Body,
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct User {
    pub username: String,
    pub email: String,
    pub password_hash: String,
    pub created_at: Option<String>,
}
// TODO implement actual user creation in database
pub async fn create_user() -> impl IntoResponse {
    Response::builder()
        .status(StatusCode::CREATED)
        .body(Body::from("User created"))
        .unwrap()
}
// TODO implement actual user listing from database
pub async fn list_users() -> Json<Vec<User>> {
    Json(vec![
        User {
            username: "user1".into(),
            email: "user1@example.com".into(),
            password_hash: "hashed_password1".into(),
            created_at: Some("2023-01-01T00:00:00Z".into()),
        },
        User {
            username: "user2".into(),
            email: "user2@example.com".into(),
            password_hash: "hashed_password2".into(),
            created_at: Some("2023-01-02T00:00:00Z".into()),
        },
    ])
}
