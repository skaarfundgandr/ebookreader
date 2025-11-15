use crate::{
    data::repos::implementors::{author_repo::AuthorRepo, book_repo::BookRepo},
    data::repos::traits::repository::Repository,
};
use axum::{extract::Query, http::StatusCode, response::IntoResponse, Json};
use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
pub struct SearchQuery {
    pub q: Option<String>,
    pub title: Option<String>,
    pub author: Option<String>,
    pub publisher: Option<String>,
    pub isbn: Option<String>,
}

#[derive(Serialize)]
pub struct SearchBookDTO {
    pub book_id: i32,
    pub title: String,
    pub published_date: Option<String>,
    pub isbn: Option<String>,
    pub file_type: Option<String>,
    pub file_path: Option<String>,
    pub cover_image_path: Option<String>,
}

pub async fn search_books(Query(params): Query<SearchQuery>) -> impl IntoResponse {
    let book_repo = BookRepo::new().await;

    // If general query is provided, search across all fields
    if let Some(query) = params.q {
        match book_repo.search_by_title(&query).await {
            Ok(Some(books)) => {
                let dtos: Vec<SearchBookDTO> = books
                    .into_iter()
                    .map(|b| SearchBookDTO {
                        book_id: b.book_id,
                        title: b.title,
                        published_date: b.published_date,
                        isbn: b.isbn,
                        file_type: b.file_type,
                        file_path: b.file_path,
                        cover_image_path: b.cover_image_path,
                    })
                    .collect();
                return (StatusCode::OK, Json(dtos)).into_response();
            }
            Ok(None) => return (StatusCode::OK, Json(Vec::<SearchBookDTO>::new())).into_response(),
            Err(_) => {
                return (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    Json(Vec::<SearchBookDTO>::new()),
                )
                    .into_response()
            }
        }
    }

    // Search by title
    if let Some(title) = params.title {
        match book_repo.search_by_title(&title).await {
            Ok(Some(books)) => {
                let dtos: Vec<SearchBookDTO> = books
                    .into_iter()
                    .map(|b| SearchBookDTO {
                        book_id: b.book_id,
                        title: b.title,
                        published_date: b.published_date,
                        isbn: b.isbn,
                        file_type: b.file_type,
                        file_path: b.file_path,
                        cover_image_path: b.cover_image_path,
                    })
                    .collect();
                return (StatusCode::OK, Json(dtos)).into_response();
            }
            Ok(None) => return (StatusCode::OK, Json(Vec::<SearchBookDTO>::new())).into_response(),
            Err(_) => {
                return (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    Json(Vec::<SearchBookDTO>::new()),
                )
                    .into_response()
            }
        }
    }

    // Search by ISBN
    if let Some(isbn) = params.isbn {
        match book_repo.search_by_isbn(&isbn).await {
            Ok(Some(books)) => {
                let dtos: Vec<SearchBookDTO> = books
                    .into_iter()
                    .map(|b| SearchBookDTO {
                        book_id: b.book_id,
                        title: b.title,
                        published_date: b.published_date,
                        isbn: b.isbn,
                        file_type: b.file_type,
                        file_path: b.file_path,
                        cover_image_path: b.cover_image_path,
                    })
                    .collect();
                return (StatusCode::OK, Json(dtos)).into_response();
            }
            Ok(None) => return (StatusCode::OK, Json(Vec::<SearchBookDTO>::new())).into_response(),
            Err(_) => {
                return (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    Json(Vec::<SearchBookDTO>::new()),
                )
                    .into_response()
            }
        }
    }

    // If no search parameters provided, return empty
    (StatusCode::OK, Json(Vec::<SearchBookDTO>::new())).into_response()
}

#[derive(Serialize)]
pub struct AuthorDTO {
    pub author_id: i32,
    pub name: String,
}

pub async fn search_authors(Query(params): Query<SearchQuery>) -> impl IntoResponse {
    if let Some(query) = params.author.or(params.q) {
        let author_repo = AuthorRepo::new().await;
        match author_repo.search_by_name(&query).await {
            Ok(Some(authors)) => {
                let dtos: Vec<AuthorDTO> = authors
                    .into_iter()
                    .map(|a| AuthorDTO {
                        author_id: a.author_id,
                        name: a.name,
                    })
                    .collect();
                return (StatusCode::OK, Json(dtos)).into_response();
            }
            Ok(None) => {
                return (StatusCode::OK, Json(Vec::<AuthorDTO>::new())).into_response();
            }
            Err(_) => {
                return (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    Json(Vec::<AuthorDTO>::new()),
                )
                    .into_response();
            }
        }
    }

    (StatusCode::BAD_REQUEST, Json(Vec::<AuthorDTO>::new())).into_response()
}

pub async fn list_all_books() -> impl IntoResponse {
    let book_repo = BookRepo::new().await;
    match book_repo.get_all().await {
        Ok(Some(books)) => {
            let dtos: Vec<SearchBookDTO> = books
                .into_iter()
                .map(|b| SearchBookDTO {
                    book_id: b.book_id,
                    title: b.title,
                    published_date: b.published_date,
                    isbn: b.isbn,
                    file_type: b.file_type,
                    file_path: b.file_path,
                    cover_image_path: b.cover_image_path,
                })
                .collect();
            (StatusCode::OK, Json(dtos)).into_response()
        }
        Ok(None) => (StatusCode::OK, Json(Vec::<SearchBookDTO>::new())).into_response(),
        Err(_) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(Vec::<SearchBookDTO>::new()),
        )
            .into_response(),
    }
}
