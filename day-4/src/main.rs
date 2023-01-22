extern crate core;

use std::fs;
use std::io::{BufRead, BufReader};
use std::num::ParseIntError;
use std::str::FromStr;

struct Assignment {
    start: u32,
    end: u32,
}

impl Assignment {
    fn from_strings(str: &str) -> Result<(Assignment, Assignment), ParseIntError> {
        let split: Vec<&str> = str.split(',').collect();
        Ok((Assignment::from_string(split[0])?,
            Assignment::from_string(split[1])?))
    }
    fn from_string(str: &str) -> Result<Assignment, ParseIntError> {
        let split: Vec<&str> = str.split('-').collect();
        let from = u32::from_str(split[0])?;
        let to = u32::from_str(split[1])?;
        Ok(Assignment { start: from, end: to })
    }
    fn contains(&self, other: &Assignment) -> bool {
        self.start <= other.start &&
            self.end >= other.end
    }
}

fn main() {
    let input_file = fs::File::open("input.txt")
        .expect("input.txt does not exist");
    let reader = BufReader::new(input_file);
    let mut lines = reader.lines();

    let count = lines.into_iter()
        .map(|line| line.unwrap())
        .filter(|line|
            {
                let assignments = Assignment::from_strings(&line)
                    .expect(format!("Something is of with this line: {}", line).as_str());
                assignments.0.contains(&assignments.1) ||
                    assignments.1.contains(&assignments.0)
            }
        )
        .count();

    println!("Found {} lines with one assignments contains another one", count);
}
