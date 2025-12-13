use std::cell::RefCell;
use std::collections::HashMap;
use std::fs;
use std::rc::Rc;

type Loc = i64;
type Id = usize;

#[derive(Eq, PartialEq, Clone, Copy, Debug, Hash)]
struct Point3d {
    x: Loc,
    y: Loc,
    z: Loc,
    id: Id
}

struct Game {
    distinct_points: Vec<Point3d>,
    sorted_distances: Vec<(Point3d, Point3d, f64)>
}

impl Point3d {
    fn new(x: Loc, y: Loc, z: Loc, id: Id) -> Point3d {
        Point3d {x, y, z, id}
    }

    fn distance(&self, other: &Self) -> f64 {
        let sq_dist = ((self.x - other.x).pow(2) +
            (self.y - other.y).pow(2) +
            (self.z - other.z).pow(2)) as f64;

        sq_dist.sqrt()
    }
}

fn main() {
    part1();
    part2();
}

fn part1() {
    let game = load_points();

    let mut circuit_map: HashMap<Id, Rc<RefCell<Vec<Id>>>> = HashMap::new();
    let mut distinct_circuits = vec!();
    for (point_a, point_b, _) in game.sorted_distances[..1000].iter() {
        // get the nearest 2 points
        println!("A: {:?}", point_a);
        println!("B: {:?}", point_b);

        // remove the circuit for each point from the map, if present
        let point_a_circ = circuit_map.remove(&point_a.id);
        let point_b_circ = circuit_map.remove(&point_b.id);

        if let Some(point_a_circ) = point_a_circ {
            if let Some(point_b_circ) = point_b_circ {
                if point_a_circ == point_b_circ {
                    // they're already in the same circuit, easy peasy,
                    // just put 'em back in the map
                    println!("  Both already in same circuit");
                    circuit_map.insert(point_a.id, point_a_circ);
                    circuit_map.insert(point_b.id, point_b_circ);
                    continue;
                }
                // ugh, we need to merge the circuits
                // for every id in B's circuit, add that id to A's circuit,
                // then change the circuit map for that id to point at A's circuit instead
                println!("  Both in different circuits");
                for id in point_b_circ.borrow().iter() {
                    point_a_circ.borrow_mut().push(*id);
                    circuit_map.insert(*id, Rc::clone(&point_a_circ));
                }
                circuit_map.insert(point_a.id, Rc::clone(&point_a_circ));
                point_b_circ.borrow_mut().clear();
            } else {
                println!("  A already in circuit");
                // point a is already in a circuit, but point b is not
                point_a_circ.borrow_mut().push(point_b.id);
                println!("    A circuit: {:?}", point_a_circ);
                println!("    Distinct circuits {:?}", distinct_circuits);
                circuit_map.insert(point_b.id, Rc::clone(&point_a_circ));
                circuit_map.insert(point_a.id, point_a_circ);
            }
        } else {
            if let Some(point_b_circ) = point_b_circ {
                println!("  B already in circuit");
                // point b is already in a circuit, but point a is not
                point_b_circ.borrow_mut().push(point_a.id);
                println!("    A circuit: {:?}", point_b_circ);
                println!("    Distinct circuits {:?}", distinct_circuits);
                circuit_map.insert(point_a.id, Rc::clone(&point_b_circ));
                circuit_map.insert(point_b.id, point_b_circ);
            } else {
                println!("  Creating new circuit");
                // neither point is already in a circuit, create a new one
                let new_circ = Rc::new(RefCell::new(vec!(point_a.id, point_b.id)));
                circuit_map.insert(point_a.id, Rc::clone(&new_circ));
                circuit_map.insert(point_b.id, Rc::clone(&new_circ));
                distinct_circuits.push(new_circ);
            }
        }
    }

    println!("Distinct circuits: {:?}", distinct_circuits);

    let mut circuit_lengths: Vec<usize> = distinct_circuits
        .iter()
        .map(|circuit| circuit.borrow().len())
        .collect();
    circuit_lengths.sort_by(|a, b| b.cmp(a));

    let multiplier = circuit_lengths[0] * circuit_lengths[1] * circuit_lengths[2];
    println!("Part 1: {multiplier}");
}

fn part2() {
    let game = load_points();

    let mut circuit_map: HashMap<Id, Rc<RefCell<Vec<Id>>>> = HashMap::new();
    let mut distinct_circuits = vec!();
    for (point_a, point_b, _) in game.sorted_distances.iter() {
        // get the nearest 2 points
        // println!("A: {:?}", point_a);
        // println!("B: {:?}", point_b);
        let mut modified_circuit = None;

        // remove the circuit for each point from the map, if present
        let point_a_circ = circuit_map.remove(&point_a.id);
        let point_b_circ = circuit_map.remove(&point_b.id);

        if let Some(point_a_circ) = point_a_circ {
            if let Some(point_b_circ) = point_b_circ {
                if point_a_circ == point_b_circ {
                    // they're already in the same circuit, easy peasy,
                    // just put 'em back in the map
                    // println!("  Both already in same circuit");
                    circuit_map.insert(point_a.id, point_a_circ);
                    circuit_map.insert(point_b.id, point_b_circ);
                    continue;
                }
                // ugh, we need to merge the circuits
                // for every id in B's circuit, add that id to A's circuit,
                // then change the circuit map for that id to point at A's circuit instead
                // println!("  Both in different circuits");
                for id in point_b_circ.borrow().iter() {
                    point_a_circ.borrow_mut().push(*id);
                    circuit_map.insert(*id, Rc::clone(&point_a_circ));
                }
                modified_circuit = Some(Rc::clone(&point_a_circ));
                circuit_map.insert(point_a.id, Rc::clone(&point_a_circ));
                point_b_circ.borrow_mut().clear();
            } else {
                // println!("  A already in circuit");
                // point a is already in a circuit, but point b is not
                point_a_circ.borrow_mut().push(point_b.id);
                circuit_map.insert(point_b.id, Rc::clone(&point_a_circ));
                modified_circuit = Some(Rc::clone(&point_a_circ));
                circuit_map.insert(point_a.id, point_a_circ);
            }
        } else {
            if let Some(point_b_circ) = point_b_circ {
                // println!("  B already in circuit");
                // point b is already in a circuit, but point a is not
                point_b_circ.borrow_mut().push(point_a.id);
                circuit_map.insert(point_a.id, Rc::clone(&point_b_circ));
                modified_circuit = Some(Rc::clone(&point_b_circ));
                circuit_map.insert(point_b.id, point_b_circ);
            } else {
                // println!("  Creating new circuit");
                // neither point is already in a circuit, create a new one
                let new_circ = Rc::new(RefCell::new(vec!(point_a.id, point_b.id)));
                circuit_map.insert(point_a.id, Rc::clone(&new_circ));
                circuit_map.insert(point_b.id, Rc::clone(&new_circ));
                modified_circuit = Some(Rc::clone(&new_circ));
                distinct_circuits.push(new_circ);
            }
        }

        if let Some(modified_circuit) = modified_circuit {
            if modified_circuit.borrow().len() == game.distinct_points.len() {
                let x_dist = point_a.x * point_b.x;
                println!("Part 2: {x_dist}");
                break;
            }
        }
    }
}

fn load_points() -> Game {
    let points: Vec<Point3d> = fs::read_to_string("d08/input")
        .unwrap()
        .lines()
        .enumerate()
        .map(|(id, line)| {
            let coords: Vec<Loc> = line
                .split(",")
                .map(|d| d.parse().unwrap())
                .collect();
            Point3d::new(coords[0], coords[1], coords[2], id)
        })
        .collect();

    let mut distances = vec!();

    // for every point in the list
    for (idx, point_a) in points.iter().enumerate() {
        // for every _later_ point in the list
        for point_b in points[(idx + 1)..].iter() {
            // add the pair and their distance to the distance list
            distances.push((point_a.clone(), point_b.clone(), point_a.distance(&point_b)))
        }
    }
    // sort the points by their distance apart
    distances.sort_by(|a, b| a.2.total_cmp(&b.2));

    Game {distinct_points: points, sorted_distances: distances}
}