mod annealing;
mod common;
mod simple;
mod staronly;

pub use annealing::*;
pub use simple::*;
pub use staronly::*;

use crate::spaceship_input;
use crate::spaceship_solution::SpaceshipSolution;

pub trait HeadAI {
    fn solve(&mut self, input: &spaceship_input::SpaceshipInput) -> SpaceshipSolution;
}

pub trait ChainedAI {
    #[allow(dead_code)]
    fn solve(
        &mut self,
        input: &spaceship_input::SpaceshipInput,
        solution: &SpaceshipSolution,
    ) -> SpaceshipSolution;
}
