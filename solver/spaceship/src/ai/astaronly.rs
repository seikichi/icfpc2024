use std::collections::{BinaryHeap, HashSet};

use glam::I64Vec2;
use log::debug;

use crate::spaceship_input::SpaceshipInput;
use crate::spaceship_solution::SpaceshipSolution;

use super::HeadAI;

pub struct AStarOnlyAI {
    pub allowed_miss: usize,
}

type V2 = I64Vec2;

impl HeadAI for AStarOnlyAI {
    fn solve(&mut self, input: &SpaceshipInput) -> SpaceshipSolution {
        let (is_star, n_stars) = make_field(input);
        let n_stars_left = if is_star.contains(&V2::ZERO) {
            n_stars - 1
        } else {
            n_stars
        };
        //print_field(&is_star);
        let mut visit: HashSet<V2> = HashSet::new();
        let best_moves = astar(&is_star, V2::ZERO, V2::ZERO, self.allowed_miss, n_stars_left);
        let best_moves: Vec<char> = best_moves.expect("AStarOnly: no solutions found").into_iter().rev().collect();
        SpaceshipSolution {
            moves: best_moves,
            order: vec![],
        }
    }
}

fn make_field(input: &SpaceshipInput) -> (HashSet<V2>, usize) {
    let mut is_star = HashSet::new();
    let mut n_stars = 0;
    for pos in &input.poss {
        let x = pos[0];
        let y = pos[1];
        if !is_star.contains(&V2::new(x, y)) {
            is_star.insert(V2::new(x, y));
            n_stars += 1;
        }
    }
    (is_star, n_stars)
}

#[allow(dead_code)]
fn print_field(is_star: &[Vec<bool>]) {
    for row in is_star {
        for b in row {
            print!("{}", if *b { '*' } else { '.' });
        }
        println!();
    }
}

fn to_move(dx: i64, dy: i64) -> char {
    //    -1 0 1
    // -1  1 2 3
    //  0  4 5 6
    //  1  7 8 9
    (((dy + 1) * 3 + (dx + 1) + 1) as u8 + '0' as u8) as char
}
#[derive(Debug, Clone, PartialEq, Eq)]
struct State {
    f: i64, // f = g + h
    p: V2,
    v: V2,
    visit: im_rc::HashSet<V2>,
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for State {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        (-self.f, self.p.x, self.p.y, self.v.x, self.v.y).cmp(
            &(-other.f, other.p.x, other.p.y, other.v.x, other.v.y)
        )
    }
}

fn bar1d(px: i64, vx: i64, starx: i64) -> usize {
    let dx = starx - px;
    // 1: v+1
    // 2: 2v+(1+2)
    // 3: 3v+(1+2+3)
    // 4: 4v+(1+2+3+4)
    // ...
    // n: nv+n(n-1)/2
    // => nv + n(n-1)/2 >= dx
    //    nv + n^2/2 - n/2
    //    n^2/2 + (v-1/2)n - dx >= 0
    let n = f64::ceil(
        (
            f64::sqrt((4*vx*vx + 4*vx + 8*dx + 1) as f64) +
            2.0*(vx as f64) + 1.0
        ) * 0.5
    ) as i64;
    todo!()
}

// 与えられた点から指定した星に何手でたどり着けるか
fn bar(p: V2, v: V2, star: V2) -> usize {
    todo!()
}

// 与えられた点から任意の未到達の星に何手でたどり着けるか
fn foo(
    stars: &HashSet<V2>,
    p: V2, v: V2,
    visit: &im_rc::HashSet<V2>,
) -> usize {
    todo!()
}

fn astar(
    is_star: &HashSet<V2>,
    p: V2, v: V2,
    allowed_miss: usize,
    n_stars_left: usize,
) -> Option<Vec<char>> {
    let mut queue: BinaryHeap<State> = BinaryHeap::new();

    todo!()

    /*
    debug!("p={p}, v={v}, allowed_miss={allowed_miss}, n_stars_left={n_stars_left}");

    if n_stars_left == 0 {
        // クリア
        return Some(vec![]);
    }

    // 状態(p, v)から遷移可能な状態をすべてバックトラックで探索する
    let mut best_moves: Option<Vec<char>> = None;
    for dy in -1..=1 {
        for dx in -1..=1 {
            let nv = v + V2::new(dx, dy);
            let np = p + nv;
            let mut miss = 0;
            if !is_star.contains(&np) {
                if allowed_miss > 0 {
                    // 星じゃない場所を踏むことを許す
                    miss = 1;
                } else {
                    continue;
                }
            }
            if visit.contains(&np) {
                continue;
            }
            visit.insert(np);
            if let Some(mut moves) = astar(
                is_star, visit,
                np, nv,
                allowed_miss - miss,
                n_stars_left - (1 - miss),
            ) {
                moves.push(to_move(dx, dy));

                // update best_moves
                if best_moves.is_none() {
                    best_moves = Some(moves);
                } else {
                    let best = best_moves.unwrap();
                    best_moves = if moves.len() < best.len() {
                        Some(moves)
                    } else {
                        Some(best)
                    };
                }
            }
            visit.remove(&np);
        }
    }
    best_moves
    */
}
