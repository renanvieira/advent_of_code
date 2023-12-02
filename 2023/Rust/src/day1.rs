fn part1(input: Vec<&str>) -> u32 {
    let mut ret: Vec<u32> = Vec::new();

    for line in input {
        let num: Vec<_> = line.chars().filter(|c| c.is_numeric()).collect();

        let first_int = num.first().unwrap();
        let second_int = num.last().unwrap();

        let mut n = String::new();

        n.push(*first_int);
        n.push(*second_int);

        ret.push(n.parse::<u32>().unwrap());
    }

    println!("Ret: {:?}", ret);
    ret.iter().sum()
}

fn get_numeric<'a>(num: &'a str) -> &'a str {
    let parse_result = num.parse::<usize>();

    if parse_result.is_err() {
        match num {
            "one" => "1",
            "two" => "2",
            "three" => "3",
            "four" => "4",
            "five" => "5",
            "six" => "6",
            "seven" => "7",
            "eight" => "8",
            "nine" => "9",
            _ => panic!("Unknown number"),
        }
    } else {
        num
    }
}

fn part2(input: Vec<&str>) -> u32 {
    let lookup = vec![
        "1", "2", "3", "4", "5", "6", "7", "8", "9", "one", "two", "three", "four", "five", "six",
        "seven", "eight", "nine",
    ];

    let mut ret: Vec<u32> = Vec::new();

    for line in input {
        let mut stack: Vec<(usize, &str)> = Vec::new();

        println!("Line: {}", line);
        for n in &lookup {
            let indices: Vec<_> = line.match_indices(n).collect();
            if indices.len() > 0 {
                stack.extend(indices);
            }
        }

        stack.sort_by(|a, b| a.0.cmp(&b.0));
        println!("Stack: {:?}", stack);

        let first_match = stack.first()
            .expect("Line should have first number");
        let last_match = stack.last()
            .expect("Line should have last number");

        let first_num = first_match.1;
        let last_num =last_match.1;

        // Transform to digits

        let mut final_num = String::new();
        final_num.push_str(get_numeric(first_num));
        final_num.push_str(get_numeric(last_num));

        println!("Final: {:#?}", final_num);

        ret.push(final_num.parse::<u32>().unwrap());
    }

    println!("---------------");
    ret.iter().sum()
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_part2() {
        use crate::day1::part2;

        let input = vec!["mrheightwogfglthreeeight6threeeighttwodfkjgp"];

        let result = part2(input);

        assert_eq!(result, 82);
    }
}

pub fn run(input: Vec<&str>) -> (u32, i32) {
    let result1 = part2(input);
    print!("Part 2: {}", result1);

    (result1, 0)
}
