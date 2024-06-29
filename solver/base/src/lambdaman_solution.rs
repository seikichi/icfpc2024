#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct LambdamanSolution {
    pub moves: Vec<char>,
    pub order: Vec<usize>,
}
