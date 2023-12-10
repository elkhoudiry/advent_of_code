use std::collections::HashMap;

use crate::utils::files;

const TAG: &str = "[DAY 8-2]";

pub fn execute(file_path: &str) -> i64 {
    println!("{TAG} starting ...");
    let answer = handle_input(files::get_file_contents(file_path).as_str());
    println!("{TAG} answer: {answer}");
    println!("{TAG} ==========");
    answer
}

fn handle_input(input: &str) -> i64 {
    let diretions = input.lines().nth(0).unwrap();
    let network = input.lines().skip(2);
    let mut map: HashMap<&str, [&str; 2]> = HashMap::new();

    for line in network {
        handle_line(line, &mut map);
    }

    navigate(&diretions.chars().collect(), map)
}

fn handle_line<'a>(line: &'a str, map: &mut HashMap<&'a str, [&'a str; 2]>) {
    dbg!(line);
    let (node, network) = line.split_once('=').unwrap();
    let trimmed = network
        .trim()
        .trim_start_matches(|char| char == '(')
        .trim_end_matches(|char| char == ')');
    let (left, right) = trimmed.split_once(',').unwrap();

    map.insert(node.trim(), [left.trim(), right.trim()]);
}

fn navigate(directions: &Vec<char>, map: HashMap<&str, [&str; 2]>) -> i64 {
    let mut node = "AAA";
    let mut counter = 0;
    let directions_count = directions.len() as i32;

    while node != "ZZZ" {
        let next_direction = if counter < directions_count {
            counter
        } else {
            abs(counter % directions_count)
        } as usize;
        let next_node = map[node];
        if directions.get(next_direction).unwrap() == &'L' {
            node = next_node[0];
        } else {
            node = next_node[1];
        }
        counter += 1;
    }

    counter as i64
}

fn abs(x: i32) -> i32 {
    if x >= 0 {
        x
    } else {
        -x
    }
}
