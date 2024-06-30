use core::panic;

use super::three_d_input::ThreeDInput;

#[derive(Clone, Debug)]
pub struct Board {
    pub left: i64, // 左の端の座標, 未実装
    pub up: i64,   // 上の端の座標, 未実装
    pub field: Vec<Vec<Cell>>,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum Cell {
    Empty,
    Integer(i64),
    Right,
    Down,
    Left,
    Up,
    Plus,
    Minus,
    Mul,
    Div,
    Mod,
    Warp,
    Equal,
    NotEqual,
    Submit,
}

impl Board {
    pub fn new(input: &ThreeDInput, a: i64, b: i64) -> Self {
        let h = input.field.len();
        let w = input.field[0].len();
        let mut field = vec![vec![Cell::Empty; w]; h];
        for y in 0..h {
            for x in 0..w {
                let s = &input.field[y][x];
                field[y][x] = match s.as_str() {
                    "." => Cell::Empty,
                    ">" => Cell::Right,
                    "v" => Cell::Down,
                    "<" => Cell::Left,
                    "^" => Cell::Up,
                    "+" => Cell::Plus,
                    "-" => Cell::Minus,
                    "*" => Cell::Mul,
                    "/" => Cell::Div,
                    "%" => Cell::Mod,
                    "@" => Cell::Warp,
                    "=" => Cell::Equal,
                    "#" => Cell::NotEqual,
                    "S" => Cell::Submit,
                    "A" => Cell::Integer(a),
                    "B" => Cell::Integer(b),
                    _ => {
                        if let Ok(v) = s.parse::<i64>() {
                            if v < -99 || 99 < v {
                                panic!("invalid value: {}", s);
                            }
                            Cell::Integer(v)
                        } else {
                            panic!("invalid cell: {}", s);
                        }
                    }
                }
            }
        }
        Board {
            left: 0,
            up: 0,
            field,
        }
    }
}

impl std::fmt::Display for Board {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let h = self.field.len();
        let w = self.field[0].len();
        for y in 0..h {
            for x in 0..w {
                if x != 0 {
                    write!(f, " ")?;
                }
                write!(f, "{}", self.field[y][x])?;
            }
            let _ = write!(f, "\n");
        }
        Ok(())
    }
}

impl std::fmt::Display for Cell {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Cell::Empty => write!(f, "."),
            Cell::Integer(v) => write!(f, "{}", v),
            Cell::Right => write!(f, ">"),
            Cell::Down => write!(f, "v"),
            Cell::Left => write!(f, "<"),
            Cell::Up => write!(f, "^"),
            Cell::Plus => write!(f, "+"),
            Cell::Minus => write!(f, "-"),
            Cell::Mul => write!(f, "*"),
            Cell::Div => write!(f, "/"),
            Cell::Mod => write!(f, "%"),
            Cell::Warp => write!(f, "@"),
            Cell::Equal => write!(f, "="),
            Cell::NotEqual => write!(f, "#"),
            Cell::Submit => write!(f, "S"),
        }
    }
}

impl Cell {
    pub fn is_integer(&self) -> bool {
        match self {
            Cell::Integer(_) => true,
            _ => false,
        }
    }
}
