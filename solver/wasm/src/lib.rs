use base::eval;
use std::rc::Rc;

use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn add(a: u32, b: u32) -> u64 {
    base::add(u64::from(a), u64::from(b))
}

#[wasm_bindgen]
pub fn eval_str(input: &str) -> Result<String, String> {
    let tokens = eval::tokenize(input);
    let (ast, rest) = eval::parse(&tokens).map_err(|e| e.to_string())?;
    if !rest.is_empty() {
        return Err("トークンが余っています".into());
    }
    // eprintln!("AST: {ast}");
    let frame = eval::Frame::new();
    let value = eval::eval(Rc::new(frame), &ast).map_err(|e| e.to_string())?;
    Ok(format!("{}", value))
}
