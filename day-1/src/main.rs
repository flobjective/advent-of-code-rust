extern crate core;

use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();
    let some_file_path = &args.get(1);
    let output = match some_file_path {
        None => None,
        Some(path) => match fs::read_to_string(&path) {
            Ok(content) => Some(content),
            _=> None
        }
    };

    match output {
        None => print!("Please provide an existing file path argument"),
        Some(lines) => {
            let mut best_deer = 0;
            let mut max_calories = 0;
            let mut current_deer = 0;
            let mut current_calories = 0;
            for line in lines.lines().into_iter() {
                if line.is_empty() {
                    if current_calories > max_calories {
                        best_deer = current_deer;
                        max_calories = current_calories;
                        dbg!(best_deer);
                    }
                    current_calories = 0;
                    current_deer = current_deer + 1;
                } else {
                    current_calories = current_calories + line.parse::<i32>().unwrap();
                }
            }
            println!("Best deer {} with {} calories", best_deer, max_calories);
        }
    }
}