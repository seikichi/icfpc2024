use crate::ai::common;
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
        let mut visited = vec![false; n];
        let mut order = vec![0; n];
        for iter in 0..n {
            // 一番近い点へ順番に向う
            let mut min_dist = 1 << 30;
            let mut target = 0;
            for i in 0..n {
                if visited[i] {
                    continue;
                }
                let d = std::cmp::max(
                    (input.poss[i][0] - p[0]).abs(),
                    (input.poss[i][1] - p[1]).abs(),
                );
                if d < min_dist {
                    min_dist = d;
                    target = i;
                }
            }
            order[iter] = target;
            visited[target] = true;
            let tp = &input.poss[target];
            let (nv, mut nmoves) = common::make_move2(&p, &tp, &v);
            p = tp.clone();
            v = nv;
            moves.append(&mut nmoves);
        }
        SpaceshipSolution { moves, order }
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
