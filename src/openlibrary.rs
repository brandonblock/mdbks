use serde::{Deserialize, Serialize};
use std::collections::HashMap;

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
        "https://openlibrary.org/search.json?q={}&fields=title,author_name,first_publish_year,key,isbn&limit=10",
        urlencoding::encode(title)
    );
    let resp: SearchResponse = reqwest::blocking::get(&url)?.json()?;

    if resp.docs.is_empty() {
        eprintln!("No results found.")
    }

    // println!("raw doc:{:?}", resp.docs[0]);
    Ok(resp)
}

#[derive(Deserialize, Debug)]
pub struct BookData {
    pub title: String,
    pub authors: Option<Vec<Author>>,
    pub publishers: Option<Vec<Publisher>>,
    pub number_of_pages: Option<u32>,
    pub subjects: Option<Vec<Subject>>,
    pub identifiers: Option<Identifiers>,
    pub publish_date: Option<String>,
    pub isbn: Option<String>,
}

#[derive(Deserialize, Debug)]
pub struct Author {
    pub name: String,
    pub url: Option<String>,
}

#[derive(Deserialize, Debug)]
pub struct Publisher {
    pub name: String,
}

#[derive(Deserialize, Debug)]
pub struct Subject {
    pub name: String,
    pub url: Option<String>,
}

#[derive(Deserialize, Debug)]
pub struct Identifiers {
    pub isbn_10: Option<Vec<String>>,
    pub isbn_13: Option<Vec<String>>,
}

pub fn book_select(isbns: &Option<Vec<String>>) -> Result<BookData, Box<dyn std::error::Error>> {
    let best_isbn = pick_isbn(isbns).unwrap_or("nope".to_string());
    //TODO: add error handling for no isbn and exit early
    println!("best isbn: {}", best_isbn);

    let url = format!(
        "https://openlibrary.org/api/books?bibkeys=ISBN:{}&jscmd=data&format=json",
        best_isbn
    );

    let resp: HashMap<String, BookData> = reqwest::blocking::get(&url)
        .inspect_err(|e| log::error!("Failed to fetch from {}: {}", url, e))?
        .json()
        .inspect_err(|e| log::error!("Failed to parse book data JSON: {}", e))?;

    let book_data = resp.into_values().next().ok_or_else(|| {
        log::error!("No book data returned from API response");
        "No book data returned"
    })?;

    Ok(book_data)
}

fn pick_isbn(isbns: &Option<Vec<String>>) -> Option<String> {
    let isbns = isbns.as_ref()?; // Now isbns is &Vec<String>

    isbns
        .iter()
        .find(|isbn| isbn.len() == 13)
        .or(isbns.first()) // .first() works on &Vec
        .cloned()
}
