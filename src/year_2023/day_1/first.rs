use crate::utils::files;
use regex::Regex;

const TAG: &str = "[DAY 1-1]";

pub fn run(file_path: &str) -> i32 {
    println!("{TAG} Starting ...");
    let answer = handle_input(files::get_file_contents(file_path).as_str());
    println!("{TAG} Answer: {answer}");
    println!("{TAG} ==========");
    answer
}

fn handle_input(input: &str) -> i32 {
    let regex = Regex::new("[0-9]").unwrap();

    let result: i32 = input
        .lines()
        .enumerate()
        .map(|(index, item)| {
            let result = handle_line(item, regex.clone());
            let line_number = index + 1;
            println!("{TAG} Line #{} result: {result}", line_number);
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
