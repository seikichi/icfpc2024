use std::rc::Rc;

use super::{
    ast::Expr,
    token::Token,
};
use crate::eval::{BinOp, UnaryOp};

#[derive(Debug, Clone, PartialEq, thiserror::Error)]
pub enum ParseError {
    #[error("unexpected end-of-file")]
    UnexpectedEndOfFile,

    #[error("unexpected token: {0}")]
    UnexpectedToken(Token),

    #[error("unexpected token: {0} was expected, but got {1}")]
    UnexpectedToken2(String, Token),
}

type Result<T> = std::result::Result<T, ParseError>;

fn ok(expr: Expr, tokens: &[Token]) -> Result<(Box<Expr>, &[Token])> {
    Ok((Box::new(expr), tokens))
}

fn take_one(tokens: &[Token]) -> Result<(&Token, &[Token])> {
    let Some(token) = tokens.first() else {
        return Err(ParseError::UnexpectedEndOfFile);
    };
    Ok((token, &tokens[1..]))
}

macro_rules! take_exact {
    ($pattern:pat, $tokens:ident, $expected:expr) => {
        let (token, $tokens) = take_one($tokens)?;
        let $pattern = token else {
            return Err(ParseError::UnexpectedToken2(
                $expected.into(),
                token.clone(),
            ));
        };
    };
}

pub fn parse(tokens: &[Token]) -> Result<Box<Expr>> {
    let (token, rest) = parse_expr(tokens)?;
    if let Some(token) = rest.first() {
        return Err(ParseError::UnexpectedToken(token.clone()));
    }
    Ok(token)
}

// expr ::=
//   | term
//   | expr <binop> term
//   | expr "==" term
// TODO: 演算子の優先順位を実装する
fn parse_expr(tokens: &[Token]) -> Result<(Box<Expr>, &[Token])> {
    let (mut expr, mut tokens) = parse_term(tokens)?;
    loop {
        let Ok((t_binop, tokens_)) = take_one(tokens) else {
            break;
        };
        let (expr_, tokens_) = parse_term(tokens_)?;
        match t_binop {
            Token::Plus => {
                expr = Box::new(Expr::BinOp(BinOp::Add, expr, expr_));
            }
            Token::Minus => {
                expr = Box::new(Expr::BinOp(BinOp::Sub, expr, expr_));
            }
            Token::Asterisk => {
                expr = Box::new(Expr::BinOp(BinOp::Mul, expr, expr_));
            }
            Token::Slash => {
                expr = Box::new(Expr::BinOp(BinOp::Div, expr, expr_));
            }
            Token::Percent => {
                expr = Box::new(Expr::BinOp(BinOp::Mod, expr, expr_));
            }
            Token::Lt => {
                expr = Box::new(Expr::BinOp(BinOp::Lt, expr, expr_));
            }
            Token::Gt => {
                expr = Box::new(Expr::BinOp(BinOp::Gt, expr, expr_));
            }
            Token::EqEq => {
                expr = Box::new(Expr::BinOp(BinOp::Eq, expr, expr_));
            }
            Token::OrOr => {
                expr = Box::new(Expr::BinOp(BinOp::Or, expr, expr_));
            }
            Token::AndAnd => {
                expr = Box::new(Expr::BinOp(BinOp::And, expr, expr_));
            }
            Token::Dot => {
                expr = Box::new(Expr::BinOp(BinOp::Concat, expr, expr_));
            }
            Token::T => {
                expr = Box::new(Expr::BinOp(BinOp::Take, expr, expr_));
            }
            Token::D => {
                expr = Box::new(Expr::BinOp(BinOp::Drop, expr, expr_));
            }
            _ => break,
        }
        tokens = tokens_;
    }
    Ok((expr, tokens))
}

// term ::=
//   | factor
//   | factor factor...
//   | "\" identifier "->" expr
//   | "if" expr "then" expr "else" expr
fn parse_term(tokens: &[Token]) -> Result<(Box<Expr>, &[Token])> {
    let (token, tokens_) = take_one(tokens)?;
    match token {
        Token::Backslash => parse_lambda(tokens_),
        Token::If => parse_if(tokens_),
        _ => {
            // factor | factor factor...
            let (mut expr, mut tokens) = parse_factor(tokens)?;
            while let Ok((expr_, tokens_)) = parse_factor(tokens) {
                expr = Box::new(Expr::Apply(expr, expr_));
                tokens = tokens_;
            }
            Ok((expr, tokens))
        }
    }
}

// factor ::=
//   | bool
//   | integer
//   | string
//   | identifier
//   | <unary_op> factor
//   | "(" expr ")"
fn parse_factor(tokens: &[Token]) -> Result<(Box<Expr>, &[Token])> {
    let (token, tokens) = take_one(tokens)?;
    match token {
        Token::True => ok(Expr::Bool(true), tokens),
        Token::False => ok(Expr::Bool(false), tokens),
        Token::Int(n) => ok(Expr::Int(n.clone()), tokens),
        Token::String(s) => ok(Expr::String(s.clone()), tokens),
        Token::Identifier(ident) => ok(Expr::Variable(ident.clone()), tokens),
        Token::LParen => parse_paren(tokens),
        // 単項マイナスを導入すると文法が曖昧になってしまうので、
        // OCaml を真似てマイナスの変わりにチルダを使う
        Token::Tilde => {
            let (operand, tokens) = parse_factor(tokens)?;
            ok(Expr::UnaryOp(UnaryOp::Neg, operand), tokens)
        }
        Token::Exclamation => {
            let (operand, tokens) = parse_factor(tokens)?;
            ok(Expr::UnaryOp(UnaryOp::Not, operand), tokens)
        }
        Token::Sharp => {
            let (operand, tokens) = parse_factor(tokens)?;
            ok(Expr::UnaryOp(UnaryOp::StringToInt, operand), tokens)
        }
        Token::Dollar => {
            let (operand, tokens) = parse_factor(tokens)?;
            ok(Expr::UnaryOp(UnaryOp::IntToString, operand), tokens)
        }
        t => return Err(ParseError::UnexpectedToken(t.clone())),
    }
}

// "(" expr ")"
fn parse_paren(tokens: &[Token]) -> Result<(Box<Expr>, &[Token])> {
    let (expr, tokens) = parse_expr(tokens)?;
    take_exact!(Token::RParen, tokens, "')'");
    Ok((expr, tokens))
}

// "\" identifier "->" expr
fn parse_lambda(tokens: &[Token]) -> Result<(Box<Expr>, &[Token])> {
    take_exact!(Token::Identifier(t_ident), tokens, "identifier");
    take_exact!(Token::RArrow, tokens, "'->'");
    let (expr, tokens) = parse_expr(tokens)?;
    let lambda = Expr::Lambda(t_ident.clone(), Rc::new(*expr));
    ok(lambda, tokens)
}

// "if" expr "then" expr "else" expr
fn parse_if(tokens: &[Token]) -> Result<(Box<Expr>, &[Token])> {
    let (cond_expr, tokens) = parse_expr(tokens)?;
    take_exact!(Token::Then, tokens, "'then'");
    let (then_expr, tokens) = parse_expr(tokens)?;
    take_exact!(Token::Else, tokens, "'else'");
    let (else_expr, tokens) = parse_expr(tokens)?;
    let if_expr = Expr::If(cond_expr, then_expr, else_expr);
    ok(if_expr, tokens)
}
