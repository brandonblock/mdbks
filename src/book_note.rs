use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug)]
pub struct BookNote {
    // frontmatter
    pub title: String,
    pub author_name: Option<Vec<String>>,
    pub genre: Option<Vec<String>>,
    pub first_publish_year: Option<u32>,
    pub pages: Option<i32>,
    pub isbn: Option<String>,
    pub reads: Option<Vec<ReadSession>>,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct ReadSession {
    pub started: chrono::NaiveDate,
    pub finished: chrono::NaiveDate,
    pub status: Status,
}

#[derive(Deserialize, Serialize, Debug)]
pub enum Status {
    ToRead,
    Read,
    NotFinished,
}
