use chrono::Datelike;
use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct SearchResponse {
    pub docs: Vec<SearchDoc>,
}

impl SearchResponse {
    pub fn display_items(&self) -> Vec<String> {
        self.docs
            .iter()
            .map(|d| {
                let author = d
                    .author_name
                    .as_ref()
                    .map(|a| a.join(", "))
                    .unwrap_or_else(|| "Unknown".into());
                let year = d
                    .first_publish_year
                    .map(|y| y.to_string())
                    .unwrap_or_else(|| "??".into());
                format!("{} - {} - ({})", d.title, author, year)
            })
            .collect()
    }
}

#[derive(Deserialize, Debug)]
pub struct SearchDoc {
    pub title: String,
    pub author_name: Option<Vec<String>>,
    pub first_publish_year: Option<u32>,
    pub key: String, // fallback identifier
}

pub fn book_search(title: &str) -> Result<SearchResponse, Box<dyn std::error::Error>> {
    let url = format!(
        "https://openlibrary.org/search.json?q={}&fields=title,author_name,first_publish_year,key&limit=10",
        urlencoding::encode(title)
    );
    let resp: SearchResponse = reqwest::blocking::get(&url)?.json()?;

    if resp.docs.is_empty() {
        eprintln!("No results found.")
    }

    Ok(resp)
}

#[derive(Deserialize, Debug)]
pub struct WorkData {
    pub title: String,
    pub description: Option<Description>,
    pub first_publish_date: Option<String>,
    #[serde(skip)]
    pub authors: Option<Vec<String>>,
    #[serde(skip)]
    pub search_publish_year: Option<u32>,
}
impl WorkData {
    fn resolved_year(&self) -> Option<i32> {
        self.first_publish_date
            .as_ref()
            .and_then(|d| parse_publish_date(d))
            .or_else(|| self.search_publish_year.map(|y| y as i32))
    }
    fn formatted_authors(&self) -> Option<Vec<String>> {
        self.authors
            .as_ref()
            .map(|a| a.iter().map(|a| format!("[[{}]]", a)).collect())
    }
    pub fn into_note_parts(self) -> (String, Option<Vec<String>>, Option<i32>, Option<String>) {
        let year = self.resolved_year();
        let authors = self.formatted_authors();
        let description = self.description.map(|d| d.into_string());
        (self.title, authors, year, description)
    }
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

#[derive(Deserialize, Debug)]
#[serde(untagged)]
pub enum Description {
    Text(String),
    Object { value: String },
}

impl Description {
    pub fn into_string(self) -> String {
        match self {
            Description::Text(s) => s,
            Description::Object { value } => value,
        }
    }
}

pub fn work_fetch(key: &str) -> Result<WorkData, Box<dyn std::error::Error>> {
    let work_id = key.trim_start_matches("/works/");
    let url = format!("https://openlibrary.org/works/{}.json", work_id);

    let work_data: WorkData = reqwest::blocking::get(&url)
        .inspect_err(|e| log::error!("Failed to fetch work from {}: {}", url, e))?
        .json()
        .inspect_err(|e| log::error!("Failed to parse work JSON: {}", e))?;

    Ok(work_data)
}
