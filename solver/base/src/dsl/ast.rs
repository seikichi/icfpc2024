use std::rc::Rc;

use num_bigint::BigInt;

use super::symbol::Symbol;
use crate::eval::{BinOp, UnaryOp};

#[derive(Debug)]
pub enum Expr {
    Bool(bool),
    Int(BigInt),
    String(String),
    Lambda(Symbol, Rc<Expr>),
    Variable(Symbol),
    If(Box<Expr>, Box<Expr>, Box<Expr>),
    UnaryOp(UnaryOp, Box<Expr>),
    BinOp(BinOp, Box<Expr>, Box<Expr>),
    Apply(Box<Expr>, Box<Expr>),
    Let(Symbol, Box<Expr>, Box<Expr>),
}
