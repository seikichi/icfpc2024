use std::collections::{HashSet, VecDeque};

use glam::I64Vec2;
use log::{debug, info};

use crate::spaceship_input::SpaceshipInput;
use crate::spaceship_solution::SpaceshipSolution;

use super::HeadAI;

pub struct NearestAI {
    pub allowed_miss: usize,
}

type V2 = I64Vec2;

impl HeadAI for NearestAI {
    fn solve(&mut self, input: &SpaceshipInput) -> SpaceshipSolution {
        let (stars, _) = make_field(input);
        //print_field(&is_star);
        let best_moves = doit(&stars, self.allowed_miss);
        let best_moves: Vec<char> = best_moves.into_iter().rev().collect();
        SpaceshipSolution {
            moves: best_moves,
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

fn doit(
    stars: &HashSet<V2>,
    allowed_miss: usize,
) -> Vec<char> {
    let mut state = State {
        p: V2::ZERO,
        v: V2::ZERO,
        miss: 0,
        visit: im_rc::HashSet::new(),
        moves: vec![],
    };
    let mut path: Vec<char> = vec![];
    let mut visit: HashSet<V2> = HashSet::new();
    let mut current_miss = 0;
    while visit.len() < stars.len() {
        info!("p={}, v={}, miss={}, cost={}, stars_visit={}", state.p, state.v, current_miss, path.len(), visit.len());

        // 現在の座標からもっともユークリット距離が近い星に行く
        let mut min_dist = 10000000;
        let mut target = V2::ZERO;
        for star in stars {
            if visit.contains(&star) {
                continue;
            }
            let dist = (*star - state.p).length_squared();
            if dist < min_dist {
                min_dist = dist;
                target = *star;
            }
        }

        let nstate = bfs(stars, state.p, state.v, target, allowed_miss - current_miss).expect("Nearest: no solutions");
        state = nstate.clone();
        path.extend(nstate.moves);
        visit.insert(target);
        current_miss += nstate.miss;
    }
    path
}

#[derive(Debug, Clone)]
struct State {
    p: V2,
    v: V2,
    miss: usize,
    visit: im_rc::HashSet<V2>,
    moves: Vec<char>,
}

fn bfs(
    stars: &HashSet<V2>,
    start: V2, initial_v: V2, target: V2,
    allowed_miss: usize,
) -> Option<State> {
    let mut queue = VecDeque::new();
    queue.push_front(State {
        p: start,
        v: initial_v,
        miss: 0,
        visit: im_rc::HashSet::new(),
        moves: vec![],
    });

    while let Some(state) = queue.pop_back() {
        debug!("p={}, v={}, target={}, miss={}", state.p, state.v, target, state.miss);

        if state.p == target {
            // クリア
            return Some(state);
        }

        // 状態(p, v)から遷移可能な状態をすべてバックトラックで探索する
        for dy in -1..=1 {
            for dx in -1..=1 {
                let nv = state.v + V2::new(dx, dy);
                let np = state.p + nv;
                let mut is_miss = 0;
                if !stars.contains(&np) {
                    if state.miss < allowed_miss {
                        // 星じゃない場所を踏むことを許す
                        is_miss = 1;
                    } else {
                        continue;
                    }
                }
                if state.visit.contains(&np) {
                    continue;
                }
                let nvisit = state.visit.update(np);
                let mut nmoves = state.moves.clone();
                nmoves.push(to_move(dx, dy));

                queue.push_front(State {
                    p: np,
                    v: nv,
                    miss: state.miss + is_miss,
                    visit: nvisit,
                    moves: nmoves,
                });
            }
        }
    }
    None
}
