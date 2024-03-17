use std::{collections::HashMap, str::FromStr};

const CUBE_RED_LIMIT: i32 = 12;
const CUBE_GREEN_LIMIT: i32 = 13;
const CUBE_BLUE_LIMIT: i32 = 14;

#[derive(Debug, Hash, Eq, PartialEq)]
enum Cube {
    Red,
    Green,
    Blue,
}

impl FromStr for Cube {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "red" => Ok(Cube::Red),
            "green" => Ok(Cube::Green),
            "blue" => Ok(Cube::Blue),
            _ => Err(format!("Unknown Cube: {}", s)),
        }
    }
}
fn parse_input(input: Vec<&str>) -> Vec<(i32, Vec<Vec<(Cube, i32)>>)> {
    input
        .iter()
        .map(|l| {
            let line_split: Vec<&str> = l.split(":").collect();
            let game: Option<&&str> = line_split.first();

            let game_number = match game {
                Some(g) => g
                    .split(" ")
                    .collect::<Vec<&str>>()
                    .last()
                    .expect("Game must have number")
                    .parse::<i32>()
                    .expect("Game Number should be an integer"),
                None => panic!("Game not found: {:?}", game),
            };

            let sets = line_split.last();
            let cube_sets = match sets {
                Some(s) => {
                    let mut cube_set: Vec<Vec<(Cube, i32)>> = Vec::new();

                    for bag in s.split(";").map(|s| s.trim()) {
                        let cubes: Vec<_> = bag
                            .split(",")
                            .filter(|c| !c.is_empty())
                            .flat_map(|c| c.split(",").map(|el| el.trim()))
                            .map(|c| c.split_at(c.find(" ").unwrap()))
                            .map(|(n, c)| {
                                (Cube::from_str(c.trim()).unwrap(), n.parse::<i32>().unwrap())
                            })
                            .collect();

                        cube_set.push(cubes);
                    }

                    cube_set
                }
                None => panic!("Cube Sets empty: {:?}", sets),
            };

            (game_number, cube_sets)
        })
        .collect()
}

pub fn part1(input: Vec<&str>) -> i32 {
    let parsed_games: _ = parse_input(input);

    let _result = parsed_games
        .iter()
        .filter(|(_, cube_sets)| {
            cube_sets.iter().all(|s| {
                s.iter().any(|(cube, count)| match cube {
                    Cube::Red => count > &CUBE_RED_LIMIT,
                    Cube::Green => count > &CUBE_GREEN_LIMIT,
                    Cube::Blue => count > &CUBE_BLUE_LIMIT,
                }) == false
            })
        })
        .map(|(game_number, _cube_sets)| *game_number)
        .collect::<Vec<_>>();

    _result.iter().sum()
}

pub fn part2(input: Vec<&str>) -> i32 {
    let parsed_games: _ = parse_input(input);

    parsed_games
        .iter()
        .map(|(_, cube_sets)| {
            let mut map = HashMap::new();
            for sets in cube_sets {
                for (cube, count) in sets {
                    let entry = map.entry(cube).or_insert(count);
                    if count > entry {
                        *entry = count;
                    }
                }
            }

            map
        })
        .map(|m| m.iter().fold(1, |acc, (_, count)| acc * *count))
        .sum::<i32>()
}

#[cfg(test)]
mod tests {
    use super::{part1, part2};

    const INPUT_EXAMPLE: [&str; 5] = [
        "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green",
        "Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue",
        "Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red",
        "Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red",
        "Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green",
    ];

    #[test]
    fn test_part1_with_example() {
        let result = part1(INPUT_EXAMPLE.to_vec());
        assert_eq!(result, 8);
    }

    #[test]
    fn test_part2() {
        let result = part2(INPUT_EXAMPLE.to_vec());
        assert_eq!(result, 2286);
    }
}

pub(crate) fn run(input_lines: Vec<&str>) {
    let result1 = part1(input_lines.clone());
    let result2 = part2(input_lines.clone());

    println!("Part 1: {}", result1);
    println!("Part 2: {}", result2);
}
