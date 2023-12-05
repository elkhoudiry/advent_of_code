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
    assert_eq!(
        281,
        year_2023::day_1::second::run("/src/year_2023/day_1/second_sample")
    );
    assert_eq!(
        54985,
        year_2023::day_1::second::run("/src/year_2023/day_1/first_input")
    );
    assert_eq!(
        8,
        year_2023::day_2::first::run("/src/year_2023/day_2/first_sample")
    );
    assert_eq!(
        2176,
        year_2023::day_2::first::run("/src/year_2023/day_2/first_input")
    );
    assert_eq!(
        2286,
        year_2023::day_2::second::run("/src/year_2023/day_2/first_sample")
    );
    assert_eq!(
        63700,
        year_2023::day_2::second::run("/src/year_2023/day_2/first_input")
    );
}
