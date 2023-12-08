use crate::utils::files;

const TAG: &str = "[DAY 4-2]";

pub fn run(file_path: &str) -> i64 {
    println!("{TAG} Starting ...");
    let answer = handle_input(files::get_file_contents(file_path).as_str());
    println!("{TAG} Answer: {answer}");
    println!("{TAG} ==========");
    answer
}

fn handle_input(input: &str) -> i64 {
    let lines = input.lines();
    let size = lines.clone().collect::<Vec<_>>().len();
    let mut cards: Vec<i64> = vec![];

    for _ in 0..size {
        cards.push(1);
    }

    assert_eq!(size, cards.len());

    lines
        .clone()
        .enumerate()
        .map(|(index, line)| {
            for _ in 0..cards[index] {
                handle_line(index, line.to_string(), &mut cards);
            }
            let result = cards[index];

            let line_number = index + 1;
            println!("{TAG} Line #{line_number} result: {result}",);
            result
        })
        .sum()
}

fn handle_line(index: usize, line: String, cards: &mut Vec<i64>) -> i64 {
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

    let mut counter = index;

    while counter < index + mine_winning.len() && counter + 1 < cards.len() {
        cards[counter + 1] = cards[counter + 1] + 1;
        counter += 1;
    }

    0
}
