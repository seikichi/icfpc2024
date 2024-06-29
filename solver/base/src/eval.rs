use std::{
    collections::HashMap,
    fmt::{Debug, Display},
    rc::Rc,
};

use num_bigint::BigInt;
use num_traits::ToPrimitive;
use once_cell::sync::{Lazy, OnceCell};
use regex::Regex;

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum Value {
    Bool(bool),
    Int(BigInt),
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
    Take, // Take first x chars of string y
    Drop, // Drop first x chars of string y
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
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
pub enum Node {
    Literal(Value),
    Variable(usize),
    UnaryOp(UnaryOp, Box<Node>),
    BinOp(BinOp, Box<Node>, Box<Node>),
    Apply(Box<Node>, Rc<Node>),
    If(Box<Node>, Box<Node>, Box<Node>),
    Lambda(usize, Rc<Node>),
}

impl Display for Node {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Node::Literal(v) => write!(f, "{v}"),
            Node::UnaryOp(op, node) => write!(f, "{op}{node}"),
            Node::BinOp(op, l, r) => write!(f, "({l} {op} {r})"),
            Node::Apply(func, arg) => write!(f, "({func} $ {arg})"),
            Node::If(cond, then, else_) => write!(f, "(if {cond} then {then} else {else_})"),
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

fn parse_integer_raw(body: &str) -> ParseResult<BigInt> {
    let mut ret: BigInt = BigInt::ZERO;
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
    assert_eq!(*node, Node::Literal(Value::Int(1337.into())));
}

static TABLE: &'static [u8] = b"abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789!\"#$%&'()*+,-./:;<=>?@[\\]^_`|~ \n";

static RTABLE: Lazy<HashMap<u8, usize>> = Lazy::new(|| {
    let mut m = HashMap::new();
    for (i, c) in TABLE.iter().enumerate() {
        m.insert(*c, i);
    }
    m
});

fn parse_string(body: &str) -> ParseResult<Box<Node>> {
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
            if body == "$" {
                let (lhs, rest1) = parse(&tokens[1..])?;
                let (rhs, rest2) = parse(rest1)?;
                ok(Node::Apply(lhs, Rc::new(*rhs)), rest2)
            } else {
                let op = parse_bin_op(body)?;
                let (lhs, rest1) = parse(&tokens[1..])?;
                let (rhs, rest2) = parse(rest1)?;
                ok(Node::BinOp(op, lhs, rhs), rest2)
            }
        }
        '?' => {
            let (cond, rest1) = parse(&tokens[1..])?;
            let (then, rest2) = parse(rest1)?;
            let (else_, rest3) = parse(rest2)?;
            ok(Node::If(cond, then, else_), rest3)
        }
        'L' => {
            let var = parse_integer_raw(body)?.to_usize().unwrap();
            let (expr, rest) = parse(&tokens[1..])?;
            ok(Node::Lambda(var, Rc::new(*expr)), rest)
        }
        'v' => {
            let var = parse_integer_raw(body)?.to_usize().unwrap();
            ok(Node::Variable(var), &tokens[1..])
        }
        _ => Err(ParseError::UnknownIndicator(indicator)),
    }
}

#[derive(Debug, PartialEq, Eq, thiserror::Error)]
pub enum EvalError {
    #[error("{0}: type error: expected a value of type {1}, but the given value is {2}")]
    TypeError(String, String, String),

    #[error("value error: {0}")]
    ValueError(String),

    #[error("undefined variable: v{0}")]
    UndefinedVariable(usize),
}

type EvalResult<T> = Result<T, EvalError>;

#[derive(Debug)]
pub struct Thunk {
    frame: Rc<Frame>,
    node: Rc<Node>,
    memo: OnceCell<Value>,
}

impl Thunk {
    pub fn new(frame: Rc<Frame>, node: Rc<Node>) -> Self {
        Self { frame, node, memo: OnceCell::new() }
    }

    pub fn force(&self) -> EvalResult<Value> {
        let value = self.memo.get_or_try_init(|| {
            eval(Rc::clone(&self.frame), &self.node)
        })?;
        Ok(value.clone())
    }
}

impl PartialEq for Thunk {
    fn eq(&self, other: &Self) -> bool {
        self.force() == other.force()
    }
}

impl Eq for Thunk {}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Frame {
    variables: im_rc::HashMap<usize, Rc<Thunk>>,
}

impl Frame {
    pub fn new() -> Self {
        Self {
            variables: im_rc::HashMap::new(),
        }
    }

    pub fn lookup(&self, var: usize) -> Option<Rc<Thunk>> {
        self.variables.get(&var).cloned()
    }

    pub fn with_variable(&self, var: usize, value: Rc<Thunk>) -> Frame {
        Self {
            variables: self.variables.update(var, value),
        }
    }
}

macro_rules! type_check {
    ($value:ident, $pattern:pat, $expected:expr, $context:expr) => {
        let $pattern = $value else {
            return Err(EvalError::TypeError(
                $context.into(),
                $expected.into(),
                $value.to_string(),
            ));
        };
    };
}

fn string_to_int_94(s: &str) -> EvalResult<BigInt> {
    let mut ret: BigInt = BigInt::ZERO;
    for c in s.bytes() {
        let index = RTABLE
            .get(&c)
            .ok_or_else(|| EvalError::ValueError(format!("string-to-int: invalid number: {s}")))?;
        ret = ret * 94 + *index as i64;
    }
    Ok(ret as BigInt)
}

fn int_to_string_94(mut i: BigInt) -> String {
    if i == BigInt::ZERO {
        return "!".into();
    }
    let mut neg = false;
    if i < BigInt::ZERO {
        neg = true;
        i = -i;
    }
    let mut ret = Vec::new();
    while i > BigInt::ZERO {
        let index = (i.clone() % 94usize).to_usize().unwrap();
        ret.push(TABLE[index]);
        i /= 94;
    }
    if neg {
        ret.push(b'-');
    }
    ret.reverse();
    String::from_utf8_lossy(&ret).to_string()
}

fn eval_unary_op(frame: Rc<Frame>, op: UnaryOp, operand: &Node) -> EvalResult<Value> {
    let inner_value = eval(frame, operand)?;
    match op {
        UnaryOp::Neg => {
            type_check!(inner_value, Value::Int(i), "Int", "neg");
            Ok(Value::Int(-i))
        }
        UnaryOp::Not => {
            type_check!(inner_value, Value::Bool(b), "Bool", "not");
            Ok(Value::Bool(!b))
        }
        UnaryOp::IntToString => {
            type_check!(inner_value, Value::Int(i), "Int", "int-to-string");
            let s = int_to_string_94(i);
            Ok(Value::String(s))
        }
        UnaryOp::StringToInt => {
            type_check!(inner_value, Value::String(s), "String", "string-to-int");
            let i = string_to_int_94(&s)?;
            Ok(Value::Int(i))
        }
    }
}

fn eval_apply(frame: Rc<Frame>, func: &Node, arg: Rc<Node>) -> EvalResult<Value> {
    let f_value = eval(Rc::clone(&frame), func)?;
    let a_value = Thunk::new(frame, arg);
    type_check!(
        f_value,
        Value::Closure(c_frame, c_var, c_body),
        "Closure",
        "apply"
    );
    let new_frame = Rc::new(c_frame.with_variable(c_var, Rc::new(a_value)));
    eval(new_frame, &c_body)
}

fn eval_bin_op(frame: Rc<Frame>, op: BinOp, lhs: &Node, rhs: &Node) -> EvalResult<Value> {
    let l_value = eval(Rc::clone(&frame), lhs)?;
    let r_value = eval(frame, rhs)?;
    match op {
        BinOp::Add => {
            type_check!(l_value, Value::Int(l), "Int", "add(lhs)");
            type_check!(r_value, Value::Int(r), "Int", "add(rhs)");
            Ok(Value::Int(l + r))
        }
        BinOp::Sub => {
            type_check!(l_value, Value::Int(l), "Int", "sub(lhs)");
            type_check!(r_value, Value::Int(r), "Int", "sub(rhs)");
            Ok(Value::Int(l - r))
        }
        BinOp::Mul => {
            type_check!(l_value, Value::Int(l), "Int", "mul(lhs)");
            type_check!(r_value, Value::Int(r), "Int", "mul(rhs)");
            Ok(Value::Int(l * r))
        }
        BinOp::Div => {
            type_check!(l_value, Value::Int(l), "Int", "div(lhs)");
            type_check!(r_value, Value::Int(r), "Int", "div(rhs)");
            Ok(Value::Int(l / r)) // TODO: 丸め方向が正しいかチェックする
        }
        BinOp::Mod => {
            type_check!(l_value, Value::Int(l), "Int", "mod(lhs)");
            type_check!(r_value, Value::Int(r), "Int", "mod(rhs)");
            Ok(Value::Int(l % r)) // TODO: 負の数に対する挙動が正しいかチェックする
        }
        BinOp::Lt => {
            type_check!(l_value, Value::Int(l), "Int", "lt(lhs)");
            type_check!(r_value, Value::Int(r), "Int", "lt(rhs)");
            Ok(Value::Bool(l < r))
        }
        BinOp::Gt => {
            type_check!(l_value, Value::Int(l), "Int", "gt(lhs)");
            type_check!(r_value, Value::Int(r), "Int", "gt(rhs)");
            Ok(Value::Bool(l > r))
        }
        BinOp::Eq => Ok(Value::Bool(l_value == r_value)),
        BinOp::Or => {
            type_check!(l_value, Value::Bool(l), "Bool", "or(lhs)");
            type_check!(r_value, Value::Bool(r), "Bool", "or(rhs)");
            Ok(Value::Bool(l || r))
        }
        BinOp::And => {
            type_check!(l_value, Value::Bool(l), "Bool", "and(lhs)");
            type_check!(r_value, Value::Bool(r), "Bool", "and(rhs)");
            Ok(Value::Bool(l && r))
        }
        BinOp::Concat => {
            type_check!(l_value, Value::String(mut l), "String", "concat(lhs)");
            type_check!(r_value, Value::String(r), "String", "concat(rhs)");
            l.push_str(&r);
            Ok(Value::String(l))
        }
        BinOp::Take => {
            type_check!(l_value, Value::Int(n), "Int", "take(lhs)");
            type_check!(r_value, Value::String(s), "String", "take(rhs)");
            let ret = s.chars().take(n.to_usize().unwrap()).collect(); // TODO: パフォーマンスの問題があるかも
            Ok(Value::String(ret))
        }
        BinOp::Drop => {
            type_check!(l_value, Value::Int(n), "Int", "drop(lhs)");
            type_check!(r_value, Value::String(s), "String", "drop(rhs)");
            let ret = s.chars().skip(n.to_usize().unwrap()).collect(); // TODO: パフォーマンスの問題があるかも
            Ok(Value::String(ret))
        }
    }
}

fn eval_if(frame: Rc<Frame>, cond: &Node, then: &Node, else_: &Node) -> EvalResult<Value> {
    let cond_value = eval(Rc::clone(&frame), cond)?;
    type_check!(cond_value, Value::Bool(b), "Bool", "if(cond)");
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
        Some(value) => Ok(value.force()?),
        None => Err(EvalError::UndefinedVariable(var)),
    }
}

pub fn eval(frame: Rc<Frame>, node: &Node) -> EvalResult<Value> {
    match node {
        Node::Literal(value) => Ok(value.clone()),
        Node::UnaryOp(op, operand) => eval_unary_op(frame, *op, operand),
        Node::BinOp(op, lhs, rhs) => eval_bin_op(frame, *op, lhs, rhs),
        Node::Apply(func, arg) => eval_apply(frame, func, Rc::clone(arg)),
        Node::If(cond, then, else_) => eval_if(frame, cond, then, else_),
        Node::Lambda(var, body) => eval_lambda(frame, *var, Rc::clone(body)),
        Node::Variable(var) => eval_variable(frame, *var),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn eval_str(input: &str) -> anyhow::Result<Value> {
        let tokens = tokenize(input);
        let (ast, rest) = parse(&tokens)?;
        if !rest.is_empty() {
            anyhow::bail!("there are extra tokens")
        }
        let frame = Frame::new();
        let value = eval(Rc::new(frame), &ast)?;
        Ok(value)
    }

    #[test]
    fn language_test() {
        let source = r#"? B= B$ B$ B$ B$ L$ L$ L$ L# v$ I" I# I$ I% I$ ? B= B$ L$ v$ I+ I+ ? B= BD I$ S4%34 S4 ? B= BT I$ S4%34 S4%3 ? B= B. S4% S34 S4%34 ? U! B& T F ? B& T T ? U! B| F F ? B| F T ? B< U- I$ U- I# ? B> I$ I# ? B= U- I" B% U- I$ I# ? B= I" B% I( I$ ? B= U- I" B/ U- I$ I# ? B= I# B/ I( I$ ? B= I' B* I# I$ ? B= I$ B+ I" I# ? B= U$ I4%34 S4%34 ? B= U# S4%34 I4%34 ? U! F ? B= U- I$ B- I# I& ? B= I$ B- I& I# ? B= S4%34 S4%34 ? B= F F ? B= I$ I$ ? T B. B. SM%,&k#(%#+}IEj}3%.$}z3/,6%},!.'5!'%y4%34} U$ B+ I# B* I$> I1~s:U@ Sz}4/}#,!)-}0/).43}&/2})4 S)&})3}./4}#/22%#4 S").!29}q})3}./4}#/22%#4 S").!29}q})3}./4}#/22%#4 S").!29}q})3}./4}#/22%#4 S").!29}k})3}./4}#/22%#4 S5.!29}k})3}./4}#/22%#4 S5.!29}_})3}./4}#/22%#4 S5.!29}a})3}./4}#/22%#4 S5.!29}b})3}./4}#/22%#4 S").!29}i})3}./4}#/22%#4 S").!29}h})3}./4}#/22%#4 S").!29}m})3}./4}#/22%#4 S").!29}m})3}./4}#/22%#4 S").!29}c})3}./4}#/22%#4 S").!29}c})3}./4}#/22%#4 S").!29}r})3}./4}#/22%#4 S").!29}p})3}./4}#/22%#4 S").!29}{})3}./4}#/22%#4 S").!29}{})3}./4}#/22%#4 S").!29}d})3}./4}#/22%#4 S").!29}d})3}./4}#/22%#4 S").!29}l})3}./4}#/22%#4 S").!29}N})3}./4}#/22%#4 S").!29}>})3}./4}#/22%#4 S!00,)#!4)/.})3}./4}#/22%#4 S!00,)#!4)/.})3}./4}#/22%#4"#;
        let value = eval_str(source).unwrap();
        let expected = "Self-check OK, send `solve language_test 4w3s0m3` to claim points for it";
        assert_eq!(value, Value::String(expected.into()));
    }

    #[test]
    fn big_int() {
        let source = r#"IuX$k"!#+$//2w"#;
        let value = eval_str(source).unwrap();
        assert_eq!(value, Value::Int(40255974450631082918621388i128.into()));
    }

    #[test]
    fn lazy() {
        let source = r#"B$ L# B$ L" B+ v" v" B* I$ I# v8"#;
        let value = eval_str(source).unwrap();
        assert_eq!(value, Value::Int(12.into()));
    }

    #[test]
    fn lazy2() {
        // 無限ループする項: (λx.x x c) (λx.x x c)
        //                 B$ Lx B$ B$ vx vx I0 Lx B$ B$ vx vx I0
        // fst 関数: λx. λy. x
        //          Lx Ly vx
        // fst 3 loop: B$ B$ Lx Ly vx I$ B$ Lx B$ B$ vx vx I0 Lx B$ B$ vx vx I0
        let source = r#"B$ B$ Lx Ly vx I$ B$ Lx B$ B$ vx vx I0 Lx B$ B$ vx vx I0"#;
        let value = eval_str(source).unwrap();
        assert_eq!(value, Value::Int(3.into()));
    }
}
