use axum::{
    body::Body,
    extract::Query,
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};

use crate::{data::repos::traits::repository::Repository, services::authentication_service::AuthenticationService};
use crate::{controllers::dto::user_dto::*, data::repos::implementors::user_repo::UserRepo};
// TODO: Test this endpoint
/// Endpoint to register a new user (/register)
/// Password is hashed using Argon2 before storing in the database
pub async fn create_user(Json(user): Json<NewUserDTO>) -> impl IntoResponse {
    let auth = AuthenticationService::new();
    let repo = UserRepo::new().await;

    use crate::data::models::users::NewUser;
    let new_user = NewUser {
        username: &user.username,
        email: &user.email,
        role: user.role.as_deref(),
        password_hash: &auth.hash_and_verify(&user.password).unwrap(),
        created_at: user.created_at.as_deref(),
    };
    
    match repo.add(new_user).await {
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
/// List all users - for testing purposes
pub async fn list_users() -> Json<Vec<UserDTO>> {
    let repo = UserRepo::new().await;
    let users = match repo.get_all().await {
        Ok(Some(user_list)) => user_list
            .into_iter()
            .map(|u| UserDTO {
                username: u.username,
                email: u.email,
                role: u.role,
                created_at: u.created_at,
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
/// For testing purposes, get user by id via query param
pub async fn get_user(
    Query(params): Query<std::collections::HashMap<String, String>>,
) -> impl IntoResponse {
    // Extract user_id from query params
    let username = match params.get("username").and_then(|id| id.parse::<String>().ok()) {
        Some(id) => id,
        None => {
            return Response::builder()
                .status(StatusCode::BAD_REQUEST)
                .header("Content-Type", "application/json")
                .body(Body::from(r#"{"error":"Missing or invalid user id"}"#))
                .unwrap();
        }
    };

    let repo = UserRepo::new().await;
    return match repo.search_by_username(&username).await {
        Ok(Some(user)) => {
            let user_response = UserDTO {
                username: user.username,
                email: user.email,
                role: user.role,
                created_at: user.created_at,
            };
            Response::builder()
                .status(StatusCode::OK)
                .header("Content-Type", "application/json")
                .body(Body::from(serde_json::to_string(&user_response).unwrap()))
                .unwrap()
        }
        Ok(None) => Response::builder()
            .status(StatusCode::NOT_FOUND)
            .header("Content-Type", "application/json")
            .body(Body::from(r#"{"error":"User not found"}"#))
            .unwrap(),
        Err(e) => {
            eprintln!("Error fetching user: {}", e);
            Response::builder()
                .status(StatusCode::INTERNAL_SERVER_ERROR)
                .header("Content-Type", "application/json")
                .body(Body::from(r#"{"error":"Failed to fetch user"}"#))
                .unwrap()
        }
    };
}
