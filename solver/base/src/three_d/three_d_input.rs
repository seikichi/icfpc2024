use std::{io, path::Path};

#[derive(Clone, Debug)]
pub struct ThreeDInput {
    pub field: Vec<Vec<String>>,
}

impl ThreeDInput {
    pub fn new(field: Vec<Vec<String>>) -> Self {
        for y in 0..field.len() {
            if field[y].len() != field[0].len() {
                panic!(
                    "invalid field. line width {} is different from first line width {}",
                    field[y].len(),
                    field[0].len()
                );
            }
        }
        ThreeDInput { field }
    }
}

pub fn load_from_file<P: AsRef<Path>>(path: P) -> io::Result<ThreeDInput> {
    let s = std::fs::read_to_string(path)?;
    load_from_str(&s)
}

pub fn load_from_str(s: &str) -> io::Result<ThreeDInput> {
    let mut field = vec![];
    let lines: Vec<_> = s.trim().lines().collect::<Vec<_>>();
    for line in lines.iter() {
        let l = line
            .split_whitespace()
            .map(|s| s.to_string())
            .collect::<Vec<String>>();
        field.push(l);
    }
    Ok(ThreeDInput::new(field))
}
