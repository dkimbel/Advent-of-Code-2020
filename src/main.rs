use std::fmt;
use std::fmt::Formatter;

use anyhow::Result;

mod day1;

#[derive(Debug)]
pub enum ProblemPart {
    One,
    Two,
}

impl fmt::Display for ProblemPart {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let as_int = match self {
            Self::One => 1,
            Self::Two => 2,
        };
        write!(f, "Part {}", as_int)
    }
}

fn main() -> Result<()> {
    day1::solve(ProblemPart::Two, day1::INPUT_FILE_PATH)
}
