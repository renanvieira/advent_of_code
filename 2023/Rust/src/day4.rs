use std::collections::{HashMap, VecDeque};

#[derive(Debug, Clone)]
struct Game {
    num: i32,
    winning_numbers: Vec<i32>,
    cards: Vec<i32>,
    points: Points,
}

#[derive(Debug, Clone)]
struct Points(i32);

impl Points {
    fn set(&mut self, num: i32) {
        self.0 = num;
    }
}

impl Game {
    fn new(num: i32, winning_numbers: Vec<i32>, cards: Vec<i32>) -> Self {
        Self {
            num,
            winning_numbers,
            cards,
            points: Points(0),
        }
    }
}

fn parse_input(input: Vec<&str>) -> Vec<Game> {
    let mut full_games: Vec<Game> = Vec::new();

    for line in input {
        let game_split = line.split(":").map(|s| s.trim()).collect::<Vec<_>>();

        let game_sec_split = game_split[0]
            .split(" ")
            .map(|s| s.trim())
            .filter(|s| !s.is_empty())
            .collect::<Vec<_>>();

        let game_number = game_sec_split[1]
            .parse::<i32>()
            .expect(format!("Game Number is not an integer: {:?}", game_sec_split).as_str());

        let raw_card = game_split[1];
        let card_split = raw_card.split("|").map(|s| s.trim()).collect::<Vec<_>>();

        let winning_numbers = card_split[0]
            .split(" ")
            .filter(|s| !s.is_empty())
            .map(|s| {
                s.parse::<i32>()
                    .expect(format!("winning number error: {:?}", s).as_str())
            })
            .collect::<Vec<_>>();

        let my_numbers = card_split[1]
            .split(" ")
            .filter(|s| !s.is_empty())
            .map(|s| s.parse::<i32>().unwrap())
            .collect::<Vec<_>>();

        let game = Game::new(game_number, winning_numbers, my_numbers);

        full_games.push(game);
    }
    return full_games;
}
fn part1(input: Vec<&str>) -> i32 {
    let mut full_games = parse_input(input);

    for game in &mut full_games {
        let mut points: i32 = 0;

        // Calculate points);
        for &my_n in &game.cards {
            let won = game.winning_numbers.contains(&my_n);

            if won {
                points = if points == 0 { 1 } else { points * 2 };
            }
        }

        // Set points after calculation is complete
        game.points.set(points);
    }

    return full_games.iter().map(|g| g.points.0).sum();
}

fn part2(input: Vec<&str>) -> i32 {
    // Each match make win the games below equal the number of matches

    let full_games = parse_input(input);

    let mut map: HashMap<i32, Vec<Game>> = HashMap::new();

    for game in &full_games {
        map.entry(game.num).or_insert(Vec::new()).push(game.clone());
    }

    let mut sorted_keys: Vec<i32> = map.keys().cloned().collect();
    sorted_keys.sort();

    for key in sorted_keys {
        if let Some(cards) = map.get(&key).cloned() {
            for c in cards {
                let mut win_count = 0;
                for n in &c.cards {
                    for w in &c.winning_numbers {
                        if n == w {
                            win_count += 1;
                        }
                    }
                }

                if win_count > 0 {
                    let start = key;
                    let target = start + win_count - 1;

                    for i in start..=target {
                        let idx = i as usize;
                        if let Some(next_card) = &full_games.get(idx) {
                            map.entry(next_card.num)
                                .or_insert_with(Vec::new)
                                .push(next_card.to_owned().clone());
                        }
                    }

                }
            }
        }
    }

    println!("---------------------");
    let mut count: i32 = 0;
    for (_, v) in map {
        count += v.len() as i32;
    }

    return count;
}

pub fn run(input: Vec<&str>) {
    println!("------------------");
    let result1 = part1(input.clone());
    println!("Part 1: {}", result1);

    let result2 = part2(input.clone());
    println!("Part 2: {}", result2);
}

#[cfg(test)]
mod tests {

    #[test]
    fn test_part1() {
        let input = [
            "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53",
            "Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19",
            "Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1",
            "Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83",
            "Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36",
            "Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11",
        ];

        let result = super::part1(input.to_vec());

        assert_eq!(result, 13);
    }

    #[test]
    fn test_part2() {
        let input = [
            "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53",
            "Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19",
            "Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1",
            "Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83",
            "Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36",
            "Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11",
        ];

        let result = super::part2(input.to_vec());

        assert_eq!(result, 30);
    }
}
