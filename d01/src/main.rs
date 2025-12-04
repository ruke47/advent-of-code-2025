use std::fs;
use std::io::BufRead;

fn main() {
    part1();
    part2();
}

fn part1() {
    let mut cur_val = 50;
    let mut zero_count = 0;
    for (dir, turns) in read_input() {
        cur_val += dir * turns;
        cur_val = pos_mod(cur_val, 100);
        if cur_val == 0 {
            zero_count += 1;
        }
    }

    println!("Part 1: {:?}", zero_count);
}

fn pos_mod(val: i32, modulo: i32) -> i32 {
    ((val % modulo) + modulo) % modulo
}

fn part2() {
    let mut cur_val = 50;
    let mut zero_count = 0;
    for (dir, turns) in read_input() {
        let prev_val = cur_val;
        cur_val += dir * turns;
        // println!("{:?} -> {:?} -> {:?}", prev_val, cur_val, pos_mod(cur_val, 100));
        if cur_val < 1 && prev_val > 0{
            zero_count += 1;
            // println!("negative: {zero_count}")
        }
        let full_turns = cur_val.abs() / 100;
        if full_turns > 0 {
            zero_count += full_turns;
            // println!("{full_turns} full turns: {zero_count}");
        }
        cur_val = pos_mod(cur_val, 100);
    }

    println!("Part 2: {:?}", zero_count);
}

fn read_input() -> Vec<(i32, i32)> {
    fs::read_to_string("d01/input")
        .unwrap()
        .lines()
        .map(|line| {
            let dir: char = line.chars().nth(0).unwrap();
            let turns: i32 = line[1..].parse().unwrap();
            let dir_i = match dir {
                'L' => -1,
                'R' => 1,
                _ => panic!("Got a first character that isn't L/R! {:?}", line)
            };

            (dir_i, turns)
        })
        .collect()
}