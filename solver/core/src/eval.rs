use std::{
    fmt::{Debug, Display},
    num::ParseIntError,
    rc::Rc,
};

use once_cell::sync::Lazy;
use regex::Regex;

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum Value {
    Bool(bool),
    Int(i64),
    String(String),
    Closure(Rc<Frame>, usize, Rc<Node>),
}

impl Display for Value {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Value::Bool(b) => write!(f, "{b}"),
            Value::Int(i) => write!(f, "{i}"),
            Value::String(s) => write!(f, "{:?}", s),
            Value::Closure(_, v, _) => write!(f, "<closure v{v}>"),
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
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

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
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
    Lambda(usize, Rc<Node>),
}

impl Display for Node {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Node::Literal(v) => write!(f, "{v}"),
            Node::UnaryOp(op, node) => write!(f, "{op}{node}"),
            Node::BinOp(op, l, r) => write!(f, "({l} {op} {r})"),
            Node::If(cond, then, else_) => write!(f, "if {cond} then {then} else {else_}"),
            Node::Lambda(var, body) => write!(f, "[λ v{var}. {body}]"),
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

pub fn tokenize(input: &str) -> Vec<Token> {
    static RE_SPACES: Lazy<Regex> = Lazy::new(|| Regex::new(r"\s+").unwrap());
    RE_SPACES
        .split(input.trim())
        .map(|s| s.to_owned())
        .collect()
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

pub fn parse(tokens: &[Token]) -> ParseResult<(Box<Node>, &[Token])> {
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
            ok(Node::Lambda(var, Rc::new(*expr)), rest)
        }
        'v' => {
            let var = parse_integer_raw(body)? as usize;
            ok(Node::Variable(var), &tokens[1..])
        }
        _ => Err(ParseError::UnknownIndicator(indicator)),
    }
}

#[derive(Debug, thiserror::Error)]
pub enum EvalError {
    #[error("type error: expected a value of type {0}, but the given value is {1}")]
    TypeError(String, String),

    #[error("value error: {0}")]
    ValueError(String),

    #[error("undefined variable: v{0}")]
    UndefinedVariable(usize),
}

type EvalResult<T> = Result<T, EvalError>;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Frame {
    variables: im_rc::HashMap<usize, Rc<Value>>,
}

impl Frame {
    pub fn new() -> Self {
        Self {
            variables: im_rc::HashMap::new(),
        }
    }

    pub fn lookup(&self, var: usize) -> Option<Rc<Value>> {
        self.variables.get(&var).cloned()
    }

    pub fn with_variable(&self, var: usize, value: Rc<Value>) -> Frame {
        Self {
            variables: self.variables.update(var, value),
        }
    }
}

macro_rules! type_check {
    ($value:ident, $pattern:pat, $expected:expr) => {
        let $pattern = $value else {
            return Err(EvalError::TypeError($expected.into(), $value.to_string()));
        };
    };
}

fn eval_unary_op(frame: Rc<Frame>, op: UnaryOp, operand: &Node) -> EvalResult<Value> {
    let inner_value = eval(frame, operand)?;
    match op {
        UnaryOp::Neg => {
            type_check!(inner_value, Value::Int(i), "Int");
            Ok(Value::Int(-i))
        }
        UnaryOp::Not => {
            type_check!(inner_value, Value::Bool(b), "Bool");
            Ok(Value::Bool(!b))
        }
        UnaryOp::IntToString => {
            type_check!(inner_value, Value::Int(i), "Int");
            Ok(Value::String(i.to_string()))
        }
        UnaryOp::StringToInt => {
            type_check!(inner_value, Value::String(s), "String");
            let i: i64 = s
                .parse()
                .map_err(|e: ParseIntError| EvalError::ValueError(e.to_string()))?;
            Ok(Value::Int(i))
        }
    }
}

fn eval_apply(frame: Rc<Frame>, func: &Node, arg: &Node) -> EvalResult<Value> {
    // 本来は遅延評価しないといけない
    let f_value = eval(Rc::clone(&frame), func)?;
    let a_value = eval(frame, arg)?;
    type_check!(f_value, Value::Closure(c_frame, c_var, c_body), "Closure");
    let new_frame = Rc::new(c_frame.with_variable(c_var, Rc::new(a_value)));
    eval(new_frame, &c_body)
}

fn eval_bin_op(frame: Rc<Frame>, op: BinOp, lhs: &Node, rhs: &Node) -> EvalResult<Value> {
    // Apply だけは評価順序が違うので別で実装する
    if op == BinOp::Apply {
        return eval_apply(frame, lhs, rhs);
    }
    let l_value = eval(Rc::clone(&frame), lhs)?;
    let r_value = eval(frame, rhs)?;
    match op {
        BinOp::Add => {
            type_check!(l_value, Value::Int(l), "Int");
            type_check!(r_value, Value::Int(r), "Int");
            Ok(Value::Int(l + r))
        }
        BinOp::Sub => {
            type_check!(l_value, Value::Int(l), "Int");
            type_check!(r_value, Value::Int(r), "Int");
            Ok(Value::Int(l - r))
        }
        BinOp::Mul => {
            type_check!(l_value, Value::Int(l), "Int");
            type_check!(r_value, Value::Int(r), "Int");
            Ok(Value::Int(l * r))
        }
        BinOp::Div => {
            type_check!(l_value, Value::Int(l), "Int");
            type_check!(r_value, Value::Int(r), "Int");
            Ok(Value::Int(l / r)) // TODO: 丸め方向が正しいかチェックする
        }
        BinOp::Mod => {
            type_check!(l_value, Value::Int(l), "Int");
            type_check!(r_value, Value::Int(r), "Int");
            Ok(Value::Int(l % r)) // TODO: 負の数に対する挙動が正しいかチェックする
        }
        BinOp::Lt => {
            type_check!(l_value, Value::Int(l), "Int");
            type_check!(r_value, Value::Int(r), "Int");
            Ok(Value::Bool(l < r))
        }
        BinOp::Gt => {
            type_check!(l_value, Value::Int(l), "Int");
            type_check!(r_value, Value::Int(r), "Int");
            Ok(Value::Bool(l > r))
        }
        BinOp::Eq => Ok(Value::Bool(l_value == r_value)),
        BinOp::Or => {
            type_check!(l_value, Value::Bool(l), "Bool");
            type_check!(r_value, Value::Bool(r), "Bool");
            Ok(Value::Bool(l || r))
        }
        BinOp::And => {
            type_check!(l_value, Value::Bool(l), "Bool");
            type_check!(r_value, Value::Bool(r), "Bool");
            Ok(Value::Bool(l && r))
        }
        BinOp::Concat => {
            type_check!(l_value, Value::String(mut l), "String");
            type_check!(r_value, Value::String(r), "String");
            l.push_str(&r);
            Ok(Value::String(l))
        }
        BinOp::Take => {
            type_check!(l_value, Value::String(s), "String");
            type_check!(r_value, Value::Int(n), "Int");
            let ret = s.chars().take(n as usize).collect(); // TODO: パフォーマンスの問題があるかも
            Ok(Value::String(ret))
        }
        BinOp::Drop => {
            type_check!(l_value, Value::String(s), "String");
            type_check!(r_value, Value::Int(n), "Int");
            let ret = s.chars().skip(n as usize).collect(); // TODO: パフォーマンスの問題があるかも
            Ok(Value::String(ret))
        }
        BinOp::Apply => unreachable!(),
    }
}

fn eval_if(frame: Rc<Frame>, cond: &Node, then: &Node, else_: &Node) -> EvalResult<Value> {
    let cond_value = eval(Rc::clone(&frame), cond)?;
    type_check!(cond_value, Value::Bool(b), "Bool");
    if b {
        eval(frame, then)
    } else {
        eval(frame, else_)
    }
}

fn eval_lambda(frame: Rc<Frame>, var: usize, body: Rc<Node>) -> EvalResult<Value> {
    Ok(Value::Closure(frame, var, body))
}

fn eval_variable(frame: Rc<Frame>, var: usize) -> EvalResult<Value> {
    match frame.lookup(var) {
        Some(value) => Ok((*value).clone()), // TODO: この clone はパフォーマンス的にまずいかも
        None => Err(EvalError::UndefinedVariable(var)),
    }
}

pub fn eval(frame: Rc<Frame>, node: &Node) -> EvalResult<Value> {
    match node {
        Node::Literal(value) => Ok(value.clone()),
        Node::UnaryOp(op, operand) => eval_unary_op(frame, *op, operand),
        Node::BinOp(op, lhs, rhs) => eval_bin_op(frame, *op, lhs, rhs),
        Node::If(cond, then, else_) => eval_if(frame, cond, then, else_),
        Node::Lambda(var, body) => eval_lambda(frame, *var, Rc::clone(body)),
        Node::Variable(var) => eval_variable(frame, *var),
    }
}
