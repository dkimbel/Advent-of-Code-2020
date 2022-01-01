use anyhow::Result;

use crate::problem::{Part, Solved};

const INPUT_FILE_PATH: &str = "src/day3/puzzle_inputs";

pub struct Day3;

impl Day3 {
    fn solve(problem_part: Part, input_file_path: &str) -> Result<u32> {
        todo!()
    }
}

impl Solved for Day3 {
    fn print_solution(part: Part) {
        let solution = Self::solve(part, INPUT_FILE_PATH).unwrap();
        println!("Day 3 {} solution: {}", part, solution);
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    const TEST_FILE_PATH: &str = "src/day3/sample";

    #[test]
    fn test_part_one() {
        let solution = Day3::solve(Part::One, TEST_FILE_PATH).unwrap();
        todo!()
        // assert_eq!(solution, 2);
    }

    #[test]
    fn test_part_two() {
        let solution = Day3::solve(Part::Two, TEST_FILE_PATH).unwrap();
        todo!()
        // assert_eq!(solution, 1);
    }
}
