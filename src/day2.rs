use std::fs::File;
use std::io::{BufRead, BufReader};

use anyhow::{Context, Result};

use crate::problem::{Part, Solved};

// TODO(dkimbel): POTENTIAL IMPROVEMENTS
//   - Reduce boilerplate shared between Day modules. Is a trait the key? At the
//     worst, create a 'template' file I can `cp` to make a new Day, and add it
//     to README.

const INPUT_FILE_PATH: &str = "src/day2/resources/puzzle_inputs";

pub struct Day2;

impl Day2 {
    fn solve(problem_part: Part, input_file_path: &str) -> Result<u32> {
        let file = File::open(input_file_path)?;
        let reader = BufReader::new(file);
        let mut solution = 0;
        for line in reader.lines() {
            let validator = PasswordValidator::new(&line?)?;
            if validator.is_valid(problem_part)? {
                solution += 1;
            }
        }
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

    fn is_valid(&self, problem_part: Part) -> Result<bool> {
        self.policy.allows_password(&self.password, problem_part)
    }
}

struct PasswordPolicy {
    character:       char,
    lower_range_num: usize,
    upper_range_num: usize,
}

impl PasswordPolicy {
    // expects a string like "1-3 a"
    fn new(input: &str) -> Result<Self> {
        let (range_input, char_input) = input
            .split_once(" ")
            .context("Failed to split policy input into range and character")?;
        let (lower_range_input, upper_range_input) = range_input
            .split_once("-")
            .context("Failed to split range input into min and max")?;
        Ok(Self {
            lower_range_num: lower_range_input.parse::<usize>()?,
            upper_range_num: upper_range_input.parse::<usize>()?,
            character:       char_input
                .chars()
                .next()
                .context("Failed to extract char from policy input")?,
        })
    }

    fn allows_password(&self, password: &str, problem_part: Part) -> Result<bool> {
        match problem_part {
            Part::One => {
                // In part one, we treat the policy's upper_range_num as the max allowable
                // number of occurrences of the character. Similarly, we treat lower_range_num
                // as the min allowable number.
                let char_occurrences =
                    password.chars().fold(
                        0,
                        |acc, char| if char == self.character { acc + 1 } else { acc },
                    );
                Ok(char_occurrences <= self.upper_range_num
                    && char_occurrences >= self.lower_range_num)
            },
            Part::Two => {
                // In part two, we require that the character number specified by the lower num
                // XOR the character number specified by the upper num must equal the given
                // char. Note that these are one-based indexes, not zero-based.
                let chars = password.chars().collect::<Vec<_>>();
                let first_char_match = chars
                    .get(self.lower_range_index()?)
                    .context("Password must contain lower char number specified by policy")?
                    == &self.character;
                let second_char_match = chars
                    .get(self.upper_range_index()?)
                    .context("Password must contain upper char number specified by policy")?
                    == &self.character;
                Ok((first_char_match && !second_char_match)
                    || (!first_char_match && second_char_match))
            },
        }
    }

    fn lower_range_index(&self) -> Result<usize> {
        Self::convert_to_index(self.lower_range_num)
    }

    fn upper_range_index(&self) -> Result<usize> {
        Self::convert_to_index(self.upper_range_num)
    }

    fn convert_to_index(num: usize) -> Result<usize> {
        num.checked_sub(1).context("Subtraction from usize failed")
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
        assert_eq!(solution, 1);
    }
}
