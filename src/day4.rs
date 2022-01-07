use std::fs::read_to_string;

use anyhow::Result;

use crate::problem::{Part, Solved};

// const INPUT_FILE_PATH: &str = "src/day4/puzzle_inputs";
const INPUT_FILE_PATH: &str = "src/day4/sample";

pub struct Day4;

impl Day4 {
    fn solve(problem_part: Part, input_file_path: &str) -> Result<u32> {
        match problem_part {
            Part::One => {
                // read input file to string
                // split input string on \n\n
                // initialize empty vec of valid creds
                // pass each resulting string chunk to ValidCredentiaL::new; given success add
                // to vec return len of vec
                todo!()
            },
            Part::Two => todo!(),
        }
    }
}

enum ValidCredential {
    Passport(Passport),
    NorthPoleID(NorthPoleID),
}

impl ValidCredential {
    fn new(input: &str) -> Result<Self> {
        let core_credentials = CoreCredentials::new(input)?;
        let passport = Passport::new(input, core_credentials.clone()).map(Self::Passport);
        if passport.is_ok() {
            passport
        } else {
            Ok(Self::NorthPoleID(NorthPoleID::new(core_credentials)))
        }
    }
}

#[derive(Clone)]
struct CoreCredentials {
    byr: u32,
    iyr: u32,
    eyr: u32,
    hgt: u32,
    hcl: String,
    ecl: String,
    pid: u32,
}

impl CoreCredentials {
    fn new(input: &str) -> Result<Self> {
        // use regex?
        todo!()
    }
}

struct Passport {
    core_credentials: CoreCredentials,
    cid: String,
}

impl Passport {
    fn new(input: &str, core_credentials: CoreCredentials) -> Result<Self> {
        // use regex?
        todo!()
    }
}

struct NorthPoleID {
    core_credentials: CoreCredentials,
}

impl NorthPoleID {
    fn new(core_credentials: CoreCredentials) -> Self {
        Self { core_credentials }
    }
}

impl Solved for Day4 {
    fn print_solution(part: Part) {
        let solution = Self::solve(part, INPUT_FILE_PATH).unwrap();
        println!("Day 4 {} solution: {}", part, solution);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_FILE_PATH: &str = "src/day4/sample";

    #[test]
    fn test_part_one() {
        let solution = Day4::solve(Part::One, TEST_FILE_PATH).unwrap();
        assert_eq!(solution, 2);
    }

    #[test]
    fn test_part_two() {
        let solution = Day4::solve(Part::Two, TEST_FILE_PATH).unwrap();
        assert_eq!(solution, 2);
    }
}
