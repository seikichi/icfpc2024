use base::eval;
use std::{
    io::{self, Read},
    rc::Rc, thread::{self, Builder},
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

fn do_main() -> anyhow::Result<()> {
    let mut buffer = String::new();
    io::stdin().read_to_string(&mut buffer)?;
    let value = eval_str(&buffer)?;
    println!("{value}");
    Ok(())
}

fn main() -> anyhow::Result<()> {
    let builder = Builder::new().stack_size(1024 * 1024 * 1024);
    thread::scope(|s| {
        let handle = builder.spawn_scoped(s, do_main).unwrap();
        handle.join()
    }).unwrap()
}
