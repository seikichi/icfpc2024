use std::fmt::{self, Display, Formatter};

use num_bigint::BigInt;

use super::symbol::Symbol;

#[derive(Clone, PartialEq, Debug)]
pub enum Token {
    True,
    False,
    Int(BigInt),
    String(String),
    Identifier(Symbol),

    If,
    Then,
    Else,
    Let,
    Rec,

    Dot,
    Colon,
    Semicolon,
    Comma,
    LParen,
    RParen,
    LBracket,
    RBracket,
    LBrace,
    RBrace,
    Plus,
    Minus,
    RArrow,
    Asterisk,
    Slash,
    Backslash,
    Percent,
    Eq,
    EqEq,
    Lt,
    Gt,
    Exclamation,
    NotEq,
    Ampersand,
    AndAnd,
    Pipe,
    OrOr,
    Sharp,
    Dollar,
    T,
    D,
}

impl Display for Token {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}
