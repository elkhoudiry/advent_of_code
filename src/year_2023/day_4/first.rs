use crate::utils::files;

const TAG: &str = "[DAY 4-1]";

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
            let result = handle_line(item.to_string());
            let line_number = index + 1;
            println!("{TAG} Line #{line_number} result: {result}",);
            result
        })
        .sum()
}

fn handle_line(line: String) -> i32 {
    let all_numbers = line.split(':').nth(1).unwrap();
    let winning = all_numbers
        .split('|')
        .nth(0)
        .unwrap()
        .split(' ')
        .filter(|&item| item != "")
        .collect::<Vec<_>>();
    let mine = all_numbers
        .split('|')
        .nth(1)
        .unwrap()
        .split(' ')
        .filter(|&item| item != "");

    let mut mine_winning: Vec<String> = vec![];

    mine.for_each(|number| {
        if winning.contains(&number) {
            mine_winning.push(number.to_string())
        }
    });

    if mine_winning.len() > 1 {
        let mut result = 1;
        mine_winning[1..]
            .iter()
            .for_each(|item| result = result * 2); // total can't remember optimal operation
        result
    } else {
        mine_winning.len() as i32
    }
}
