use std::error::Error;
use std::fs;
use std::io::{BufRead, BufReader};

use anyhow::{anyhow, Result};

use crate::ProblemPart;

// TODO(dkimbel): POTENTIAL IMPROVEMENTS
//   - make some trait that lets every day's solution impl solve with an input
//     file path and a ProblemPart. Also make it expose the input file path
//     const/attr?
//   - refactor ProblemPart into its own file?
//   - start my own custom rustfmt config file
//   - make my own Clippy config
//   - try out the CLion profiler on this code
//   - create a custom iterator that can take an arbitrary number of input vecs
//     and return a vec (or ideally array) of their combined values; then use
//     that to solve parts one and two using the same code

pub const SAMPLE_FILE_PATH: &str = "day1/resources/sample";
pub const INPUT_FILE_PATH: &str = "day1/resources/puzzle_inputs";

pub fn solve(problem_part: ProblemPart, file_path: &str) -> Result<()> {
    match problem_part {
        ProblemPart::One => {
            let analyzer = ExpenseAnalyzer::new(file_path)?;
            let (expense_1, expense_2) = analyzer.find_summing_pair(2020)?;
            let solution = expense_1 * expense_2;
            println!("{} solution: {}", problem_part, solution);
        },
        ProblemPart::Two => {
            let analyzer = ExpenseAnalyzer::new(file_path)?;
            let (expense_1, expense_2, expense_3) = analyzer.find_summing_triple(2020)?;
            let solution = expense_1 * expense_2 * expense_3;
            println!("{} solution: {}", problem_part, solution);
        },
    }
    Ok(())
}

#[derive(Debug)]
struct ExpenseAnalyzer {
    expenses: Vec<u32>,
}

impl ExpenseAnalyzer {
    fn new(file_path: &str) -> Result<Self> {
        let file = fs::File::open(file_path)?;
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
        Err(anyhow!("Could not find a pair of expenses summing to {}", target_sum))
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
        Err(anyhow!("Could not find a set of expenses summing to {}", target_sum))
    }
}
