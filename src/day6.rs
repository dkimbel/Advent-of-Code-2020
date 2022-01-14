use std::collections::HashSet;
use std::fs;

use anyhow::{anyhow, Context, Result};

use crate::problem::{Part, Solved};

const INPUT_FILE_PATH: &str = "src/day6/puzzle_inputs";

pub struct Day6;

impl Day6 {
    fn solve(problem_part: Part, input_file_path: &str) -> Result<usize> {
        let input = fs::read_to_string(input_file_path).context("Unable to read input file")?;
        match problem_part {
            Part::One => {
                let groups = input.split("\n\n").map(UniqueGroup::new);
                let summed_totals: usize = groups.map(|g| g.num_unique_answers()).sum();
                Ok(summed_totals)
            },
            Part::Two => {
                let groups = input.split("\n\n").map(ConsensusGroup::new);
                let mut summed_totals = 0;
                for maybe_group in groups {
                    let group = maybe_group.context("All groups must be defined")?;
                    summed_totals += group.num_shared_answers();
                }
                Ok(summed_totals)
            },
        }
    }
}

struct ConsensusGroup {
    shared_answers: HashSet<char>,
}

impl ConsensusGroup {
    fn new(input: &str) -> Result<Self> {
        let mut maybe_shared_answers: Option<HashSet<char>> = None;
        let cleaned_input = input
            .split('\n')
            .map(|s| s.trim())
            .filter(|s| !s.is_empty());
        for individuals_answers in cleaned_input {
            let individuals_set = individuals_answers.chars().collect::<HashSet<_>>();
            maybe_shared_answers = match maybe_shared_answers {
                None => Some(individuals_set),
                Some(shared_set) => {
                    let intersect_refs = shared_set.intersection(&individuals_set);
                    let intersect = intersect_refs.copied();
                    Some(intersect.collect::<HashSet<char>>())
                },
            }
        }

        let shared_answers = maybe_shared_answers.ok_or(anyhow!("Input was empty"))?;
        Ok(Self { shared_answers })
    }

    fn num_shared_answers(&self) -> usize {
        self.shared_answers.len()
    }
}

struct UniqueGroup {
    unique_answers: HashSet<char>,
}

impl UniqueGroup {
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
    fn test_part_two() {
        let solution = Day6::solve(Part::Two, TEST_FILE_PATH).unwrap();
        assert_eq!(solution, 6);
    }
}
