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
        let (stars, n_stars) = make_field(input);
        let n_stars_left = if stars.contains(&V2::ZERO) {
            n_stars - 1
        } else {
            n_stars
        };
        //print_field(&stars);
        let best_moves = astar(&stars, self.allowed_miss, n_stars_left);
        SpaceshipSolution {
            moves: best_moves.expect("AStarOnly: no solutions found"),
            order: vec![],
        }
    }
}

fn make_field(input: &SpaceshipInput) -> (HashSet<V2>, usize) {
    let mut stars = HashSet::new();
    let mut n_stars = 0;
    for pos in &input.poss {
        let x = pos[0];
        let y = pos[1];
        if !stars.contains(&V2::new(x, y)) {
            stars.insert(V2::new(x, y));
            n_stars += 1;
        }
    }
    (stars, n_stars)
}

#[allow(dead_code)]
fn print_field(stars: &[Vec<bool>]) {
    for row in stars {
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
    g: i64,
    p: V2,
    v: V2,
    miss: i64,
    visit: im_rc::HashSet<V2>,
    moves: Vec<char>,
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for State {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        (-self.f, -self.miss, self.p.x, self.p.y, self.v.x, self.v.y).cmp(&(
            -other.f,
            -other.miss,
            other.p.x,
            other.p.y,
            other.v.x,
            other.v.y,
        ))
    }
}

fn approx1d(px: i64, mut vx: i64, starx: i64) -> i64 {
    let mut dx = starx - px;
    if dx < 0 {
        dx *= -1;
        vx *= -1;
    }
    if vx < 0 {
        dx += vx.abs() * (vx.abs() - 1) / 2;
        vx = 0;
    }
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
        (f64::sqrt((4 * vx * vx - 4 * vx + 8 * dx + 1) as f64) - 2.0 * (vx as f64) + 1.0) * 0.5,
    ) as i64;
    return n;
}

// 与えられた点から指定した星に何手でたどり着けるか
fn approx2d(p: V2, v: V2, star: V2) -> i64 {
    let c1 = approx1d(p.x, v.x, star.x);
    let c2 = approx1d(p.y, v.y, star.y);
    return std::cmp::max(c1, c2);
}

// 与えられた点から任意の未到達の星に何手でたどり着けるか
fn heuristic(stars: &HashSet<V2>, p: V2, v: V2, visit: &im_rc::HashSet<V2>) -> i64 {
    let mut ret = 1 << 30;
    for star in stars {
        if !visit.contains(star) {
            ret = std::cmp::max(ret, approx2d(p, v, *star));
        }
    }
    ret
}

fn astar(stars: &HashSet<V2>, allowed_miss: usize, n_stars_left: usize) -> Option<Vec<char>> {
    let mut queue: BinaryHeap<State> = BinaryHeap::new();
    queue.push(State {
        f: 0,
        g: 0,
        p: V2::ZERO,
        v: V2::ZERO,
        miss: 0,
        visit: im_rc::HashSet::new(),
        moves: vec![],
    });

    while let Some(state) = queue.pop() {
        if n_stars_left + state.miss as usize == state.visit.len() {
            // クリア
            return Some(state.moves);
        }

        // 状態(p, v)から遷移可能な状態をすべてバックトラックで探索する
        for dy in -1..=1 {
            for dx in -1..=1 {
                let nv = state.v + V2::new(dx, dy);
                let np = state.p + nv;
                let mut nmiss = state.miss;
                if !stars.contains(&np) {
                    nmiss = state.miss + 1;
                    if allowed_miss < nmiss as usize {
                        continue;
                    }
                }
                if state.visit.contains(&np) {
                    continue;
                }
                let nvisit = state.visit.update(np);
                let mut nmoves = state.moves.clone();
                nmoves.push(to_move(dx, dy));
                queue.push(State {
                    f: state.g + 1 + heuristic(stars, np, nv, &nvisit),
                    g: state.g + 1,
                    p: np,
                    v: nv,
                    miss: nmiss,
                    visit: nvisit,
                    moves: nmoves,
                });
            }
        }
    }
    None
}
