use std::num::Saturating;

use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug)]
pub struct SearchResponse {
    pub docs: Vec<SearchDoc>,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct SearchDoc {
    pub title: String,
    pub author_name: Option<Vec<String>>,
    pub first_publish_year: Option<u32>,
    pub isbn: Option<Vec<String>>, // grab first for Books API lookup
    pub key: String,               // fallback identifier
}

pub fn book_search(title: &str) -> Result<SearchResponse, Box<dyn std::error::Error>> {
    let url = format!(
        "https://openlibrary.org/search.json?title={}&limit=10",
        urlencoding::encode(title)
    );
    let resp: SearchResponse = reqwest::blocking::get(&url)?.json()?;

    if resp.docs.is_empty() {
        eprintln!("No results found.")
    }

    println!("raw doc:{:?}", resp.docs[0]);
    Ok(resp)
}
