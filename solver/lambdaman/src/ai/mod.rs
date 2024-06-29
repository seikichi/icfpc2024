mod annealing;
mod simple;

pub use annealing::*;
pub use simple::*;

use crate::lambdaman_input;
use crate::lambdaman_solution::LambdamanSolution;

pub trait HeadAI {
    fn solve(&mut self, input: &lambdaman_input::LambdamanInput) -> LambdamanSolution;
}

pub trait ChainedAI {
    #[allow(dead_code)]
    fn solve(
        &mut self,
        input: &lambdaman_input::LambdamanInput,
        solution: &LambdamanSolution,
    ) -> LambdamanSolution;
}
