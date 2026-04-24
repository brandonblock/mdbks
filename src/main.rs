use std::{
    fs::File,
    path::{Path, PathBuf},
};

use clap::{Parser, Subcommand};
use dialoguer::Select;

mod book_note;
mod openlibrary;
use book_note::{BookNote, FrontMatter, Status};
use openlibrary::{work_fetch, WorkData};

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
            let work_data = fetch_selected(&title)?;
            let raw_authors = &work_data.authors;
            if let Some(authors) = raw_authors {
                for author in authors {
                    File::create_new(format!("Authors/{}.md", author))?;
                }
            }
            let (title, authors, year, description) = work_data.into_note_parts();
            let note = BookNote::new(FrontMatter::new(title, authors, year), description);
            note.create(&output.unwrap_or(PathBuf::from("./")).join(note.filename()))
        }
        Command::Start { path, date } => {
            let mut note = BookNote::from_file(&path)?;
            note.update_status(
                &path,
                Status::Reading,
                date.unwrap_or(chrono::Local::now().date_naive()),
            )
        }
        Command::Finish { path, date } => {
            let mut note = BookNote::from_file(&path)?;
            note.update_status(
                &path,
                Status::Read,
                date.unwrap_or(chrono::Local::now().date_naive()),
            )?;
            open_in_editor(&note, &path)
        }
        Command::NotFinish { path } => {
            let mut note = BookNote::from_file(&path)?;
            note.update_status(&path, Status::NotFinished, chrono::NaiveDate::default())?;
            open_in_editor(&note, &path)
        }
        Command::ReRead { path } => {
            let mut note = BookNote::from_file(&path)?;
            note.reread(&path)
        }
    }
}

fn fetch_selected(title: &str) -> Result<WorkData, Box<dyn std::error::Error>> {
    let resp = openlibrary::book_search(title)?;
    let selection = Select::new()
        .with_prompt("What do you choose?")
        .items(resp.display_items())
        .interact()?;
    let selected = &resp.docs[selection];
    let mut work_data = work_fetch(&selected.key)?;
    work_data.authors = selected.author_name.clone();
    work_data.search_publish_year = selected.first_publish_year;
    Ok(work_data)
}

fn open_in_editor(note: &BookNote, path: &Path) -> Result<(), Box<dyn std::error::Error>> {
    std::process::Command::new("hx")
        .arg(format!(
            "{}:{}",
            path.to_string_lossy(),
            note.line_after_thoughts()?
        ))
        .status()?;
    Ok(())
}
