use std::collections::{BinaryHeap, HashSet};

use glam::I64Vec2;
use log::info;

use crate::spaceship_input::SpaceshipInput;
use crate::spaceship_solution::SpaceshipSolution;

use super::HeadAI;

pub struct AStarOnlyAI {
    pub allowed_miss: usize,
    pub max_diff_star: usize,
}

type V2 = I64Vec2;

impl HeadAI for AStarOnlyAI {
    fn solve(&mut self, input: &SpaceshipInput) -> SpaceshipSolution {
        let (stars, n_stars) = make_field(input);
        let n_stars_left = if stars.contains(&V2::ZERO) {
            // n_stars - 1
            n_stars
        } else {
            n_stars
        };
        //print_field(&stars);
        let best_moves = astar(
            &stars,
            self.allowed_miss,
            self.max_diff_star,
            n_stars_left as i64,
        );
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
    prev: V2,
}

impl State {
    fn visit_star(&self) -> i64 {
        self.visit.len() as i64 - self.miss
    }
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for State {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        (
            -self.f,
            self.visit_star(),
            -self.miss,
            self.p.x,
            self.p.y,
            self.v.x,
            self.v.y,
        )
            .cmp(&(
                -other.f,
                other.visit_star(),
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
    let mut cnt = 0;
    if dx < 0 {
        dx *= -1;
        vx *= -1;
    }
    if vx < 0 {
        dx += vx.abs() * (vx.abs() - 1) / 2;
        cnt += -vx;
        vx = 0;
    }
    if vx + 1 > dx {
        return 2;
    }
    // 1: v+1
    // 2: 2v+(1+2)
    // 3: 3v+(1+2+3)
    // 4: 4v+(1+2+3+4)
    // ...
    // n: nv+n(n+1)/2
    // => nv + n(n+1)/2 >= dx
    //    nv + n^2/2 + n/2
    //    n^2/2 + (v+1/2)n - dx >= 0
    let n = f64::ceil(
        (f64::sqrt((4 * vx * vx + 4 * vx + 8 * dx + 1) as f64) - 2.0 * (vx as f64) - 1.0) * 0.5,
    ) as i64;
    return n + cnt;
}

// 与えられた点から指定した星に何手でたどり着けるか
fn approx2d(p: V2, v: V2, star: V2) -> i64 {
    let c1 = approx1d(p.x, v.x, star.x);
    let c2 = approx1d(p.y, v.y, star.y);
    assert!(c1 > 0 || c2 > 0);
    return std::cmp::max(c1, c2);
}

// 与えられた点から任意の未到達の星に何手でたどり着けるか
fn heuristic(stars: &HashSet<V2>, p: V2, v: V2, visit: &im_rc::HashSet<V2>, left_star: i64) -> i64 {
    if left_star == 0 {
        return 0;
    }
    let mut ret = 1 << 30;
    for star in stars {
        if !visit.contains(star) {
            ret = std::cmp::min(ret, approx2d(p, v, *star));
        }
    }
    ret + left_star - 1
}

fn astar(
    stars: &HashSet<V2>,
    allowed_miss: usize,
    max_diff_star: usize,
    n_stars_left: i64,
) -> Option<Vec<char>> {
    // let mut open = HashSet::new();
    info!("star_left: {}", n_stars_left);
    let mut queue: BinaryHeap<State> = BinaryHeap::new();
    // let mut visit = im_rc::HashSet::new();
    // visit.insert(V2::ZERO);
    queue.push(State {
        f: 0,
        g: 0,
        p: V2::ZERO,
        v: V2::ZERO,
        miss: 0,
        visit: im_rc::HashSet::new(),
        moves: vec![],
        prev: V2::ZERO,
    });

    let mut max_visit_star = 0;
    let mut iter = 0;
    while let Some(state) = queue.pop() {
        iter += 1;
        if n_stars_left == state.visit_star() as i64 {
            // クリア
            return Some(state.moves);
        }
        // let u = (state.p, state.v, state.visit.clone());
        // if open.contains(&u) {
        //     continue;
        // }
        // open.insert(u);
        if max_visit_star - state.visit_star() > max_diff_star as i64 {
            continue;
        }
        if iter % 1000000 == 0 {
            info!(
                "{:?} {}",
                state,
                String::from_iter(state.moves.clone().iter())
            );
        }
        if state.visit_star() > max_visit_star {
            max_visit_star = state.visit_star();
            info!(
                "f: {} get: {} miss: {}, p: {}, v: {}",
                state.f,
                state.visit_star(),
                state.miss,
                state.p,
                state.v
            );
            // if state.f > 5 {
            //     info!("{:?} {:?}", state, queue);
            // }
        }
        let mut next = V2::ONE * 1000000;
        for &s in stars.iter() {
            if state.visit.contains(&s) {
                continue;
            }
            let d = (s - state.prev).length_squared();
            if d < (next - state.prev).length_squared() {
                next = s;
            }
        }

        // 状態(p, v)から遷移可能な状態をすべて探索する
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
                let mut nprev = state.prev;
                let nvisit = state.visit.update(np);
                // let mut nvisit = state.visit.clone();
                if stars.contains(&np) {
                    if np != next {
                        continue;
                    }
                    // nvisit.insert(np);
                    nprev = np;
                }
                let mut nmoves = state.moves.clone();
                nmoves.push(to_move(dx, dy));
                let nvisit_star = nvisit.len() as i64 - nmiss;
                let nleft_star = n_stars_left - nvisit_star;
                queue.push(State {
                    f: state.g + 1 + heuristic(stars, np, nv, &nvisit, nleft_star),
                    g: state.g + 1,
                    p: np,
                    v: nv,
                    miss: nmiss,
                    visit: nvisit,
                    moves: nmoves,
                    prev: nprev,
                });
            }
        }
    }
    None
}

#[test]
fn approx1d_test() {
    let c = approx1d(1, 0, 0);
    assert_eq!(c, 1);
    let c = approx1d(0, 3, 10);
    assert_eq!(c, 3);
    let c = approx1d(0, -2, 10);
    assert_eq!(c, 7);
    let c = approx1d(10, 3, 20);
    assert_eq!(c, 3);
    let c = approx1d(-10, 3, -20);
    assert_eq!(c, 8);
    let c = approx1d(-1, -1, 1);
    assert_eq!(c, 3);
}
