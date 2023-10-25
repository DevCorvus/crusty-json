mod lexer;
mod parser;
mod utils;

use clap::{ArgGroup, Parser};
use nonblock::NonBlockingReader;
use std::{fs, io, path::PathBuf};
use utils::parse_json_and_print;

/// Crusty JSON parser
#[derive(Parser)]
#[clap(group = ArgGroup::new("input").required(true).args(&["json", "file", "url"]))]
struct Args {
    /// In-line json to parse from
    #[clap(conflicts_with_all = ["file", "url"])]
    json: Option<String>,

    /// Path to json file to parse from
    #[clap(short, long, conflicts_with_all = ["json", "url"])]
    file: Option<PathBuf>,

    /// Path to json file to parse from
    #[clap(short, long, conflicts_with_all = ["json", "file"])]
    url: Option<String>,
}

fn cli() {
    let args = Args::parse();

    match args {
        Args {
            json: Some(text), ..
        } => {
            parse_json_and_print(text);
        }
        Args {
            file: Some(file_path),
            ..
        } => match fs::read_to_string(file_path) {
            Ok(file_content) => parse_json_and_print(file_content),
            Err(err) => eprintln!("{}", err),
        },
        Args { url: Some(url), .. } => match reqwest::blocking::get(url) {
            Ok(res) => match res.text() {
                Ok(text) => {
                    parse_json_and_print(text);
                }
                Err(err) => eprintln!("{}", err),
            },
            Err(err) => eprintln!("{}", err),
        },
        _ => unreachable!(),
    }
}

fn main() {
    let stdin = io::stdin();
    let mut nonblock_stdin = NonBlockingReader::from_fd(stdin).unwrap();

    while !nonblock_stdin.is_eof() {
        let mut buffer = String::new();
        nonblock_stdin
            .read_available_to_string(&mut buffer)
            .unwrap();

        if !buffer.is_empty() {
            parse_json_and_print(buffer);
            break;
        } else {
            cli();
            break;
        }
    }
}
