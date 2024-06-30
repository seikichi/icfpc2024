pub mod dsl;
pub mod eval;
pub mod lambdaman_input;
pub mod lambdaman_solution;
pub mod spaceship_input;
pub mod spaceship_solution;
pub mod three_d;

// NOTE: 動作確認用
pub fn add(a: u64, b: u64) -> u64 {
    a + b
}

#[cfg(test)]
mod tests {
    use super::*;

    // NOTE: 動作確認用
    #[test]
    fn add_test() {
        assert_eq!(add(40, 2), 42);
    }
}
