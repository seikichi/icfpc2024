use glam::I64Vec2;

use crate::spaceship_input::SpaceshipInput;
use crate::spaceship_solution::SpaceshipSolution;

use super::HeadAI;

pub struct StarOnlyAI {}

type V2 = I64Vec2;

impl HeadAI for StarOnlyAI {
    fn solve(&mut self, input: &SpaceshipInput) -> SpaceshipSolution {
        let (is_star, start_pos, n_stars) = make_field(input);
        let n_stars_left = if is_star[start_pos.y as usize][start_pos.x as usize] {
            n_stars - 1
        } else {
            n_stars
        };
        print_field(&is_star);
        let mut visit = vec![vec![false; is_star[0].len()]; is_star.len()];
        let best_moves = dfs(&is_star, &mut visit, start_pos, V2::ZERO, n_stars_left);
        let best_moves: Vec<char> = best_moves.expect("StarOnly: no solutions found").into_iter().rev().collect();
        SpaceshipSolution {
            moves: best_moves,
            order: vec![],
        }
    }
}

fn detect_field_bounds(input: &SpaceshipInput) -> (i64, i64, i64, i64) {
    let mut min_x = 0;
    let mut min_y = 0;
    let mut max_x = 0;
    let mut max_y = 0;
    for pos in &input.poss {
        let x = pos[0];
        let y = pos[1];
        min_x = min_x.min(x);
        min_y = min_y.min(y);
        max_x = max_x.max(x);
        max_y = max_y.max(y);
    }
    (min_x, min_y, max_x, max_y)
}

fn make_field(input: &SpaceshipInput) -> (Vec<Vec<bool>>, V2, usize) {
    let (min_x, min_y, max_x, max_y) = detect_field_bounds(input);
    let size_y = (max_y - min_y + 1) as usize;
    let size_x = (max_x - min_x + 1) as usize;
    let mut is_star = vec![vec![false; size_x]; size_y];
    let mut n_stars = 0;
    for pos in &input.poss {
        let x = (pos[0] - min_x) as usize;
        let y = (pos[1] - min_y) as usize;
        if !is_star[y][x] {
            is_star[y][x] = true;
            n_stars += 1;
        }
    }
    (is_star, V2::new(-min_x, -min_y), n_stars)
}

fn print_field(is_star: &[Vec<bool>]) {
    for row in is_star {
        for b in row {
            print!("{}", if *b { '*' } else { '.' });
        }
        println!();
    }
}

fn is_in_field<T>(field: &[Vec<T>], p: V2) -> bool {
    (0 <= p.y)
        && ((p.y as usize) < field.len())
        && (0 <= p.x)
        && ((p.x as usize) < field[p.y as usize].len())
}

fn to_move(dx: i64, dy: i64) -> char {
    //    -1 0 1
    // -1  1 2 3
    //  0  4 5 6
    //  1  7 8 9
    (((dy + 1) * 3 + (dx + 1) + 1) as u8 + '0' as u8) as char
}

fn dfs(is_star: &[Vec<bool>], visit: &mut [Vec<bool>], p: V2, v: V2, n_stars_left: usize) -> Option<Vec<char>> {
    eprintln!("p={p}, v={v}, n_stars_left={n_stars_left}");

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
            if !is_in_field(is_star, np) {
                continue;
            }
            if !is_star[np.y as usize][np.x as usize] {
                continue;
            }
            if visit[np.y as usize][np.x as usize] {
                continue;
            }
            visit[np.y as usize][np.x as usize] = true;
            if let Some(mut moves) = dfs(is_star, visit, np, nv, n_stars_left - 1) {
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
            visit[np.y as usize][np.x as usize] = false;
        }
    }
    best_moves
}
