use num_bigint::BigInt;
use num_traits::ToPrimitive;

use crate::eval::RTABLE;

use super::ast::Expr;

fn encode_int(mut i: BigInt) -> String {
    if i == BigInt::ZERO {
        return "!".into();
    }
    if i < BigInt::ZERO {
        panic!("negative integer literal is invalid");
    }
    let mut ret: Vec<u8> = Vec::new();
    while i > BigInt::ZERO {
        let b: BigInt = &i % 94;
        i /= 94;
        ret.push(b.to_u8().unwrap() + 33);
    }
    ret.reverse();
    String::from_utf8_lossy(&ret).to_string()
}

fn encode_string(s: &str) -> String {
    let mut ret = String::new();
    for c in s.chars() {
        let index = RTABLE[&(c as u8)] + 33;
        let encoded = char::from_u32(index as u32).unwrap();
        ret.push(encoded);
    }
    ret
}

pub fn codegen(expr: &Expr) -> Vec<String> {
    match expr {
        Expr::Bool(b) => if *b {
            vec!["T".into()]
        } else {
            vec!["F".into()]
        }
        Expr::Int(i) => {
            vec![format!("I{}", encode_int(i.clone()))]
        }
        Expr::String(s) => {
            vec![format!("S{}", encode_string(s))]
        }
        Expr::Variable(name) => {
            let id = BigInt::from(name.id());
            vec![format!("v{}", encode_int(id))]
        }
        Expr::Apply(lhs, rhs) => {
            let mut ret = vec!["B$".into()];
            ret.extend(codegen(lhs));
            ret.extend(codegen(rhs));
            ret
        }
        Expr::BinOp(op, lhs, rhs) => {
            let mut ret = vec![format!("B{}", op.symbol())];
            ret.extend(codegen(lhs));
            ret.extend(codegen(rhs));
            ret
        }
        Expr::UnaryOp(op, operand) => {
            let mut ret = vec![format!("U{}", op.symbol())];
            ret.extend(codegen(operand));
            ret
        }
        Expr::Lambda(arg, body) => {
            let id = BigInt::from(arg.id());
            let mut ret = vec![format!("L{}", encode_int(id))];
            ret.extend(codegen(body));
            ret
        }
        Expr::If(cond, then, else_) => {
            let mut ret = vec!["?".into()];
            ret.extend(codegen(cond));
            ret.extend(codegen(then));
            ret.extend(codegen(else_));
            ret
        }
    }
}
