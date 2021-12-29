use std::fs;
use std::io::{BufRead, BufReader};

// TODO(dkimbel): POTENTIAL IMPROVEMENTS
//   - only panic in `solve`, otherwise use Result
//   - add some kind of 'display' or 'String::from' impl for super::ProblemPart
//   - make some trait that lets every day's solution impl solve with an input
//     file path and a ProblemPart
//   - find a way to not hardcode `day1` module name into file path, even
//     if it costs me the use of const?
//   - have an `Expense` tuplestruct for u32
//   - create a custom iterator that can take an arbitrary number of input vecs
//     and return a vec of their combined values; then use that to solve parts one and two
//     using the same code

pub const SAMPLE_FILE_PATH: &str = "day1/resources/sample";
pub const INPUT_FILE_PATH: &str = "day1/resources/puzzle_inputs";

use crate::ProblemPart;

pub fn solve(problem_part: ProblemPart, file_path: &str) {
    match problem_part {
        ProblemPart::One => {
            let analyzer = ExpenseAnalyzer::new(file_path);
            let (expense_1, expense_2) = analyzer.find_summing_pair(2020);
            let solution = expense_1 * expense_2;
            println!("{:?} solution: {}", problem_part, solution);
        }
        ProblemPart::Two => {
            let analyzer = ExpenseAnalyzer::new(file_path);
            let (expense_1, expense_2, expense_3) = analyzer.find_summing_triple(2020);
            let solution = expense_1 * expense_2 * expense_3;
            println!("{:?} solution: {}", problem_part, solution);
        }
    }
}

#[derive(Debug)]
struct ExpenseAnalyzer {
    expenses: Vec<u32>,
}

impl ExpenseAnalyzer {
    fn new(file_path: &str) -> Self {
        let file = fs::File::open(file_path).expect("Unable to find input file");
        let reader = BufReader::new(file);

        let mut expenses = Vec::new();
        for line in reader.lines().map(|r| r.unwrap()) {
            let expense = line
                .parse::<u32>()
                .expect("Could not parse input expense as u32");
            expenses.push(expense);
        }

        Self { expenses }
    }

    fn find_summing_pair(&self, target_sum: u32) -> (u32, u32) {
        let num_expenses = self.expenses.len();
        for i in 0..num_expenses {
            for j in (i + 1)..num_expenses {
                let expense_i = self.expenses[i];
                let expense_j = self.expenses[j];
                if expense_i + expense_j == target_sum {
                    return (expense_i, expense_j);
                }
            }
        }
        panic!(
            "Could not find a pair of expenses summing to {}",
            target_sum
        );
    }

    fn find_summing_triple(&self, target_sum: u32) -> (u32, u32, u32) {
        let num_expenses = self.expenses.len();
        for i in 0..num_expenses {
            for j in (i + 1)..num_expenses {
                for k in (j + 1)..num_expenses {
                    let expense_i = self.expenses[i];
                    let expense_j = self.expenses[j];
                    let expense_k = self.expenses[k];
                    if expense_i + expense_j + expense_k == target_sum {
                        return (expense_i, expense_j, expense_k);
                    }
                }
            }
        }
        panic!(
            "Could not find a set of three expenses summing to {}",
            target_sum
        );
    }
}
