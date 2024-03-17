enum Type {
    Seed,
    Soil,
    Fertilizer,
    Water,
    Light,
    Temperature,
    Humidity,
    Location,
}

impl Type {}

impl From<&str> for Type {
    fn from(value: &str) -> Self {
        match value {
            "seeds" => Type::Seed,
            "soil" => Type::Soil,
            "fertilizer" => Type::Fertilizer,
            "water" => Type::Water,
            "light" => Type::Light,
            "temperature" => Type::Temperature,
            "humidity" => Type::Humidity,
            "location" => Type::Location,
            _ => panic!("Unknown type: {}", value),
        }
    }
}

struct Almanac<TSource, TDestination> {
    source_type: TSource,
    destination_type: TDestination,
    source_range: Vec<i32>,
    destination_range: Vec<i32>
}

fn part1(input: Vec<&str>) -> i32 {
    
    // Parse seeds
    let seeds = input[0].split(": ").collect::<Vec<&str>>()[1].split(" ").collect::<Vec<&str>>();
    dbg!(seeds);

    for line in input.iter().skip(2) {
        
        if line.contains(":") {
        }

    }


    todo!()
}

fn part2(input: Vec<&str>) -> i32 {
    todo!()
}

pub fn run(input: Vec<&str>) {
    println!("------------------------");
    println!("Part 1: {}", part1(input.clone()));
    println!("Part 2: {}", part2(input.clone()));
}

#[cfg(test)]
mod tests {

    #[test]
    fn test_part1() {
        let input: [&str; 33] = [
            "seeds: 79 14 55 13",
            "",
            "seed-to-soil map:",
            "50 98 2",
            "52 50 48",
            "",
            "soil-to-fertilizer map:",
            "0 15 37",
            "37 52 2",
            "39 0 15",
            "",
            "fertilizer-to-water map:",
            "49 53 8",
            "0 11 42",
            "42 0 7",
            "57 7 4",
            "",
            "water-to-light map:",
            "88 18 7",
            "18 25 70",
            "",
            "light-to-temperature map:",
            "45 77 23",
            "81 45 19",
            "68 64 13",
            "",
            "temperature-to-humidity map:",
            "0 69 1",
            "1 0 69",
            "",
            "humidity-to-location map:",
            "60 56 37",
            "56 93 4",
        ];

        let result = super::part1(input.to_vec());

        assert_eq!(result, 999999);
    }

    #[test]
    fn test_part2() {}
}
