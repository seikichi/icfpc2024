use crate::input;
use crate::solution::Solution;

use super::{ChainedAI, HeadAI};

pub struct SimpleHeadAI {}
pub struct SimpleChainedAI {}

impl HeadAI for SimpleHeadAI {
    fn solve(&mut self, _input: &input::Input) -> Solution {
        Solution {}
    }
}

impl ChainedAI for SimpleChainedAI {
    fn solve(&mut self, _input: &input::Input, _solution: &Solution) -> Solution {
        Solution {}
    }
}
