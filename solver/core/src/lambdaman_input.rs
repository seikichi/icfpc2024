use std::{io, path::Path};

#[derive(Clone, Debug)]
pub struct LambdamanInput {
    pub field: Vec<Vec<char>>,
    pub w: usize,
    pub h: usize,
}

impl LambdamanInput {
    pub fn new(field: Vec<Vec<char>>) -> Self {
        let h = field.len();
        let w = field[0].len();
        LambdamanInput { field, w, h }
    }
}

pub fn load_from_file<P: AsRef<Path>>(path: P) -> io::Result<LambdamanInput> {
    let s = std::fs::read_to_string(path)?;
    load_from_str(&s)
}

pub fn load_from_str(s: &str) -> io::Result<LambdamanInput> {
    let mut field = vec![];
    let lines: Vec<_> = s.lines().collect::<Vec<_>>();
    for line in lines.iter() {
        let l = line.chars().collect::<Vec<char>>();
        field.push(l);
    }
    Ok(LambdamanInput::new(field))
}
