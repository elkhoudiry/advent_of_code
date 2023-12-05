use crate::utils::files;

const TAG: &str = "[DAY 2-2]";

pub fn run(file_path: &str) -> i32 {
    println!("{TAG} Starting ...");
    let answer = handle_input(files::get_file_contents(file_path).as_str());
    println!("{TAG} Answer: {answer}");
    println!("{TAG} ==========");
    answer
}

fn handle_input(input: &str) -> i32 {
    input
        .lines()
        .enumerate()
        .map(|(index, item)| {
            let result = handle_line(item);
            let line_number = index + 1;
            println!("{TAG} Line #{} result: {result}", line_number);
            result
        })
        .sum()
}

fn handle_line(line: &str) -> i32 {
    let splits = line.split([':', ';']).collect::<Vec<_>>();
    let game_sets_vec = splits[1..].to_vec();
    let max_arrays = game_sets_vec
        .iter()
        .map(|set| handle_game_set(set))
        .reduce(|acc, counts| reduce_max_arrays(&acc, &counts))
        .unwrap_or([0, 0, 0]);

    // println!("{TAG} Game sets: {:#?}", game_sets.clone());

    max_arrays[0] * max_arrays[1] * max_arrays[2]
}

fn handle_game_set(set: &str) -> [i32; 3] {
    let cubes = set.trim().split(", ");

    let counts = cubes
        .map(|cube| handle_cube(cube))
        .reduce(|acc, counts| reduce_max_arrays(&acc, &counts));

    counts.unwrap_or([0, 0, 0])
}

fn handle_cube(cube: &str) -> [i32; 3] {
    let splits = cube.split(" ").collect::<Vec<_>>();
    let count = splits[0];
    let color = splits[1];

    [
        if color == "red" {
            count.parse::<i32>().unwrap()
        } else {
            0
        },
        if color == "green" {
            count.parse::<i32>().unwrap()
        } else {
            0
        },
        if color == "blue" {
            count.parse::<i32>().unwrap()
        } else {
            0
        },
    ]
}

fn reduce_max_arrays(arr1: &[i32; 3], arr2: &[i32; 3]) -> [i32; 3] {
    [
        if arr1[0] > arr2[0] { arr1[0] } else { arr2[0] },
        if arr1[1] > arr2[1] { arr1[1] } else { arr2[1] },
        if arr1[2] > arr2[2] { arr1[2] } else { arr2[2] },
    ]
}
