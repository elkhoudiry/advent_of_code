use crate::utils::files;

const TAG: &str = "[DAY 5-2]";

pub fn run(file_path: &str) -> i64 {
    println!("{TAG} Starting ...");
    let answer = handle_input(files::get_file_contents(file_path).as_str());
    println!("{TAG} Answer: {answer}");
    println!("{TAG} ==========");
    answer
}

fn handle_input(input: &str) -> i64 {
    let lines = input;
    let locations_map = lines
        .split("humidity-to-location map:")
        .nth(1)
        .unwrap()
        .trim()
        .lines()
        .map(|item| {
            item.split(' ')
                .map(|item| item.parse::<i64>().unwrap())
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();
    let humidities_map = get_map(
        lines,
        "temperature-to-humidity map:",
        "humidity-to-location map:",
    );
    let temperature_map = get_map(
        lines,
        "light-to-temperature map:",
        "temperature-to-humidity map:",
    );
    let seeds = &get_seeds(lines, "seeds:", "seed-to-soil map:");
    let light_map = get_map(lines, "water-to-light map:", "light-to-temperature map:");
    let water_map = get_map(lines, "fertilizer-to-water map:", "water-to-light map:");
    let fertalizer_map = get_map(lines, "soil-to-fertilizer map:", "fertilizer-to-water map:");
    let soil_map = get_map(lines, "seed-to-soil map:", "soil-to-fertilizer map:");
}

fn get_map(source: &str, map: &str, next: &str) -> Vec<Vec<i64>> {
    source
        .split(next)
        .nth(0)
        .unwrap()
        .split(map)
        .nth(1)
        .unwrap()
        .trim()
        .lines()
        .map(|item| {
            item.split(' ')
                .map(|item| item.parse::<i64>().unwrap())
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>()
}
// # [79, 14], [55, 13] -> [57, 5] , [62, 8]
// [98, 50, 2]
// [60, 62, 38]

fn input(input_min: i64, input_range: i64, map: Vec<Vec<i64>>) -> i64 {
    let input_max = input_min + input_range - 1;
    let mut mapped: Vec<[i64; 2]> = vec![];
    let mut counter = input_min;

    for map_line in map {
        let source = map_line[0];
        let destination = map_line[1];
        let range = map_line[2];

        if source >= input_min && source <= input_max {
            let start_input = if input_min > source {
                input_min
            } else {
                source
            };
            let range_offset = destination - source;
            let mapped_destination = start_input + range_offset;

            let limit = if source + range < input_max {
                source + range - input_min
            } else {
                input_max - source + 1
            };

            mapped.push([mapped_destination, limit])
        }
    }

    while counter <= input_max {
        for map in mapped {}
    }

    0
}
// # [79, 14, 55, 13]
// [98, 50, 2]
// [50, 52, 48]
// # 13 -> 13

// [15, 0, 37]
// [52, 37, 2]
// [0, 39, 15]
// # 13 -> 39 + 13 - 0 = 52

// [11, 0, 42]
// [0, 42, 7]
// [53, 49, 8]
// [7, 57, 4]
// # 52 -> 0 + 52 - 11 = 41

// [25, 18, 70]
// [18, 88, 7]
// # 41 -> 18 + 41 - 25 = 34

// [77, 45, 23]4
// [64, 68, 13]
// [45, 81, 19]
// # 34 -> 34

// [69, 0, 1]
// [0, 1, 69]
// # 34 -> 1 + 34 - 0 = 35

// [93, 56, 4]
// [56, 60, 37]
// # 35 -> 35

// # PART 2
// # [79, 14] = 93, [55, 13] -> [55, 3] , [60, 10]
// [98, 50, 2]
// [58, 60, 40]

// [15, 0, 37]
// [52, 37, 2]
// [0, 39, 15]

// [11, 0, 42]
// [0, 42, 7]
// [53, 49, 8]
// [7, 57, 4]

// [25, 18, 70]
// [18, 88, 7]

// [77, 45, 23]
// [64, 68, 13]
// [45, 81, 19]

// [69, 0, 1]
// [0, 1, 69]

// [93, 56, 4]
// [56, 60, 37]
