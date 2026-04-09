use serde::{Deserialize, Serialize};

use crate::openlibrary::BookData;

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
    pub first_added: Option<chrono::NaiveDate,
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

pub fn create_new_note(book_data: BookData) -> Result<(), Box<dyn std::error::Error>> {
    // TODO: convert to BookNote
    let authors = book_data.authors.unwrap_or_default();
    let note_authors: Vec<String> = authors.iter().map(|a| a.name.clone()).collect();
    // TODO: convert to md file
    Ok(())
}
