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
                    "invalid field. line width {} is different from first line width {}, {:?}",
                    field[y].len(),
                    field[0].len(),
                    field[y]
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
    let mut field: Vec<Vec<String>> = vec![];
    let lines: Vec<_> = s.trim().lines().collect::<Vec<_>>();
    let mut first_solve = true;
    for line in lines.iter() {
        if line.starts_with("solve") {
            if first_solve {
                first_solve = false;
                continue;
            } else {
                // 2回目のsolveが出たら止める
                break;
            }
        }
        let l = line
            .split_whitespace()
            .map(|s| s.to_string())
            .collect::<Vec<String>>();
        if l.len() == 0 {
            continue;
        }
        field.push(l);
    }
    Ok(ThreeDInput::new(field))
}
