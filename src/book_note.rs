use serde::{Deserialize, Serialize};
use std::io::Write;

use crate::openlibrary::BookData;

// TODO: genre support
// TODO: get isbn from identifiers
#[derive(Deserialize, Serialize, Debug)]
struct FrontMatter {
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
struct ReadSession {
    pub started: Option<chrono::NaiveDate>,
    pub finished: Option<chrono::NaiveDate>,
    pub status: Status,
}

#[derive(Deserialize, Serialize, Debug)]
enum Status {
    ToRead,
    Read,
    NotFinished,
}

impl FrontMatter {
    fn new(
        title: String,
        authors: Option<Vec<String>>,
        published: Option<chrono::NaiveDate>,
        pages: Option<u32>,
        // isbn: Option<String>,
    ) -> Self {
        let sessions = vec![ReadSession {
            started: None,
            finished: None,
            status: Status::ToRead,
        }];
        FrontMatter {
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
    // TODO: Format authors as links [[author]]
    let note_authors: Vec<String> = authors.iter().map(|a| a.name.clone()).collect();

    let date = book_data
        .publish_date
        .as_ref()
        .and_then(|d| parse_publish_date(d))
        .unwrap_or_else(|| {
            log::warn!("No valid publish date, using default");
            chrono::NaiveDate::from_ymd_opt(1970, 1, 1).unwrap()
        });
    let new_note = FrontMatter::new(
        book_data.title,
        Some(note_authors),
        Some(date),
        book_data.number_of_pages,
        // book_data.identifiers,
    );
    write_to_markdown(new_note)
}

fn write_to_markdown(frontmatter: FrontMatter) -> Result<(), Box<dyn std::error::Error>> {
    // TODO: get base path from config
    let filename = format!("{}.md", sanitize_filename(&frontmatter.title));
    let mut f = std::fs::File::create(&filename)?;

    writeln!(f, "---")?;
    serde_yml::to_writer(&f, &frontmatter)?;
    writeln!(f, "---")?;
    writeln!(f)?;
    writeln!(f, "## Description")?;
    writeln!(f)?;
    writeln!(f, "## Thoughts")?;
    writeln!(f)?;

    Ok(())
}

fn parse_publish_date(s: &str) -> Option<chrono::NaiveDate> {
    let formats = ["%Y-%m-%d", "%B %d, %Y", "%b %d, %Y"];

    for fmt in formats {
        if let Ok(date) = chrono::NaiveDate::parse_from_str(s.trim(), fmt) {
            return Some(date);
        }
    }

    if let Ok(year) = s.trim().parse::<i32>() {
        return chrono::NaiveDate::from_ymd_opt(year, 1, 1);
    }

    None
}

fn sanitize_filename(title: &str) -> String {
    title.chars().map(|c| match c{
        '/' | '\\' | ':' | '*' | '?' | '"' | '<' | '>' | '|' => '-',
        c => c, 
    }).collect::<String>().trim().to_string()
}
