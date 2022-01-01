use std::fs::File;
use std::io::{BufRead, BufReader};

use anyhow::{anyhow, Result};

use crate::problem::{Part, Solved};

// TODO(dkimbel): POTENTIAL IMPROVEMENTS
//   - create a custom iterator that can take an arbitrary number of input vecs
//     and return a vec (or ideally array) of their combined values; then use
//     that to solve parts one and two using the same code

const INPUT_FILE_PATH: &str = "src/day1/puzzle_inputs";

pub struct Day1;

impl Day1 {
    fn solve(problem_part: Part, input_file_path: &str) -> Result<u32> {
        let analyzer = ExpenseAnalyzer::new(input_file_path)?;

        let solution = match problem_part {
            Part::One => {
                let (expense_1, expense_2) = analyzer.find_summing_pair(2020)?;
                expense_1 * expense_2
            },
            Part::Two => {
                let (expense_1, expense_2, expense_3) = analyzer.find_summing_triple(2020)?;
                expense_1 * expense_2 * expense_3
            },
        };
        Ok(solution)
    }
}

impl Solved for Day1 {
    fn print_solution(part: Part) {
        let solution = Self::solve(part, INPUT_FILE_PATH).unwrap();
        println!("Day 1 {} solution: {}", part, solution);
    }
}

#[derive(Debug)]
struct ExpenseAnalyzer {
    expenses: Vec<u32>,
}

impl ExpenseAnalyzer {
    fn new(file_path: &str) -> Result<Self> {
        let file = File::open(file_path)?;
        let reader = BufReader::new(file);

        let mut expenses = Vec::new();
        for line in reader.lines() {
            let expense: u32 = line?.parse()?;
            expenses.push(expense);
        }

        Ok(Self { expenses })
    }

    fn find_summing_pair(&self, target_sum: u32) -> Result<(u32, u32)> {
        let num_expenses = self.expenses.len();
        for i in 0..num_expenses {
            for j in (i + 1)..num_expenses {
                let expense_i = self.expenses[i];
                let expense_j = self.expenses[j];
                if expense_i + expense_j == target_sum {
                    return Ok((expense_i, expense_j));
                }
            }
        }
        Err(anyhow!(
            "Could not find a pair of expenses summing to {}",
            target_sum
        ))
    }

    fn find_summing_triple(&self, target_sum: u32) -> Result<(u32, u32, u32)> {
        let num_expenses = self.expenses.len();
        for i in 0..num_expenses {
            for j in (i + 1)..num_expenses {
                for k in (j + 1)..num_expenses {
                    let expense_i = self.expenses[i];
                    let expense_j = self.expenses[j];
                    let expense_k = self.expenses[k];
                    if expense_i + expense_j + expense_k == target_sum {
                        return Ok((expense_i, expense_j, expense_k));
                    }
                }
            }
        }
        Err(anyhow!(
            "Could not find a set of expenses summing to {}",
            target_sum
        ))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_FILE_PATH: &str = "src/day1/sample";

    #[test]
    fn test_part_one() {
        let solution = Day1::solve(Part::One, TEST_FILE_PATH).unwrap();
        assert_eq!(solution, 514579);
    }

    #[test]
    fn test_part_two() {
        let solution = Day1::solve(Part::Two, TEST_FILE_PATH).unwrap();
        assert_eq!(solution, 241861950);
    }
}
