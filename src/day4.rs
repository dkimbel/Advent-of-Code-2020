use std::fs::read_to_string;

use anyhow::{Context, Result};

use crate::problem::{Part, Solved};

const INPUT_FILE_PATH: &str = "src/day4/puzzle_inputs";

pub struct Day4;

impl Day4 {
    fn solve(problem_part: Part, input_file_path: &str) -> Result<usize> {
        match problem_part {
            Part::One => {
                let inputs =
                    read_to_string(input_file_path).context("Failed to read input file")?;
                let split_inputs = inputs.split("\n\n");

                let valid_credentials = split_inputs
                    .map(PartOne::ValidCredential::new)
                    .filter_map(|maybe_cred| maybe_cred.ok());

                Ok(valid_credentials.count())
            },
            Part::Two => todo!(),
        }
    }
}

mod PartOne {
    use anyhow::{anyhow, Context, Result};
    use regex::Regex;

    pub enum ValidCredential<'a> {
        Pass(Passport<'a>),
        North(NorthPoleID<'a>),
    }

    impl<'a> ValidCredential<'a> {
        pub fn new(input: &'a str) -> Result<Self> {
            let passport = Passport::new(input).map(Self::Pass);
            if passport.is_ok() {
                passport
            } else {
                NorthPoleID::new(input).map(Self::North)
            }
        }
    }

    struct CoreCredentialsBuilder<'a> {
        byr: Option<&'a str>,
        iyr: Option<&'a str>,
        eyr: Option<&'a str>,
        hgt: Option<&'a str>,
        hcl: Option<&'a str>,
        ecl: Option<&'a str>,
        pid: Option<&'a str>,
    }

    // This is a ton of boilerplate, and might not actually be any more performant
    // than just avoiding builders entirely and using `find` multiple times on the
    // input string (even though, in that case, we'd inspect the early chars of
    // the input string multiple times). Still, this was a nice opportunity to learn
    // the Builder pattern.
    impl<'a> CoreCredentialsBuilder<'a> {
        fn build_from_input(input: &'a str) -> Result<CoreCredentials<'a>> {
            let mut builder = Self::new();
            let tag_value_pairs = input.split_whitespace();
            for pair in tag_value_pairs {
                let (tag, val) = pair
                    .split_once(':')
                    .context(anyhow!("Couldn't split tag-value pair {}", pair))?;
                builder = match tag {
                    "byr" => builder.byr(val),
                    "iyr" => builder.iyr(val),
                    "eyr" => builder.eyr(val),
                    "hgt" => builder.hgt(val),
                    "hcl" => builder.hcl(val),
                    "ecl" => builder.ecl(val),
                    "pid" => builder.pid(val),
                    _ => builder,
                }
            }

            builder.build()
        }

        fn new() -> Self {
            Self {
                byr: None,
                iyr: None,
                eyr: None,
                hgt: None,
                hcl: None,
                ecl: None,
                pid: None,
            }
        }

        fn byr(mut self, byr: &'a str) -> Self {
            self.byr = Some(byr);
            self
        }

        fn iyr(mut self, iyr: &'a str) -> Self {
            self.iyr = Some(iyr);
            self
        }

        fn eyr(mut self, eyr: &'a str) -> Self {
            self.eyr = Some(eyr);
            self
        }

        fn hgt(mut self, hgt: &'a str) -> Self {
            self.hgt = Some(hgt);
            self
        }

        fn hcl(mut self, hcl: &'a str) -> Self {
            self.hcl = Some(hcl);
            self
        }

        fn ecl(mut self, ecl: &'a str) -> Self {
            self.ecl = Some(ecl);
            self
        }

        fn pid(mut self, pid: &'a str) -> Self {
            self.pid = Some(pid);
            self
        }

        fn build(self) -> Result<CoreCredentials<'a>> {
            Ok(CoreCredentials {
                byr: self.byr.context("Missing byr")?,
                iyr: self.iyr.context("Missing iyr")?,
                eyr: self.eyr.context("Missing eyr")?,
                hgt: self.hgt.context("Missing hgt")?,
                hcl: self.hcl.context("Missing hcl")?,
                ecl: self.ecl.context("Missing ecl")?,
                pid: self.pid.context("Missing pid")?,
            })
        }
    }

    struct CoreCredentials<'a> {
        byr: &'a str,
        iyr: &'a str,
        eyr: &'a str,
        hgt: &'a str,
        hcl: &'a str,
        ecl: &'a str,
        pid: &'a str,
    }

    impl<'a> CoreCredentials<'a> {
        fn new(input: &'a str) -> Result<Self> {
            CoreCredentialsBuilder::build_from_input(input)
        }
    }

    struct Passport<'a> {
        core_credentials: CoreCredentials<'a>,
        cid: &'a str,
    }

    impl<'a> Passport<'a> {
        fn new(input: &'a str) -> Result<Self> {
            let core_credentials = CoreCredentials::new(input)?;
            let re = Regex::new(r"cid:(\S+)").context("Failed to build Regex pattern")?;
            let captures = re.captures(input).context("Failed to capture anything")?;
            let &cid = &captures.get(1).context("Failed to capture cid")?.as_str();

            Ok(Self { core_credentials, cid })
        }
    }

    struct NorthPoleID<'a> {
        core_credentials: CoreCredentials<'a>,
    }

    impl<'a> NorthPoleID<'a> {
        fn new(input: &'a str) -> Result<Self> {
            let core_credentials = CoreCredentials::new(input)?;
            Ok(Self { core_credentials })
        }
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
