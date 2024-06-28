use std::fmt::{Debug, Display};

use once_cell::sync::Lazy;
use regex::Regex;

#[derive(Debug, PartialEq, Eq)]
pub enum Value {
    Bool(bool),
    Int(i64),
    String(String),
    Closure(usize),
}

impl Display for Value {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Value::Bool(b) => write!(f, "{b}"),
            Value::Int(i) => write!(f, "{i}"),
            Value::String(s) => write!(f, "{:?}", s),
            Value::Closure(v) => write!(f, "<closure v{v}>"),
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
pub enum UnaryOp {
    Neg,
    Not,
    StringToInt,
    IntToString,
}

impl Display for UnaryOp {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            UnaryOp::Neg => write!(f, "-"),
            UnaryOp::Not => write!(f, "!"),
            UnaryOp::StringToInt => write!(f, "#"),
            UnaryOp::IntToString => write!(f, "$"),
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
pub enum BinOp {
    Add,
    Sub,
    Mul,
    Div,
    Mod,
    Lt,
    Gt,
    Eq,
    Or,
    And,
    Concat,
    Take,  // Take first x chars of string y
    Drop,  // Drop first x chars of string y
    Apply, // Apply term x to y
}

impl Display for BinOp {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            BinOp::Add => write!(f, "+"),
            BinOp::Sub => write!(f, "-"),
            BinOp::Mul => write!(f, "*"),
            BinOp::Div => write!(f, "/"),
            BinOp::Mod => write!(f, "%"),
            BinOp::Lt => write!(f, "<"),
            BinOp::Gt => write!(f, ">"),
            BinOp::Eq => write!(f, "=="),
            BinOp::Or => write!(f, "||"),
            BinOp::And => write!(f, "&&"),
            BinOp::Concat => write!(f, ".."),
            BinOp::Take => write!(f, "`take`"),
            BinOp::Drop => write!(f, "`drop`"),
            BinOp::Apply => write!(f, "$"),
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
pub enum Node {
    Literal(Value),
    Variable(usize),
    UnaryOp(UnaryOp, Box<Node>),
    BinOp(BinOp, Box<Node>, Box<Node>),
    If(Box<Node>, Box<Node>, Box<Node>),
    Lambda(usize, Box<Node>),
}

impl Display for Node {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Node::Literal(v) => write!(f, "{v}"),
            Node::UnaryOp(op, node) => write!(f, "{op}{node}"),
            Node::BinOp(op, l, r) => write!(f, "({l} {op} {r})"),
            Node::If(cond, then, else_) => write!(f, "if {cond} then {then} else {else_}"),
            Node::Lambda(var, body) => write!(f, "λ v{var}: {body}"),
            Node::Variable(var) => write!(f, "v{var}"),
        }
    }
}

type Token = String;

#[derive(Debug, thiserror::Error)]
pub enum ParseError {
    #[error("unexpected EOF")]
    UnexpectedEof,

    #[error("unknown indicator: {0}")]
    UnknownIndicator(char),

    #[error("invalid integer literal: body={0}")]
    InvalidIntegerLiteral(String),

    #[error("invalid string literal: body={0}")]
    InvalidStringLiteral(String),

    #[error("invalid unary op: body={0}")]
    InvalidUnaryOp(String),

    #[error("invalid binary op: body={0}")]
    InvalidBinaryOp(String),
}

type ParseResult<T> = Result<T, ParseError>;

fn tokenize(input: &str) -> Vec<Token> {
    static RE_SPACES: Lazy<Regex> = Lazy::new(|| Regex::new(r"\s+").unwrap());
    RE_SPACES.split(input).map(|s| s.to_owned()).collect()
}

fn ok(node: Node, rest: &[Token]) -> ParseResult<(Box<Node>, &[Token])> {
    Ok((Box::new(node), rest))
}

fn parse_integer_raw(body: &str) -> ParseResult<i64> {
    let mut ret: i64 = 0;
    for c in body.chars() {
        let i = c as i64;
        if i < 33 || i >= 33 + 94 {
            return Err(ParseError::InvalidIntegerLiteral(body.into()));
        }
        ret = ret * 94 + (i - 33);
    }
    Ok(ret)
}

fn parse_integer(body: &str) -> ParseResult<Box<Node>> {
    let ret = parse_integer_raw(body)?;
    Ok(Box::new(Node::Literal(Value::Int(ret))))
}

#[test]
fn test_parse_integer() {
    let node = parse_integer("/6").unwrap();
    assert_eq!(*node, Node::Literal(Value::Int(1337)));
}

fn parse_string(body: &str) -> ParseResult<Box<Node>> {
    static TABLE: &'static [u8] = b"abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789!\"#$%&'()*+,-./:;<=>?@[\\]^_`|~ \n";
    let mut ret = String::new();
    for c in body.chars() {
        let i = c as usize;
        if i < 33 || i >= 33 + 94 {
            return Err(ParseError::InvalidStringLiteral(body.into()));
        }
        let decoded = char::from_u32(TABLE[i - 33] as u32).unwrap();
        ret.push(decoded);
    }
    Ok(Box::new(Node::Literal(Value::String(ret))))
}

#[test]
fn test_parse_string() {
    let node = parse_string("B%,,/}Q/2,$_").unwrap();
    assert_eq!(*node, Node::Literal(Value::String("Hello World!".into())));
}

fn parse_unary_op(body: &str) -> ParseResult<UnaryOp> {
    if body.len() != 1 {
        return Err(ParseError::InvalidUnaryOp(body.into()));
    }
    let op = body.chars().next().unwrap();
    match op {
        '-' => Ok(UnaryOp::Neg),
        '!' => Ok(UnaryOp::Not),
        '#' => Ok(UnaryOp::StringToInt),
        '$' => Ok(UnaryOp::IntToString),
        _ => Err(ParseError::InvalidUnaryOp(body.into())),
    }
}

fn parse_bin_op(body: &str) -> ParseResult<BinOp> {
    if body.len() != 1 {
        return Err(ParseError::InvalidUnaryOp(body.into()));
    }
    let op = body.chars().next().unwrap();
    match op {
        '+' => Ok(BinOp::Add),
        '-' => Ok(BinOp::Sub),
        '*' => Ok(BinOp::Mul),
        '/' => Ok(BinOp::Div),
        '%' => Ok(BinOp::Mod),
        '<' => Ok(BinOp::Lt),
        '>' => Ok(BinOp::Gt),
        '=' => Ok(BinOp::Eq),
        '|' => Ok(BinOp::Or),
        '&' => Ok(BinOp::And),
        '.' => Ok(BinOp::Concat),
        'T' => Ok(BinOp::Take),
        'D' => Ok(BinOp::Drop),
        '$' => Ok(BinOp::Apply),
        _ => Err(ParseError::InvalidBinaryOp(body.into())),
    }
}

fn parse(tokens: &[Token]) -> ParseResult<(Box<Node>, &[Token])> {
    let Some(t) = tokens.first() else {
        return Err(ParseError::UnexpectedEof);
    };
    let mut it = t.chars();
    let indicator = it.next().expect("token must not be empty");
    let body = it.as_str();
    match indicator {
        'T' => ok(Node::Literal(Value::Bool(true)), &tokens[1..]),
        'F' => ok(Node::Literal(Value::Bool(false)), &tokens[1..]),
        'I' => parse_integer(body).map(|node| (node, &tokens[1..])),
        'S' => parse_string(body).map(|node| (node, &tokens[1..])),
        'U' => {
            let op = parse_unary_op(body)?;
            let (operand, rest) = parse(&tokens[1..])?;
            ok(Node::UnaryOp(op, operand), rest)
        }
        'B' => {
            let op = parse_bin_op(body)?;
            let (lhs, rest1) = parse(&tokens[1..])?;
            let (rhs, rest2) = parse(rest1)?;
            ok(Node::BinOp(op, lhs, rhs), rest2)
        }
        '?' => {
            let (cond, rest1) = parse(&tokens[1..])?;
            let (then, rest2) = parse(rest1)?;
            let (else_, rest3) = parse(rest2)?;
            ok(Node::If(cond, then, else_), rest3)
        }
        'L' => {
            let var = parse_integer_raw(body)? as usize;
            let (expr, rest) = parse(&tokens[1..])?;
            ok(Node::Lambda(var, expr), rest)
        }
        'v' => {
            let var = parse_integer_raw(body)? as usize;
            ok(Node::Variable(var), &tokens[1..])
        }
        _ => Err(ParseError::UnknownIndicator(indicator)),
    }
}

pub fn eval(input: &str) -> ParseResult<Value> {
    let tokens = tokenize(input);
    let (ast, rest) = parse(&tokens)?;
    if !rest.is_empty() {
        return Err(ParseError::UnexpectedEof);
    }
    println!("AST: {ast}");
    todo!()
}
