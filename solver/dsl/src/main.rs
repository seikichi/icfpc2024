use base::dsl;
use std::io::{self, Read};

fn main() -> anyhow::Result<()> {
    let mut buffer = String::new();
    io::stdin().read_to_string(&mut buffer)?;
    let code = dsl::transpile(&buffer, true)?;
    println!("{}", code);
    Ok(())
}
