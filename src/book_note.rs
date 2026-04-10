use serde::{Deserialize, Serialize};
use std::io::Write;

use crate::openlibrary::WorkData;

// TODO: genre support
#[derive(Deserialize, Serialize, Debug)]
struct FrontMatter {
    // frontmatter
    pub title: String,
    pub authors: Option<Vec<String>>,
    pub published: Option<chrono::NaiveDate>,
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
pub enum Status {
    ToRead,
    Reading,
    Read,
    NotFinished,
}

impl FrontMatter {
    fn new(
        title: String,
        authors: Option<Vec<String>>,
        published: Option<chrono::NaiveDate>,
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
            reads: sessions,
            first_added: chrono::Local::now().date_naive(),
        }
    }
}

pub fn create_new_note(work_data: WorkData) -> Result<(), Box<dyn std::error::Error>> {
    let date = work_data
        .first_publish_date
        .as_ref()
        .and_then(|d| parse_publish_date(d))
        .or_else(|| {
            work_data
                .search_publish_year
                .and_then(|y| chrono::NaiveDate::from_ymd_opt(y as i32, 1, 1))
        });
    let description = work_data.description.map(|d| d.into_string());
    let authors: Option<Vec<String>> = work_data
        .authors
        .map(|authors| authors.into_iter().map(|a| format!("[[{}]]", a)).collect());

    let new_note = FrontMatter::new(work_data.title, authors, date);
    write_to_markdown(new_note, description)
}

pub fn update_status(path: &str, status: Status) -> Result<(), Box<dyn std::error::Error>> {
    let book_note = std::fs::read_to_string(path)?;
    let parts: Vec<&str> = book_note.splitn(3, "---\n").collect();
    if parts.len() < 3 {
        return Err("Invalid frontmatter format".into());
    }

    let mut frontmatter: FrontMatter = serde_yml::from_str(parts[1])?;

    let session = frontmatter
        .reads
        .last_mut()
        .ok_or("No read sessions found")?;

    let now = Some(chrono::Local::now().date_naive());
    match (&session.status, &status) {
        (Status::ToRead, Status::Reading) => session.started = now,
        (Status::Reading, Status::Read) => session.finished = now,
        _ => return Err(format!("Invalid update: {:?}, -> {:?}", session.status, status).into()),
    }
    session.status = status;
    let new_frontmatter = serde_yml::to_string(&frontmatter)?;
    std::fs::write(path, format!("---\n{}---\n{}", new_frontmatter, parts[2]))?;
    Ok(())
}

fn write_to_markdown(
    frontmatter: FrontMatter,
    description: Option<String>,
) -> Result<(), Box<dyn std::error::Error>> {
    // TODO: get base path from config
    let filename = format!("{}.md", sanitize_filename(&frontmatter.title));
    let mut f = std::fs::File::create(&filename)?;

    writeln!(f, "---")?;
    serde_yml::to_writer(&f, &frontmatter)?;
    writeln!(f, "---")?;
    writeln!(f)?;
    writeln!(f, "## Description")?;
    writeln!(f)?;
    if let Some(desc) = description {
        writeln!(f, "{}", desc)?;
        writeln!(f)?;
    }
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
    title
        .chars()
        .map(|c| match c {
            '/' | '\\' | ':' | '*' | '?' | '"' | '<' | '>' | '|' => '-',
            c => c,
        })
        .collect::<String>()
        .trim()
        .to_string()
}
