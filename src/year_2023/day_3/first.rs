use std::{
    cmp::{max, min},
    ops::Add,
};

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
    let numbers = line.chars();
    let number_size = numbers.clone().collect::<Vec<_>>().len();
    let mut counter = 0;

    let mut sum = 0;

    while counter < number_size {
        let nth = numbers.clone().nth(counter);
        let char = nth.clone().unwrap();
        let mut buf: String = "".to_string();

        if is_digit(char) {
            buf += char.to_string().as_str();
            while counter + 1 < number_size && is_digit(numbers.clone().nth(counter + 1).unwrap()) {
                counter += 1;
                let nth = numbers.clone().nth(counter);
                let char = nth.clone().unwrap();
                buf += char.to_string().as_str();
            }
        }

        if buf != "" {
            sum += handle_number(buf.as_str(), line, counter + 1 - buf.len(), previous, next);
        }

        counter += 1;
    }

    sum
}

fn handle_number(
    number: &str,
    line: &str,
    index: usize,
    previous: Option<&str>,
    next: Option<&str>,
) -> i32 {
    let number_start_index = index;
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

fn is_digit(char: char) -> bool {
    match char {
        '1' | '2' | '3' | '4' | '5' | '6' | '7' | '8' | '9' | '0' => true,
        _ => false,
    }
}
