use thiserror::Error;

#[derive(Debug, Clone, PartialEq)]
pub enum JsonToken {
    String(String),
    Number(String),
    Boolean(String),
    Null(String),
    OpenCurlyBracket,
    CloseCurlyBracket,
    OpenSquareBracket,
    CloseSquareBracket,
    Colon,
    Comma,
}

#[derive(Error, Debug, PartialEq)]
pub enum JsonTokenError {
    #[error("Expected end-of-string")]
    ExpectedEndOfString,
    #[error("Invalid token, got `{0}`")]
    InvalidToken(char),
}

fn is_number_char(c: char) -> bool {
    match c {
        '-' | '.' | '0' | '1' | '2' | '3' | '4' | '5' | '6' | '7' | '8' | '9' => true,
        _ => false,
    }
}

fn check_end_of_token_value(c: char) -> Option<JsonToken> {
    match c {
        ',' => Some(JsonToken::Comma),
        '}' => Some(JsonToken::CloseCurlyBracket),
        ']' => Some(JsonToken::CloseSquareBracket),
        _ => None,
    }
}

pub fn lexer(raw: String) -> Result<Vec<JsonToken>, JsonTokenError> {
    let mut vec: Vec<JsonToken> = vec![];

    let mut chars = raw.chars();

    while let Some(c) = chars.next() {
        match c {
            '{' => {
                vec.push(JsonToken::OpenCurlyBracket);
            }
            '}' => {
                vec.push(JsonToken::CloseCurlyBracket);
            }
            '[' => {
                vec.push(JsonToken::OpenSquareBracket);
            }
            ']' => {
                vec.push(JsonToken::CloseSquareBracket);
            }
            ':' => {
                vec.push(JsonToken::Colon);
            }
            ',' => {
                vec.push(JsonToken::Comma);
            }
            '"' => {
                let mut json_string = String::new();

                let mut done = false;
                while let Some(str_c) = chars.next() {
                    if str_c != '"' {
                        json_string.push(str_c);
                    } else {
                        done = true;
                        break;
                    }
                }

                if !done {
                    return Err(JsonTokenError::ExpectedEndOfString);
                }

                vec.push(JsonToken::String(json_string));
            }
            'f' => {
                let false_len = 5;
                let mut json_false = String::from(c);

                let mut letter_count = 1;
                while letter_count < false_len {
                    if let Some(false_c) = chars.next() {
                        json_false.push(false_c);
                    } else {
                        break;
                    }
                    letter_count += 1;
                }

                vec.push(JsonToken::Boolean(json_false));
            }
            't' => {
                let true_len = 4;
                let mut json_true = String::from(c);

                let mut letter_count = 1;
                while letter_count < true_len {
                    if let Some(true_c) = chars.next() {
                        json_true.push(true_c);
                    } else {
                        break;
                    }
                    letter_count += 1;
                }

                vec.push(JsonToken::Boolean(json_true));
            }
            'n' => {
                let null_len = 4;
                let mut json_null = String::from(c);

                let mut letter_count = 1;
                while letter_count < null_len {
                    if let Some(null_c) = chars.next() {
                        json_null.push(null_c);
                    } else {
                        break;
                    }
                    letter_count += 1;
                }

                vec.push(JsonToken::Null(json_null));
            }
            '-' | '.' | '0' | '1' | '2' | '3' | '4' | '5' | '6' | '7' | '8' | '9' => {
                let mut json_number = String::from(c);

                let mut end_token: Option<JsonToken> = None;
                while let Some(num_c) = chars.next() {
                    if is_number_char(num_c) {
                        json_number.push(num_c);
                    } else if let Some(t) = check_end_of_token_value(num_c) {
                        end_token = Some(t);
                        break;
                    } else {
                        return Err(JsonTokenError::InvalidToken(num_c));
                    }
                }

                vec.push(JsonToken::Number(json_number));
                if let Some(t) = end_token {
                    vec.push(t);
                }
            }
            ' ' | '\n' | '\t' => {
                // Ignore them
            }
            _ => {
                return Err(JsonTokenError::InvalidToken(c));
            }
        };
    }

    return Ok(vec);
}

#[cfg(test)]
mod tests {
    use super::{lexer, JsonToken, JsonTokenError};

    #[test]
    fn test_empty_input() -> Result<(), JsonTokenError> {
        let input = "".to_string();

        let tokens = lexer(input)?;
        let expected = vec![];

        assert_eq!(tokens, expected);

        Ok(())
    }

    #[test]
    fn test_curly_bracket_tokens() -> Result<(), JsonTokenError> {
        let input = "{}".to_string();

        let tokens = lexer(input)?;
        let expected = vec![JsonToken::OpenCurlyBracket, JsonToken::CloseCurlyBracket];

        assert_eq!(tokens, expected);

        Ok(())
    }

    #[test]
    fn test_square_bracket_tokens() -> Result<(), JsonTokenError> {
        let input = "[]".to_string();

        let tokens = lexer(input)?;
        let expected = vec![JsonToken::OpenSquareBracket, JsonToken::CloseSquareBracket];

        assert_eq!(tokens, expected);

        Ok(())
    }

    #[test]
    fn test_string_token() -> Result<(), JsonTokenError> {
        let input = "\"name\"".to_string();

        let tokens = lexer(input)?;
        let expected = vec![JsonToken::String("name".into())];

        assert_eq!(tokens, expected);

        Ok(())
    }

    #[test]
    fn test_missing_string_token_end() {
        let input = "\"name".to_string();
        assert_eq!(lexer(input), Err(JsonTokenError::ExpectedEndOfString));
    }

    #[test]
    fn test_true_token() -> Result<(), JsonTokenError> {
        let input = "true".to_string();

        let tokens = lexer(input.to_owned())?;
        let expected = vec![JsonToken::Boolean(input)];

        assert_eq!(tokens, expected);

        Ok(())
    }

    #[test]
    fn test_invalid_true_token() {
        let input = "truea".to_string();
        assert_eq!(lexer(input), Err(JsonTokenError::InvalidToken('a')));
    }

    #[test]
    fn test_false_token() -> Result<(), JsonTokenError> {
        let input = "false".to_string();

        let tokens = lexer(input)?;
        let expected = vec![JsonToken::Boolean("false".into())];

        assert_eq!(tokens, expected);

        Ok(())
    }

    #[test]
    fn test_invalid_false_token() {
        let input = "falseo".to_string();
        assert_eq!(lexer(input), Err(JsonTokenError::InvalidToken('o')));
    }

    #[test]
    fn test_null_token() -> Result<(), JsonTokenError> {
        let input = "null".to_string();

        let tokens = lexer(input)?;
        let expected = vec![JsonToken::Null("null".into())];

        assert_eq!(tokens, expected);

        Ok(())
    }

    #[test]
    fn test_invalid_null_token() {
        let input = "Null".to_string();
        assert_eq!(lexer(input), Err(JsonTokenError::InvalidToken('N')));
    }

    #[test]
    fn test_number_token() -> Result<(), JsonTokenError> {
        let input = "360".to_string();

        let tokens = lexer(input)?;
        let expected = vec![JsonToken::Number("360".into())];

        assert_eq!(tokens, expected);

        Ok(())
    }

    #[test]
    fn test_invalid_number_token() {
        let input = "360f".to_string();
        assert_eq!(lexer(input), Err(JsonTokenError::InvalidToken('f')));
    }

    #[test]
    fn test_comma_token() -> Result<(), JsonTokenError> {
        let input = ",".to_string();

        let tokens = lexer(input)?;
        let expected = vec![JsonToken::Comma];

        assert_eq!(tokens, expected);

        Ok(())
    }

    #[test]
    fn test_colon_token() -> Result<(), JsonTokenError> {
        let input = ":".to_string();

        let tokens = lexer(input)?;
        let expected = vec![JsonToken::Colon];

        assert_eq!(tokens, expected);

        Ok(())
    }

    #[test]
    fn test_json_tokens() -> Result<(), JsonTokenError> {
        let input = "[{\"money\": null, \"age\": 20}, true, false]".to_string();

        let tokens = lexer(input)?;
        let expected = vec![
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

        assert_eq!(tokens, expected);

        Ok(())
    }
}
