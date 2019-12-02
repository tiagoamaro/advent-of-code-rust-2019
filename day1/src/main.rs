use math::round;
use std::fs::File;
use std::io::prelude::*;
use std::io::{BufReader};

fn module_fuel(mass: i32) -> i32 {
    let raw_fuel = (mass as f64) / 3.0;
    (round::floor(raw_fuel, 0) as i32) - 2
}

fn main() {
    let file = File::open("src/input").unwrap();
    let reader = BufReader::new(file);
    let mut total_fuel = 0;

    for line in reader.lines() {
        let mass = line.unwrap().parse::<i32>().unwrap();
        total_fuel += module_fuel(mass);       
    }

    println!("{}", total_fuel)
}
