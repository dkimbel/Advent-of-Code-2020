use std::fs::read_to_string;

use anyhow::{Context, Result};

use crate::problem::{Part, Solved};

const INPUT_FILE_PATH: &str = "src/day4/puzzle_inputs";

pub struct Day4;

impl Day4 {
    fn solve(problem_part: Part, input_file_path: &str) -> Result<usize> {
        let inputs = read_to_string(input_file_path).context("Failed to read input file")?;
        let split_inputs = inputs.split("\n\n");

        match problem_part {
            Part::One => {
                let valid_credentials = split_inputs
                    .map(part_one::ValidCredential::new)
                    .filter_map(|maybe_cred| maybe_cred.ok());

                Ok(valid_credentials.count())
            },
            Part::Two => {
                let valid_credentials = split_inputs
                    .map(part_two::ValidCredential::new)
                    .filter_map(|maybe_cred| maybe_cred.ok());

                Ok(valid_credentials.count())
            },
        }
    }
}

mod part_two {
    use std::str::FromStr;

    use anyhow::{anyhow, Context, Result};
    use regex::Regex;

    pub enum ValidCredential<'a> {
        Pass(Passport<'a>),
        North(NorthPoleID),
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

    enum EyeColor {
        Amber,
        Blue,
        Brown,
        Gray,
        Green,
        Hazel,
        Other,
    }

    impl FromStr for EyeColor {
        type Err = anyhow::Error;

        fn from_str(s: &str) -> Result<Self, Self::Err> {
            match s {
                "amb" => Ok(Self::Amber),
                "blu" => Ok(Self::Blue),
                "brn" => Ok(Self::Brown),
                "gry" => Ok(Self::Gray),
                "grn" => Ok(Self::Green),
                "hzl" => Ok(Self::Hazel),
                "oth" => Ok(Self::Other),
                _ => Err(anyhow!("Invalid eye color {}", s)),
            }
        }
    }

    #[derive(Debug)]
    enum Measure {
        Inches,
        Centimeters,
    }

    impl FromStr for Measure {
        type Err = anyhow::Error;

        fn from_str(s: &str) -> Result<Self, Self::Err> {
            match s {
                "in" => Ok(Self::Inches),
                "cm" => Ok(Self::Centimeters),
                _ => Err(anyhow!("Could not parse measure from {}", s)),
            }
        }
    }

    struct Height(u32, Measure);

    impl FromStr for Height {
        type Err = anyhow::Error;

        fn from_str(s: &str) -> Result<Self, Self::Err> {
            let chars = s.chars().collect::<Vec<_>>();
            let len = chars.len();
            if chars.len() < 3 {
                return Err(anyhow!("Height must have at least three chars"));
            }

            let measure = Measure::from_str(&chars[(len - 2)..].iter().collect::<String>())?;
            let magnitude: &u32 = &chars[..(len - 2)]
                .iter()
                .collect::<String>()
                .parse::<u32>()
                .context("Failed to parse u32 from Height")?;

            let required_range = match measure {
                Measure::Inches => 59_u32..=76_u32,
                Measure::Centimeters => 150_u32..=193_u32,
            };
            if required_range.contains(magnitude) {
                Ok(Height(*magnitude, measure))
            } else {
                Err(anyhow!(
                    "Height magnitude {} out of range for measurement {:?}",
                    magnitude,
                    measure
                ))
            }
        }
    }

    struct HairColor(u32);

    impl FromStr for HairColor {
        type Err = anyhow::Error;

        fn from_str(s: &str) -> Result<Self, Self::Err> {
            let chars = s.chars().collect::<Vec<_>>();
            if chars[0] != '#' {
                return Err(anyhow!("Hair color string must start with #"));
            }
            if chars.len() != 7 {
                return Err(anyhow!("Hair color must be seven characters total"));
            }

            let mut color: u32 = 0;
            for (i, char) in chars[1..].iter().enumerate() {
                let char = char.to_digit(16).context("Char must be hex digit")?;
                color += format!("{}", char)
                    .parse::<u32>()
                    .context("Digit must be u32")?
                    .pow(u32::try_from(i).context("Must convert usize to u32")?);
            }
            Ok(HairColor(color))
        }
    }

    struct CoreCredentialsBuilder {
        byr: Option<u32>,
        iyr: Option<u32>,
        eyr: Option<u32>,
        hgt: Option<Height>,
        hcl: Option<HairColor>,
        ecl: Option<EyeColor>,
        pid: Option<u32>,
    }

    // This is a ton of boilerplate, and might not actually be any more performant
    // than just avoiding builders entirely and using `find` multiple times on the
    // input string (even though, in that case, we'd inspect the early chars of
    // the input string multiple times). Still, this was a nice opportunity to learn
    // the Builder pattern.
    impl CoreCredentialsBuilder {
        fn build_from_input(input: &str) -> Result<CoreCredentials> {
            let mut builder = Self::new();
            let tag_value_pairs = input.split_whitespace();
            for pair in tag_value_pairs {
                let (tag, val) = pair
                    .split_once(':')
                    .context(anyhow!("Couldn't split tag-value pair {}", pair))?;
                builder = match tag {
                    "byr" => builder.byr(val)?,
                    "iyr" => builder.iyr(val)?,
                    "eyr" => builder.eyr(val)?,
                    "hgt" => builder.hgt(val)?,
                    "hcl" => builder.hcl(val)?,
                    "ecl" => builder.ecl(val)?,
                    "pid" => builder.pid(val)?,
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

        fn byr(mut self, byr: &str) -> Result<Self> {
            let byr = byr
                .parse::<u32>()
                .context(anyhow!("Could not parse byr from {}", byr))?;
            if (1920u32..=2002u32).contains(&byr) {
                self.byr = Some(byr);
                Ok(self)
            } else {
                Err(anyhow!("byr out of valid range"))
            }
        }

        fn iyr(mut self, iyr: &str) -> Result<Self> {
            let iyr = iyr
                .parse::<u32>()
                .context(anyhow!("Could not parse iyr from {}", iyr))?;
            if (2010u32..=2020u32).contains(&iyr) {
                self.iyr = Some(iyr);
                Ok(self)
            } else {
                Err(anyhow!("iyr out of valid range"))
            }
        }

        fn eyr(mut self, eyr: &str) -> Result<Self> {
            let eyr = eyr
                .parse::<u32>()
                .context(anyhow!("Could not parse eyr from {}", eyr))?;
            if (2020u32..=2030u32).contains(&eyr) {
                self.eyr = Some(eyr);
                Ok(self)
            } else {
                Err(anyhow!("eyr out of valid range"))
            }
        }

        fn hgt(mut self, hgt: &str) -> Result<Self> {
            let hgt = Height::from_str(hgt)?;
            self.hgt = Some(hgt);
            Ok(self)
        }

        fn hcl(mut self, hcl: &str) -> Result<Self> {
            let hcl = HairColor::from_str(hcl)?;
            self.hcl = Some(hcl);
            Ok(self)
        }

        fn ecl(mut self, ecl: &str) -> Result<Self> {
            let ecl = EyeColor::from_str(ecl)?;
            self.ecl = Some(ecl);
            Ok(self)
        }

        fn pid(mut self, pid: &str) -> Result<Self> {
            if pid.chars().count() != 9 {
                return Err(anyhow!("pid must be nine digits"));
            }
            let pid = pid
                .parse::<u32>()
                .context(anyhow!("Could not parse pid from {}", pid))?;
            self.pid = Some(pid);
            Ok(self)
        }

        fn build(self) -> Result<CoreCredentials> {
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

    struct CoreCredentials {
        byr: u32,
        iyr: u32,
        eyr: u32,
        hgt: Height,
        hcl: HairColor,
        ecl: EyeColor,
        pid: u32,
    }

    impl CoreCredentials {
        fn new(input: &str) -> Result<Self> {
            CoreCredentialsBuilder::build_from_input(input)
        }
    }

    struct Passport<'a> {
        core_credentials: CoreCredentials,
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

    struct NorthPoleID {
        core_credentials: CoreCredentials,
    }

    impl NorthPoleID {
        fn new(input: &str) -> Result<Self> {
            let core_credentials = CoreCredentials::new(input)?;
            Ok(Self { core_credentials })
        }
    }
}

mod part_one {
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

    const PART_ONE_TEST_FILE_PATH: &str = "src/day4/part_one_sample";
    const PART_TWO_VALIDS_FILE_PATH: &str = "src/day4/part_two_valids";
    const PART_TWO_INVALIDS_FILE_PATH: &str = "src/day4/part_two_invalids";

    #[test]
    fn test_part_one() {
        let solution = Day4::solve(Part::One, PART_ONE_TEST_FILE_PATH).unwrap();
        assert_eq!(solution, 2);
    }

    #[test]
    fn test_part_two_valids() {
        let solution = Day4::solve(Part::Two, PART_TWO_VALIDS_FILE_PATH).unwrap();
        assert_eq!(solution, 4);
    }

    #[test]
    fn test_part_two_invalids() {
        let solution = Day4::solve(Part::Two, PART_TWO_INVALIDS_FILE_PATH).unwrap();
        assert_eq!(solution, 0);
    }
}
