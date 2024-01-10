//! # Minigrep
//! A simple grep-like utility written in Rust, following the tutorial in [The Rust Programming Language](https://doc.rust-lang.org/book/ch12-00-an-io-project.html).
#![warn(missing_debug_implementations, missing_docs, rust_2018_idioms)]

use std::{
    collections::HashMap,
    fs::File,
    io::{BufRead, BufReader},
    path::PathBuf,
};

use clap::Parser;

#[derive(Debug, Parser)]
#[command(author, version, about, long_about = None)]
/// Clap command-line interface for the minigrep application.
pub struct MinigrepCli {
    #[arg(short, long)]
    /// Enable verbose output.
    pub verbose: bool,
    #[arg(short, long)]
    /// Perform a case-insensitive search, default false.
    pub ignore_case: bool,
    /// The query string to search for.
    pub query: String,
    /// The path to file to search in.
    pub file: PathBuf,
}

/// Run the minigrep application with the given configuration.
pub fn run(config: &MinigrepCli) -> Result<(), Box<dyn std::error::Error>> {
    let file = File::open(&config.file)?;
    let file_reader = BufReader::new(file);

    let results = if config.ignore_case {
        search_case_insensitive_lazy(&config.query, file_reader)
    } else {
        search_lazy(&config.query, file_reader)
    }?;

    if config.verbose {
        println!("Searching for {}", config.query);
        println!("In file {}", config.file.display());
    }

    display(&results);

    Ok(())
}

/// Display the given results of a search.
fn display<'a>(results: &HashMap<usize, String>) {
    let mut results = results.iter().collect::<Vec<_>>();
    results.sort_by(|(a, _), (b, _)| a.cmp(b));

    for (line_number, line) in results {
        println!("#{line_number}: {line}");
    }
}

/// Lazily search for the given query string in the given contents, case-sensitive.
fn search_lazy<R: BufRead>(
    query: &str,
    reader: R,
) -> Result<HashMap<usize, String>, Box<dyn std::error::Error>> {
    let mut results = HashMap::new();

    for (idx, line) in reader.lines().enumerate() {
        let line = line?;

        if line.contains(query) {
            results.insert(idx + 1, line.into());
        }
    }

    Ok(results)
}

/// Lazily search for the given query string in the given contents, case-insensitive.
fn search_case_insensitive_lazy<R: BufRead>(
    query: &str,
    reader: R,
) -> Result<HashMap<usize, String>, Box<dyn std::error::Error>> {
    let mut results = HashMap::new();
    let query = query.to_lowercase();

    for (idx, line) in reader.lines().enumerate() {
        let line = line?;

        if line.to_lowercase().contains(&query) {
            results.insert(idx + 1, line);
        }
    }

    Ok(results)
}

#[cfg(test)]
mod tests {
    use std::io::Cursor;

    use super::*;

    /// Create a reader from the given contents using a cursor instead of a file, for testing purposes only.
    fn create_reader(contents: &str) -> BufReader<Cursor<&str>> {
        BufReader::new(Cursor::new(contents))
    }

    #[test]
    fn case_sensitive() {
        let query = "duct";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Duct tape.";

        let reader = create_reader(contents);
        let result = search_lazy(query, reader).unwrap();
        let mut expected = HashMap::new();
        expected.insert(2, "safe, fast, productive.".into());

        assert_eq!(expected, result);
    }

    #[test]
    fn case_insensitive() {
        let query = "rUsT";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Trust me.";

        let reader = create_reader(contents);
        let result = search_case_insensitive_lazy(query, reader).unwrap();
        let mut expected = HashMap::new();
        expected.insert(1, "Rust:".into());
        expected.insert(4, "Trust me.".into());

        assert_eq!(expected, result);
    }
}
