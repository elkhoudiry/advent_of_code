use crate::utils::files;

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
    let start_position = get_start_position(lines);

    let mut all_moves: Vec<Vec<String>> = vec![];
    let mut counter = 0;

    for line_index in 0..lines.len() {
        let line = &lines[line_index];

        all_moves.push(line.iter().map(|_| ".".to_string()).collect::<Vec<_>>())
    }

    all_moves[start_position.0][start_position.1] = counter.to_string();
    let mut current_moves: Vec<(usize, usize)> =
        get_start_moves(lines, start_position, &mut all_moves);

    while !current_moves.is_empty() {
        counter += 1;

        current_moves = move_in_map(lines, &current_moves, &mut all_moves, counter);
    }

    counter
}

fn move_in_map(
    lines: &Vec<Vec<char>>,
    current_moves: &Vec<(usize, usize)>,
    all_moves: &mut Vec<Vec<String>>,
    counter: i64,
) -> Vec<(usize, usize)> {
    let mut next_moves: Vec<(usize, usize)> = vec![];

    for mov in current_moves {
        all_moves[mov.0][mov.1] = counter.to_string();

        match get_next_move(lines, mov, all_moves) {
            Some(mov) => next_moves.push(mov),
            None => {}
        }
    }

    next_moves
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

fn get_start_moves(
    lines: &Vec<Vec<char>>,
    location: (usize, usize),
    all_moves: &mut Vec<Vec<String>>,
) -> Vec<(usize, usize)> {
    let mut moves: Vec<(usize, usize)> = vec![];

    if can_move_north(lines, &location, all_moves) {
        moves.push((location.0 - 1, location.1))
    }

    if can_move_south(lines, &location, all_moves) {
        moves.push((location.0 + 1, location.1))
    }

    if can_move_west(lines, &location, all_moves) {
        moves.push((location.0, location.1 - 1))
    }

    if can_move_east(lines, &location, all_moves) {
        moves.push((location.0, location.1 + 1))
    }

    moves
}

fn get_next_move(
    lines: &Vec<Vec<char>>,
    location: &(usize, usize),
    all_moves: &mut Vec<Vec<String>>,
) -> Option<(usize, usize)> {
    let char = lines[location.0][location.1];

    return if char == '-' && can_move_east(lines, location, all_moves) {
        Some((location.0, location.1 + 1))
    } else if char == '-' && can_move_west(lines, location, all_moves) {
        Some((location.0, location.1 - 1))
    } else if char == '|' && can_move_north(lines, location, all_moves) {
        Some((location.0 - 1, location.1))
    } else if char == '|' && can_move_south(lines, location, all_moves) {
        Some((location.0 + 1, location.1))
    } else if char == '7' && can_move_south(lines, location, all_moves) {
        Some((location.0 + 1, location.1))
    } else if char == '7' && can_move_west(lines, location, all_moves) {
        Some((location.0, location.1 - 1))
    } else if char == 'F' && can_move_south(lines, location, all_moves) {
        Some((location.0 + 1, location.1))
    } else if char == 'F' && can_move_east(lines, location, all_moves) {
        Some((location.0, location.1 + 1))
    } else if char == 'J' && can_move_north(lines, location, all_moves) {
        Some((location.0 - 1, location.1))
    } else if char == 'J' && can_move_west(lines, location, all_moves) {
        Some((location.0, location.1 - 1))
    } else if char == 'L' && can_move_north(lines, location, all_moves) {
        Some((location.0 - 1, location.1))
    } else if char == 'L' && can_move_east(lines, location, all_moves) {
        Some((location.0, location.1 + 1))
    } else {
        None
    };
}

fn can_move_north(
    lines: &Vec<Vec<char>>,
    location: &(usize, usize),
    all_moves: &mut Vec<Vec<String>>,
) -> bool {
    let can_move_north = location.0 > 0 && all_moves[location.0 - 1][location.1] == ".";

    can_move_north && ['|', '7', 'F'].contains(&lines[location.0 - 1][location.1])
}

fn can_move_south(
    lines: &Vec<Vec<char>>,
    location: &(usize, usize),
    all_moves: &mut Vec<Vec<String>>,
) -> bool {
    let can_move_south =
        location.0 < lines.len() - 1 && all_moves[location.0 + 1][location.1] == ".";

    can_move_south && ['|', 'J', 'L'].contains(&lines[location.0 + 1][location.1])
}

fn can_move_west(
    lines: &Vec<Vec<char>>,
    location: &(usize, usize),
    all_moves: &mut Vec<Vec<String>>,
) -> bool {
    let can_move_west = location.1 != 0 && all_moves[location.0][location.1 - 1] == ".";

    can_move_west && ['-', 'F', 'L'].contains(&lines[location.0][location.1 - 1])
}

fn can_move_east(
    lines: &Vec<Vec<char>>,
    location: &(usize, usize),
    all_moves: &mut Vec<Vec<String>>,
) -> bool {
    let can_move_east =
        location.1 != lines[location.0].len() - 1 && all_moves[location.0][location.1 + 1] == ".";

    can_move_east && ['-', '7', 'J'].contains(&lines[location.0][location.1 + 1])
}
