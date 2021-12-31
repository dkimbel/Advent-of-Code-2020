use anyhow::Result;
use day1::Day1;
use problem::{Part, Solvable};

mod day1;
mod problem;

fn main() -> Result<()> {
    Day1::solve(Part::Two)
}
