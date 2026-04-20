use std::path::PathBuf;

use clap::{Parser, Subcommand};
use dialoguer::Select;

mod book_note;
mod openlibrary;
use book_note::{create_new_note, update_status, Status};
use openlibrary::{work_fetch, SearchResponse};

#[derive(Clone, Subcommand)]
enum Command {
    New {
        title: String,
        #[clap(short, long)]
        output: Option<PathBuf>,
    },
    Start {
        path: String,
    },
    Finish {
        path: String,
    },
    NotFinish {
        path: String,
    },
    ReRead {
        path: String,
    },
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
        Command::New { title, output } => {
            let resp: SearchResponse = openlibrary::book_search(&title)?;
            let display_items: Vec<String> = resp.display_items();
            let selection = Select::new()
                .with_prompt("What do you choose?")
                .items(&display_items)
                .interact()
                .unwrap();

            let selected = &resp.docs[selection];
            let mut work_data = work_fetch(&selected.key)?;
            work_data.authors = selected.author_name.clone();
            work_data.search_publish_year = selected.first_publish_year;

            create_new_note(work_data, output.unwrap_or(PathBuf::from("./")))
        }
        Command::Start { path } => update_status(&path, Status::Reading),
        Command::Finish { path } => {
            // TODO: open editor at Thoughts section
            update_status(&path, Status::Read)?;
            std::process::Command::new("hx").arg(&path).status()?;
            Ok(())
        }
        Command::NotFinish { path } => {
            update_status(&path, Status::NotFinished)?;
            std::process::Command::new("hx").arg(&path).status()?;
            Ok(())
        }
        Command::ReRead { path } => todo!(),
        Command::List => todo!(),
    }
}
