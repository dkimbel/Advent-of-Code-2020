use std::fs::File;
use std::io::{BufRead, BufReader};

use anyhow::{Context, Result};

use crate::problem::{Part, Solved};

// TODO(dkimbel): POTENTIAL IMPROVEMENTS
//   - Figure out how to use `?` within a closure
//   - Figure out how to collapse validator.is_valid check to one line
//   - Reduce boilerplate shared between Day modules. Is a trait the key? At the
//     worst, create a 'template' file I can `cp` to make a new Day, and add it
//     to README.

const INPUT_FILE_PATH: &str = "src/day2/resources/puzzle_inputs";

pub struct Day2;

impl Day2 {
    fn solve(problem_part: Part, input_file_path: &str) -> Result<u32> {
        let file = File::open(input_file_path)?;
        let reader = BufReader::new(file);
        let solution = match problem_part {
            Part::One => reader.lines().fold(0, |acc, line| {
                let validator = PasswordValidator::new(&line.unwrap()).unwrap();
                if validator.is_valid() {
                    acc + 1
                } else {
                    acc
                }
            }),
            Part::Two => todo!(),
        };
        Ok(solution)
    }
}

impl Solved for Day2 {
    fn print_solution(part: Part) {
        let solution = Self::solve(part, INPUT_FILE_PATH).unwrap();
        println!("Day 2 {} solution: {}", part, solution);
    }
}

struct PasswordValidator {
    policy:   PasswordPolicy,
    password: String,
}

impl PasswordValidator {
    // expects a string like "1-3 a: abcde"
    fn new(input: &str) -> Result<Self> {
        let (policy_input, password) = input
            .split_once(": ")
            .context("Failed to split validator input into policy and password")?;
        let policy = PasswordPolicy::new(policy_input)?;
        Ok(Self {
            policy,
            password: String::from(password),
        })
    }

    fn is_valid(&self) -> bool {
        self.policy.allows_password(&self.password)
    }
}

struct PasswordPolicy {
    character: char,
    min_count: u8,
    max_count: u8,
}

impl PasswordPolicy {
    // expects a string like "1-3 a"
    fn new(input: &str) -> Result<Self> {
        let (range_input, char_input) = input
            .split_once(" ")
            .context("Failed to split policy input into range and character")?;
        let (min_input, max_input) = range_input
            .split_once("-")
            .context("Failed to split range input into min and max")?;
        Ok(Self {
            min_count: min_input.parse::<u8>()?,
            max_count: max_input.parse::<u8>()?,
            character: char_input
                .chars()
                .next()
                .context("Failed to extract char from policy input")?,
        })
    }

    fn allows_password(&self, password: &str) -> bool {
        let char_occurrences =
            password.chars().fold(
                0,
                |acc, char| if char == self.character { acc + 1 } else { acc },
            );
        char_occurrences <= self.max_count && char_occurrences >= self.min_count
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_FILE_PATH: &str = "src/day2/resources/sample";

    #[test]
    fn test_part_one() {
        let solution = Day2::solve(Part::One, TEST_FILE_PATH).unwrap();
        assert_eq!(solution, 2);
    }

    #[test]
    fn test_part_two() {
        let solution = Day2::solve(Part::Two, TEST_FILE_PATH).unwrap();
        assert_eq!(solution, 241861950);
    }
}
