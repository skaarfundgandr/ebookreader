use serde::{Deserialize, Serialize};

#[derive(Serialize)]
pub struct AnnotationDTO {
    pub annotation_id: Option<i32>,
    pub user_id: i32,
    pub book_id: i32,
    pub chapter_title: Option<String>,
    pub start_position: String,
    pub end_position: String,
    pub highlighted_text: Option<String>,
    pub note: Option<String>,
    pub color: Option<String>,
    pub created_at: Option<String>,
    pub updated_at: Option<String>,
}

#[derive(Deserialize)]
pub struct NewAnnotationDTO {
    pub book_id: i32,
    pub chapter_title: Option<String>,
    pub start_position: String,
    pub end_position: String,
    pub highlighted_text: Option<String>,
    pub note: Option<String>,
    pub color: Option<String>,
}

#[derive(Deserialize)]
pub struct UpdateAnnotationDTO {
    pub chapter_title: Option<String>,
    pub start_position: Option<String>,
    pub end_position: Option<String>,
    pub highlighted_text: Option<String>,
    pub note: Option<String>,
    pub color: Option<String>,
}
