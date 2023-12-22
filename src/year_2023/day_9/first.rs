use std::vec;

use crate::utils::files;

const TAG: &str = "[DAY 9-1]";

pub fn execute(file_path: &str) -> i64 {
    println!("{TAG} starting ...");
    let answer = handle_input(files::get_file_contents(file_path).as_str());
    println!("{TAG} answer: {answer}");
    println!("{TAG} ==========");
    answer
}

fn handle_input(input: &str) -> i64 {
    input
        .lines()
        .enumerate()
        .map(|(index, item)| {
            let result = handle_line(item);
            // let line_number = index + 1;
            // println!("{TAG} Line #{} result: {result}", line_number);
            result
        })
        .sum()
}

fn handle_line(line: &str) -> i64 {
    let numbers = line
        .split_ascii_whitespace()
        .into_iter()
        .map(|number| number.parse::<i64>().unwrap())
        .collect::<Vec<_>>();

    find_next_value(numbers)
}

fn find_next_value(arr: Vec<i64>) -> i64 {
    let mut vec_of_vecs: Vec<Vec<i64>> = vec![];

    vec_of_vecs.push(arr);

    loop {
        let current = vec_of_vecs.last().unwrap();
        assert!(!current.is_empty());
        if current.into_iter().all(|item| *item == 0) {
            break;
        }

        let mut new_vec: Vec<i64> = vec![];
        for i in 0..current.len() - 1 {
            new_vec.push(current[i + 1] - current[i])
        }

        vec_of_vecs.push(new_vec)
    }

    let mut counter = 0;
    let mut result = 0;

    while counter < vec_of_vecs.len() {
        result = result + vec_of_vecs[vec_of_vecs.len() - 1 - counter].last().unwrap();
        counter += 1;
    }

    result
}
