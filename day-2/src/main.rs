extern crate core;

use std::cmp::Ordering;
use std::fs;
use crate::Shape::{PAPER, ROCK, SCISSOR};

fn main() {
    let file_content = fs::read_to_string("input.txt")
        .expect("File \"input.txt\" does not exist");

    let file_lines = file_content.lines();
    let mut score : u64 = 0;
    for (index, line) in file_lines.into_iter().enumerate() {
        let oppo_turn = line.chars().next().and_then(Shape::from_char)
            .expect(format!("Problem reading first char at line {}", index).as_ref());
        let my_turn = line.chars().nth(2).and_then(Shape::from_char)
            .expect(format!("Problem reading 3rd char at line {}", index).as_ref());
        match my_turn.cmp(&oppo_turn) {
            Ordering::Greater => score = score + 6,
            Ordering::Equal => score = score + 3,
            Ordering::Less => score = score + 0
        }
        score = score + my_turn.points();
    }
    println!("Our final score: {}", score);
}

enum Shape {
    ROCK,
    PAPER,
    SCISSOR,
}

impl Shape {
    fn points(&self) -> u64 {
        match *self {
            ROCK => 1,
            PAPER => 2,
            SCISSOR => 3,
        }
    }
    fn from_char(c: char) -> Option<Shape> {
        match c {
            'A' | 'X' => Some(ROCK),
            'B' | 'Y' => Some(PAPER),
            'C' | 'Z' => Some(SCISSOR),
            _ => None,
        }
    }
}

impl Eq for Shape {}

impl PartialEq<Self> for Shape {
    fn eq(&self, other: &Self) -> bool {
        self == other
    }
}

impl PartialOrd<Self> for Shape {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Shape {
    fn cmp(&self, other: &Self) -> Ordering {
        match self {
            ROCK => {
                match other {
                    ROCK => Ordering::Equal,
                    PAPER => Ordering::Less,
                    SCISSOR => Ordering::Greater,
                }
            }
            PAPER => {
                match other {
                    ROCK => Ordering::Greater,
                    PAPER => Ordering::Equal,
                    SCISSOR => Ordering::Less,
                }
            }
            SCISSOR => {
                match other {
                    ROCK => Ordering::Less,
                    PAPER => Ordering::Greater,
                    SCISSOR => Ordering::Equal,
                }
            }
        }
    }
}



