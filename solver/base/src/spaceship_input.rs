use std::{io, path::Path};

#[derive(Clone, Debug)]
pub struct SpaceshipInput {
    pub n: usize,
    pub poss: Vec<Vec<i64>>,
}

impl SpaceshipInput {
    pub fn new(poss: Vec<Vec<i64>>) -> Self {
        let n = poss.len();
        SpaceshipInput { n, poss }
    }
}

pub fn load_from_file<P: AsRef<Path>>(path: P) -> io::Result<SpaceshipInput> {
    let s = std::fs::read_to_string(path)?;
    load_from_str(&s)
}

pub fn load_from_str(s: &str) -> io::Result<SpaceshipInput> {
    let mut poss = vec![];
    let lines: Vec<_> = s.trim().lines().collect::<Vec<_>>();
    for line in lines.iter() {
        let pos = line
            .split_whitespace()
            .map(|x| x.parse::<i64>().unwrap())
            .collect::<Vec<_>>();
        poss.push(pos);
    }
    Ok(SpaceshipInput::new(poss))
}
