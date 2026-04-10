use clap::{Parser, ValueEnum};
use dialoguer::Select;

mod book_note;
mod openlibrary;
use book_note::create_new_note;
use openlibrary::SearchResponse;

use crate::openlibrary::pick_isbn;

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

fn main() -> Result<(), Box<dyn std::error::Error>> {
    env_logger::init();
    let args = Args::parse();

    let resp: SearchResponse = openlibrary::book_search(&args.title)?;
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

    let isbn = pick_isbn(&resp.docs[selection].isbn).ok_or("No suitable ISBN found for book")?;

    let book_data = match openlibrary::book_select(&isbn) {
        Ok(data) => data,
        Err(e) => {
            log::error!("book_select failed: {}", e);
            return Err(e); // or handle differently
        }
    };
    create_new_note(book_data)?;
    Ok(())
}
