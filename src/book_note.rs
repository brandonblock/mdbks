use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug)]
pub struct BookNote {
    // frontmatter
    pub title: String,
    pub author: Vec<String>,
    pub genre: Vec<String>,
    pub published: chrono::NaiveDate,
    pub pages: Option<i32>,
    pub isbn: Option<String>,
    pub reads: Vec<ReadSession>,
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
