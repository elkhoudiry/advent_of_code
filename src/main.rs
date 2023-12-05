mod utils;
mod year_2023;

fn main() {
    assert_eq!(
        142,
        year_2023::day_1::first::run("/src/year_2023/day_1/first_sample")
    );
    assert_eq!(
        55130,
        year_2023::day_1::first::run("/src/year_2023/day_1/first_input")
    );

    year_2023::day_1::second::run("/src/year_2023/day_1/second_sample");
}
