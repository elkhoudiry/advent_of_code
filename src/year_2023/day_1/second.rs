use crate::utils::files;

const TAG: &str = "[DAY 1-2]";

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
    let chars = line.as_bytes();
    let mut results = vec![];
    let mut counter = 0;

    let digits = ['1', '2', '3', '4', '5', '6', '7', '8', '9'].map(|digit| digit as u8);
    let str_digits = [
        "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
    ];

    'outer: while counter < chars.len() {
        let char = chars[counter];

        if digits.contains(&char) {
            results.push(char);
            counter += 1;
            continue;
        }

        for str_digit in str_digits {
            let bytes = &chars
                .get(counter..counter + str_digit.len())
                .unwrap_or_default();
            let new_word = std::str::from_utf8(&bytes).unwrap_or_default().to_string();
            if str_digit.starts_with(char as char) && new_word == str_digit {
                let result = digits[str_digits
                    .iter()
                    .position(|&element| element == str_digit)
                    .unwrap()];
                results.push(result);
                counter += str_digit.len() - 1;
                continue 'outer;
            }
        }

        counter += 1;
    }

    format!(
        "{}{}",
        results[0] as char,
        results.last().unwrap_or(&results[0]).clone() as char
    )
    .parse::<i32>()
    .unwrap()
}
