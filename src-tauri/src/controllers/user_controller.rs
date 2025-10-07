use axum::{
    body::Body,
    extract::Query,
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};

use crate::controllers::dto::user_dto::*;
use crate::data::repos::user_repo;

pub async fn create_user(Json(user): Json<NewUserDTO>) -> impl IntoResponse {
    match user_repo::create_user(user).await {
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

pub async fn list_users() -> Json<Vec<UserDTO>> {
    let users = match user_repo::get_all_users().await {
        Ok(Some(user_list)) => user_list
            .into_iter()
            .map(|u| UserDTO {
                username: u.username,
                email: u.email,
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

pub async fn get_user(
    Query(params): Query<std::collections::HashMap<String, String>>,
) -> impl IntoResponse {
    // Extract user_id from query params
    let user_id = match params.get("user_id").and_then(|id| id.parse::<i32>().ok()) {
        Some(id) => id,
        None => {
            return Response::builder()
                .status(StatusCode::BAD_REQUEST)
                .header("Content-Type", "application/json")
                .body(Body::from(r#"{"error":"Missing or invalid user id"}"#))
                .unwrap();
        }
    };

    return match user_repo::get_user_by_id(user_id).await {
        Ok(Some(user)) => {
            let user_response = UserDTO {
                username: user.username,
                email: user.email,
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
// TODO: Add update_user and delete_user handlers and add authentication/authorization
