use chrono::Datelike;
use serde::{Deserialize, Serialize};
use std::fmt::Write as _;
use std::fs::File;
use std::io::Write;
use std::path::{Path, PathBuf};

use crate::openlibrary::WorkData;

// TODO: move to this booknote model, reduce the naked funcs
// #[derive(Deserialize, Serialize, Debug)]
// struct BookNote {
//     frontmatter: Frontmatter,
//     body: string,
// }

#[derive(Deserialize, Serialize, Debug)]
struct FrontMatter {
    pub title: String,
    pub authors: Option<Vec<String>>,
    pub published: Option<i32>,
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
#[serde(rename_all = "snake_case")]
pub enum Status {
    ToRead,
    Reading,
    Read,
    NotFinished,
}

impl FrontMatter {
    fn new(title: String, authors: Option<Vec<String>>, published: Option<i32>) -> Self {
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
    pub fn from_note(s: &str) -> Result<(Self, &str), Box<dyn std::error::Error>> {
        let parts: Vec<&str> = s.splitn(3, "---\n").collect();
        if parts.len() < 3 {
            return Err("Invalid frontmatter format".into());
        }
        Ok((serde_yml::from_str(parts[1])?, parts[2]))
    }
    pub fn to_note(&self, body: &str) -> Result<String, Box<dyn std::error::Error>> {
        Ok(format!("---\n{}---\n{}", serde_yml::to_string(self)?, body))
    }
    pub fn update_status(
        &mut self,
        status: Status,
        date: chrono::NaiveDate,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let session = self.reads.first_mut().ok_or("No read sessions found")?;

        match (&session.status, &status) {
            (Status::ToRead, Status::Reading) => session.started = Some(date),
            (Status::Reading, Status::Read) => session.finished = Some(date),
            (Status::Reading, Status::NotFinished) => {}
            _ => {
                return Err(format!("Invalid update: {:?} -> {:?}", session.status, status).into())
            }
        }
        session.status = status;
        Ok(())
    }
    pub fn add_read(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        let latest_session = self
            .reads
            .first_mut()
            .ok_or(format!("{:?} is missing read sessions", self.title))?;

        let new_session = ReadSession {
            started: None,
            finished: None,
            status: Status::ToRead,
        };

        match &latest_session.status {
            Status::ToRead => {
                return Err(format!("{}'s status is already 'to_read'", self.title).into());
            }
            Status::Reading => {
                latest_session.status = Status::NotFinished;
            }
            _ => {}
        }
        self.reads.insert(0, new_session);
        Ok(())
    }
}

pub fn create_new_note(
    work_data: WorkData,
    output_path: PathBuf,
) -> Result<(), Box<dyn std::error::Error>> {
    let date = work_data
        .first_publish_date
        .as_ref()
        .and_then(|d| parse_publish_date(d))
        .or_else(|| work_data.search_publish_year.map(|y| y as i32));

    let description = work_data.description.map(|d| d.into_string());
    let authors: Option<Vec<String>> = work_data
        .authors
        .map(|authors| authors.into_iter().map(|a| format!("[[{}]]", a)).collect());

    let new_note = FrontMatter::new(work_data.title, authors, date);
    write_to_markdown(new_note, output_path, description)
}

pub fn update_status(
    path: &Path,
    status: Status,
    date: chrono::NaiveDate,
) -> Result<(), Box<dyn std::error::Error>> {
    let note = std::fs::read_to_string(path)?;
    let (mut frontmatter, body) = FrontMatter::from_note(&note)?;
    frontmatter.update_status(status, date)?;

    std::fs::write(path, frontmatter.to_note(body)?)?;
    Ok(())
}

pub fn reread(path: &Path) -> Result<(), Box<dyn std::error::Error>> {
    let note = std::fs::read_to_string(path)?;
    let (mut frontmatter, body) = FrontMatter::from_note(&note)?;
    frontmatter.add_read()?;
    std::fs::write(path, frontmatter.to_note(body)?)?;
    Ok(())
}

fn write_to_markdown(
    frontmatter: FrontMatter,
    output_path: PathBuf,
    description: Option<String>,
) -> Result<(), Box<dyn std::error::Error>> {
    let path = output_path.join(format!("{}.md", sanitize_filename(&frontmatter.title)));
    println!("filename: {}", path.display());

    let mut out = String::new();
    out.push_str("---\n");
    out.push_str(&serde_yml::to_string(&frontmatter)?);
    out.push_str("---\n\n## Description\n\n");
    if let Some(desc) = description {
        write!(out, "{desc}\n\n")?;
    }
    out.push_str("## Thoughts\n\n\n");
    let mut file = File::create_new(&path)?;
    file.write_all(out.as_bytes())?;
    Ok(())
}

fn parse_publish_date(s: &str) -> Option<i32> {
    let formats = ["%Y-%m-%d", "%B %d, %Y", "%b %d, %Y"];

    for fmt in formats {
        if let Ok(date) = chrono::NaiveDate::parse_from_str(s.trim(), fmt) {
            return Some(date.year());
        }
    }
    s.trim().parse::<i32>().ok()
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
