use std::error::Error;
use std::{env, fs};

pub struct Config {
    pub query: String,
    pub file_path: String,
    pub ignore_case: bool,
}

impl Config {
    pub fn build(mut args: impl Iterator<Item = String>) -> Result<Config, &'static str> {
        args.next();
        let query = match args.next() {
            Some(arg) => arg,
            None => return Err("Nie otrzymano łańcucha zanków zapytania"),
        };
        let file_path = match args.next() {
            Some(arg) => arg,
            None => return Err("Nie otrzymano ścieżki pliku"),
        };
        let ignore_case = env::var("IGNORE_CASE").is_ok();
        Ok (Config { query, file_path, ignore_case })
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.file_path)?;
    let results = if config.ignore_case {
        search_case_insensitive(&config.query, &contents)
    } else {
        search(&config.query, &contents)
    };
    for line in results {
        println!("{line}");
    }
    Ok(())
}

pub fn search<'a>(
    query: &str,
    contents: &'a str
) -> Vec<&'a str> {
    contents
        .lines()
        .filter(|line| line.contains(query))
        .collect()
}

fn search_case_insensitive<'a>(
    query: &str,
    contents: &'a str
) -> Vec<&'a str> {
    let query = query.to_lowercase();
    contents
        .lines()
        .filter(|line| line.to_lowercase().contains(&query))
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn case_sensitive() {
        let query = "produkt";
        let contents = "\
Rust:
bezpiecznie, szybko, produktywnie.
Wybierz trzy.
Produkt marketingowy";
        assert_eq!(
            vec!["bezpiecznie, szybko, produktywnie."],
            search(query, contents)
        );
    }

    #[test]
    fn case_insensitive() {
        let query = "rUsT";
        let contents = "\
Rust:
bezpiecznie, szybko, produktywnie.
Wybierz trzy.
rustanin.";
        assert_eq!(
            vec!["Rust:", "rustanin."],
            search_case_insensitive(query, contents)
        );
    }
}