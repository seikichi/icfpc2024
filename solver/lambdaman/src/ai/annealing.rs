use crate::lambdaman_input;
use crate::lambdaman_solution::LambdamanSolution;

use log::info;
use rand::prelude::*;
use std::{
    collections::HashMap,
    time::{Duration, Instant},
};

use super::ChainedAI;

pub struct AnnealingChainedAI {
    pub time_limit: Duration,
    pub initial_temperature: f64,
    pub dead_end_ratio: f32,
    pub other_ratio: f32,
}

pub struct AnnealingChainedAIData {
    start_pos: (usize, usize),
    food_positions: Vec<(usize, usize)>,
    food_indexs: HashMap<(usize, usize), usize>,
    dead_end_indexs: Vec<usize>,
}

impl AnnealingChainedAI {
    pub fn neighbor_move(
        &self,
        input: &lambdaman_input::LambdamanInput,
        solution: &LambdamanSolution,
        rng: &mut SmallRng,
        data: &AnnealingChainedAIData,
    ) -> LambdamanSolution {
        let food_n = data.food_positions.len();
        let method_r = rng.gen::<f32>() * (self.dead_end_ratio + self.other_ratio);

        // targetを食べる順番をeat_timing直後に変える
        let target = if method_r < self.dead_end_ratio {
            let t = rng.gen_range(0..data.dead_end_indexs.len());
            data.dead_end_indexs[t]
        } else {
            rng.gen_range(0..food_n)
        };
        let eat_timing = rng.gen_range(0..food_n);

        let mut pos = data.start_pos;
        let mut old_solution_pos = data.start_pos;
        let mut eaten = vec![false; food_n];
        // for i in 0..n {
        //     if eaten[i] { continue; }
        //     if i == eat_timing 
        // }
        // TODO
    }
}

impl AnnealingChainedAIData {
    pub fn new(input: &lambdaman_input::LambdamanInput) -> Self {
        let start_pos = input.find_start_position();
        let food_position = input.make_food_positions();
        let food_indexs = input.make_food_indexs();
        let mut dead_end_indexs = vec![];
        for i in 0..food_position.len() {
            let mut wall_cnt = 0;
            for dir in 0..4 {
                let dx = [1, 0, -1, 0];
                let dy = [0, 1, 0, -1];
                let nx = food_position[i].0 as i64 + dx[dir];
                let ny = food_position[i].1 as i64 + dy[dir];
                if nx < 0
                    || ny < 0
                    || nx as usize >= input.w
                    || ny as usize >= input.h
                    || input.field[ny as usize][nx as usize] == '#'
                {
                    wall_cnt += 1;
                    continue;
                }
            }
            if wall_cnt == 3 {
                dead_end_indexs.push(i);
            }
        }
        AnnealingChainedAIData {
            start_pos,
            food_positions: food_position,
            food_indexs,
            dead_end_indexs,
        }
    }
}

impl ChainedAI for AnnealingChainedAI {
    fn solve(
        &mut self,
        input: &lambdaman_input::LambdamanInput,
        initial_solution: &LambdamanSolution,
    ) -> LambdamanSolution {
        let data = AnnealingChainedAIData::new(input);

        let mut solution = initial_solution.clone();
        let mut rng = SmallRng::from_entropy();
        let mut current_score = solution.score();
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
            solution = self.neighbor_move(&input, &solution, &mut rng, &data);

            //
            let new_score = solution.score();
            let is_valid_solution = true;
            if is_valid_solution {
                valid_solution_count += 1;
            } else {
                invalid_solution_count += 1;
            }

            if iter % 100 == 0 {
                if is_valid_solution {
                    info!("new_score = {}", new_score);
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
        return best_solution.clone();
    }
}
