use std::fs;

type Num = u64;

fn main() {
    part1();
    part2();
}

fn part1() {
    let mut sum = 0;
    let ranges = load_ranges();
    println!("Range count: {:?}", ranges.len());

    ranges.iter().for_each(|(lower, upper)| {
        for number in *lower..=*upper {
            let str_num = number.to_string();
            if str_num.len() % 2 != 0 {
                continue;
            }
            let half_len = str_num.len() / 2;
            let (first_half, second_half) = str_num.split_at(half_len);
            if first_half == second_half {
                sum += number;
            }
        }
    });

    println!("Part 1: {sum}");
}

fn part2() {
    let mut sum = 0;
    load_ranges().iter().for_each(|(lower, upper)| {
        'num_loop: for number in *lower..=*upper {
            let str_num = number.to_string();
            // for each potential pattern length, up to half of the number of digits
            'pattern_loop: for pattern_len in 1..=(str_num.len() / 2) {
                // if the string's length is not divisible by the pattern's length, continue
                if str_num.len() % pattern_len != 0 {
                    continue 'pattern_loop;
                }
                // get the first n digits of the number
                let substring = &str_num[0..pattern_len];
                // figure out how many times we'll need to repeat them
                let repeats = str_num.len() / pattern_len;
                // repeat them that many times & see if it matches our number string
                if substring.repeat(repeats) == str_num {
                    sum += number;
                    // if it does match, jump to the outer loop: 2222 should only be counted once,
                    // not once as "2"x4 and once as "22"x2
                    continue 'num_loop;
                }
            }
        }
    });

    println!("Part 2: {sum}");
}

fn load_ranges() -> Vec<(Num, Num)> {
    fs::read_to_string("d02/input")
        .unwrap()
        .trim()
        .split(",")
        .map(|range| {
            let mut range_parts = range.split("-");
            (range_parts.next().unwrap().parse().unwrap(),
             range_parts.next().unwrap().parse().unwrap())
        })
        .collect()

}