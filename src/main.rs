mod utils;
mod year_2023;

fn main() {
    // assert_eq!(
    //     142,
    //     year_2023::day_1::first::run("/src/year_2023/day_1/first_sample")
    // );
    // assert_eq!(
    //     55130,
    //     year_2023::day_1::first::run("/src/year_2023/day_1/first_input")
    // );
    // assert_eq!(
    //     281,
    //     year_2023::day_1::second::run("/src/year_2023/day_1/second_sample")
    // );
    // assert_eq!(
    //     54985,
    //     year_2023::day_1::second::run("/src/year_2023/day_1/first_input")
    // );

    // assert_eq!(
    //     8,
    //     year_2023::day_2::first::run("/src/year_2023/day_2/first_sample")
    // );
    // assert_eq!(
    //     2176,
    //     year_2023::day_2::first::run("/src/year_2023/day_2/first_input")
    // );
    // assert_eq!(
    //     2286,
    //     year_2023::day_2::second::run("/src/year_2023/day_2/first_sample")
    // );
    // assert_eq!(
    //     63700,
    //     year_2023::day_2::second::run("/src/year_2023/day_2/first_input")
    // );

    // assert_eq!(
    //     4361,
    //     year_2023::day_3::first::run("/src/year_2023/day_3/first_sample")
    // );
    // assert_eq!(
    //     533775,
    //     year_2023::day_3::first::run("/src/year_2023/day_3/first_input")
    // );
    // assert_eq!(
    //     2286,
    //     year_2023::day_3::second::run("/src/year_2023/day_3/first_sample")
    // );
    // assert_eq!(
    //     63700,
    //     year_2023::day_3::second::run("/src/year_2023/day_3/first_input")
    // );

    // assert_eq!(
    //     13,
    //     year_2023::day_4::first::run("/src/year_2023/day_4/first_sample")
    // );
    // assert_eq!(
    //     27454,
    //     year_2023::day_4::first::run("/src/year_2023/day_4/first_input")
    // );
    // assert_eq!(
    //     30,
    //     year_2023::day_4::second::run("/src/year_2023/day_4/first_sample")
    // );
    // assert_eq!(
    //     6857330,
    //     year_2023::day_4::second::run("/src/year_2023/day_4/first_input")
    // );

    year_2023::day_5::second::test();
    // assert_eq!(
    //     35,
    //     year_2023::day_5::first::run("/src/year_2023/day_5/first_sample")
    // );
    // assert_eq!(
    //     227653707,
    //     year_2023::day_5::first::run("/src/year_2023/day_5/first_input")
    // );
    assert_eq!(
        46,
        year_2023::day_5::second::run("/src/year_2023/day_5/first_sample")
    );
    assert_eq!(
        78775051,
        year_2023::day_5::second::run("/src/year_2023/day_5/first_input")
    );

    year_2023::day_6::run("/src/year_2023/day_6")
}
