pub mod ast;
pub mod codegen;
pub mod lexer;
pub mod parser;
pub mod span;
pub mod symbol;
pub mod token;
pub mod value;

pub fn transpile(input: &str, print_ast: bool) -> anyhow::Result<String> {
    let token_and_spans = lexer::lex(input)?;
    let tokens: Vec<_> = token_and_spans.into_iter().map(|(t, _)| t).collect();
    let ast = parser::parse(&tokens)?;
    if print_ast {
        eprintln!("AST = {:?}", ast);
    }
    let code = codegen::codegen(&ast);
    Ok(code.join(" "))
}

#[cfg(test)]
mod tests {
    use crate::dsl::transpile;

    #[test]
    fn simple_test() {
        let doit = |title: &str, source: &str, expected: &str| {
            let actual = transpile(source, false).unwrap();
            assert_eq!(expected, actual, "{}", title);
        };
        // bool
        doit("true", "true", "T");
        doit("false", "false", "F");
        // int
        doit("zero", "0", "I!");
        doit("one", "1", "I\"");
        // string
        doit("hello world", "\"Hello World!\"", "SB%,,/}Q/2,$_");
        doit("empty string", "\"\"", "S");
        // unary op
        doit("neg", "~3", "U- I$");
        doit("not", "!true", "U! T");
        doit("string-to-int", "#\"\"", "U# S");
        doit("int-to-string", "$94", "U$ I\"!");
        // binary op
        doit("add", "0 + 2", "B+ I! I#");
        doit("sub", "0 - 2", "B- I! I#");
        doit("mul", "0 * 2", "B* I! I#");
        doit("div", "0 / 2", "B/ I! I#");
        doit("mod", "0 % 2", "B% I! I#");
        doit("lt", "0 < 2", "B< I! I#");
        doit("gt", "0 > 2", "B> I! I#");
        doit("eq", "0 == true", "B= I! T");
        doit("or", "true || false", "B| T F");
        doit("and", "true && false", "B& T F");
        doit("concat", "\"te\" . \"st\"", "B. S4% S34");
        doit("take", "3 T \"test\"", "BT I$ S4%34");
        doit("drop", "3 D \"test\"", "BD I$ S4%34");
        // apply
        doit("apply", r"\f -> \x -> f x", "L! L\" B$ v! v\"");
        // if
        doit("if", "if 2 > 3 then \"yes\" else \"no\"", "? B> I# I$ S9%3 S./");
        // lambda
        doit(
            "lambda",
            r#"((\v2 -> \v3 -> v2) ("Hello" . " World!")) 42"#,
            "B$ B$ L# L$ v# B. SB%,,/ S}Q/2,$_ IK",
        );
        // paren
        doit("paren", "(((0)))", "I!");
    }
}
