use clap::{Parser, Subcommand};
use dialoguer::Select;

mod book_note;
mod openlibrary;
use book_note::{create_new_note, start_reading};
use openlibrary::{SearchResponse, work_fetch};

#[derive(Clone, Subcommand)]
enum Command {
    New { title: String },
    Start { path: String },
    Finish,
    NotFinish,
    List,
}

#[derive(Parser)]
#[command(about = "Search OpenLibrary by title")]
struct Args {
    #[command(subcommand)]
    command: Command,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    env_logger::init();
    let args = Args::parse();

    match args.command {
        Command::New { title } => {
            let resp: SearchResponse = openlibrary::book_search(&title)?;
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

            let selected = &resp.docs[selection];
            let mut work_data = work_fetch(&selected.key)?;
            work_data.authors = selected.author_name.clone();
            work_data.search_publish_year = selected.first_publish_year;

            create_new_note(work_data)
        }
        Command::Start { path } => start_reading(&path),
        Command::Finish => todo!(),
        Command::NotFinish => todo!(),
        Command::List => todo!(),
    }
}
