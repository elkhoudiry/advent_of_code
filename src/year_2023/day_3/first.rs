use std::cmp::{max, min};

use crate::utils::files;

const TAG: &str = "[DAY 3-1]";

pub fn run(file_path: &str) -> i32 {
    println!("{TAG} Starting ...");
    let answer = handle_input(files::get_file_contents(file_path).as_str());
    println!("{TAG} Answer: {answer}");
    println!("{TAG} ==========");
    answer
}

fn handle_input(input: &str) -> i32 {
    let lines = input.lines();

    lines
        .clone()
        .enumerate()
        .map(|(index, item)| {
            let result = handle_line(
                item,
                if index != 0 {
                    lines.clone().nth(index - 1)
                } else {
                    None
                },
                lines.clone().nth(index + 1),
            );
            let line_number = index + 1;
            println!("{TAG} Line #{} result: {result}", line_number);
            result
        })
        .sum()
}

fn handle_line(line: &str, previous: Option<&str>, next: Option<&str>) -> i32 {
    let numbers = line
        .split('.')
        .map(|item| {
            item.chars()
                .filter(|char| !is_symbol(char))
                .collect::<Vec<_>>()
                .into_iter()
                .collect::<String>()
        })
        .filter(|item| item != "");

    println!(
        "{TAG} Line numbers: {:#?}",
        numbers.clone().collect::<Vec<_>>()
    );

    numbers
        .map(|number| handle_number(number.as_str(), line, previous, next))
        .sum()
}

fn handle_number(number: &str, line: &str, previous: Option<&str>, next: Option<&str>) -> i32 {
    let number_start_index = line.find(number).unwrap();
    let number_end_index = number_start_index + number.len() - 1;

    println!("{TAG} handling number: {number}, start index: {number_start_index}, end index: {number_end_index}");

    if is_same_line_check(line, number_start_index, number_end_index) {
        return number.parse::<i32>().unwrap();
    }

    if is_near_line_check(previous, number_start_index, number_end_index) {
        return number.parse::<i32>().unwrap();
    }

    if is_near_line_check(next, number_start_index, number_end_index) {
        return number.parse::<i32>().unwrap();
    }

    0
}

fn is_same_line_check(line: &str, start_index: usize, end_index: usize) -> bool {
    if start_index > 0 && is_symbol(&line.chars().nth(start_index - 1).unwrap()) {
        return true;
    }

    if is_symbol(
        &line
            .chars()
            .nth(min(line.len() - 1, end_index + 1))
            .unwrap(),
    ) {
        return true;
    }

    false
}

fn is_near_line_check(line: Option<&str>, start_index: usize, end_index: usize) -> bool {
    let unwrapped_line = line.unwrap_or("");

    if unwrapped_line == "" {
        return false;
    }

    for i in start_index..=end_index {
        if is_symbol(&unwrapped_line.chars().nth(i).unwrap()) {
            return true;
        }
    }

    is_same_line_check(unwrapped_line, start_index, end_index)
}

fn is_symbol(char: &char) -> bool {
    match char {
        '1' | '2' | '3' | '4' | '5' | '6' | '7' | '8' | '9' | '0' | '.' => false,
        _ => true,
    }
}
