use std::str::FromStr;
use std::sync::OnceLock;

use num_bigint::BigInt;
use regex::Regex;

use super::span::Span;
use super::token::Token;

#[derive(Debug, Clone, PartialEq, thiserror::Error)]
pub enum LexicalError {
    #[error("unexpected character: {0}")]
    UnexpectedCharacter(char),

    #[error("unexpected end of file")]
    UnexpectedEndOfFile,

    #[error("undefined escape: '\\{0}'")]
    UndefinedEscape(char),
}

// Success: Ok(Some((token, bytes_consumed)))
// Failure: Err(LexicalError)
// EOF:     Ok(None)
type LexResult = Result<Option<(Token, usize)>, LexicalError>;

fn ok(token: Token, bytes_consumed: usize) -> LexResult {
    Ok(Some((token, bytes_consumed)))
}

fn err(e: LexicalError) -> LexResult {
    Err(e)
}

fn eof() -> LexResult {
    Ok(None)
}

macro_rules! static_regex {
    ($pattern:expr) => {{
        static RE: OnceLock<Regex> = OnceLock::new();
        RE.get_or_init(|| Regex::new($pattern).unwrap())
    }};
}

fn lex_string_literal(input: &str) -> LexResult {
    let mut chars = input.chars();
    assert_eq!(chars.next(), Some('"'));

    let mut string_closed = false;
    let mut buffer = String::new();
    while let Some(c) = chars.next() {
        match c {
            '\\' => {
                let Some(c2) = chars.next() else {
                    return Err(LexicalError::UnexpectedEndOfFile);
                };
                match c2 {
                    '"' => buffer.push('"'),
                    '\\' => buffer.push('\\'),
                    '/' => buffer.push('/'),
                    'n' => buffer.push('\n'),
                    'r' => buffer.push('\r'),
                    't' => buffer.push('\t'),
                    _ => return Err(LexicalError::UndefinedEscape(c2)),
                }
            }
            '"' => {
                string_closed = true;
                break;
            }
            _ => buffer.push(c),
        }
    }
    if !string_closed {
        return Err(LexicalError::UnexpectedEndOfFile);
    }

    let bytes_consumed = input.len() - chars.as_str().len();
    ok(Token::String(buffer.into()), bytes_consumed)
}

// Cuts a single token from `input` and returns `(token, bytes_consumed)`.
fn lex_one(input: &str) -> LexResult {
    let Some(first) = input.chars().next() else {
        return eof();
    };
    match first {
        '.' => return ok(Token::Dot, 1),
        ':' => return ok(Token::Colon, 1),
        ';' => return ok(Token::Semicolon, 1),
        ',' => return ok(Token::Comma, 1),
        '(' => return ok(Token::LParen, 1),
        ')' => return ok(Token::RParen, 1),
        '[' => return ok(Token::LBracket, 1),
        ']' => return ok(Token::RBracket, 1),
        '{' => return ok(Token::LBrace, 1),
        '}' => return ok(Token::RBrace, 1),
        '+' => return ok(Token::Plus, 1),
        '-' => {
            return if second(input) == Some('>') {
                ok(Token::RArrow, 2)
            } else {
                ok(Token::Minus, 1)
            }
        }
        '*' => return ok(Token::Asterisk, 1),
        '/' => return ok(Token::Slash, 1),
        '\\' => return ok(Token::Backslash, 1),
        '%' => return ok(Token::Percent, 1),
        '=' => {
            return if second(input) == Some('=') {
                ok(Token::EqEq, 2)
            } else {
                ok(Token::Eq, 1)
            };
        }
        '<' => return ok(Token::Lt, 1),
        '>' => return ok(Token::Gt, 1),
        '!' => {
            return if second(input) == Some('=') {
                ok(Token::NotEq, 2)
            } else {
                ok(Token::Exclamation, 1)
            };
        }
        '&' => {
            return if second(input) == Some('&') {
                ok(Token::AndAnd, 2)
            } else {
                ok(Token::Ampersand, 1)
            }
        }
        '|' => {
            return if second(input) == Some('|') {
                ok(Token::OrOr, 2)
            } else {
                ok(Token::Pipe, 1)
            }
        }
        '#' => return ok(Token::Sharp, 1),
        '$' => return ok(Token::Dollar, 1),
        '~' => return ok(Token::Tilde, 1),
        _ => {}
    }

    let re_identifier_or_reserved = static_regex!("^[a-zA-Z_][a-zA-Z0-9_]*");
    if let Some(m) = re_identifier_or_reserved.find(input) {
        let s = m.as_str();
        let token = match s {
            "true" => Token::True,
            "false" => Token::False,
            "if" => Token::If,
            "then" => Token::Then,
            "else" => Token::Else,
            "let" => Token::Let,
            "rec" => Token::Rec,
            "in" => Token::In,
            "T" => Token::T,
            "D" => Token::D,
            _ => Token::Identifier(s.to_owned().into()),
        };
        return ok(token, m.end());
    }

    let re_integer = static_regex!(r"^[0-9]+");
    if let Some(m) = re_integer.find(input) {
        let n = BigInt::from_str(m.as_str()).unwrap();
        return ok(Token::Int(n), m.end());
    }

    if input.starts_with('"') {
        return lex_string_literal(input);
    }

    err(LexicalError::UnexpectedCharacter(first))
}

// Same as `lex_one` except that it ignores leading whitespaces and comments.
fn lex_strip(input: &str) -> LexResult {
    #[rustfmt::skip]
    let re_whitespaces = static_regex!(r"(?x)^
        [\t\n\r\ ]*
        (
          ( -- .*? (\n|$)     # line comment
          | \{- (?s:.)*? -\}  # block comment
          )
          [\t\n\r\ ]*
        )*
    ");
    match re_whitespaces.find(input) {
        Some(m) if !m.is_empty() => {
            let r = lex_one(&input[m.end()..]);
            match r {
                Ok(Some((token, bytes_consumed))) => ok(token, m.end() + bytes_consumed),
                _ => r,
            }
        }
        _ => lex_one(input),
    }
}

// Returns the second character of `input`.
fn second(input: &str) -> Option<char> {
    let mut chars = input.chars();
    chars.next();
    chars.next()
}

pub fn lex(input: &str) -> Result<Vec<(Token, Span)>, LexicalError> {
    let mut ret = Vec::new();
    let mut bytes_consumed = 0;
    loop {
        match lex_strip(&input[bytes_consumed..]) {
            // Success
            Ok(Some((token, n))) => {
                let span = Span {
                    start: bytes_consumed,
                    end: bytes_consumed + n,
                };
                bytes_consumed = span.end;
                ret.push((token, span));
            }
            // Failure
            Err(e) => return Err(e),
            // EOF
            Ok(None) => return Ok(ret),
        }
    }
}
