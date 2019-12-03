// Not able to tackle. Source from:
// https://github.com/frerich/aoc2019/blob/master/rust/day3/src/main.rs
use std::collections::HashSet;

fn parse_step(step: &str) -> impl Iterator<Item = (isize, isize)> {
    let mut chars = step.chars();

    let coord = match chars.next() {
        Some('U') => (0, -1),
        Some('D') => (0, 1),
        Some('L') => (-1, 0),
        Some('R') => (1, 0),
        _ => (0, 0),
    };

    let dist: usize = chars.collect::<String>().parse().unwrap();

    std::iter::repeat(coord).take(dist)
}

fn parse_line(line: &str) -> Vec<(isize, isize)> {
    line.split(",")
        .flat_map(parse_step)
        .scan((0, 0), |pos, step| {
            pos.0 += step.0;
            pos.1 += step.1;
            Some(pos.clone())
        })
        .collect()
}

fn manhattan_distance(pos: &(isize, isize)) -> usize {
    pos.0.abs() as usize + pos.1.abs() as usize
}

fn main() {
    let input = std::fs::read_to_string("src/input").expect("Failed to read input.txt");

    let mut lines = input.lines();
    let steps_0 = parse_line(lines.next().unwrap());
    let steps_1 = parse_line(lines.next().unwrap());

    let positions_0: HashSet<_> = steps_0.iter().cloned().collect();
    let positions_1: HashSet<_> = steps_1.iter().cloned().collect();
    let collisions = positions_0.intersection(&positions_1);

    let closest_collision = collisions.clone().map(manhattan_distance).min();
    println!("Part One: {:?}", closest_collision.unwrap());

    let quickest_collision = collisions
        .map(|pos| {
            steps_0.iter().position(|x| x == pos).unwrap()
                + steps_1.iter().position(|x| x == pos).unwrap()
                + 2
        })
        .min();
    println!("Part Two: {:?}", quickest_collision.unwrap());
}

// use std::collections::HashSet;
// use std::fs;

// #[derive(Clone, Copy, Debug)]
// struct Coordinate {
//     x: i16,
//     y: i16,
// }

// impl PartialEq for Coordinate {
//     fn eq(&self, other: &Self) -> bool {
//         self.x == other.x && self.y == other.y
//     }
// }

// impl Coordinate {
//     fn distance(&self) -> i16 {
//         self.x + self.y
//     }

//     fn cross(&self, other: &Coordinate) -> bool {
//         self.x < other.x && self.y > other.y
//     }
// }

// fn manhattan_distance(coordinate_a: &Coordinate, coordinate_b: &Coordinate) -> i16 {
//     (coordinate_a.x - coordinate_b.x).abs() + (coordinate_a.y - coordinate_b.y).abs()
// }

// fn wire_mapping(wire_path: Vec<&str>) -> Vec<Coordinate> {
//     // let mut coordinates: Vec<Coordinate> = vec![Coordinate { x: 0, y: 0 }];
//     let mut mapping: Vec<Vec<u8>> = vec![];
//     let mut current_position: Vec<Vec<u8>> = vec![[0]]

//     for coordinate in wire_path {
//         let direction: &str = &coordinate[..1];
//         let value_string: &str = &coordinate[1..];
//         let value: i16 = value_string.parse::<i16>().unwrap();

//         match direction {
//             "U" => {
//                 for _ in (1..value) {
//                     mapping
//                 }
//             },
//             "D" => {},
//             "R" => {},
//             "L" => {},
//             _ => {},
//         };
//     }

//     // for coordinate in wire_path {
//     //     let direction: &str = &coordinate[..1];
//     //     let value_string: &str = &coordinate[1..];
//     //     let value: i16 = value_string.parse::<i16>().unwrap();

//     //     let previous_coordinate = coordinates[index - 1];

//     //     let next_coordinate = match direction {
//     //         "U" => Coordinate {
//     //             x: previous_coordinate.x,
//     //             y: previous_coordinate.y + value,
//     //         },
//     //         "D" => Coordinate {
//     //             x: previous_coordinate.x,
//     //             y: previous_coordinate.y - value,
//     //         },
//     //         "R" => Coordinate {
//     //             x: previous_coordinate.x + value,
//     //             y: previous_coordinate.y,
//     //         },
//     //         "L" => Coordinate {
//     //             x: previous_coordinate.x - value,
//     //             y: previous_coordinate.y,
//     //         },
//     //         _ => Coordinate { x: 0, y: 0 },
//     //     };

//     //     coordinates.push(next_coordinate.clone());
//     //     index += 1;
//     // }
//     //
//     // coordinates
// }

// fn main() {
//     let input = fs::read_to_string("src/input").unwrap();
//     let coordinates: Vec<&str> = input.split("\n").collect();

//     let wire_a: Vec<&str> = coordinates[0].split(",").collect();
//     let wire_b: Vec<&str> = coordinates[1].split(",").collect();

//     let wire_a_mapping = wire_mapping(wire_a);
//     let wire_b_mapping = wire_mapping(wire_b);

//     println!("{:?}", wire_a_mapping);
//     println!("{:?}", wire_b_mapping);

//     for ea in wire_a_mapping {
//         for eb in &wire_b_mapping {
//             // println!("{:?}", manhattan_distance(&ea ,&eb));
//             println!("{:?}", ea);
//             println!("{:?}", eb);
//             println!("----");
//         }
//     }

//     // let intersection_distances: Vec<i16> = wire_a_mapping
//     //     .intersect(wire_b_mapping)
//     //     .into_iter()
//     //     .map(|coordinate| coordinate.distance())
//     //     .filter(|distance| distance.clone() != 0 as i16)
//     //     .collect();

//     // println!("{:?}", intersection_distances)
// }
