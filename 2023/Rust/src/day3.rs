use regex::Regex;
use std::collections::{HashMap, HashSet};

pub fn part1(input: Vec<&str>) -> i32 {
    let mut matrix: Vec<Vec<char>> = Vec::new();

    for line in &input {
        let line: Vec<char> = line.chars().collect();

        matrix.push(line);
    }

    let rows = matrix.len();
    let cols = matrix[0].len() - 1;

    let mut symbols_pos: Vec<(i32, i32)> = Vec::new();

    // Find all the symbols
    for (idx_row, l) in matrix.iter().enumerate() {
        for (idx_col, _) in l.iter().enumerate() {
            let char = matrix[idx_row][idx_col];

            match char {
                '0'..='9' | '.' => (),
                _ => {
                    symbols_pos.push((idx_row as i32, idx_col as i32));
                }
            };
        }
    }

    // find all the numbers
    let mut number_pos: Vec<((i32, i32), (i32, i32))> = Vec::new();

    let regex = Regex::new(r"(\d+)").unwrap();

    for (idx, line) in input.iter().enumerate() {
        let matches = regex
            .find_iter(line)
            .map(|m| (m.start(), m.end()))
            .collect::<Vec<_>>();

        number_pos.extend(
            matches
                .iter()
                .map(|m| ((idx as i32, m.0 as i32), (idx as i32, m.1 as i32)))
                .collect::<Vec<_>>(),
        );
    }

    // Navigate symbols and find adjacent numbers
    let adjacent_neighbors: Vec<(i32, i32)> = vec![
        (-1, 0),  // Up
        (1, 0),   // Down
        (0, -1),  // Left
        (0, 1),   // Right
        (-1, -1), // Upper Left Diagonal
        (-1, 1),  // Upper Right Diagonal
        (1, -1),  // Lower Left Diagonal
        (1, 1),   // Lower Right Diagonal
    ];

    let mut number_list: Vec<i32> = Vec::new();
    let mut res: HashSet<((i32, i32), (i32, i32))> = HashSet::new();
    for symbol_pos in &symbols_pos {
        for neighbor in &adjacent_neighbors {
            let current_row = symbol_pos.0 + neighbor.0;
            let current_col = symbol_pos.1 + neighbor.1;

            if current_row < 0
                || current_row >= rows as i32
                || current_col < 0
                || current_col >= cols.try_into().unwrap()
            {
                continue;
            }

            let char = matrix[current_row as usize][current_col as usize];

            if !char.is_digit(10) {
                continue;
            }

            let mut positions = Vec::new();

            for &(start, end) in &number_pos {
                // Check if the number's row matches the current row, and the number overlaps the current column
                let is_adjacent_row = start.0 == current_row || end.0 == current_row;
                let is_overlapping_col = start.1 <= current_col && end.1 >= current_col;

                // Check for adjacency
                if is_adjacent_row && is_overlapping_col {
                    positions.push((start, end));
                }
            }

            res.extend(positions);
        }
    }
    for (start, end) in res {
        let aux: String = (start.1..end.1)
            .map(|col| matrix[start.0 as usize][col as usize])
            .collect();

        match aux.parse::<i32>() {
            Ok(number) => number_list.push(number),
            Err(e) => eprintln!("Failed to parse '{}': {}", aux, e),
        }
    }

    number_list.iter().sum()
}

pub fn part2(input: Vec<&str>) -> i32 {
    let mut matrix: Vec<Vec<char>> = Vec::new();

    for line in &input {
        let line: Vec<char> = line.chars().collect();

        matrix.push(line);
    }

    let rows = matrix.len();
    let cols = matrix[0].len() - 1;

    let mut symbols_pos: Vec<(i32, i32)> = Vec::new();

    // Find all the symbols
    for (idx_row, l) in matrix.iter().enumerate() {
        for (idx_col, _) in l.iter().enumerate() {
            let char = matrix[idx_row][idx_col];

            match char {
                '0'..='9' | '.' => (),
                _ => {
                    symbols_pos.push((idx_row as i32, idx_col as i32));
                }
            };
        }
    }

    // find all the numbers
    let mut number_pos: Vec<((i32, i32), (i32, i32))> = Vec::new();

    let regex = Regex::new(r"(\d+)").unwrap();

    for (idx, line) in input.iter().enumerate() {
        let matches = regex
            .find_iter(line)
            .map(|m| (m.start(), m.end()))
            .collect::<Vec<_>>();

        number_pos.extend(
            matches
                .iter()
                .map(|m| ((idx as i32, m.0 as i32), (idx as i32, m.1 as i32)))
                .collect::<Vec<_>>(),
        );
    }

    // Navigate symbols and find adjacent numbers
    let adjacent_neighbors: Vec<(i32, i32)> = vec![
        (-1, 0),  // Up
        (1, 0),   // Down
        (0, -1),  // Left
        (0, 1),   // Right
        (-1, -1), // Upper Left Diagonal
        (-1, 1),  // Upper Right Diagonal
        (1, -1),  // Lower Left Diagonal
        (1, 1),   // Lower Right Diagonal
    ];

    let mut dict: HashMap<(i32, i32), HashSet<((i32, i32), (i32, i32))>> = HashMap::new();

    for symbol_pos in &symbols_pos {
        for neighbor in &adjacent_neighbors {
            let current_row = symbol_pos.0 + neighbor.0;
            let current_col = symbol_pos.1 + neighbor.1;

            if current_row < 0
                || current_row >= rows as i32
                || current_col < 0
                || current_col >= cols.try_into().unwrap()
            {
                continue;
            }

            let char = matrix[current_row as usize][current_col as usize];

            if !char.is_digit(10) {
                continue;
            }

            let mut positions = Vec::new();

            for &(start, end) in &number_pos {
                // Check if the number's row matches the current row, and the number overlaps the current column
                let is_adjacent_row = start.0 == current_row || end.0 == current_row;
                let is_overlapping_col = start.1 <= current_col && end.1 >= current_col;

                // Check for adjacency
                if is_adjacent_row && is_overlapping_col {
                    positions.push((start, end));
                }
            }

            dict.entry(symbol_pos.clone())
                .or_insert_with(HashSet::new)
                .extend(positions);
        }
    }

    let mut res_list = Vec::new();
    for v in dict.values().filter(|v| v.len() == 2) {
        let n_list = v
            .iter()
            .map(|(start, end)| {
                let aux: String = (start.1..end.1)
                    .map(|col| matrix[start.0 as usize][col as usize])
                    .collect();

                aux.parse::<i32>().unwrap()
            })
            .collect::<Vec<_>>();

        res_list.push(n_list.iter().product::<i32>());
    }

    return res_list.iter().sum();
}

pub fn run(input: Vec<&str>) {
    let result1 = part1(input.clone());
    let result2 = part2(input);

    println!("---------------------");

    println!("Part 1: {}", result1);
    println!("Part 2: {}", result2);
}

#[cfg(test)]
mod tests {

    use crate::day3::{part1, part2};

    const INPUT_EXAMPLE: [&str; 10] = [
        "467..114..",
        "...*......",
        "..35..633.",
        "......#...",
        "617*......",
        ".....+.58.",
        "..592.....",
        "......755.",
        "...$.*....",
        ".664.598..",
    ];

    #[test]
    fn test_part1() {
        let result = part1(INPUT_EXAMPLE.to_vec());

        assert_eq!(result, 4361);
    }

    #[test]
    fn test_part2() {
        let result = part2(INPUT_EXAMPLE.to_vec());

        assert_eq!(result, 467835);
    }

    #[test]
    fn test_part1_puzzle_edge() {
        let input = [
"...317..........214.....................................751.................................630...479..205....41.993............416.........",
"...*....813........%....572........%...629.154......518....*....365..................-.......*.......#.....................422...........661",
"269.......*...58...........=......264.....*..........*......937.-...........235...303.........848..............195.....154*.........144.-...",
"........476..@...162.855................$....288...821..............107.....-...........290......../..301.........=...........135..*........"
        ];

        let result = part1(input.to_vec());
    }

    #[test]
    fn test_part1_wrong_parse() {
        let input = [
"...*.......864................348..........758.......439...................1........83....748...............325.535...................=.....",
"...368....*........382*250....*.....317.......*................473*....877.+.........*..................@................%....331..513......",
".........55..239...........2...377...=......1.792...294*607........831..........*.950...&....-....#....864.....139.......512................"
        ];
        let result = part1(input.to_vec());
    }
}
