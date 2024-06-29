use crate::spaceship_input;
use crate::spaceship_solution::SpaceshipSolution;

use super::{ChainedAI, HeadAI};

pub struct SimpleHeadAI {}
pub struct SimpleChainedAI {}

// tpに着いた時の速度の絶対値を1以下にする前提で最適な移動を計算する
fn calc_move(f: i64, t: i64, v: i64) -> i64 {
    let d = t - f;
    if v != 0 && d.signum() != v.signum() {
        return v.signum() * -1;
    }
    let length1 = v.abs() * (v.abs() + 1) / 2;
    if d.abs() < length1 {
        return v.signum() * -1;
    }
    let length2 = (v.abs() + 1) * (v.abs() + 2) / 2;
    if d.abs() < length2 {
        return 0;
    }
    return d.signum();
}

impl HeadAI for SimpleHeadAI {
    // in order AI
    fn solve(&mut self, input: &spaceship_input::SpaceshipInput) -> SpaceshipSolution {
        let mut moves = vec![];
        let n = input.n;
        let mut p = vec![0, 0];
        let mut v = vec![0, 0];
        let mut visited = vec![false; n];
        for _iter in 0..n {
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
            visited[target] = true;
            let tp = &input.poss[target];
            loop {
                if p == *tp {
                    break;
                }
                let mut m = vec![1, 1];
                for dir in 0..2 {
                    let a = calc_move(p[dir], tp[dir], v[dir]);
                    v[dir] += a;
                    m[dir] = 1 + a;
                    p[dir] += v[dir];
                }
                // println!("{:?} {:?} {:?}", p, v, m);
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
