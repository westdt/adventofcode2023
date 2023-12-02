#![allow(dead_code)]

use std::fs;

mod a1;
mod a2;
mod b1;
mod b2;

pub fn str_to_lines(input: &str) -> Vec<String> {
	let mut lines = Vec::new();
	{
		let mut current = String::new();
		for char in input.chars() {
			if char == '\n' {
				lines.push(current);
				current = String::new();
			} else {
				current.push(char);
			}
		}
		lines.push(current);
	}
	lines
}

fn read_file(path: &str) -> String {
	fs::read_to_string(&path).expect(format!("Failed to read {}", &path).as_str())
}

fn main() {
	//println!("Day 1, Part 1: {}", a1::trebuchet(read_file("input/trebuchet.txt").as_str()));
	//println!("Day 1, Part 2: {}", a2::trebuchet(read_file("input/trebuchet.txt").as_str()));
	//println!("Day 2, Part 1: {}", b1::cubes(read_file("input/cubes.txt").as_str()));
    println!("Day 2, Part 2: {}", b2::cubes(read_file("input/cubes.txt").as_str()));
}
