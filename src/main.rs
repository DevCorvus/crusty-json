mod lexer;
mod parser;
mod utils;

use clap::{ArgGroup, Parser};
use nonblock::NonBlockingReader;
use std::{fs, io, path::PathBuf};
use utils::parse_json_and_print;

/// Crusty JSON parser
#[derive(Parser)]
#[clap(group = ArgGroup::new("input").required(true).args(&["json", "file"]))]
struct Args {
    /// In-line json to parse from
    #[clap(conflicts_with = "file")]
    json: Option<String>,

    /// Path to json file to parse from
    #[clap(short, long, conflicts_with = "json")]
    file: Option<PathBuf>,
}

fn cli() {
    let args = Args::parse();

    if let Some(json) = args.json {
        parse_json_and_print(json);
    } else if let Some(file_path) = args.file {
        match fs::read_to_string(file_path) {
            Ok(file_content) => {
                parse_json_and_print(file_content);
            }
            Err(err) => {
                eprintln!("{}", err);
            }
        }
    } else {
        unreachable!();
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
