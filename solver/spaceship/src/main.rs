mod ai;

use crate::ai::{ChainedAI, HeadAI};
use base::*;
use std::path::PathBuf;

use anyhow::bail;
use log::info;
use std::time::Duration;
use structopt::StructOpt;

fn main() -> anyhow::Result<()> {
    run()?;
    Ok(())
}

#[derive(Debug, StructOpt)]
#[structopt(name = "solver", about = "A solver of ICFPC 2024 problems")]
struct Opt {
    // Original: https://github.com/seikichi/icfpc2022/blob/master/solver/core/src/lib.rs
    #[structopt(
        short = "a",
        long = "ai",
        help = "comma separated list of AIs, e.g. 'Cross,Refine'"
    )]
    ai: String,

    #[structopt(short = "i", long = "input", parse(from_os_str))]
    input_path: PathBuf,

    // #[structopt(short = "o", long = "output-dir", parse(from_os_str))]
    // output_dir: PathBuf,
    #[structopt(short = "q", help = "disable debug log")]
    quiet: bool,

    #[structopt(short = "Q", help = "disable debug and info log")]
    super_quiet: bool,

    #[structopt(long = "annealing-seconds", default_value = "10")]
    annealing_seconds: u64,

    #[structopt(long = "annealing-initial-temperature", default_value = "1000.0")]
    annealing_initial_temperature: f64,

    #[structopt(long = "nearest-allowed-miss", default_value = "0")]
    nearest_allowed_miss: usize,

    #[structopt(long = "staronly-allowed-miss", default_value = "0")]
    staronly_allowed_miss: usize,

    #[structopt(long = "staronly-max-diff-star", default_value = "3")]
    staronly_max_diff_star: usize,
    // #[structopt(short = "p", default_value = "500.0", help = "prune threshold")]
    // prune_threshold: f32,
    // #[structopt(long = "annealing-swap-ratio", default_value = "30.0")]
    // annealing_swap_ratio: f32,

    // #[structopt(long = "annealing-move-ratio", default_value = "60.0")]
    // annealing_move_ratio: f32,

    // #[structopt(long = "annealing-multi-move-ratio", default_value = "10.0")]
    // annealing_multi_move_ratio: f32,

    // #[structopt(long = "load-path", default_value = "")]
    // load_path: String,

    // #[structopt(long = "greed-move-initial-move-distance", default_value = "1.0")]
    // greed_move_initial_move_distance: f32,

    // #[structopt(long = "greed-move-iteration-num", default_value = "20")]
    // greed_move_iteration_num: usize,

    // #[structopt(long = "greed-swap-iteration-num", default_value = "5")]
    // greed_swap_iteration_num: usize,

    // #[structopt(
    //     long = "load-old",
    //     help = "use old score (This flag is checked by Lambda)"
    // )]
    // load_old: bool,
}

// 標準出力に JSON 形式で出力し、Lambda の JS が DB に書き込む
#[derive(Debug, serde::Serialize)]
struct Output {
    solution: String,
    score: usize, // usize でなくてもいい (JS から見たら一緒...)
}

fn parse_ai_string(
    ai_str: &str,
    opt: &Opt,
) -> anyhow::Result<(Box<dyn HeadAI>, Vec<Box<dyn ChainedAI>>)> {
    let parts = ai_str.split(',').collect::<Vec<_>>();
    let head_ai: Box<dyn ai::HeadAI> = match parts[0] {
        // "Grid" => Box::new(ai::GridAI {}),
        // "GridGreed" => Box::new(ai::GridGreedAI {}),
        // "RingSide" => Box::new(ai::RingSideAI {}),
        // "RandomPut" => Box::new(ai::RandomPutAI {}),
        // "Load" => Box::new(ai::LoadAI {
        //     path: opt.load_path.clone(),
        // }),
        "Simple" => Box::new(ai::SimpleHeadAI {}),
        "Nearest" => Box::new(ai::NearestAI {
            allowed_miss: opt.nearest_allowed_miss,
        }),
        "StarOnly" => Box::new(ai::StarOnlyAI {
            allowed_miss: opt.staronly_allowed_miss,
        }),
        "AStarOnly" => Box::new(ai::AStarOnlyAI {
            allowed_miss: opt.staronly_allowed_miss,
            max_diff_star: opt.staronly_max_diff_star,
        }),
        "StarOnlySparse" => Box::new(ai::StarOnlySparseAI {
            allowed_miss: opt.staronly_allowed_miss,
        }),
        x => bail!("'{x}' is not a HeadAI"),
    };
    let mut chained_ais = vec![];
    for name in &parts[1..] {
        let chained_ai: Box<dyn ai::ChainedAI> = match *name {
            "Annealing" => Box::new(ai::AnnealingChainedAI {
                time_limit: Duration::from_secs(opt.annealing_seconds),
                initial_temperature: opt.annealing_initial_temperature,
            }),
            // "GreedMove" => Box::new(ai::GreedMoveAI {
            //     initial_move_distance: opt.greed_move_initial_move_distance,
            //     iteration_num: opt.greed_move_iteration_num,
            // }),
            // "GreedSwap" => Box::new(ai::GreedSwapAI {
            //     iteration_num: opt.greed_swap_iteration_num,
            // }),
            "Simple" => Box::new(ai::SimpleChainedAI {}),
            x => bail!("'{x}' is not a ChainedAI"),
        };
        chained_ais.push(chained_ai);
    }
    Ok((head_ai, chained_ais))
}

pub fn run() -> anyhow::Result<()> {
    let opt = Opt::from_args();

    // init logger
    let loglevel = if opt.super_quiet {
        "warn"
    } else if opt.quiet {
        "info"
    } else {
        "debug"
    };
    env_logger::Builder::from_env(env_logger::Env::default().default_filter_or(loglevel)).init();

    let (mut head_ai, chained_ais) = parse_ai_string(&opt.ai, &opt)?;

    // if !opt.output_dir.is_dir() {
    //     bail!("'{}' is not a directory", opt.output_dir.to_string_lossy());
    // }

    // let problem_id = opt
    //     .input_path
    //     .file_stem()
    //     .expect("--input should be a file name.")
    //     .to_string_lossy()
    //     .to_string();

    // let problem_number = problem_id.parse::<i32>().unwrap();

    let original_input = spaceship_input::load_from_file(opt.input_path.clone())?;

    let mut solution = head_ai.solve(&original_input);
    let mut score_history = vec![];
    score_history.push(solution.score());

    for mut chained_ai in chained_ais {
        solution = chained_ai.solve(&original_input, &solution);
        score_history.push(solution.score());
    }

    info!("Score History:");
    for (i, score) in score_history.iter().enumerate() {
        info!("    {i}: {score}")
    }

    // let volumes = score::make_volumes(&original_input, &solution);
    // let output_filename = opt.output_dir.join(problem_id.clone() + ".json");
    // info!("output JSON to: {}", output_filename.to_string_lossy());
    // output::save_to_file(output_filename, &solution, &volumes)?;

    // let score = score_history.last().unwrap();
    // let output = Output { score: *score };
    // println!("{}", serde_json::to_string(&output)?);

    let answer = String::from_iter(solution.moves.iter());
    let output = Output {
        solution: answer.clone(),
        score: answer.len(),
    };
    println!("{}", serde_json::to_string(&output)?);

    Ok(())
}
