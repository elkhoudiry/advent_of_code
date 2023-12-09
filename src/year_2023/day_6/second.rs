use crate::utils::files;

const TAG: &str = "[DAY 6-2]";

pub fn execute(file_path: &str) -> i64 {
    println!("{TAG} atarting ...");
    let answer = handle_input(files::get_file_contents(file_path).as_str());
    println!("{TAG} answer: {answer}");
    println!("{TAG} ==========");
    answer
}

fn handle_input(input: &str) -> i64 {
    let numbers = input
        .lines()
        .enumerate()
        .map(|(index, item)| {
            let result = handle_line(item);
            println!("{TAG} line #{} result: {:#?}", index + 1, result);
            result
        })
        .collect::<Vec<_>>();
    let times = numbers.get(0).unwrap(); // why no number[0]
    let distances = numbers.get(1).unwrap();
    let races: i64 = times
        .into_iter()
        .zip(distances)
        .enumerate()
        .map(|(index, race)| {
            let result = handle_race((*race.0, *race.1));
            println!("{TAG} race #{} result: {:#?}", index + 1, result);

            result
        })
        .sum();

    races
}

fn handle_line(line: &str) -> Vec<i64> {
    let (_, numbers) = line.split_once(":").unwrap();

    vec![numbers
        .trim()
        .split_ascii_whitespace()
        .map(|number| number.to_string())
        .reduce(|acc, number| acc + number.as_str())
        .unwrap()
        .parse::<i64>()
        .unwrap()]
}

fn handle_race(race: (i64, i64)) -> i64 {
    let (time, distance) = race;
    let acceptable_times = get_race_possibilities(time, distance);

    acceptable_times.len() as i64
}

fn get_race_possibilities(time: i64, distance: i64) -> Vec<i64> {
    let mut acceptable: Vec<i64> = vec![];

    for current_time in 0..time {
        let speed = current_time;
        let remaining_time = time - current_time;
        let moved_distance = speed * remaining_time;

        if moved_distance > distance {
            acceptable.push(current_time)
        }
    }

    acceptable
}
