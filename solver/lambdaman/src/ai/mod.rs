mod simple;

pub use simple::*;

use crate::lambdaman_input;
use crate::lambdaman_solution::LambdamanSolution;

pub trait HeadAI {
    fn solve(&mut self, input: &lambdaman_input::LambdamanInput) -> LambdamanSolution;
}

pub trait ChainedAI {
    fn solve(
        &mut self,
        input: &lambdaman_input::LambdamanInput,
        solution: &LambdamanSolution,
    ) -> LambdamanSolution;
}
