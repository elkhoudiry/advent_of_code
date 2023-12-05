use crate::utils::files;
use regex::Regex;

const TAG: &str = "DAY 1-1";

pub fn run(file_path: &str) -> i32 {
    let answer = handle_input(files::get_file_contents(file_path).as_str());
    println!("[{TAG}] Answer: {answer}");
    answer
}

fn handle_input(input: &str) -> i32 {
    let regex = Regex::new("[0-9]").unwrap();

    let result: i32 = input
        .lines()
        .enumerate()
        .map(|(number, item)| {
            let result = handle_line(item, regex.clone());
            println!("[{TAG}] Line #{} result: {result}", number + 1);
            result
        })
        .sum();

    result
}

fn handle_line(line: &str, regex: Regex) -> i32 {
    let results = regex
        .find_iter(line)
        .map(|m| m.as_str())
        .collect::<Vec<_>>();

    format!("{}{}", results[0], results.last().unwrap_or(&results[0]))
        .parse::<i32>()
        .unwrap()
}
