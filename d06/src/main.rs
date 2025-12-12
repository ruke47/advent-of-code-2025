use std::collections::HashMap;
use std::fs;

type Num = u64;
fn main() {
    part1();
    part2();
}

fn part1() {
    let mut sum: Num = 0;
    let problems = load_math_1();
    for (symbol, numbers) in problems.iter() {
        match symbol.as_str() {
            "+" => sum += numbers.iter().sum::<Num>(),
            "*" => sum += numbers.iter().product::<Num>(),
            _ => panic!("Got an unexpected method {symbol}")
        }
    }

    println!("Part 1: {sum}")
}

fn part2() {
    let problems = load_math_2();
    let mut sum = 0;
    for (symbol, numbers) in problems {
        // println!("{symbol} {:?}", numbers);
        match symbol.as_str() {
            "+" => sum += numbers.iter().sum::<Num>(),
            "*" => sum += numbers.iter().product::<Num>(),
            _ => panic!("Unrecognized symbol {symbol}")
        }
    }

    println!("Part 2: {sum}");
}

fn load_math_1() -> Vec<(String, Vec<Num>)> {
    let lines: Vec<String> = fs::read_to_string("d06/input")
        .unwrap()
        .lines()
        .map(|line| line.to_string())
        .collect();

    let mut number_map: HashMap<usize, Vec<Num>> = HashMap::new();
    let (symbols, numbers) = lines.split_last().unwrap();
    for line in numbers {
        for (idx, number_str) in line.split_whitespace().enumerate() {
            let mut array = number_map.remove(&idx).unwrap_or_else(|| vec!());
            array.push(number_str.parse().unwrap());
            number_map.insert(idx, array);
        }
    }
    symbols
        .split_whitespace()
        .enumerate()
        .map(|(idx, symbol)| {
            (symbol.to_string(), number_map.remove(&idx).unwrap())
        })
        .collect()
}

fn load_math_2() -> Vec<(String, Vec<Num>)> {
    let lines: Vec<String> = fs::read_to_string("d06/input")
        .unwrap()
        .lines()
        .map(|l| l.to_string())
        .collect();
    let (symbols, numbers) = lines.split_last().unwrap();

    let line_length = numbers[0].len();
    let line_count = numbers.len();

    let mut rotated_lines = vec!();

    for x in (0..line_length).into_iter().rev() {
        let mut rot_line = String::new();
        for y in 0..line_count {
            rot_line.push(numbers[y].chars().nth(x).unwrap());
        }
        rotated_lines.push(rot_line);
    }

    let mut numbers = vec!();
    let mut current_number_set = vec!();
    for line in rotated_lines {
        if line.trim().is_empty() {
            numbers.push(current_number_set);
            current_number_set = vec!();
        } else {
            current_number_set.push(line.trim().parse::<Num>().unwrap())
        }
    }
    if !current_number_set.is_empty() {
        numbers.push(current_number_set)
    }

    return symbols
        .split_whitespace()
        .rev()
        .map(|s| s.to_string())
        .zip(numbers)
        .collect();
}