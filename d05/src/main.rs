use std::fs;

type Ingredient = u64;

#[derive(Copy, Clone, Debug)]
struct Range {
    begin: Ingredient,
    end: Ingredient
}

impl Range {
    pub fn new(start: Ingredient, end: Ingredient) -> Self {
        Self { begin: start, end}
    }

    pub fn contains(self, &ingredient: &Ingredient) -> bool {
        ingredient >= self.begin && ingredient <= self.end
    }
}


fn main() {
    part1();
    part2();
}

fn part1() {
    let mut fresh_count = 0;
    let (ranges, ingredients) = load_db();
    for ingredient in ingredients.iter() {
        for range in ranges.iter() {
            if range.contains(ingredient) {
                fresh_count += 1;
                break;
            }
        }
    }

    println!("Part 1: {fresh_count}");
}

fn part2() {
    let (mut ranges, _) = load_db();
    ranges.sort_by(|a, b| a.begin.cmp(&b.begin));

    let mut joined_ranges = vec!();
    let mut iter = ranges.iter();
    let mut cur_range = iter.next().unwrap().clone();
    loop {
        if let Some(next_range) = iter.next() {
            if next_range.begin <= cur_range.end {
                if next_range.end > cur_range.end {
                    cur_range = Range::new(cur_range.begin, next_range.end);
                }
            } else {
                joined_ranges.push(cur_range);
                cur_range = next_range.clone();
            }
        } else {
            joined_ranges.push(cur_range);
            break;
        }
    }

    let mut sum_size = 0;
    for range in joined_ranges.iter() {
        sum_size += range.end - range.begin + 1;
    }
    println!("Part 2: {sum_size}");
}

fn load_db() -> (Vec<Range>, Vec<Ingredient>) {
    let file_str = fs::read_to_string("d05/input").unwrap();
    let mut parts = file_str.split("\n\n");

    let ranges = parts
        .next()
        .unwrap()
        .lines()
        .map(|line| {
            let mut line_parts = line.split("-");
            let begin_incl = line_parts.next().unwrap().parse().unwrap();
            let end_incl = line_parts.next().unwrap().parse().unwrap();
            return Range::new(begin_incl, end_incl);
        })
        .collect();

    let ids = parts
        .next()
        .unwrap()
        .lines()
        .map(|line| line.parse().unwrap())
        .collect();

    return (ranges, ids);

}