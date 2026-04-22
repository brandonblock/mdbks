use serde::{Deserialize, Serialize};
use std::fs::File;
use std::io::Write;
use std::path::Path;

#[derive(Deserialize, Serialize, Debug)]
pub struct BookNote {
    pub frontmatter: FrontMatter,
    pub body: String,
}

impl BookNote {
    pub fn new(frontmatter: FrontMatter, description: Option<String>) -> Self {
        let mut body = String::from("\n## Description\n\n");
        if let Some(desc) = description {
            body.push_str(&format!("{desc}\n\n"));
        }
        body.push_str("## Thoughts\n\n\n");
        Self { frontmatter, body }
    }

    pub fn create(&self, path: &Path) -> Result<(), Box<dyn std::error::Error>> {
        let mut file = File::create_new(path)?;
        file.write_all(self.serialize()?.as_bytes())?;
        Ok(())
    }

    pub fn filename(&self) -> String {
        let sanitized = self
            .frontmatter
            .title
            .chars()
            .map(|c| match c {
                '/' | '\\' | ':' | '*' | '?' | '"' | '<' | '>' | '|' => '-',
                c => c,
            })
            .collect::<String>()
            .trim()
            .to_string();
        format!("{}.md", sanitized)
    }

    pub fn from_file(path: &Path) -> Result<Self, Box<dyn std::error::Error>> {
        let s = std::fs::read_to_string(path)?;
        let parts: Vec<&str> = s.splitn(3, "---\n").collect();
        if parts.len() < 3 {
            return Err("Invalid frontmatter format".into());
        }
        Ok(BookNote {
            frontmatter: serde_yml::from_str(parts[1])?,
            body: parts[2].to_string(),
        })
    }

    pub fn line_after_thoughts(&self) -> Result<usize, Box<dyn std::error::Error>> {
        let content = self.serialize()?;
        Ok(content
            .lines()
            .enumerate()
            .find(|(_, l)| l.contains("## Thoughts"))
            .map(|(i, _)| i + 3)
            .unwrap_or(0))
    }

    pub fn reread(&mut self, path: &Path) -> Result<(), Box<dyn std::error::Error>> {
        self.frontmatter.add_read()?;
        self.write(path)
    }

    fn serialize(&self) -> Result<String, Box<dyn std::error::Error>> {
        Ok(format!(
            "---\n{}---\n{}",
            serde_yml::to_string(&self.frontmatter)?,
            self.body
        ))
    }

    pub fn update_status(
        &mut self,
        path: &Path,
        status: Status,
        date: chrono::NaiveDate,
    ) -> Result<(), Box<dyn std::error::Error>> {
        self.frontmatter.update_status(status, date)?;

        self.write(path)
    }
    pub fn write(&self, path: &Path) -> Result<(), Box<dyn std::error::Error>> {
        std::fs::write(path, self.serialize()?)?;
        Ok(())
    }
}

#[derive(Deserialize, Serialize, Debug)]
pub struct FrontMatter {
    pub title: String,
    pub authors: Option<Vec<String>>,
    pub published: Option<i32>,
    pub reads: Vec<ReadSession>,
    pub first_added: chrono::NaiveDate,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct ReadSession {
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
    pub fn new(title: String, authors: Option<Vec<String>>, published: Option<i32>) -> Self {
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
}
