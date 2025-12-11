use std::fs;

fn main() {
    part1();
    part2();
}

fn part1() {
    let mut sum = 0;
    let banks = load_banks();
    for bank in banks.iter() {
        let mut tens = 0;
        let mut tens_idx = 0;
        // all but the last digit
        for (idx, &digit) in bank.split_last().unwrap().1.iter().enumerate() {
            if digit > tens {
                tens = digit;
                tens_idx = idx;
            }
        }

        let mut ones = 0;
        for &digit in bank[(tens_idx + 1)..].iter() {
            if digit > ones {
                ones = digit;
            }
        }

        let bank_power = tens * 10 + ones;
        sum += bank_power;
    }

    println!("Part 1: {sum}");
}

fn part2() {
    let mut sum = 0;
    for bank in load_banks() {
        sum += jolt(&bank, 12);
    }

    println!("Part 2: {sum}")
}

fn jolt(bank: &Vec<u32>, digit_count: usize) -> u64 {
    let mut banked_power: u64 = 0;
    let mut prior_idx = 0;

    for digit_idx in 0..digit_count {
        // shift the current power over by 1 digit
        banked_power *= 10;

        // figure out how deep into the bank we can look for this digit
        // (length - digits remaining)
        let last_valid_index = bank.len() + digit_idx - digit_count;
        // the valid range for this digit is (after the prior index) until (the last valid index)
        let sub_bank = &bank[prior_idx..=last_valid_index];

        let mut cur_digit = 0;
        let mut sub_bank_idx = 0;

        for (cur_idx, &digit) in sub_bank.iter().enumerate() {
            if digit > cur_digit {
                cur_digit = digit;
                sub_bank_idx = cur_idx;
            }
        }
        // add the largest digit in our valid range to the banked power
        banked_power += cur_digit as u64;
        // update the valid starting location for the next digit to be after this one
        prior_idx += sub_bank_idx + 1;
    }

    return banked_power;
}

fn load_banks() -> Vec<Vec<u32>> {
    fs::read_to_string("d03/input")
        .unwrap()
        .lines()
        .map(|line| {
            line.chars()
                .map(|c| c.to_digit(10).unwrap())
                .collect()
        })
        .collect()
}