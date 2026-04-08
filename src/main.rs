use clap::{Parser, ValueEnum};

#[derive(ValueEnum, Clone)]
enum Command {
    New,
}

#[derive(Parser)]
#[command(about = "Search OpenLibrary by title")]
struct Args {
    command: Command,
    // user input title for search
    title: String,
}

fn main() {
    let args = Args::parse();
    println!("{}", &args.title);
}
