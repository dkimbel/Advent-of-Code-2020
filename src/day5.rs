use std::cmp::max;
use std::fs::File;
use std::io::{BufRead, BufReader};

use anyhow::{Context, Result};

use crate::problem::{Part, Solved};

const INPUT_FILE_PATH: &str = "src/day5/puzzle_inputs";

pub struct Day5;

impl Day5 {
    fn solve(problem_part: Part, input_file_path: &str) -> Result<u32> {
        let file = File::open(input_file_path).context("Failed to open input file")?;
        let reader = BufReader::new(file);

        match problem_part {
            Part::One => {
                let mut max_id = u32::MIN;
                for line in reader.lines() {
                    let str = line?;
                    let pass = boarding_pass::BoardingPass::new(&str);
                    let seat_id = pass.parse_seat_id()?;
                    max_id = max(max_id, seat_id);
                }
                Ok(max_id)
            },
            Part::Two => todo!(),
        }
    }
}

impl Solved for Day5 {
    fn print_solution(part: Part) {
        let solution = Self::solve(part, INPUT_FILE_PATH).unwrap();
        println!("Day 5 {} solution: {}", part, solution);
    }
}

mod boarding_pass {
    use anyhow::{anyhow, Result};

    const MIN_ROW: u8 = 0;
    const MAX_ROW: u8 = 127;
    const MIN_COL: u8 = 0;
    const MAX_COL: u8 = 7;

    pub struct BoardingPass<'a>(&'a str);

    impl<'a> BoardingPass<'a> {
        pub fn new(input: &'a str) -> Self {
            Self(input)
        }

        pub fn parse_seat_id(&self) -> Result<u32> {
            let row = self.parse_row()?;
            let col = self.parse_col()?;
            Ok(u32::from(row) * 8 + u32::from(col))
        }

        fn parse_row(&self) -> Result<u8> {
            let mut min_row = MIN_ROW;
            let mut max_row = MAX_ROW;
            for char in self.0.chars() {
                match char {
                    'F' => max_row = min_row + ((max_row - min_row) / 2),
                    // the 'plus one' here is a bit of a hack to get ceiling division
                    'B' => min_row = min_row + ((max_row - min_row + 1) / 2),
                    _ => (),
                }
            }
            if min_row != max_row {
                Err(anyhow!(
                    "Failed to parse row for pass {}. Min {}, max {}",
                    self.0,
                    min_row,
                    max_row
                ))
            } else {
                Ok(min_row)
            }
        }

        fn parse_col(&self) -> Result<u8> {
            let mut min_col = MIN_COL;
            let mut max_col = MAX_COL;
            for char in self.0.chars() {
                match char {
                    'L' => max_col = min_col + ((max_col - min_col) / 2),
                    // the 'plus one' here is a bit of a hack to get ceiling division
                    'R' => min_col = min_col + ((max_col - min_col + 1) / 2),
                    _ => (),
                }
            }
            if min_col != max_col {
                Err(anyhow!(
                    "Failed to parse col for pass {}. Min {}, max {}",
                    self.0,
                    min_col,
                    max_col
                ))
            } else {
                Ok(min_col)
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_FILE_PATH: &str = "src/day5/sample";

    #[test]
    fn test_part_one() {
        let solution = Day5::solve(Part::One, TEST_FILE_PATH).unwrap();
        assert_eq!(solution, 820);
    }

    #[test]
    fn test_part_two() {
        let solution = Day5::solve(Part::Two, TEST_FILE_PATH).unwrap();
        assert_eq!(solution, 4);
    }
}
