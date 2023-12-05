#[path = "2023/day_1/1_1.rs"]
mod day_1_1;

fn main() {
    let day_1_1_sample_result = day_1_1::run("/src/2023/day_1/1_1_sample");
    assert_eq!(142, day_1_1_sample_result);
    let day_1_1_input_result = day_1_1::run("/src/2023/day_1/1_1_input");
    assert_eq!(55130, day_1_1_input_result);
}
