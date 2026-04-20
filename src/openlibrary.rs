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
    // TODO: Genre parsing to main categories
    pub subjects: Option<Vec<String>>,
    #[serde(skip)]
    pub authors: Option<Vec<String>>,
    #[serde(skip)]
    pub search_publish_year: Option<u32>,
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
