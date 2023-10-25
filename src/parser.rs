use crate::lexer::JsonToken;
use std::collections::HashMap;
use thiserror::Error;

#[derive(Debug, PartialEq)]
pub enum JsonValue {
    String(String),
    Number(f64),
    Boolean(bool),
    Null,
    Array(Vec<JsonValue>),
    Object(HashMap<String, JsonValue>),
}

#[derive(Error, Debug, PartialEq)]
pub enum JsonParseError {
    #[error("No tokens to parse from")]
    NoTokens,
    #[error("Expected object or array as root, got `{0:?}`")]
    ExpectedObjectOrArrayAsRoot(JsonToken),
    #[error("Expected end-of-object")]
    ExpectedEndOfObject,
    #[error("Expected end-of-array")]
    ExpectedEndOfArray,
    #[error("Expected object key, got `{0:?}`")]
    ExpectedObjectKey(JsonToken),
    #[error("Expected colon after key, got `{0:?}`")]
    ExpectedColonAfterKey(Option<JsonToken>),
    #[error("Expected comma or end-of-object, got `{0:?}`")]
    ExpectedCommaOrEndOfObject(Option<JsonToken>),
    #[error("Expected comma or end-of-array, got `{0:?}`")]
    ExpectedCommaOrEndOfArray(Option<JsonToken>),
    #[error("Invalid json value, got `{0:?}`")]
    InvalidValue(Option<JsonToken>),
    #[error("Invalid json number, got `{0}`")]
    InvalidNumberValue(String),
    #[error("Invalid json boolean, got `{0}`")]
    InvalidBooleanValue(String),
    #[error("Invalid json null, got `{0}`")]
    InvalidNullValue(String),
    #[error("Trailing comma")]
    TrailingComma,
}

fn parse_value(
    token: Option<&JsonToken>,
    iter: &mut dyn Iterator<Item = &JsonToken>,
) -> Result<JsonValue, JsonParseError> {
    let value_token = match token {
        Some(v) => v,
        None => iter.next().ok_or(JsonParseError::InvalidValue(None))?,
    };

    match value_token {
        JsonToken::String(json_string) => {
            return Ok(JsonValue::String(json_string.to_string()));
        }
        JsonToken::Number(json_number) => match json_number.parse::<f64>() {
            Ok(number) => {
                return Ok(JsonValue::Number(number));
            }
            Err(_) => {
                return Err(JsonParseError::InvalidNumberValue(json_number.to_string()));
            }
        },
        JsonToken::Boolean(json_boolean) => match json_boolean.as_str() {
            "true" => {
                return Ok(JsonValue::Boolean(true));
            }
            "false" => {
                return Ok(JsonValue::Boolean(false));
            }
            _ => {
                return Err(JsonParseError::InvalidBooleanValue(
                    json_boolean.to_string(),
                ));
            }
        },
        JsonToken::Null(json_null) => match json_null.as_str() {
            "null" => {
                return Ok(JsonValue::Null);
            }
            _ => {
                return Err(JsonParseError::InvalidNullValue(json_null.to_string()));
            }
        },
        JsonToken::OpenCurlyBracket => {
            return Ok(parse_object(iter)?);
        }
        JsonToken::OpenSquareBracket => {
            return Ok(parse_array(iter)?);
        }
        _ => {
            return Err(JsonParseError::InvalidValue(Some(value_token.to_owned())));
        }
    };
}

fn parse_object(iter: &mut dyn Iterator<Item = &JsonToken>) -> Result<JsonValue, JsonParseError> {
    let mut obj: HashMap<String, JsonValue> = HashMap::new();

    let mut done = false;
    let mut comma_after_value = false;

    while let Some(token) = iter.next() {
        if let JsonToken::CloseCurlyBracket = token {
            if comma_after_value {
                return Err(JsonParseError::TrailingComma);
            } else {
                done = true;
                break;
            }
        }

        let key = match token {
            JsonToken::String(json_string) => json_string.to_string(),
            _ => {
                return Err(JsonParseError::ExpectedObjectKey(token.to_owned()));
            }
        };

        match iter.next() {
            Some(t) => {
                if let JsonToken::Colon = t {
                    // Do nothing
                } else {
                    return Err(JsonParseError::ExpectedColonAfterKey(Some(t.to_owned())));
                }
            }
            None => {
                return Err(JsonParseError::ExpectedColonAfterKey(None));
            }
        };

        let value = parse_value(None, iter)?;
        obj.insert(key, value);

        match iter.next() {
            Some(t) => match t.to_owned() {
                JsonToken::Comma => {
                    comma_after_value = true;
                    continue;
                }
                JsonToken::CloseCurlyBracket => {
                    done = true;
                    break;
                }
                _ => {
                    return Err(JsonParseError::ExpectedCommaOrEndOfObject(Some(
                        t.to_owned(),
                    )));
                }
            },
            None => {
                return Err(JsonParseError::ExpectedCommaOrEndOfObject(None));
            }
        }
    }

    if done {
        return Ok(JsonValue::Object(obj));
    } else {
        return Err(JsonParseError::ExpectedEndOfObject);
    }
}

fn parse_array(iter: &mut dyn Iterator<Item = &JsonToken>) -> Result<JsonValue, JsonParseError> {
    let mut arr: Vec<JsonValue> = Vec::new();

    let mut done = false;
    let mut comma_after_value = false;

    while let Some(token) = iter.next() {
        if let JsonToken::CloseSquareBracket = token {
            if comma_after_value {
                return Err(JsonParseError::TrailingComma);
            } else {
                done = true;
                break;
            }
        }

        let value = parse_value(Some(token), iter)?;
        arr.push(value);

        match iter.next() {
            Some(t) => match t.to_owned() {
                JsonToken::Comma => {
                    comma_after_value = true;
                    continue;
                }
                JsonToken::CloseSquareBracket => {
                    done = true;
                    break;
                }
                _ => {
                    return Err(JsonParseError::ExpectedCommaOrEndOfArray(Some(
                        t.to_owned(),
                    )));
                }
            },
            None => {
                return Err(JsonParseError::ExpectedCommaOrEndOfArray(None));
            }
        }
    }

    if done {
        return Ok(JsonValue::Array(arr));
    } else {
        return Err(JsonParseError::ExpectedEndOfArray);
    }
}

pub fn parser(tokens: &Vec<JsonToken>) -> Result<JsonValue, JsonParseError> {
    let mut iter = tokens.iter();

    if let Some(first_token) = iter.next() {
        match first_token {
            JsonToken::OpenCurlyBracket => {
                return Ok(parse_object(&mut iter)?);
            }
            JsonToken::OpenSquareBracket => {
                return Ok(parse_array(&mut iter)?);
            }
            _ => {
                return Err(JsonParseError::ExpectedObjectOrArrayAsRoot(
                    first_token.to_owned(),
                ));
            }
        };
    } else {
        return Err(JsonParseError::NoTokens);
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use crate::lexer::JsonToken;

    use super::{parser, JsonParseError, JsonValue};

    #[test]
    fn test_empty_input() {
        let input = vec![];
        assert_eq!(parser(&input), Err(JsonParseError::NoTokens));
    }

    #[test]
    fn test_invalid_root() {
        let invalid_token = JsonToken::String("fulano".into());
        let input = vec![invalid_token.to_owned()];

        assert_eq!(
            parser(&input),
            Err(JsonParseError::ExpectedObjectOrArrayAsRoot(invalid_token))
        );
    }

    #[test]
    fn test_missing_object_close() {
        let input = vec![JsonToken::OpenCurlyBracket];
        assert_eq!(parser(&input), Err(JsonParseError::ExpectedEndOfObject));
    }

    #[test]
    fn test_missing_array_close() {
        let input = vec![JsonToken::OpenSquareBracket];
        assert_eq!(parser(&input), Err(JsonParseError::ExpectedEndOfArray));
    }

    #[test]
    fn test_missing_object_key() {
        let invalid_token = JsonToken::Number("360".into());

        let input = vec![
            JsonToken::OpenCurlyBracket,
            invalid_token.to_owned(),
            JsonToken::CloseCurlyBracket,
        ];

        assert_eq!(
            parser(&input),
            Err(JsonParseError::ExpectedObjectKey(invalid_token))
        );
    }

    #[test]
    fn test_missing_colon_after_object_key() {
        let input = vec![
            JsonToken::OpenCurlyBracket,
            JsonToken::String("name".into()),
        ];

        assert_eq!(
            parser(&input),
            Err(JsonParseError::ExpectedColonAfterKey(None))
        );
    }

    #[test]
    fn test_missing_object_value_after_colon() {
        let input = vec![
            JsonToken::OpenCurlyBracket,
            JsonToken::String("name".into()),
            JsonToken::Colon,
        ];

        assert_eq!(parser(&input), Err(JsonParseError::InvalidValue(None)));
    }

    #[test]
    fn test_invalid_object_value_after_colon() {
        let invalid_token = JsonToken::CloseCurlyBracket;

        let input = vec![
            JsonToken::OpenCurlyBracket,
            JsonToken::String("name".into()),
            JsonToken::Colon,
            invalid_token.to_owned(),
        ];

        assert_eq!(
            parser(&input),
            Err(JsonParseError::InvalidValue(Some(invalid_token)))
        );
    }

    #[test]
    fn test_invalid_array_value() {
        let invalid_token = JsonToken::Colon;

        let input = vec![JsonToken::OpenSquareBracket, invalid_token.to_owned()];

        assert_eq!(
            parser(&input),
            Err(JsonParseError::InvalidValue(Some(invalid_token)))
        );
    }

    #[test]
    fn test_missing_end_after_object_value() {
        let input = vec![
            JsonToken::OpenCurlyBracket,
            JsonToken::String("name".into()),
            JsonToken::Colon,
            JsonToken::String("fulano".into()),
        ];

        assert_eq!(
            parser(&input),
            Err(JsonParseError::ExpectedCommaOrEndOfObject(None))
        );
    }

    #[test]
    fn test_missing_end_after_array_value() {
        let input = vec![
            JsonToken::OpenSquareBracket,
            JsonToken::String("name".into()),
        ];

        assert_eq!(
            parser(&input),
            Err(JsonParseError::ExpectedCommaOrEndOfArray(None))
        );
    }

    #[test]
    fn test_invalid_number() {
        let invalid_number = String::from("4-.5");

        let input = vec![
            JsonToken::OpenSquareBracket,
            JsonToken::Number(invalid_number.to_owned()),
        ];

        assert_eq!(
            parser(&input),
            Err(JsonParseError::InvalidNumberValue(invalid_number))
        );
    }

    #[test]
    fn test_invalid_true() {
        let invalid_true = String::from("trua");

        let input = vec![
            JsonToken::OpenSquareBracket,
            JsonToken::Boolean(invalid_true.to_owned()),
        ];

        assert_eq!(
            parser(&input),
            Err(JsonParseError::InvalidBooleanValue(invalid_true))
        );
    }

    #[test]
    fn test_invalid_false() {
        let invalid_false = String::from("falso");

        let input = vec![
            JsonToken::OpenSquareBracket,
            JsonToken::Boolean(invalid_false.to_owned()),
        ];

        assert_eq!(
            parser(&input),
            Err(JsonParseError::InvalidBooleanValue(invalid_false))
        );
    }

    #[test]
    fn test_invalid_null() {
        let invalid_null = String::from("nulo");

        let input = vec![
            JsonToken::OpenSquareBracket,
            JsonToken::Null(invalid_null.to_owned()),
        ];

        assert_eq!(
            parser(&input),
            Err(JsonParseError::InvalidNullValue(invalid_null))
        );
    }

    #[test]
    fn test_trailing_comma_in_object() {
        let input = vec![
            JsonToken::OpenSquareBracket,
            JsonToken::Null("null".into()),
            JsonToken::Comma,
            JsonToken::CloseSquareBracket,
        ];

        assert_eq!(parser(&input), Err(JsonParseError::TrailingComma));
    }

    #[test]
    fn test_trailing_comma_in_array() {
        let input = vec![
            JsonToken::OpenCurlyBracket,
            JsonToken::String("name".into()),
            JsonToken::Colon,
            JsonToken::String("fulano".into()),
            JsonToken::Comma,
            JsonToken::CloseCurlyBracket,
        ];

        assert_eq!(parser(&input), Err(JsonParseError::TrailingComma));
    }

    #[test]
    fn test_parser() -> Result<(), JsonParseError> {
        let input = vec![
            JsonToken::OpenSquareBracket,
            JsonToken::OpenCurlyBracket,
            JsonToken::String("money".into()),
            JsonToken::Colon,
            JsonToken::Null("null".into()),
            JsonToken::Comma,
            JsonToken::String("age".into()),
            JsonToken::Colon,
            JsonToken::Number("20".into()),
            JsonToken::CloseCurlyBracket,
            JsonToken::Comma,
            JsonToken::Boolean("true".into()),
            JsonToken::Comma,
            JsonToken::Boolean("false".into()),
            JsonToken::CloseSquareBracket,
        ];

        let mut obj: HashMap<String, JsonValue> = HashMap::new();
        obj.insert("money".into(), JsonValue::Null);
        obj.insert("age".into(), JsonValue::Number(20.0));

        let arr = vec![
            JsonValue::Object(obj),
            JsonValue::Boolean(true),
            JsonValue::Boolean(false),
        ];

        let expected = JsonValue::Array(arr);

        let json = parser(&input)?;

        assert_eq!(json, expected);

        Ok(())
    }
}
