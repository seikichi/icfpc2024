#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct SpaceshipSolution {
    pub moves: Vec<char>,
    pub order: Vec<usize>,
}

impl SpaceshipSolution {
    pub fn score(&self) -> i64 {
        return self.moves.len() as i64;
    }
}
