use crate::ai::common;
use crate::spaceship_input;
use crate::spaceship_solution::SpaceshipSolution;

use log::info;
use rand::prelude::*;
use std::{
    // thread::current,
    time::{Duration, Instant},
    vec,
};

use super::ChainedAI;

pub struct AnnealingChainedAI {
    pub time_limit: Duration,
    pub initial_temperature: f64,
}
impl AnnealingChainedAI {
    fn neighbor_move(
        &self,
        input: &spaceship_input::SpaceshipInput,
        solution: &SpaceshipSolution,
        rng: &mut SmallRng,
    ) -> SpaceshipSolution {
        let index1 = rng.gen_range(0..input.n);
        let mut index2 = rng.gen_range(0..input.n);
        while index1 == index2 {
            index2 = rng.gen_range(0..input.n);
        }
        let mut p = vec![0, 0];
        let mut v = vec![0, 0];
        let mut moves = vec![];
        let mut order = solution.order.clone();
        order.swap(index1, index2);
        for i in 0..input.n {
            let target = order[i];
            let tp = &input.poss[target];
            let (nv, mut nmoves) = common::make_move(&p, &tp, &v);
            p = tp.clone();
            v = nv;
            moves.append(&mut nmoves);
        }
        return SpaceshipSolution { moves, order };
    }
}

impl ChainedAI for AnnealingChainedAI {
    fn solve(
        &mut self,
        input: &spaceship_input::SpaceshipInput,
        initial_solution: &SpaceshipSolution,
    ) -> SpaceshipSolution {
        let mut solution = initial_solution.clone();
        let mut rng = SmallRng::from_entropy();
        let mut current_score = solution.score() * -1;
        let start_at = Instant::now();

        let mut best_solution = solution.clone();
        let mut best_score = current_score;

        let mut temperature = self.initial_temperature;

        let mut valid_solution_count = 0;
        let mut invalid_solution_count = 0;
        let mut accept_count = 0;
        let mut reject_count = 0;

        let mut iter = 0;
        loop {
            // check time limit
            iter += 1;
            if iter % 5 == 0 {
                let elapsed = Instant::now() - start_at;
                if elapsed >= self.time_limit {
                    // print stats
                    info!("iter = {}", iter);
                    info!(
                        "#valid_move   = {} ({:.2} %)",
                        valid_solution_count,
                        100.0 * valid_solution_count as f64
                            / (valid_solution_count + invalid_solution_count) as f64
                    );
                    info!(
                        "#invalid_move = {} ({:.2} %)",
                        invalid_solution_count,
                        100.0 * invalid_solution_count as f64
                            / (valid_solution_count + invalid_solution_count) as f64
                    );
                    info!(
                        "#accept = {} ({:.2} %)",
                        accept_count,
                        100.0 * accept_count as f64 / (accept_count + reject_count) as f64
                    );
                    info!(
                        "#reject = {} ({:.2} %)",
                        reject_count,
                        100.0 * reject_count as f64 / (accept_count + reject_count) as f64
                    );
                    // done!
                    return best_solution;
                }

                // tweak temperature
                let progress = elapsed.as_secs_f64() / self.time_limit.as_secs_f64();
                temperature = self.initial_temperature * (1.0 - progress) * (-progress).exp2();
            }

            // 後でロールバックできるように解を保存しておく
            // TODO: もっと効率よく保持できるかも
            let old_solution = solution.clone();

            // move to neighbor
            solution = self.neighbor_move(&input, &solution, &mut rng);

            //
            let new_score = solution.score() * -1;
            let is_valid_solution = true;
            if is_valid_solution {
                valid_solution_count += 1;
            } else {
                invalid_solution_count += 1;
            }

            if iter % 100 == 0 {
                if is_valid_solution {
                    info!(
                        "new_score = {}, best_score = {}",
                        new_score * -1,
                        best_score * -1
                    );
                } else {
                    info!("new_score = n/a");
                }
            }

            // 新しい解を受理するか決める
            let accept = {
                // 解が不正な場合は受理しない
                if !is_valid_solution {
                    false
                }
                // スコアが改善するなら必ず受理する
                else if new_score > current_score {
                    true
                }
                // そうでない場合はある確率で受理する
                else {
                    // new_score <= current_score
                    let delta = current_score - new_score;
                    let accept_prob = (-delta as f64 / temperature).exp();
                    rng.gen::<f64>() < accept_prob
                }
            };
            if accept {
                // accept candidate
                current_score = new_score;
                accept_count += 1;
            } else {
                // reject candidate
                solution = old_solution;
                reject_count += 1;
            }

            if current_score > best_score {
                best_score = current_score;
                best_solution = solution.clone();
            }
        }
    }
}
