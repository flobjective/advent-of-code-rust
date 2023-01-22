extern crate core;

use std::collections::HashMap;
use std::fs;

fn main() {
    let priorities = priorities();

    let file_content = fs::read_to_string("input.txt")
        .expect("File \"input.txt\" does not exist");

    let file_lines = file_content.lines();
    let mut sum = 0;
    let mut sum_2 = 0;
    let mut last_line: Option<&str> = None;
    let mut last_line2: Option<&str> = None;
    for (index, line) in file_lines.into_iter().enumerate() {
        let (first_compartment, second_compartment) = line.split_at(line.len() / 2);
        let duplicates = find_duplicates(&first_compartment, &second_compartment);
        let duplicate =  duplicates.first()
            .expect(format!("Something is wrong with line {}", index).as_str());
        sum += priorities.get(duplicate).expect(format!("No value for {}!", duplicate).as_str());
        if last_line2.is_some() && last_line.is_some() && (index + 1) % 3 == 0 {
            let duplicates = find_duplicates(last_line2.unwrap(), last_line.unwrap());
            let possible_badges = duplicates.iter()
                .filter(|ch| line.contains(**ch))
                .collect::<Vec<&char>>();
            let badge = possible_badges.first().unwrap();
            sum_2 += priorities.get(badge).expect(format!("No value for {}!", badge).as_str());
        }
        last_line2 = last_line;
        last_line = Some(line);
    }
    println!("Total sum is {}", sum);
    println!("Total sum2 is {}", sum_2);
}

fn find_duplicates<'a>(p0: &'a str, p1: &'a str) -> Vec<char> {
    let mut result: Vec<char> = Vec::new();
    for x in p0.chars().into_iter() {
        for y in p1.chars().into_iter() {
            if x == y {
                result.push(x)
            }
        }
    }
    result
}

fn priorities() -> HashMap<char, u32> {
    let mut priorities: HashMap<char, u32> = HashMap::new();
    let mut value = 1;
    for x in 'a' as u8..='z' as u8 {
        priorities.insert(x as char, value);
        value += 1;
    }
    let mut value = 27;
    for x in 'A' as u8..='Z' as u8 {
        priorities.insert(x as char, value);
        value += 1;
    }
    return priorities;
}



