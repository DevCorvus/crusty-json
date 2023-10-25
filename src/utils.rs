use crate::{
    lexer::{lexer, JsonToken},
    parser::{parser, JsonValue},
};

pub fn parse_json(text: String) -> anyhow::Result<(Vec<JsonToken>, JsonValue)> {
    let tokens = lexer(text)?;
    let json = parser(&tokens)?;
    return Ok((tokens, json));
}
