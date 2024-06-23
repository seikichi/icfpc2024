mod simple;

pub use simple::*;

use crate::input;
use crate::solution::Solution;

pub trait HeadAI {
    // TODO: allow(dead_code) 削除
    #[allow(dead_code)]
    fn solve(&mut self, input: &input::Input) -> Solution;
}

pub trait ChainedAI {
    // TODO: allow(dead_code) 削除
    #[allow(dead_code)]
    fn solve(&mut self, input: &input::Input, solution: &Solution) -> Solution;
}
