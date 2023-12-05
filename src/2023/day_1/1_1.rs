#[path = "../../utils/files_utils.rs"]
mod files_utils;

use regex::Regex;

pub fn run(file_path: &str) -> i32 {
    handle_input(files_utils::get_file_contents(file_path).as_str())
}

fn handle_input(input: &str) -> i32 {
    let regex = Regex::new(r"[0-9]").unwrap();

    let result: i32 = input
        .split("\n")
        .map(|item| handle_line(item, regex.clone())) // clone !!
        .sum();

    result
}

fn handle_line(line: &str, regex: Regex) -> i32 {
    let results = regex
        .find_iter(line)
        .map(|m| m.as_str())
        .collect::<Vec<_>>();

    if results.len() > 1 {
        format!("{}{}", results.first().unwrap(), results.last().unwrap())
            .parse::<i32>()
            .unwrap()
    } else {
        format!("{}{}", results.first().unwrap(), results.first().unwrap())
            .parse::<i32>()
            .unwrap()
    }
}
