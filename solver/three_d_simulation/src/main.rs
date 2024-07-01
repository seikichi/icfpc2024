use base::*;
use std::path::PathBuf;

// use anyhow::bail;
use log::info;
use structopt::StructOpt;

fn main() -> anyhow::Result<()> {
    run()?;
    Ok(())
}

#[derive(Debug, StructOpt)]
#[structopt(name = "solver", about = "A solver of ICFPC 2024 problems")]
struct Opt {
    #[structopt(short = "i", long = "input", parse(from_os_str))]
    input_path: PathBuf,

    // #[structopt(short = "o", long = "output-dir", parse(from_os_str))]
    // output_dir: PathBuf,
    #[structopt(short = "q", help = "disable debug log")]
    quiet: bool,

    #[structopt(short = "Q", help = "disable debug and info log")]
    super_quiet: bool,

    #[structopt(short = "a")]
    initial_a: i128,

    #[structopt(short = "b")]
    initial_b: i128,

    #[structopt(short = "t", long = "max-step", default_value = "100")]
    max_step: i64,
    // #[structopt(long = "load-path", default_value = "")]
    // load_path: String,

    // #[structopt(
    //     long = "load-old",
    //     help = "use old score (This flag is checked by Lambda)"
    // )]
    // load_old: bool,
}

// 標準出力に JSON 形式で出力し、Lambda の JS が DB に書き込む
// #[derive(Debug, serde::Serialize)]
// struct Output {
//     solution: String,
//     score: usize, // usize でなくてもいい (JS から見たら一緒...)
// }

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

    let original_input = three_d::three_d_input::load_from_file(opt.input_path.clone())?;
    let mut simulator =
        three_d::simulator::Simulator::new(&original_input, opt.initial_a, opt.initial_b);

    for t in 0..opt.max_step {
        info!("step: {}", t);
        if let Some(ans) = simulator.step() {
            println!("answer: {}", ans);
            break;
        }
        println!("{}", simulator.boards.last().unwrap());
    }
    // let answer = solution.answer();
    // let output = Output {
    //     solution: answer.clone(),
    //     score: answer.len(),
    // };
    // println!("{}", serde_json::to_string(&output)?);

    Ok(())
}
