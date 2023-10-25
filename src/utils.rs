use crate::{
    lexer::{lexer, JsonToken},
    parser::{parser, JsonValue},
};

fn parse_json(text: String) -> anyhow::Result<(Vec<JsonToken>, JsonValue)> {
    let tokens = lexer(text)?;
    let json = parser(&tokens)?;
    return Ok((tokens, json));
}

pub fn parse_json_and_print(text: String) {
    match parse_json(text) {
        Ok((tokens, json)) => {
            println!("Tokens: {:?}", tokens);
            println!("JSON: {:?}", json);
        }
        Err(err) => {
            eprintln!("Error: {}", err);
        }
    };
}
