use serde::{Deserialize, Serialize};

#[derive(Serialize)]
pub struct ReadingProgressDTO {
    pub progress_id: Option<i32>,
    pub user_id: i32,
    pub book_id: i32,
    pub current_position: String,
    pub chapter_title: Option<String>,
    pub page_number: Option<i32>,
    pub progress_percentage: Option<f32>,
    pub last_read_at: Option<String>,
}

#[derive(Deserialize)]
pub struct UpdateProgressDTO {
    pub book_id: i32,
    pub current_position: String,
    pub chapter_title: Option<String>,
    pub page_number: Option<i32>,
    pub progress_percentage: Option<f32>,
}
