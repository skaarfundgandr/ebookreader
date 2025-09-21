use axum::{
    body::Body,
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use serde::{Deserialize, Serialize};

use crate::data::models::users::NewUser;

#[derive(Serialize, Deserialize)]
pub struct User {
    username: String,
    email: String,
    password_hash: String,
    created_at: Option<String>,
}

pub async fn create_user(Json(user): Json<User>) -> impl IntoResponse {
    let new_user = NewUser {
        username: &user.username,
        email: &user.email,
        password_hash: &user.password_hash,
        created_at: user.created_at.as_deref(),
    };

    match crate::data::repos::user_repo::create_user(new_user).await {
        Ok(_) => (),
        Err(e) => {
            eprintln!("Error creating user: {}", e);
            return Response::builder()
                .status(StatusCode::INTERNAL_SERVER_ERROR)
                .body(Body::from("Failed to create user"))
                .unwrap();
        }
    }

    return Response::builder()
        .status(StatusCode::CREATED)
        .body(Body::from("User created"))
        .unwrap();
}

pub async fn list_users() -> Json<Vec<User>> {
    let users = match crate::data::repos::user_repo::get_all_users().await {
        Ok(Some(user_list)) => user_list
            .into_iter()
            .map(|u| User {
                username: u.username,
                email: u.email,
                password_hash: u.password_hash,
                created_at: Some(u.created_at),
            })
            .collect(),
        Ok(None) => vec![],
        Err(e) => {
            eprintln!("Error fetching users: {}", e);
            vec![]
        }
    };

    return Json(users);
}
// TODO: Add more user-related handlers (e.g., get_user, update_user, delete_user)
// and implement authentication and authorization as needed.