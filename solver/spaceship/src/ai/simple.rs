use crate::spaceship_input;
use crate::spaceship_solution::SpaceshipSolution;

use super::{ChainedAI, HeadAI};

pub struct SimpleHeadAI {}
pub struct SimpleChainedAI {}

impl HeadAI for SimpleHeadAI {
    // in order AI
    fn solve(&mut self, input: &spaceship_input::SpaceshipInput) -> SpaceshipSolution {
        let mut moves = vec![];
        let n = input.n;
        let mut p = vec![0, 0];
        let mut v = vec![0, 0];
        for i in 0..n {
            let tp = &input.poss[i];
            loop {
                if p == *tp {
                    break;
                }
                let mut m = vec![1, 1];
                for dir in 0..2 {
                    if tp[dir] - p[dir] > 0 && v[dir] <= 0 {
                        v[dir] += 1;
                        m[dir] = 2;
                    } else if tp[dir] - p[dir] < 0 && v[dir] >= 0 {
                        v[dir] -= 1;
                        m[dir] = 0;
                    } else if tp[dir] == p[dir] {
                        if v[dir] > 0 {
                            v[dir] -= 1;
                            m[dir] = 0;
                        } else if v[dir] < 0 {
                            v[dir] += 1;
                            m[dir] = 2;
                        } else if v[dir] > 0 {
                            v[dir] -= 1;
                            m[dir] = 0;
                        }
                    }
                    p[dir] += v[dir];
                }
                let c = ((m[1] * 3 + m[0] + 1) as u8 + '0' as u8) as char;
                moves.push(c);
            }
        }
        SpaceshipSolution { moves }
    }
}

impl ChainedAI for SimpleChainedAI {
    fn solve(
        &mut self,
        _input: &spaceship_input::SpaceshipInput,
        solution: &SpaceshipSolution,
    ) -> SpaceshipSolution {
        return solution.clone();
    }
}
