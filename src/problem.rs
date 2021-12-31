use std::fmt;

#[allow(dead_code)] // typically only one variant is used at a time
#[derive(Debug, Copy, Clone)]
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

pub trait Solved {
    fn print_solution(part: Part);
}
