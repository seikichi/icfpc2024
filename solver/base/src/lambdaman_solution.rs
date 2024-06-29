#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct LambdamanSolution {
    pub moves: Vec<Vec<char>>,
    pub order: Vec<usize>,
}

impl LambdamanSolution {
    pub fn score(&self) -> i64 {
        return self.moves.len() as i64;
    }
    pub fn answer(&self) -> String {
        return String::from_iter(self.moves.clone().concat().iter());
    }
}
