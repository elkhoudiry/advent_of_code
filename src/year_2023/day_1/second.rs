use std::{ops::Index, result};

use crate::utils::files;

pub fn run(file_path: &str) -> i32 {
    handle_input(files::get_file_contents(file_path).as_str())
}

fn handle_input(input: &str) -> i32 {
    let result: i32 = input
        .lines()
        .map(|line| handle_line(line)) // clone !!
        .sum();

    result
}

fn handle_line(line: &str) -> i32 {
    let chars = line.as_bytes();
    let mut results = vec![];
    let mut counter = 0;

    let digits = ['1', '2', '3', '4', '5', '6', '7', '8', '9'].map(|digit| digit as u8);
    let str_digits = [
        "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
    ];

    while counter < chars.len() {
        let char = chars[counter];

        if digits.contains(&char) {
            println!("Pushing from digit: {}", char as char);
            results.push(char);
            counter += 1;
            continue;
        }

        for str_digit in str_digits {
            let new_word = std::str::from_utf8(&chars[counter..=str_digit.len()])
                .unwrap_or_default()
                .to_string();
            if str_digit.starts_with(char as char) && new_word == str_digit {
                let result = digits[str_digits
                    .iter()
                    .position(|&element| element == str_digit)
                    .unwrap()];
                println!("Pushing from str: {}", result as char);
                results.push(result);
                counter += str_digit.len();
                continue;
            }
        }

        counter += 1;
    }

    println!(
        "{:#?}",
        results
            .iter()
            .map(|&result| result as char)
            .collect::<Vec<_>>()
    );

    5
}
