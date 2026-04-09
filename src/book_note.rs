use serde::{Deserialize, Serialize};

use crate::openlibrary::BookData;

// TODO: genre support
// TODO: get isbn from identifiers
#[derive(Deserialize, Serialize, Debug)]
pub struct BookNote {
    // frontmatter
    pub title: String,
    pub authors: Option<Vec<String>>,
    pub published: Option<chrono::NaiveDate>,
    pub pages: Option<u32>,
    // pub isbn: Option<String>,
    pub reads: Vec<ReadSession>,
    pub first_added: chrono::NaiveDate,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct ReadSession {
    pub started: chrono::NaiveDate,
    pub finished: Option<chrono::NaiveDate>,
    pub status: Status,
}

#[derive(Deserialize, Serialize, Debug)]
pub enum Status {
    ToRead,
    Read,
    NotFinished,
}

impl BookNote {
    pub fn new(
        title: String,
        authors: Option<Vec<String>>,
        published: Option<chrono::NaiveDate>,
        pages: Option<u32>,
        // isbn: Option<String>,
    ) -> Self {
        let sessions = vec![ReadSession {
            started: chrono::Local::now().date_naive(),
            finished: None,
            status: Status::ToRead,
        }];
        BookNote {
            title,
            authors,
            published,
            pages,
            // isbn,
            reads: sessions,
            first_added: chrono::Local::now().date_naive(),
        }
    }
}

pub fn create_new_note(book_data: BookData) -> Result<(), Box<dyn std::error::Error>> {
    let authors = book_data.authors.unwrap_or_default();
    let note_authors: Vec<String> = authors.iter().map(|a| a.name.clone()).collect();
    let date = book_data
        .publish_date
        .as_ref()
        .and_then(|d| parse_publish_date(d))
        .unwrap_or_else(|| {
            log::warn!("No valid publish date, using default");
            chrono::NaiveDate::from_ymd_opt(1970, 1, 1).unwrap()
        });
    let new_note = BookNote::new(
        book_data.title,
        Some(note_authors),
        Some(date),
        book_data.number_of_pages,
        // book_data.identifiers,
    );
    println!("new book note struct: {:?}", new_note);
    // TODO: convert to md file
    Ok(())
}

fn parse_publish_date(s: &str) -> Option<chrono::NaiveDate> {
    // Try full date formats first
    let formats = [
        "%Y-%m-%d",  // 1979-01-01
        "%B %d, %Y", // January 1, 1979
        "%b %d, %Y", // Jan 1, 1979
    ];

    for fmt in formats {
        if let Ok(date) = chrono::NaiveDate::parse_from_str(s.trim(), fmt) {
            return Some(date);
        }
    }

    // Fallback: year only → default to Jan 1
    if let Ok(year) = s.trim().parse::<i32>() {
        return chrono::NaiveDate::from_ymd_opt(year, 1, 1);
    }

    None
}
