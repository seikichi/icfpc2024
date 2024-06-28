use std::collections::VecDeque;

use crate::lambdaman_input;
use crate::lambdaman_solution::LambdamanSolution;

use super::{ChainedAI, HeadAI};

pub struct SimpleHeadAI {}
pub struct SimpleChainedAI {}

impl HeadAI for SimpleHeadAI {
    // BFS AI
    fn solve(&mut self, input: &lambdaman_input::LambdamanInput) -> LambdamanSolution {
        let mut moves = vec![];
        let h = input.h;
        let w = input.w;
        let mut sx = 0;
        let mut sy = 0;
        let mut foods = vec![];
        // println!("{:?}", input.field);
        for y in 0..h {
            for x in 0..w {
                let c = input.field[y][x];
                if c == 'L' {
                    sx = x;
                    sy = y;
                } else if c == '.' {
                    foods.push((x, y));
                }
            }
        }
        // println!("{:?}", foods);
        let mut eaten = vec![vec![false; w]; h];
        for _iter in 0..foods.len() {
            let mut visited = vec![vec![false; w]; h];
            let mut que: VecDeque<(usize, usize, Vec<char>)> = VecDeque::new();
            que.push_back((sx, sy, vec![]));
            visited[sy][sx] = true;
            while let Some((x, y, current_move)) = que.pop_front() {
                if !eaten[y][x] && input.field[y][x] == '.' {
                    eaten[y][x] = true;
                    sy = y;
                    sx = x;
                    moves.append(&mut current_move.clone());
                    // println!("{} {}", x, y);
                    break;
                }
                for dir in 0..4 {
                    let dx = [1, 0, -1, 0];
                    let dy = [0, 1, 0, -1];
                    let dirc = ['R', 'D', 'L', 'U'];
                    let nx = x as i64 + dx[dir];
                    let ny = y as i64 + dy[dir];
                    if nx < 0 || ny < 0 || nx as usize >= w || ny as usize >= h {
                        continue;
                    }
                    if input.field[ny as usize][nx as usize] == '#'
                        || visited[ny as usize][nx as usize]
                    {
                        continue;
                    }
                    visited[ny as usize][nx as usize] = true;
                    let mut nmove = current_move.clone();
                    nmove.push(dirc[dir]);
                    que.push_back((nx as usize, ny as usize, nmove));
                }
            }
        }
        LambdamanSolution { moves }
    }
}

impl ChainedAI for SimpleChainedAI {
    fn solve(
        &mut self,
        _input: &lambdaman_input::LambdamanInput,
        solution: &LambdamanSolution,
    ) -> LambdamanSolution {
        return solution.clone();
    }
}
