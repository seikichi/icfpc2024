use core::eval;
use std::{
    io::{self, Read},
    rc::Rc,
};

pub fn eval_str(input: &str) -> anyhow::Result<eval::Value> {
    let tokens = eval::tokenize(input);
    let (ast, rest) = eval::parse(&tokens)?;
    if !rest.is_empty() {
        anyhow::bail!("トークンが余っています")
    }
    eprintln!("AST: {ast}");
    let frame = eval::Frame::new();
    let value = eval::eval(Rc::new(frame), &ast)?;
    Ok(value)
}

fn main() -> anyhow::Result<()> {
    let mut buffer = String::new();
    io::stdin().read_to_string(&mut buffer)?;
    let value = eval_str(&buffer)?;
    println!("{value}");
    Ok(())
}
