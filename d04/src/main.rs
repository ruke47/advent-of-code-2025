use lib2d::Point2d;
use std::collections::HashSet;
use std::fs;

type Coord = Point2d<i32>;

fn main() {
    part1();
    part2();
}

fn part1() {
    let mut movable_rolls = 0;
    let points = load_grid();
    for point in &points {
        let mut neighbor_count = 0;
        for dx in [-1, 0, 1] {
            for dy in [-1, 0, 1] {
                if dx == 0 && dy == 0 {
                    continue;
                }
                let neighbor = *point + Point2d::new(dx, dy);
                if points.contains(&neighbor) {
                    neighbor_count += 1;
                }
            }
        }
        if neighbor_count < 4 {
            movable_rolls += 1;
        }
    }
    println!("Part 1: {movable_rolls}");
}

fn part2() {
    let mut movable_rolls = 0;
    let mut moved_rolls = 0;
    let mut points = load_grid();
    loop {
        moved_rolls = 0;
        let mut to_remove = vec!();
        for point in &points {
            let mut neighbor_count = 0;
            for dx in [-1, 0, 1] {
                for dy in [-1, 0, 1] {
                    if dx == 0 && dy == 0 {
                        continue;
                    }
                    let neighbor = *point + Point2d::new(dx, dy);
                    if points.contains(&neighbor) {
                        neighbor_count += 1;
                    }
                }
            }
            if neighbor_count < 4 {
                to_remove.push(point.clone());
            }
        }
        if to_remove.is_empty() {
            break;
        } else {
            movable_rolls += to_remove.len();
            for roll in to_remove {
                points.remove(&roll);
            }
        }
    }
    println!("Part 2: {movable_rolls}");
}

fn load_grid() -> HashSet<Coord> {
    let mut points = HashSet::new();
    fs::read_to_string("d04/input")
        .unwrap()
        .lines().enumerate()
        .for_each(|(y, line)| {
            line.chars().enumerate().for_each(|(x, char)| {
                if char == '@' {
                    points.insert(Point2d::new(x as i32, y as i32));
                }
            })
        });

    return points;
}