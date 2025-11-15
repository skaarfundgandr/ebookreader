use serde::{Deserialize, Serialize};

#[derive(Serialize)]
pub struct BookmarkDTO {
    pub bookmark_id: Option<i32>,
    pub user_id: i32,
    pub book_id: i32,
    pub chapter_title: Option<String>,
    pub page_number: Option<i32>,
    pub position: String,
    pub created_at: Option<String>,
}

#[derive(Deserialize)]
pub struct NewBookmarkDTO {
    pub book_id: i32,
    pub chapter_title: Option<String>,
    pub page_number: Option<i32>,
    pub position: String,
}
