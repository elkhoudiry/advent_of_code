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
    let mut locations_map = lines
        .split("humidity-to-location map:")
        .nth(1)
        .unwrap()
        .trim()
        .lines()
        .map(|item| {
            let split = item.split(' ').collect::<Vec<_>>();
            [
                split[0].parse::<i64>().unwrap(),
                split[1].parse::<i64>().unwrap(),
                split[2].parse::<i64>().unwrap(),
            ]
        })
        .collect::<Vec<_>>();
    let mut humidities_map = get_map(
        lines,
        "temperature-to-humidity map:",
        "humidity-to-location map:",
    );
    let mut temperature_map = get_map(
        lines,
        "light-to-temperature map:",
        "temperature-to-humidity map:",
    );
    let seeds = &get_seeds(lines, "seeds:", "seed-to-soil map:");
    let mut light_map = get_map(lines, "water-to-light map:", "light-to-temperature map:");
    let mut water_map = get_map(lines, "fertilizer-to-water map:", "water-to-light map:");
    let mut fertalizer_map = get_map(lines, "soil-to-fertilizer map:", "fertilizer-to-water map:");
    let mut soil_map = get_map(lines, "seed-to-soil map:", "soil-to-fertilizer map:");

    get_lowest_location(
        seeds,
        &mut soil_map,
        &mut fertalizer_map,
        &mut water_map,
        &mut light_map,
        &mut temperature_map,
        &mut humidities_map,
        &mut locations_map,
    )
}

fn get_lowest_location(
    seeds: &Vec<[i64; 2]>,
    soil_map: &mut Vec<[i64; 3]>,
    fertalizer_map: &mut Vec<[i64; 3]>,
    water_map: &mut Vec<[i64; 3]>,
    light_map: &mut Vec<[i64; 3]>,
    temperature_map: &mut Vec<[i64; 3]>,
    humidities_map: &mut Vec<[i64; 3]>,
    locations_map: &mut Vec<[i64; 3]>,
) -> i64 {
    let mut locations: Vec<i64> = vec![];
    for seed in seeds {
        let soils = split_ranges(seed[0], seed[1], soil_map);
        let fertalizers = soils
            .iter()
            .map(|soil| split_ranges(soil[0], soil[1], fertalizer_map))
            .collect::<Vec<_>>();

        let waters = fertalizers
            .iter()
            .flatten()
            .map(|fertalizer| split_ranges(fertalizer[0], fertalizer[1], water_map))
            .collect::<Vec<_>>();
        let lights = waters
            .iter()
            .flatten()
            .map(|water| split_ranges(water[0], water[1], light_map))
            .collect::<Vec<_>>();
        let temperatures = lights
            .iter()
            .flatten()
            .map(|light| split_ranges(light[0], light[1], temperature_map))
            .collect::<Vec<_>>();
        let humidities = temperatures
            .iter()
            .flatten()
            .map(|temperature| split_ranges(temperature[0], temperature[1], humidities_map))
            .collect::<Vec<_>>();

        let locs = humidities
            .iter()
            .flatten()
            .map(|humidity| split_ranges(humidity[0], humidity[1], locations_map))
            .flatten()
            .collect::<Vec<_>>();
        locs.iter().for_each(|item| locations.push(item[0]));
    }

    locations.sort();
    return locations[0];
}

fn get_map(source: &str, map: &str, next: &str) -> Vec<[i64; 3]> {
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
            let split = item.split(' ').collect::<Vec<_>>();
            [
                split[0].parse::<i64>().unwrap(),
                split[1].parse::<i64>().unwrap(),
                split[2].parse::<i64>().unwrap(),
            ]
        })
        .collect::<Vec<_>>()
}

fn split_ranges(input_min: i64, input_range: i64, map: &mut Vec<[i64; 3]>) -> Vec<[i64; 2]> {
    let input_max = input_min + input_range - 1;
    let mut mapped: Vec<[i64; 3]> = vec![];

    map.sort_by(|line1, line2| line1[1].cmp(&line2[1]));

    for i in 0..map.len() {
        let map_line = &map[i];
        let source = map_line[1];
        let range = map_line[2];

        if input_min <= source + range && source <= input_max {
            handle_map_line(
                map_line,
                input_min,
                input_max,
                i == map.len() - 1,
                &mut mapped,
            );
        }

        if input_max < source {
            let range = mapped.iter().map(|item| item[2]).sum::<i64>();
            if input_max - range - input_min > 0 {
                mapped.push([input_min + range, 0, input_max - range - input_min + 1]);
                break;
            }
        }
    }

    if mapped.is_empty() {
        mapped.push([input_min, 0, input_range])
    };

    assert_eq!(
        input_range,
        mapped.iter().map(|item| item[2]).sum(),
        "{TAG} lengthes not matching"
    );
    return mapped
        .iter()
        .map(|map| [map[0] + map[1], map[2]])
        .collect::<Vec<_>>();
}

fn handle_map_line(
    map_line: &[i64; 3],
    input_min: i64,
    input_max: i64,
    is_last: bool,
    mapped: &mut Vec<[i64; 3]>,
) {
    let source = map_line[1];
    let destination = map_line[0];
    let range = map_line[2];
    let start_input = if input_min > source {
        input_min
    } else {
        source
    };
    let range_offset = destination - source;
    let limit = if source + range > input_max {
        input_max - start_input + 1
    } else if input_min > source {
        range - (input_min - source)
    } else if input_min < source && input_max < source + range {
        range - source - input_min
    } else {
        range
    };

    if mapped.len() == 0 && input_min < source {
        mapped.push([input_min, 0, source - input_min])
    } else if input_min < source {
        let last_map = mapped.last().unwrap();
        let input_start = last_map[0] + last_map[2];
        let limit = source - input_start;
        if limit > 0 {
            mapped.push([input_start, 0, source - input_start]);
        }
    }

    mapped.push([start_input, range_offset, limit]);

    if is_last && input_max > source + range {
        mapped.push([start_input + limit, 0, input_max - start_input - limit + 1]);
    }
}

fn get_seeds(source: &str, map: &str, next: &str) -> Vec<[i64; 2]> {
    ranged_seeds(
        source
            .split(next)
            .nth(0)
            .unwrap()
            .split(map)
            .nth(1)
            .unwrap()
            .trim(),
    )
}

fn ranged_seeds(seeds: &str) -> Vec<[i64; 2]> {
    let splitted = seeds
        .split_ascii_whitespace()
        .filter(|item| item.trim() != "")
        .map(|seed| seed.trim().parse::<i64>().unwrap())
        .collect::<Vec<_>>();
    let mut seeds: Vec<[i64; 2]> = vec![];
    let mut counter = 0;

    while counter < splitted.len() {
        let number = splitted[counter];
        let range = splitted[counter + 1];

        seeds.push([number, range]);

        counter += 2;
    }
    seeds
}

pub fn test() {
    test1();
    test2();
    test3();
    test4();
}

fn test1() {
    let mut map1: Vec<[i64; 3]> = vec![];
    let test_result: Vec<[i64; 2]> = vec![[20, 40], [62, 5], [65, 3]];
    map1.push([62, 60, 5]);
    map1.push([30, 90, 3]);
    map1.push([98, 95, 10]);

    let result = split_ranges(20, 48, &mut map1);

    assert_eq!(test_result, result)
}

fn test2() {
    let mut map1: Vec<[i64; 3]> = vec![];
    let test_result: Vec<[i64; 2]> = vec![[57, 13]];
    map1.push([52, 50, 48]);
    map1.push([50, 98, 2]);

    let result = split_ranges(55, 13, &mut map1);

    assert_eq!(test_result, result)
}

fn test3() {
    let mut map1: Vec<[i64; 3]> = vec![];
    let test_result: Vec<[i64; 2]> = vec![[4, 7]];
    map1.push([50, 98, 2]);

    let result = split_ranges(4, 7, &mut map1);

    assert_eq!(test_result, result)
}

fn test4() {
    let mut map1: Vec<[i64; 3]> = vec![];
    let test_result: Vec<[i64; 2]> = vec![[78, 3], [45, 11]];
    map1.push([81, 45, 16]);
    map1.push([68, 64, 13]);
    map1.push([45, 77, 23]);

    let result = split_ranges(74, 14, &mut map1);

    assert_eq!(test_result, result)
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
