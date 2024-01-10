//! # Minigrep
//! A simple grep-like utility written in Rust, following the tutorial in [The Rust Programming Language](https://doc.rust-lang.org/book/ch12-00-an-io-project.html).
#![warn(missing_debug_implementations, missing_docs, rust_2018_idioms)]

#[derive(Debug)]
/// A configuration struct for the minigrep application.
pub struct Config {
    /// The query string to search for.
    pub query: String,
    /// The path to the file to search.
    pub file_path: String,
    /// Whether or not to ignore case when searching.
    pub ignore_case: bool,
}

impl Config {
    /// Build a new Config from the given arguments.
    pub fn build(args: &[String]) -> Result<Self, &'static str> {
        if args.len() < 3 {
            return Err("not enough arguments");
        }

        let query = args[1].clone();
        let file_path = args[2].clone();

        let ignore_case = std::env::var("IGNORE_CASE").is_ok();

        let config = Self {
            query,
            file_path,
            ignore_case,
        };

        Ok(config)
    }
}

/// Run the minigrep application with the given configuration.
pub fn run(config: Config) -> Result<(), Box<dyn std::error::Error>> {
    let contents = std::fs::read_to_string(config.file_path)?;

    let results = if config.ignore_case {
        search_case_insensitive(&config.query, &contents)
    } else {
        search(&config.query, &contents)
    };

    for line in results {
        println!("{}", line);
    }

    Ok(())
}

/// Search for the given query string in the given contents, case-sensitive.
fn search<'a>(query: &'a str, contents: &'a str) -> Vec<&'a str> {
    let mut results = vec![];

    for line in contents.lines() {
        if line.contains(query) {
            results.push(line);
        }
    }

    results
}

/// Search for the given query string in the given contents, case-insensitive.
fn search_case_insensitive<'a>(query: &'a str, contents: &'a str) -> Vec<&'a str> {
    let query = query.to_lowercase();

    let mut results = vec![];

    for line in contents.lines() {
        if line.to_lowercase().contains(&query) {
            results.push(line);
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

        assert_eq!(vec!["safe, fast, productive."], search(query, contents));
    }

    #[test]
    fn case_insensitive() {
        let query = "rUsT";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Trust me.";

        assert_eq!(
            vec!["Rust:", "Trust me."],
            search_case_insensitive(query, contents)
        );
    }
}
