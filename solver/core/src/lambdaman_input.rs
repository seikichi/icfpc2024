use std::{collections::HashMap, io, path::Path};

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
    pub fn find_start_position(&self) -> (usize, usize) {
        for y in 0..self.h {
            for x in 0..self.w {
                if self.field[y][x] == 'L' {
                    return (x, y);
                }
            }
        }
        panic!("Lambdaman not found");
    }
    pub fn make_food_positions(&self) -> Vec<(usize, usize)> {
        let mut food_positions = vec![];
        for y in 0..self.h {
            for x in 0..self.w {
                if self.field[y][x] == '.' {
                    food_positions.push((x, y));
                }
            }
        }
        food_positions
    }
    pub fn make_food_indexs(&self) -> HashMap<(usize, usize), usize> {
        let mut food_indexs = HashMap::new();
        for y in 0..self.h {
            for x in 0..self.w {
                if self.field[y][x] == '.' {
                    food_indexs.insert((x, y), food_indexs.len());
                }
            }
        }
        food_indexs
    }
}

pub fn load_from_file<P: AsRef<Path>>(path: P) -> io::Result<LambdamanInput> {
    let s = std::fs::read_to_string(path)?;
    load_from_str(&s)
}

pub fn load_from_str(s: &str) -> io::Result<LambdamanInput> {
    let mut field = vec![];
    let lines: Vec<_> = s.trim().lines().collect::<Vec<_>>();
    for line in lines.iter() {
        let l = line.chars().collect::<Vec<char>>();
        field.push(l);
    }
    Ok(LambdamanInput::new(field))
}
