use super::{
    board::{Board, Cell},
    three_d_input::{self, ThreeDInput},
};
use std::collections::{HashMap, HashSet};
pub struct Simulator {
    pub a: i64,
    pub b: i64,
    pub max_w: usize,
    pub max_h: usize,
    pub max_t: usize,
    pub boards: Vec<Board>,
}

impl Simulator {
    pub fn new(input: &ThreeDInput, a: i64, b: i64) -> Self {
        let max_h = input.field.len();
        let max_w = input.field[0].len();
        let max_t = 1;
        let board = Board::new(input, a, b);
        Simulator {
            a,
            b,
            max_w,
            max_h,
            max_t,
            boards: vec![board],
        }
    }
    pub fn step(&mut self) -> Option<i64> {
        let mut remove_cells: HashSet<(i64, i64)> = HashSet::new();
        let mut add_cells: HashMap<(i64, i64), Cell> = HashMap::new();
        let mut warps = vec![];
        let field = self.boards.last().unwrap().field.clone();
        let h = field.len();
        let w = field[0].len();
        // let min_x = 1 << 30;
        // let max_x = -1 << 30;
        // let min_y = 1 << 30;
        // let max_y = -1 << 30;
        // eval operator
        for y in 0..h {
            for x in 0..w {
                let left_cell = if x >= 1 { field[y][x - 1] } else { Cell::Empty };
                let right_cell = if x < w - 1 {
                    field[y][x + 1]
                } else {
                    Cell::Empty
                };
                let up_cell = if y >= 1 { field[y - 1][x] } else { Cell::Empty };
                let down_cell = if y < h - 1 {
                    field[y + 1][x]
                } else {
                    Cell::Empty
                };
                // move & not equal & warp operator
                match field[y][x] {
                    Cell::Right => {
                        if left_cell != Cell::Empty {
                            remove_cells.insert((x as i64 - 1, y as i64));
                            check_and_insert(x as i64 + 1, y as i64, left_cell, &mut add_cells);
                        }
                    }
                    Cell::Down => {
                        if up_cell != Cell::Empty {
                            remove_cells.insert((x as i64, y as i64 - 1));
                            check_and_insert(x as i64, y as i64 + 1, up_cell, &mut add_cells);
                        }
                    }
                    Cell::Left => {
                        if right_cell != Cell::Empty {
                            remove_cells.insert((x as i64 + 1, y as i64));
                            check_and_insert(x as i64 - 1, y as i64, right_cell, &mut add_cells);
                        }
                    }
                    Cell::Up => {
                        if down_cell != Cell::Empty {
                            remove_cells.insert((x as i64, y as i64 + 1));
                            check_and_insert(x as i64, y as i64 - 1, down_cell, &mut add_cells);
                        }
                    }
                    Cell::NotEqual => {
                        if let Cell::Integer(a) = left_cell {
                            if let Cell::Integer(b) = up_cell {
                                if a != b {
                                    remove_cells.insert((x as i64 - 1, y as i64));
                                    remove_cells.insert((x as i64, y as i64 - 1));
                                    check_and_insert(
                                        x as i64 + 1,
                                        y as i64,
                                        up_cell,
                                        &mut add_cells,
                                    );
                                    check_and_insert(
                                        x as i64,
                                        y as i64 + 1,
                                        left_cell,
                                        &mut add_cells,
                                    );
                                }
                            }
                        }
                    }
                    Cell::Warp => {
                        if let Cell::Integer(v) = up_cell {
                            if let Cell::Integer(dx) = left_cell {
                                if let Cell::Integer(dy) = right_cell {
                                    if let Cell::Integer(dt) = down_cell {
                                        warps.push((x as i64 - dx, y as i64 - dy, dt, v));
                                    }
                                }
                            }
                        }
                    }
                    _ => {}
                }
                // normal operator
                if let Cell::Integer(a) = left_cell {
                    if let Cell::Integer(b) = up_cell {
                        // println!("{} {}", x, y);
                        let value = match field[y as usize][x as usize] {
                            Cell::Plus => Some(Cell::Integer(a + b)),
                            Cell::Minus => Some(Cell::Integer(a - b)),
                            Cell::Mul => Some(Cell::Integer(a * b)),
                            Cell::Div => Some(Cell::Integer(a / b)),
                            Cell::Mod => Some(Cell::Integer(a % b)),
                            Cell::Equal => {
                                if a == b {
                                    Some(Cell::Integer(b))
                                } else {
                                    None
                                }
                            }
                            _ => None,
                        };
                        if let Some(v) = value {
                            remove_cells.insert((x as i64 - 1, y as i64));
                            remove_cells.insert((x as i64, y as i64 - 1));
                            check_and_insert(x as i64 + 1, y as i64, v, &mut add_cells);
                            check_and_insert(x as i64, y as i64 + 1, v, &mut add_cells);
                        }
                    }
                }
            }
        }
        // check submits
        let mut submit: Option<i64> = None;
        for y in 0..h {
            for x in 0..w {
                if field[y][x] != Cell::Submit {
                    continue;
                }
                if !add_cells.contains_key(&(x as i64, y as i64)) {
                    continue;
                }
                if let Cell::Integer(v) = add_cells[&(x as i64, y as i64)] {
                    if let Some(other) = submit {
                        if other != v {
                            panic!("multiple submit: {} & {}", other, v);
                        }
                    } else {
                        submit = Some(v);
                    }
                }
            }
        }
        if submit.is_some() {
            return submit;
        }
        // check warps
        if warps.len() > 0 {
            // check time consistency
            let dt = warps[0].2;
            if dt < 0 || dt as usize >= self.boards.len() {
                panic!("invalid warp dt: {}", dt);
            }
            if warps.iter().any(|f| f.2 != dt) {
                panic!("multiple warp dt: {:?}", warps);
            }
            // check same postion consistency
            let mut dests: HashMap<(i64, i64), i64> = HashMap::new();
            for &(x, y, _, v) in warps.iter() {
                if dests.contains_key(&(x, y)) {
                    // 異なる値の場合だけNG
                    if dests[&(x, y)] != v {
                        panic!(
                            "multiple warp is same destitnation: ({}, {}) {:?}",
                            x, y, warps
                        );
                    }
                } else {
                    dests.insert((x, y), v);
                }
            }
            let t = self.boards.len() - dt as usize;
            self.boards.truncate(t);
            let mut new_field = self.boards[t - 1].field.clone();
            for &(x, y, _, v) in warps.iter() {
                if new_field[y as usize][x as usize] == Cell::Submit {
                    unimplemented!("warp to submit is unimplemented");
                }
                new_field[y as usize][x as usize] = Cell::Integer(v);
            }
            self.boards[t - 1].field = new_field;
            return None;
        }
        // update board
        // println!("remove: {:?}", remove_cells);
        // println!("add: {:?}", add_cells);
        let mut new_field = self.boards.last().unwrap().field.clone();
        for &(x, y) in remove_cells.iter() {
            new_field[y as usize][x as usize] = Cell::Empty;
        }
        for (&(x, y), &cell) in add_cells.iter() {
            new_field[y as usize][x as usize] = cell;
        }
        if new_field == self.boards.last().unwrap().field {
            // TODO same board check
            panic!("no reduce");
        }
        self.boards.push(Board {
            left: 0,
            up: 0,
            field: new_field,
        });
        self.max_t = std::cmp::max(self.max_t, self.boards.len());
        return None;
    }
}

fn check_and_insert(x: i64, y: i64, cell: Cell, add_cells: &mut HashMap<(i64, i64), Cell>) {
    if add_cells.contains_key(&(x, y)) {
        panic!(
            "multiple write: ({}, {}) {} & {}",
            x,
            y,
            add_cells[&(x, y)],
            cell
        );
    }
    add_cells.insert((x, y), cell);
}

#[test]
fn simulator_step_test() {
    let solution_3d3 = r". -1 . 1 .
  . = S  v .
  ^ 2 .  0 .
  A / A  = .
  . . .  S .
  1 @ 2  . .
  . 1 .  . .";
    let input = three_d_input::load_from_str(solution_3d3).unwrap();
    let mut simulator = Simulator::new(&input, -11, 0);
    for _t in 0..100 {
        let result = simulator.step();
        if let Some(v) = result {
            println!("answer: {}", v);
            assert!(v == -1);
            break;
        } else {
            println!("{}", simulator.boards.last().unwrap());
        }
    }
}
