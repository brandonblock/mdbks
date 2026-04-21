use std::path::{Path, PathBuf};

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
        path: PathBuf,
        #[clap(short, long)]
        date: Option<chrono::NaiveDate>,
    },
    Finish {
        path: PathBuf,
        #[clap(short, long)]
        date: Option<chrono::NaiveDate>,
    },
    NotFinish {
        path: PathBuf,
    },
    ReRead {
        path: PathBuf,
    },
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
        Command::Start { path, date } => update_status(
            &path,
            Status::Reading,
            date.unwrap_or(chrono::Local::now().date_naive()),
        ),
        Command::Finish { path, date } => {
            update_status(
                &path,
                Status::Read,
                date.unwrap_or(chrono::Local::now().date_naive()),
            )?;
            std::process::Command::new("hx")
                .arg(find_line_after_thoughts(&path))
                .status()?;
            Ok(())
        }
        Command::NotFinish { path } => {
            update_status(&path, Status::NotFinished, chrono::NaiveDate::default())?;
            std::process::Command::new("hx")
                .arg(find_line_after_thoughts(&path))
                .status()?;
            Ok(())
        }
        Command::ReRead { path } => todo!(),
    }
}

fn find_line_after_thoughts(path: &Path) -> String {
    let book_note = std::fs::read_to_string(path).expect("aw damn");
    let line = book_note
        .lines()
        .enumerate()
        .find(|(_, line)| line.contains("## Thoughts"))
        .map(|(i, _)| i + 3);
    if let Some(l) = line {
        format!("{}:{}", path.to_string_lossy(), l)
    } else {
        format!("{}", path.to_string_lossy())
    }
}
