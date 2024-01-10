//! # Minigrep
//! A simple grep-like utility written in Rust, following the tutorial in [The Rust Programming Language](https://doc.rust-lang.org/book/ch12-00-an-io-project.html).
#![warn(missing_debug_implementations, missing_docs, rust_2018_idioms)]

use std::{collections::HashMap, path::PathBuf};

use clap::Parser;

#[derive(Debug, Parser)]
#[command(author, version, about, long_about = None)]
/// Clap command-line interface for the minigrep application.
pub struct MinigrepCli {
    #[arg(short, long)]
    verbose: bool,
    #[arg(short, long)]
    ignore_case: bool,
    query: String,
    file: PathBuf,
}

/// Run the minigrep application with the given configuration.
pub fn run(config: &MinigrepCli) -> Result<(), Box<dyn std::error::Error>> {
    let contents = std::fs::read_to_string(&config.file)?;

    let results = if config.ignore_case {
        search_case_insensitive(&config.query, &contents)
    } else {
        search(&config.query, &contents)
    };

    if config.verbose {
        println!("Searching for {}", config.query);
        println!("In file {}", config.file.display());
        println!("With text:\n{}", contents);
        println!("Results:");
    }

    display(&results);

    Ok(())
}

/// Display the given results of a search.
fn display<'a>(results: &HashMap<usize, &'a str>) {
    let mut results = results.iter().collect::<Vec<_>>();
    results.sort_by(|(a, _), (b, _)| a.cmp(b));

    for (line_number, line) in results {
        println!("#{line_number}: {line}");
    }
}

/// Search for the given query string in the given contents, case-sensitive.
fn search<'a>(query: &'a str, contents: &'a str) -> HashMap<usize, &'a str> {
    let mut results = HashMap::new();

    for (idx, line) in contents.lines().enumerate() {
        if line.contains(query) {
            results.insert(idx + 1, line);
        }
    }

    results
}

/// Search for the given query string in the given contents, case-insensitive.
fn search_case_insensitive<'a>(query: &'a str, contents: &'a str) -> HashMap<usize, &'a str> {
    let mut results = HashMap::new();
    let query = query.to_lowercase();

    for (idx, line) in contents.lines().enumerate() {
        if line.to_lowercase().contains(&query) {
            results.insert(idx + 1, line);
        }
    }

    results
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_sensitive() {
        let query = "duct";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Duct tape.";

        let mut expected = HashMap::new();
        expected.insert(2, "safe, fast, productive.");

        assert_eq!(expected, search(query, contents));
    }

    #[test]
    fn case_insensitive() {
        let query = "rUsT";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Trust me.";

        let mut expected = HashMap::new();
        expected.insert(1, "Rust:");
        expected.insert(4, "Trust me.");

        assert_eq!(expected, search_case_insensitive(query, contents));
    }
}
