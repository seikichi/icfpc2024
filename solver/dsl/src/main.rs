use base::dsl::{codegen, lexer, parser};
use std::io::{self, Read};

fn transpile(input: &str) -> anyhow::Result<()> {
    let token_and_spans = lexer::lex(input)?;
    let tokens: Vec<_> = token_and_spans.into_iter().map(|(t, _)| t).collect();
    let ast = parser::parse(&tokens)?;
    eprintln!("AST = {:?}", ast);
    let code = codegen::codegen(&ast);
    println!("{}", code.join(" "));
    Ok(())
}

fn main() -> anyhow::Result<()> {
    let mut buffer = String::new();
    io::stdin().read_to_string(&mut buffer)?;
    transpile(&buffer)
}
