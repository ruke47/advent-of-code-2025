use lib2d::Point2d;
use std::collections::{HashMap, HashSet};
use std::fs;

type Loc = i32;
type Pos = Point2d<Loc>;

struct Layout {
    start: Pos,
    splitters: HashMap<usize, Vec<Pos>>
}

fn main() {
    part1();
    part2();
}

fn part1() {
    let layout = read_layout();
    let mut split_count = 0;

    let mut beams_x = HashSet::new();
    beams_x.insert(layout.start.x);
    let mut splitter_rows: Vec<usize> = layout.splitters
        .keys()
        .map(|k| *k)
        .collect();
    splitter_rows.sort();
    for row in splitter_rows.iter() {
        let mut new_beams_x = HashSet::new();
        let row_splitters: HashSet<Loc> = layout.splitters
            .get(row)
            .unwrap()
            .iter()
            .map(|splitter| splitter.x)
            .collect();
        for old_beam in beams_x.iter() {
            if row_splitters.contains(old_beam) {
                new_beams_x.insert(old_beam - 1);
                new_beams_x.insert(old_beam + 1);
                split_count += 1;
            } else {
                new_beams_x.insert(*old_beam);
            }
        }
        beams_x = new_beams_x;
    }
    println!("Part 1: {split_count}");
}

fn part2() {
    let layout = read_layout();
    let mut split_count = 0;

    let mut beams_x = HashMap::new();
    beams_x.insert(layout.start.x, 1);
    let mut splitter_rows: Vec<usize> = layout.splitters
        .keys()
        .map(|k| *k)
        .collect();
    splitter_rows.sort();
    for row in splitter_rows.iter() {
        let mut new_beams_x = HashMap::new();
        let row_splitters: HashSet<Loc> = layout.splitters
            .get(row)
            .unwrap()
            .iter()
            .map(|splitter| splitter.x)
            .collect();
        for (old_beam, tl_count) in beams_x.iter() {
            if row_splitters.contains(old_beam) {
                for dx in [-1, 1] {
                    let new_x = old_beam + dx;
                    let old_count = new_beams_x.remove(&new_x).unwrap_or(0);
                    new_beams_x.insert(new_x, old_count + tl_count);
                }
            } else {
                let old_count = new_beams_x.remove(old_beam).unwrap_or(0);
                new_beams_x.insert(*old_beam, old_count + tl_count);
            }
        }
        beams_x = new_beams_x;
    }
    let timeline_sum: usize = beams_x.values().sum();
    println!("Part 2: {timeline_sum}");
}

fn read_layout() -> Layout {
    let mut start: Option<Pos> = None;
    let mut splitters = HashMap::new();
    fs::read_to_string("d07/input")
        .unwrap()
        .lines()
        .enumerate()
        .for_each(|(y, line)| {
            for (x, char) in line.chars().enumerate() {
                match char {
                    '.' => {},
                    'S' => start = Some(Point2d::new(x as Loc, y as Loc)),
                    '^' => {
                        let mut list = splitters.remove(&y).unwrap_or_else(|| vec!());
                        list.push(Point2d::new(x as Loc, y as Loc));
                        splitters.insert(y, list);
                    },
                    _ => panic!("Unrecognized char {char}")
                };
            }
        });

    Layout {start: start.unwrap(), splitters}
}