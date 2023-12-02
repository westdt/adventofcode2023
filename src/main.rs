#![allow(dead_code)]

use std::{fs, env};

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
    println!("{}", b2::cubes(read_file("input/cubes.txt").as_str()));
}
