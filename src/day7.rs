use std::collections::HashMap;
use std::fs;

use anyhow::{Context, Result};
use regex::Regex;

use crate::problem::{Part, Solved};

// TODO(dkimbel): POTENTIAL IMPROVEMENTS
//   - don't compile the same regex repeatedly in Bag::new
//   - make can_contain_color and num_inner_bags return a Result type so
//     there's an error if the given color can't be found?

const INPUT_FILE_PATH: &str = "src/day7/puzzle_inputs";

pub struct Day7;

impl Day7 {
    fn solve(problem_part: Part, input_file_path: &str) -> Result<usize> {
        let input = fs::read_to_string(input_file_path)?;
        let bags = Bags::new(&input)?;
        match problem_part {
            Part::One => Ok(bags.num_colors_holding_color("shiny gold")),
            Part::Two => Ok(bags.num_inner_bags("shiny gold")),
        }
    }
}

struct Bags<'a> {
    by_color: HashMap<&'a str, Vec<BagContent<'a>>>,
}

impl<'a> Bags<'a> {
    fn new(input: &'a str) -> Result<Self> {
        let by_color: HashMap<_, _> = input
            .lines()
            .map(Bag::new)
            .map(|bag| bag.map(|bag| (bag.color, bag.contents)))
            .collect::<Result<HashMap<_, _>>>()?;
        Ok(Self { by_color })
    }

    fn can_contain_color(&self, outer_color: &'a str, inner_color: &'a str) -> bool {
        // base case
        if outer_color == inner_color {
            return true;
        }

        let contents = match self.by_color.get(outer_color) {
            Some(contents) => contents,
            None => return false,
        };

        contents
            .iter()
            .map(|bc| bc.color)
            .any(|color| self.can_contain_color(color, inner_color))
    }

    fn num_colors_holding_color(&self, search_color: &str) -> usize {
        self.by_color
            .iter()
            .map(|(&color, _contents)| color)
            .filter(|color| color != &search_color && self.can_contain_color(color, search_color))
            .count()
    }

    fn num_inner_bags(&self, color: &str) -> usize {
        let contents = match self.by_color.get(color) {
            Some(contents) => contents,
            None => return 0,
        };

        // base case
        if contents.is_empty() {
            return 0;
        }

        contents
            .iter()
            .map(|bc| {
                let count = usize::from(bc.count);
                // 1 for the bag itself, so we don't just count its children
                count * (1 + self.num_inner_bags(bc.color))
            })
            .sum::<usize>()
    }
}

struct Bag<'a> {
    color: &'a str,
    contents: Vec<BagContent<'a>>,
}

impl<'a> Bag<'a> {
    fn new(line: &'a str) -> Result<Self> {
        let (color, rest) = line
            .split_once(" bags contain ")
            .context("Failed to split bags input line")?;

        let re = Regex::new(r"(\d+) (.+?) bag")?;
        let mut contents = Vec::new();
        for cap in re.captures_iter(rest) {
            let count = cap
                .get(1)
                .context("Failed to match bag content for count")?
                .as_str()
                .parse::<u8>()?;
            let content_color = cap
                .get(2)
                .context("Failed to match bag content for name")?
                .as_str();
            contents.push(BagContent { color: content_color, count });
        }

        Ok(Self { color, contents })
    }
}

struct BagContent<'a> {
    color: &'a str,
    count: u8,
}

impl Solved for Day7 {
    fn print_solution(part: Part) {
        let solution = Self::solve(part, INPUT_FILE_PATH).unwrap();
        println!("Day 7 {} solution: {}", part, solution);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_FILE_PATH: &str = "src/day7/sample";
    const TEST_FILE_PATH_LARGER: &str = "src/day7/sample_larger";

    #[test]
    fn test_part_one() {
        let solution = Day7::solve(Part::One, TEST_FILE_PATH).unwrap();
        assert_eq!(solution, 4);
    }

    #[test]
    fn test_part_two_small() {
        let solution = Day7::solve(Part::Two, TEST_FILE_PATH).unwrap();
        assert_eq!(solution, 32);
    }

    #[test]
    fn test_part_two_large() {
        let solution = Day7::solve(Part::Two, TEST_FILE_PATH_LARGER).unwrap();
        assert_eq!(solution, 126);
    }
}
