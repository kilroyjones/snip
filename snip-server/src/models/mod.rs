use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Debug, Deserialize, Serialize)]
pub struct AddImage {
    pub url: String,
    pub source: String,
    pub description: Option<String>,
    pub tags: Option<Vec<String>>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct AddBookmark {
    pub link: String,
    pub description: Option<String>,
    pub tags: Option<Vec<String>>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct AddQuote {
    pub quote: String,
    pub source: String,
    pub description: Option<String>,
    pub tags: Option<Vec<String>>,
}

#[derive(Debug, Serialize, FromRow)]
pub struct Quote {
    pub id: i32,
    pub date: NaiveDateTime,
    pub quote: String,
    pub source: String,
    pub code_type: String,
    pub description: String,
    pub icon_path: String,
}

#[derive(Debug, Serialize, FromRow)]
pub struct Bookmark {
    pub id: i32,
    pub date: NaiveDateTime,
    pub link: String,
    pub description: String,
    pub icon_path: String,
    pub trashed: bool,
}

#[derive(Debug, Serialize, FromRow)]
pub struct Image {
    pub id: i32,
    pub date: NaiveDateTime,
    pub source: String,
    pub description: String,
    pub original_path: String,
    pub thumbnail_path: String,
    pub trashed: bool,
}

#[derive(Debug, Deserialize)]
pub struct AddSnippet {
    pub snippet_type: String,
    pub data: String,
    pub source: String,
    pub description: Option<String>,
    pub tags: Option<Vec<String>>,
}

#[derive(Debug, Deserialize)]
pub struct Snippets {
    pub limit: usize,
    pub offset: usize,
    pub trash: bool,
}

#[derive(Debug, Serialize)]
pub struct QueryResponse<T> {
    pub is_succesful: bool,
    pub records: Option<Vec<T>>,
    pub pages: Option<usize>,
    pub error: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct RemoveSnippet {
    pub id: i32,
}

#[derive(Serialize)]
pub struct SnipMessage {
    pub msg: String,
}

#[derive(Deserialize, Serialize)]
pub struct SnippetReceived {
    pub id: usize,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Message {
    pub op: String,
    pub data: String,
}

// #[derive(Deserialize)]
// pub struct SnippetRequest {
//     snippet_type: Option<String>,
//     start_date: Option<String>,
//     end_date: Option<String>,
// }
