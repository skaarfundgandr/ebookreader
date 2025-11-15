use crate::{
    data::models::bookmarks::NewBookmark, data::repos::implementors::bookmark_repo::BookmarkRepo,
    data::repos::traits::repository::Repository, services::token_service::Tokenizer,
};
use axum::{
    extract::{Path, Query},
    http::{HeaderMap, StatusCode},
    response::IntoResponse,
    Json,
};
use serde::Deserialize;

use super::dto::bookmark_dto::{BookmarkDTO, NewBookmarkDTO};

#[derive(Deserialize)]
pub struct BookQueryParams {
    book_id: i32,
}

async fn get_user_from_token(headers: &HeaderMap) -> Result<i32, StatusCode> {
    let token = headers
        .get("Authorization")
        .and_then(|v| v.to_str().ok())
        .and_then(|v| v.strip_prefix("Bearer "))
        .ok_or(StatusCode::UNAUTHORIZED)?;

    let tokenizer = Tokenizer::get_instance().await;
    let claims = tokenizer
        .decode_token(token)
        .map_err(|_| StatusCode::UNAUTHORIZED)?;

    Ok(claims.sub)
}

pub async fn create_bookmark(
    headers: HeaderMap,
    Json(payload): Json<NewBookmarkDTO>,
) -> impl IntoResponse {
    let user_id = match get_user_from_token(&headers).await {
        Ok(id) => id,
        Err(status) => return (status, "Unauthorized").into_response(),
    };

    let repo = BookmarkRepo::new().await;
    let new_bookmark = NewBookmark {
        user_id,
        book_id: payload.book_id,
        chapter_title: payload.chapter_title.as_deref(),
        page_number: payload.page_number,
        position: &payload.position,
    };

    match repo.add(new_bookmark).await {
        Ok(_) => (StatusCode::CREATED, "Bookmark created").into_response(),
        Err(_) => (StatusCode::INTERNAL_SERVER_ERROR, "Database error").into_response(),
    }
}

pub async fn get_bookmarks(
    headers: HeaderMap,
    Query(params): Query<BookQueryParams>,
) -> impl IntoResponse {
    let user_id = match get_user_from_token(&headers).await {
        Ok(id) => id,
        Err(status) => return (status, Json(Vec::<BookmarkDTO>::new())).into_response(),
    };

    let repo = BookmarkRepo::new().await;
    match repo.get_by_user_and_book(user_id, params.book_id).await {
        Ok(Some(bookmarks)) => {
            let dtos: Vec<BookmarkDTO> = bookmarks
                .into_iter()
                .map(|b| BookmarkDTO {
                    bookmark_id: b.bookmark_id,
                    user_id: b.user_id,
                    book_id: b.book_id,
                    chapter_title: b.chapter_title,
                    page_number: b.page_number,
                    position: b.position,
                    created_at: b.created_at,
                })
                .collect();
            (StatusCode::OK, Json(dtos)).into_response()
        }
        Ok(None) => (StatusCode::OK, Json(Vec::<BookmarkDTO>::new())).into_response(),
        Err(_) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(Vec::<BookmarkDTO>::new()),
        )
            .into_response(),
    }
}

pub async fn delete_bookmark(
    headers: HeaderMap,
    Path(bookmark_id): Path<i32>,
) -> impl IntoResponse {
    let _user_id = match get_user_from_token(&headers).await {
        Ok(id) => id,
        Err(status) => return (status, "Unauthorized").into_response(),
    };

    let repo = BookmarkRepo::new().await;
    match repo.delete(bookmark_id).await {
        Ok(_) => (StatusCode::OK, "Bookmark deleted").into_response(),
        Err(_) => (StatusCode::INTERNAL_SERVER_ERROR, "Database error").into_response(),
    }
}
