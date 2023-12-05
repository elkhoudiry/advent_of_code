use crate::utils::files;

const TAG: &str = "DAY 2-2";

pub fn run(file_path: &str) -> i32 {
    let answer = handle_input(files::get_file_contents(file_path).as_str());
    println!("[{TAG}] Answer: {answer}");
    answer
}

fn handle_input(input: &str) -> i32 {
    input
        .lines()
        .enumerate()
        .map(|(index, item)| {
            let result = handle_line(item);
            let line_number = index + 1;
            println!("[{TAG}] Line #{} result: {result}", line_number);
            result
        })
        .sum()
}

fn handle_line(line: &str) -> i32 {
    0
}
