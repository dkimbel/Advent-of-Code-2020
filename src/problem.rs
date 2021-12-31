use std::fmt;

use anyhow::Result;

#[derive(Debug)]
pub enum Part {
    One,
    Two,
}

impl fmt::Display for Part {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let as_int = match self {
            Self::One => 1,
            Self::Two => 2,
        };
        write!(f, "Part {}", as_int)
    }
}

pub trait Solvable {
    fn solve(part: Part) -> Result<()>;
}
