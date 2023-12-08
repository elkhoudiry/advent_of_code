use crate::utils::files;

const TAG: &str = "[DAY 5-1]";

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
        .to_string();
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
    let seeds = get_map(lines, "seeds:", "seed-to-soil map:");

    get_lowest_location(
        seeds
            .split(' ')
            .map(|seed| seed.parse::<i64>().unwrap())
            .collect::<Vec<_>>(),
        soil_map.as_str(),
        fertalizer_map.as_str(),
        water_map.as_str(),
        light_map.as_str(),
        temperature_map.as_str(),
        humidities_map.as_str(),
        locations_map.as_str(),
    )
}

fn get_lowest_location(
    seeds: Vec<i64>,
    soil_map: &str,
    fertalizer_map: &str,
    water_map: &str,
    light_map: &str,
    temperature_map: &str,
    humidities_map: &str,
    locations_map: &str,
) -> i64 {
    let mut locations: Vec<i64> = vec![];

    for seed in seeds {
        println!("{TAG} seed: {seed}");

        let soil = map_number(seed, soil_map);
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

fn map_number(number: i64, map: &str) -> i64 {
    let map_lines = map.lines();
    let mapped = map_lines
        .into_iter()
        .map(|item| {
            item.split(' ')
                .map(|item| item.parse::<i64>().unwrap())
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    for map_line in mapped {
        let mapped = map_number_in_line(number, map_line);

        if mapped > 0 {
            return mapped;
        }
    }

    number
}

fn map_number_in_line(number: i64, line: Vec<i64>) -> i64 {
    if number >= line[1] && number < line[1] + line[2] {
        let difference = number - line[1];

        return line[0] + difference;
    }

    0
}

fn get_map(source: &str, map: &str, next: &str) -> String {
    return source
        .split(next)
        .nth(0)
        .unwrap()
        .split(map)
        .nth(1)
        .unwrap()
        .trim()
        .to_string();
}
