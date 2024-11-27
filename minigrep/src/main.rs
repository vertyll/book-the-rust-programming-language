use std::{env, process};
use minigrep::Config;

fn main() {
    let config = Config::build(env::args()).unwrap_or_else(|err| {
        eprintln!("Problem z parsowaniem argumentów: {}", err);
        process::exit(1);
    });

    println!("Wyszukiwanie: {}", config.query);
    println!("W pliku: {}", config.file_path);

    if let Err(e) = minigrep::run(config) {
        eprintln!("Błąd aplikacji: {e}");
        process::exit(1);
    }
}
