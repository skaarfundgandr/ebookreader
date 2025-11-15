use crate::{
    data::models::annotations::{NewAnnotation, UpdateAnnotation},
    data::repos::implementors::annotation_repo::AnnotationRepo,
    data::repos::traits::repository::Repository,
    services::token_service::Tokenizer,
};
use axum::{
    extract::{Path, Query},
    http::{HeaderMap, StatusCode},
    response::IntoResponse,
    Json,
};
use chrono::Utc;
use serde::Deserialize;

use super::dto::annotation_dto::{AnnotationDTO, NewAnnotationDTO, UpdateAnnotationDTO};

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

pub async fn create_annotation(
    headers: HeaderMap,
    Json(payload): Json<NewAnnotationDTO>,
) -> impl IntoResponse {
    let user_id = match get_user_from_token(&headers).await {
        Ok(id) => id,
        Err(status) => return (status, "Unauthorized").into_response(),
    };

    let repo = AnnotationRepo::new().await;
    let new_annotation = NewAnnotation {
        user_id,
        book_id: payload.book_id,
        chapter_title: payload.chapter_title.as_deref(),
        start_position: &payload.start_position,
        end_position: &payload.end_position,
        highlighted_text: payload.highlighted_text.as_deref(),
        note: payload.note.as_deref(),
        color: payload.color.as_deref(),
    };

    match repo.add(new_annotation).await {
        Ok(_) => (StatusCode::CREATED, "Annotation created").into_response(),
        Err(_) => (StatusCode::INTERNAL_SERVER_ERROR, "Database error").into_response(),
    }
}

pub async fn get_annotations(
    headers: HeaderMap,
    Query(params): Query<BookQueryParams>,
) -> impl IntoResponse {
    let user_id = match get_user_from_token(&headers).await {
        Ok(id) => id,
        Err(status) => return (status, Json(Vec::<AnnotationDTO>::new())).into_response(),
    };

    let repo = AnnotationRepo::new().await;
    match repo.get_by_user_and_book(user_id, params.book_id).await {
        Ok(Some(annotations)) => {
            let dtos: Vec<AnnotationDTO> = annotations
                .into_iter()
                .map(|a| AnnotationDTO {
                    annotation_id: a.annotation_id,
                    user_id: a.user_id,
                    book_id: a.book_id,
                    chapter_title: a.chapter_title,
                    start_position: a.start_position,
                    end_position: a.end_position,
                    highlighted_text: a.highlighted_text,
                    note: a.note,
                    color: a.color,
                    created_at: a.created_at,
                    updated_at: a.updated_at,
                })
                .collect();
            (StatusCode::OK, Json(dtos)).into_response()
        }
        Ok(None) => (StatusCode::OK, Json(Vec::<AnnotationDTO>::new())).into_response(),
        Err(_) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(Vec::<AnnotationDTO>::new()),
        )
            .into_response(),
    }
}

pub async fn update_annotation(
    headers: HeaderMap,
    Path(annotation_id): Path<i32>,
    Json(payload): Json<UpdateAnnotationDTO>,
) -> impl IntoResponse {
    let _user_id = match get_user_from_token(&headers).await {
        Ok(id) => id,
        Err(status) => return (status, "Unauthorized").into_response(),
    };

    let repo = AnnotationRepo::new().await;
    let now = Utc::now().format("%Y-%m-%d %H:%M:%S").to_string();
    let update = UpdateAnnotation {
        chapter_title: payload.chapter_title.as_deref(),
        start_position: payload.start_position.as_deref(),
        end_position: payload.end_position.as_deref(),
        highlighted_text: payload.highlighted_text.as_deref(),
        note: payload.note.as_deref(),
        color: payload.color.as_deref(),
        updated_at: Some(&now),
    };

    match repo.update(annotation_id, update).await {
        Ok(_) => (StatusCode::OK, "Annotation updated").into_response(),
        Err(_) => (StatusCode::INTERNAL_SERVER_ERROR, "Database error").into_response(),
    }
}

pub async fn delete_annotation(
    headers: HeaderMap,
    Path(annotation_id): Path<i32>,
) -> impl IntoResponse {
    let _user_id = match get_user_from_token(&headers).await {
        Ok(id) => id,
        Err(status) => return (status, "Unauthorized").into_response(),
    };

    let repo = AnnotationRepo::new().await;
    match repo.delete(annotation_id).await {
        Ok(_) => (StatusCode::OK, "Annotation deleted").into_response(),
        Err(_) => (StatusCode::INTERNAL_SERVER_ERROR, "Database error").into_response(),
    }
}
