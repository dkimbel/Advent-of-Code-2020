use std::fs::File;
use std::io::{BufRead, BufReader};

use anyhow::{anyhow, Context, Result};

use crate::problem::{Part, Solved};

const INPUT_FILE_PATH: &str = "src/day3/puzzle_inputs";

pub struct Day3;

impl Day3 {
    fn solve(problem_part: Part, input_file_path: &str) -> Result<usize> {
        match problem_part {
            Part::One => {
                let analyzer = PathAnalyzer::new(input_file_path)?;
                analyzer.calculate_num_trees(Slope { tiles_right: 3, tiles_down: 1 })
            },
            Part::Two => {
                let analyzer = PathAnalyzer::new(input_file_path)?;
                let slopes = [
                    Slope { tiles_right: 1, tiles_down: 1 },
                    Slope { tiles_right: 3, tiles_down: 1 },
                    Slope { tiles_right: 5, tiles_down: 1 },
                    Slope { tiles_right: 7, tiles_down: 1 },
                    Slope { tiles_right: 1, tiles_down: 2 },
                ];

                let mut tree_counts = Vec::new();
                for slope in slopes {
                    tree_counts.push(analyzer.calculate_num_trees(slope)?)
                }
                Ok(tree_counts.iter().fold(1, |acc, count| acc * count))
            },
        }
    }
}

impl Solved for Day3 {
    fn print_solution(part: Part) {
        let solution = Self::solve(part, INPUT_FILE_PATH).unwrap();
        println!("Day 3 {} solution: {}", part, solution);
    }
}

#[derive(Debug, PartialEq)]
enum Tile {
    Empty,
    Tree,
}

impl Tile {
    fn from_char(char: char) -> Result<Self> {
        match char {
            '#' => Ok(Self::Tree),
            '.' => Ok(Self::Empty),
            _ => Err(anyhow!("Unable to parse Tile from char {}", char)),
        }
    }
}

struct Slope {
    tiles_down:  usize,
    tiles_right: usize,
}

struct PathAnalyzer {
    grid: Vec<Vec<Tile>>,
}

impl PathAnalyzer {
    const ALL_SLOPES: [usize; 5] = [1, 1, 1, 1, 1];

    fn new(input_file_path: &str) -> Result<Self> {
        let file = File::open(input_file_path).context("Unable to open input file")?;
        let reader = BufReader::new(file);
        let mut grid = Vec::new();

        for line in reader.lines() {
            let row_input = line?;
            let len = row_input.len();
            let mut row = Vec::with_capacity(len);

            for char in row_input.chars() {
                let tile = Tile::from_char(char)?;
                row.push(tile);
            }
            grid.push(row);
        }

        Ok(Self { grid })
    }

    fn calculate_num_trees(&self, slope: Slope) -> Result<usize> {
        let mut num_trees = 0;
        // ignore the starting tile, so DON'T start at 0, 0
        let mut x = slope.tiles_right;
        let mut y = slope.tiles_down;

        while y < self.grid.len() {
            let next_tile = self.get_tile_at_coords(x, y)?;
            if *next_tile == Tile::Tree {
                num_trees += 1;
            }
            y += slope.tiles_down;
            x += slope.tiles_right;
        }

        Ok(num_trees)
    }

    fn get_tile_at_coords(&self, x: usize, y: usize) -> Result<&Tile> {
        let normalized_x = self.normalize_x(x);
        self.grid
            .get(y)
            .context("Could not find row in grid")?
            .get(normalized_x)
            .context("Could not find index in row")
    }

    fn normalize_x(&self, x: usize) -> usize {
        // assumes that all rows of the grid have the same len
        let row_len = self.grid[0].len();
        x % row_len
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_FILE_PATH: &str = "src/day3/sample";

    #[test]
    fn test_part_one() {
        let solution = Day3::solve(Part::One, TEST_FILE_PATH).unwrap();
        assert_eq!(solution, 7);
    }

    #[test]
    fn test_part_two() {
        let solution = Day3::solve(Part::Two, TEST_FILE_PATH).unwrap();
        assert_eq!(solution, 336);
    }
}
