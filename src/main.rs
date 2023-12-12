#![allow(dead_code)]

use std::fs;

mod a1;
mod a2;
mod b1;
mod b2;
mod c1;
mod c2;
mod d1;
mod d2;
mod e1;
mod e2;
mod f1;
mod f2;
mod g1;

fn read_file(path: &str) -> String {
    fs::read_to_string(&path).expect(format!("Failed to read {}", &path).as_str())
}

fn main() {
    // println!(
    //     "Day 1, Part 1: {}",
    //     a1::trebuchet(read_file("input/trebuchet.txt").as_str())
    // );
    // println!(
    //     "Day 1, Part 2: {}",
    //     a2::trebuchet(read_file("input/trebuchet.txt").as_str())
    // );
    // println!(
    //     "Day 2, Part 1: {}",
    //     b1::cubes(read_file("input/cubes.txt").as_str())
    // );
    // println!(
    //     "Day 2, Part 2: {}",
    //     b2::cubes(read_file("input/cubes.txt").as_str())
    // );
    // println!(
    //     "Day 3, Part 1: {}",
    //     c1::gears(read_file("input/gears.txt").as_str())
    // );
    // println!(
    //     "Day 3, Part 2: {}",
    //     c2::gears(read_file("input/gears.txt").as_str())
    // );
    // println!(
    //     "Day 4, Part 1: {}",
    //     d1::scratchcard(read_file("input/scratchcard.txt").as_str())
    // );
    // println!(
    //     "Day 4, Part 2: {}",
    //     d2::scratchcard(read_file("input/scratchcard.txt").as_str())
    // );
    // println!(
    //     "Day 5, Part 1: {}",
    //     e1::fertilizer(read_file("input/fertilizer.txt").as_str())
    // );
    // println!(
    //     "Day 5, Part 2: {}",
    //     e2::fertilizer(read_file("input/fertilizer.txt").as_str())
    // );
    // println!(
    //     "Day 6, Part 1: {}",
    //     f1::ferry(read_file("input/ferry.txt").as_str())
    // );
    // println!(
    //     "Day 6, Part 2: {}",
    //     f2::ferry(read_file("input/ferry.txt").as_str())
    // );
	println!(
        "Day 6, Part 2: {}",
        g1::camels(read_file("input/camels.txt").as_str())
    );
}
