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
    let light_map = get_map(lines, "water-to-light map:", "light-to-temperature map:");
    let water_map = get_map(lines, "fertilizer-to-water map:", "water-to-light map:");
    let fertalizer_map = get_map(lines, "soil-to-fertilizer map:", "fertilizer-to-water map:");
    let soil_map = get_map(lines, "seed-to-soil map:", "soil-to-fertilizer map:");

    get_lowest_location(
        &get_seeds(lines, "seeds:", "seed-to-soil map:"),
        &soil_map,
        &fertalizer_map,
        &water_map,
        &light_map,
        &temperature_map,
        &humidities_map,
        &locations_map,
    )
}

fn get_lowest_location(
    seeds: &Vec<i64>,
    soil_map: &Vec<Vec<i64>>,
    fertalizer_map: &Vec<Vec<i64>>,
    water_map: &Vec<Vec<i64>>,
    light_map: &Vec<Vec<i64>>,
    temperature_map: &Vec<Vec<i64>>,
    humidities_map: &Vec<Vec<i64>>,
    locations_map: &Vec<Vec<i64>>,
) -> i64 {
    let mut locations: Vec<i64> = vec![];
    for seed in seeds {
        dbg!(seed);
        let soil = map_number(*seed, soil_map);
        let fertalizer = map_number(soil, fertalizer_map);
        let water = map_number(fertalizer, water_map);
        let light = map_number(water, light_map);
        let temperature = map_number(light, temperature_map);
        let humidity = map_number(temperature, humidities_map);
        locations.push(map_number(humidity, locations_map))
    }

    locations.sort();
    return locations[0];
}

fn get_seeds(source: &str, map: &str, next: &str) -> Vec<i64> {
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

fn ranged_seeds(seeds: &str) -> Vec<i64> {
    let splitted = seeds
        .split(' ')
        .map(|seed| seed.parse::<i64>().unwrap())
        .collect::<Vec<_>>();
    let mut seeds: Vec<i64> = vec![];
    let mut counter = 0;

    while counter < splitted.len() {
        let number = splitted[counter];
        let range = splitted[counter + 1];
        dbg!(number, range);
        for i in 0..range {
            seeds.push(number + i)
        }

        counter += 2;
    }
    dbg!(seeds.len());
    seeds
}

fn map_number(number: i64, map: &Vec<Vec<i64>>) -> i64 {
    let map_lines = map;

    for map_line in map_lines {
        let mapped = map_number_in_line(number, map_line);

        if mapped > 0 {
            return mapped;
        }
    }

    number
}

fn map_number_in_line(number: i64, line: &Vec<i64>) -> i64 {
    if number >= line[1] && number < line[1] + line[2] {
        let difference = number - line[1];

        return line[0] + difference;
    }

    0
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
