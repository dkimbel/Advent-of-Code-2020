use std::cmp::max;
use std::fs::{self, File};
use std::io::{BufRead, BufReader};

use anyhow::{anyhow, Context, Result};

use crate::problem::{Part, Solved};

const INPUT_FILE_PATH: &str = "src/day5/puzzle_inputs";

pub struct Day5;

impl Day5 {
    fn solve(problem_part: Part, input_file_path: &str) -> Result<u32> {
        match problem_part {
            Part::One => {
                let file = File::open(input_file_path).context("Failed to open input file")?;
                let reader = BufReader::new(file);
                let mut max_id = u32::MIN;
                for line in reader.lines() {
                    let str = line?;
                    let pass = boarding_pass::BoardingPass::new(&str);
                    let seat_id = pass.parse_seat_id()?;
                    max_id = max(max_id, seat_id);
                }
                Ok(max_id)
            },
            Part::Two => {
                let inputs = fs::read_to_string(input_file_path)?;
                let mut seat_ids = inputs
                    .split('\n')
                    .map(boarding_pass::BoardingPass::new)
                    .filter_map(|p| p.parse_seat_id().ok())
                    .collect::<Vec<_>>();
                seat_ids.sort_unstable();
                let seat_id_pairs_iter = IterByPair::new(Box::new(seat_ids.into_iter()));

                for (first, second) in seat_id_pairs_iter {
                    if second - first == 2 {
                        return Ok(second - 1);
                    }
                }
                Err(anyhow!("Failed to find unoccupied seat"))
            },
        }
    }
}

// it was totally unnecessary to define my own iterator here -- I could have
// just looked up values in my vec by index and index + 1, or even used a
// Peekable -- but this was good practice, my first time implementing the
// Iterator trait
struct IterByPair<T: Copy> {
    next: Option<(T, T)>,
    inner_iter: Box<dyn Iterator<Item = T>>,
}

impl<T: Copy> IterByPair<T> {
    fn new(inner_iter: Box<dyn Iterator<Item = T>>) -> Self {
        Self { next: None, inner_iter }
    }
}

impl<T: Copy> Iterator for IterByPair<T> {
    type Item = (T, T);

    fn next(&mut self) -> Option<Self::Item> {
        match &self.next {
            None => {
                let first = self.inner_iter.next()?;
                let second = self.inner_iter.next()?;
                self.next = Some((first, second));
            },
            Some((_, first)) => {
                let second = self.inner_iter.next()?;
                self.next = Some((*first, second));
            },
        }
        self.next
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

impl Solved for Day5 {
    fn print_solution(part: Part) {
        let solution = Self::solve(part, INPUT_FILE_PATH).unwrap();
        println!("Day 5 {} solution: {}", part, solution);
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
}
