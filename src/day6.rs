use std::collections::HashSet;
use std::fs;

use anyhow::{Context, Result};

use crate::problem::{Part, Solved};

const INPUT_FILE_PATH: &str = "src/day6/puzzle_inputs";

pub struct Day6;

impl Day6 {
    fn solve(problem_part: Part, input_file_path: &str) -> Result<usize> {
        let input = fs::read_to_string(input_file_path).context("Unable to read input file")?;
        match problem_part {
            Part::One => {
                let groups = input.split("\n\n").map(Group::new);
                let summed_totals: usize = groups.map(|g| g.num_unique_answers()).sum();
                Ok(summed_totals)
            },
            Part::Two => todo!(),
        }
    }
}

struct Group {
    unique_answers: HashSet<char>,
}

impl Group {
    fn new(input: &str) -> Self {
        let mut unique_answers = HashSet::<char>::new();
        for individuals_answers in input.split('\n') {
            for char in individuals_answers.trim().chars() {
                unique_answers.insert(char);
            }
        }

        Self { unique_answers }
    }

    fn num_unique_answers(&self) -> usize {
        self.unique_answers.len()
    }
}

impl Solved for Day6 {
    fn print_solution(part: Part) {
        let solution = Self::solve(part, INPUT_FILE_PATH).unwrap();
        println!("Day 6 {} solution: {}", part, solution);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_FILE_PATH: &str = "src/day6/sample";

    #[test]
    fn test_part_one() {
        let solution = Day6::solve(Part::One, TEST_FILE_PATH).unwrap();
        assert_eq!(solution, 11);
    }

    #[test]
    #[ignore]
    fn test_part_two() {
        let solution = Day6::solve(Part::Two, TEST_FILE_PATH).unwrap();
        assert_eq!(solution, 820);
    }
}
