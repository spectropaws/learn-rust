use clap::{Parser, command};
use std::{
    fs::File,
    io::{BufRead, BufReader},
};

mod matcher;
use matcher::Matcher;

#[derive(Parser, Debug)]
#[command(name = "mini-grep")]
#[command(about = "A simple grep clone in Rust")]
struct Args {
    /// Pattern to search
    pattern: String,

    /// File to search
    file: String,

    /// Use regex
    #[arg(short, long)]
    regex: bool,

    /// Ignore case
    #[arg(short, long)]
    ignore_case: bool,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();
    let file = File::open(&args.file)?;
    let reader = BufReader::new(file);

    let matcher = Matcher::new(args.pattern, args.regex, args.ignore_case)?;
    for (i, line) in reader.lines().enumerate() {
        let line = line?;

        if matcher.matches(&line) {
            println!("{}:{}", i + 1, line);
        }
    }
    Ok(())
}
