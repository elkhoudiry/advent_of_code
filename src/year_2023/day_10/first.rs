use ndarray::Array2;

use crate::utils::files;
use std::fmt::Debug;
const TAG: &str = "[DAY 10-1]";

pub fn execute(file_path: &str) -> i64 {
    println!("{TAG} starting ...");
    let answer = handle_input(files::get_file_contents(file_path).as_str());
    println!("{TAG} answer: {answer}");
    println!("{TAG} ==========");
    answer
}

fn handle_input(input: &str) -> i64 {
    let lines = input
        .lines()
        .map(|line| line.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();

    start_map(&lines)
}

fn start_map(lines: &Vec<Vec<char>>) -> i64 {
    let mut all_moves: Vec<Vec<String>> = vec![];
    let current_move = get_start_position(lines);
    let prev_move = current_move;
    let mut counter = 0;

    for line_index in 0..lines.len() {
        let line = &lines[line_index];

        all_moves.push(line.iter().map(|_| ".".to_string()).collect::<Vec<_>>())
    }

    all_moves[current_move.0][current_move.1] = counter.to_string();

    counter += 1;
    move_in_map(
        lines,
        &current_move,
        &prev_move,
        &mut all_moves,
        &mut counter,
    );

    println!("{TAG} {:#?}", all_moves);

    counter
}

fn move_in_map(
    lines: &Vec<Vec<char>>,
    current_move: &(usize, usize),
    prev_move: &(usize, usize),
    mut all_moves: &mut Vec<Vec<String>>,
    counter: &mut i64,
) {
    let possible_moves =
        get_possible_moves(lines, &current_move, &prev_move, &mut all_moves, &counter);

    for mov in possible_moves {
        *counter += 1;
        move_in_map(lines, &mov, current_move, all_moves, counter)
    }
}

fn get_start_position(lines: &Vec<Vec<char>>) -> (usize, usize) {
    let mut line_index = 0;
    let mut char_index = 0;

    while line_index != lines.len() - 1 {
        let current_line = &lines[line_index];

        if current_line.contains(&'S') {
            char_index = current_line.iter().position(|char| *char == 'S').unwrap();
            break;
        }

        line_index += 1;
    }

    (line_index, char_index)
}

fn get_possible_moves(
    lines: &Vec<Vec<char>>,
    location: &(usize, usize),
    blocked: &(usize, usize),
    all_moves: &mut Vec<Vec<String>>,
    counter: &i64,
) -> Vec<(usize, usize)> {
    let mut moves: Vec<(usize, usize)> = vec![];

    match north_move(lines, location, all_moves, counter) {
        None => {}
        Some(mov) => {
            if all_moves[mov.0][mov.1] != "." {
            } else {
                all_moves[mov.0][mov.1] = counter.to_string();
                moves.push(mov);
            }
        }
    }

    match south_move(lines, location, all_moves, counter) {
        None => {}
        Some(mov) => {
            if all_moves[mov.0][mov.1] != "." {
            } else {
                all_moves[mov.0][mov.1] = counter.to_string();
                moves.push(mov);
            }
        }
    }

    match west_move(lines, location, all_moves, counter) {
        None => {}
        Some(mov) => {
            if all_moves[mov.0][mov.1] != "." {
            } else {
                all_moves[mov.0][mov.1] = counter.to_string();
                moves.push(mov);
            }
        }
    }

    match east_move(lines, location, all_moves, counter) {
        None => {}
        Some(mov) => {
            if all_moves[mov.0][mov.1] != "." {
            } else {
                all_moves[mov.0][mov.1] = counter.to_string();
                moves.push(mov);
            }
        }
    }

    moves
        .into_iter()
        .filter(|mov| mov.0 != blocked.0 || mov.1 != blocked.1)
        .collect::<Vec<_>>()
}

fn north_move(
    lines: &Vec<Vec<char>>,
    location: &(usize, usize),
    all_moves: &mut Vec<Vec<String>>,
    counter: &i64,
) -> Option<(usize, usize)> {
    if location.0 == 0 || all_moves[location.0 - 1][location.1] != "." {
        return None;
    }

    let char = lines[location.0 - 1][location.1];
    let can_move_west = location.1 != 0;
    let can_move_east = location.1 != lines[location.0].len() - 1;

    if char == '|' && location.0 >= 2 {
        all_moves[location.0 - 1][location.1] = counter.to_string();
        return Some((location.0 - 2, location.1));
    } else if can_move_east && char == 'F' {
        all_moves[location.0 - 1][location.1] = counter.to_string();
        return Some((location.0 - 1, location.1 + 1));
    } else if can_move_west && char == '7' {
        all_moves[location.0 - 1][location.1] = counter.to_string();
        return Some((location.0 - 1, location.1 - 1));
    }

    return None;
}

fn south_move(
    lines: &Vec<Vec<char>>,
    location: &(usize, usize),
    all_moves: &mut Vec<Vec<String>>,
    counter: &i64,
) -> Option<(usize, usize)> {
    if location.0 >= lines.len() - 1 || all_moves[location.0 + 1][location.1] != "." {
        return None;
    }

    let char = lines[location.0 + 1][location.1];
    let can_move_west = location.1 != 0;
    let can_move_east = location.1 != lines[location.0].len() - 1;

    if char == '|' && location.0 < lines.len() - 2 {
        all_moves[location.0 + 1][location.1] = counter.to_string();
        return Some((location.0 + 2, location.1));
    } else if can_move_east && char == 'L' {
        all_moves[location.0 + 1][location.1] = counter.to_string();
        return Some((location.0 + 1, location.1 + 1));
    } else if can_move_west && char == 'J' {
        all_moves[location.0 + 1][location.1] = counter.to_string();
        return Some((location.0 + 1, location.1 - 1));
    }

    return None;
}

fn west_move(
    lines: &Vec<Vec<char>>,
    location: &(usize, usize),
    all_moves: &mut Vec<Vec<String>>,
    counter: &i64,
) -> Option<(usize, usize)> {
    let can_move_west = location.1 != 0 && all_moves[location.0][location.1 - 1] == ".";

    if !can_move_west {
        return None;
    }

    let char = lines[location.0][location.1 - 1];
    let can_move_south = location.0 < lines.len() - 1;
    let can_move_north = location.0 > 0;

    dbg!(char);
    if char == '-' && location.1 >= 2 {
        all_moves[location.0][location.1 - 1] = counter.to_string();
        return Some((location.0, location.1 - 2));
    } else if can_move_north && char == 'L' {
        all_moves[location.0][location.1 - 1] = counter.to_string();
        return Some((location.0 - 1, location.1 - 1));
    } else if can_move_south && char == 'F' {
        all_moves[location.0][location.1 - 1] = counter.to_string();
        return Some((location.0 + 1, location.1 - 1));
    }

    return None;
}

fn east_move(
    lines: &Vec<Vec<char>>,
    location: &(usize, usize),
    all_moves: &mut Vec<Vec<String>>,
    counter: &i64,
) -> Option<(usize, usize)> {
    let can_move_east =
        location.1 < lines[location.0].len() - 1 && all_moves[location.0][location.1 + 1] == ".";

    if !can_move_east {
        return None;
    }

    let char = lines[location.0][location.1 + 1];
    let can_move_south = location.0 < lines.len() - 1;
    let can_move_north = location.0 > 0;

    if char == '-' && location.1 < lines[location.0].len() - 2 {
        all_moves[location.0][location.1 + 1] = counter.to_string();
        return Some((location.0, location.1 + 2));
    } else if can_move_north && char == 'J' {
        all_moves[location.0][location.1 + 1] = counter.to_string();
        return Some((location.0 - 1, location.1 + 1));
    } else if can_move_south && char == '7' {
        all_moves[location.0][location.1 + 1] = counter.to_string();
        return Some((location.0 + 1, location.1 + 1));
    }

    return None;
}
