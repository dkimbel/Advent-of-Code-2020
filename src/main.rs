mod day1;

#[derive(Debug)]
pub enum ProblemPart {
    One,
    Two,
}

fn main() {
    day1::solve(ProblemPart::Two, day1::INPUT_FILE_PATH);
}
