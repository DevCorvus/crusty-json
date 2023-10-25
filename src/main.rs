mod lexer;
mod parser;
mod utils;

use std::env;
use utils::parse_json;

fn main() {
    let mut args = env::args();
    args.next(); // Skip first argument

    if let Some(input) = args.next() {
        match parse_json(input) {
            Ok((tokens, json)) => {
                println!("Tokens: {:?}", tokens);
                println!("JSON: {:?}", json);
            }
            Err(err) => {
                println!("Error: {}", err);
            }
        };
    } else {
        println!("Missing input");
    }
}
