#![allow(dead_code)]

use std::fs;

mod a1;
mod a2;
mod b1;
mod b2;
mod c1;
mod c2;

fn read_file(path: &str) -> String {
    fs::read_to_string(&path).expect(format!("Failed to read {}", &path).as_str())
}

fn main() {
    println!(
        "Day 1, Part 1: {}",
        a1::trebuchet(read_file("input/trebuchet.txt").as_str())
    );
    println!(
        "Day 1, Part 2: {}",
        a2::trebuchet(read_file("input/trebuchet.txt").as_str())
    );
    println!(
        "Day 2, Part 1: {}",
        b1::cubes(read_file("input/cubes.txt").as_str())
    );
    println!(
        "Day 2, Part 2: {}",
        b2::cubes(read_file("input/cubes.txt").as_str())
    );
    println!(
        "Day 3, Part 1: {}",
        c1::gears(read_file("input/gears.txt").as_str())
    );
    println!(
        "Day 3, Part 2: {}",
        c2::gears(read_file("input/gears.txt").as_str())
    );
}
