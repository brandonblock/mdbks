use clap::{Parser, ValueEnum};
use dialoguer::Select;

mod book_note;
use book_note::BookNote;

#[derive(Clone, Default, ValueEnum)]
enum Command {
    #[default]
    New,
    Finish,
    NotFinish,
    List,
}

#[derive(Parser)]
#[command(about = "Search OpenLibrary by title")]
struct Args {
    command: Command,
    // user input title for search
    title: String,
}

#[derive(serde::Deserialize, Debug)]
struct SearchResponse {
    docs: Vec<BookNote>,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();
    let url = format!(
        "https://openlibrary.org/search.json?title={}&limit=10",
        urlencoding::encode(&args.title)
    );
    let resp: SearchResponse = reqwest::blocking::get(&url)?.json()?;

    if resp.docs.is_empty() {
        eprintln!("No results found.")
    }

    println!("raw doc:{:?}", resp.docs[0]);

    let display_items: Vec<String> = resp
        .docs
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
        .collect();

    let selection = Select::new()
        .with_prompt("What do you choose?")
        .items(&display_items)
        .interact()
        .unwrap();

    println!("You chose: {}", display_items[selection]);
    Ok(())
}
