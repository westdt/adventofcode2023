#![allow(dead_code)]

use std::{cmp::max, vec, collections::HashMap};

trait Strength {
	fn strength(&self) -> u32;
}

impl Strength for char {
	fn strength(&self) -> u32 {
		match self.to_digit(10) {
			Some(digit) => digit,
			None => match self {
				't' => 10,
				'j' => 11,
				'q' => 12,
				'k' => 13,
				'a' => 14,
				_ => 0
			}
		}
	}
}

impl Strength for String {
    fn strength(&self) -> u32 {
        let chars = self.chars().collect::<Vec<_>>();
		let mut strength = 0;
		for i in 0..5 {
			let pow = (10 as i32).pow(i * 2);
			strength += chars[(4-i) as usize].strength() * pow as u32;
		}
		strength
    }
}

fn count_unique(str: String) -> Vec<i32>  {
	let mut vec: Vec<(char, i32)> = Vec::new();
	for char in str.chars() {
		let mut found = false;
		for i in 0..vec.len() {
			if char == vec[i].0 {
				found = true;
				vec[i].1 += 1;
				break;
			}
		}
		if !found {
			vec.push((char, 1));
		}
	}
	let mut new_vec = Vec::new();
	for e in vec {
		new_vec.push(e.1);
	}
	new_vec
}

fn power(vec: Vec<i32>) -> i32 {
	if vec.len() > 4 {
		// 5 unique, 0 pair
		// no pair
		return 0;
	}

	if vec.len() > 3 {
		// 3 unique, 2 identical
		// one pair
		return 1;
	}
	
	if vec.len() > 2 {
		if vec.contains(&3) {
			// 2 unique, 3 identical
			// 3 of a kind
			return 3;
		} else {
			// 1 unique, 2 sets of identicals
			// two pair
			return 2;
		}
	}

	

	if vec.len() > 1 {
		if vec.contains(&3) {
			// 2 identical, 3 identical
			// full house
			return 4;
		} else {
			// 1 unique, 4 identical
			// 4 of a kind
			return 5;
		}
	}

	// 5 identical
	// 5 of a kind
	return 6;
}

pub fn camels(input: &str) -> i32 {
	let lines = input.lines().collect::<Vec<_>>();
	let mut values = lines.iter().map(|e| {
		let split = e.split_whitespace().filter(|e| e.len() > 0).collect::<Vec<_>>();
		let cards = split[0].to_lowercase();
		let values = split[1].parse::<i32>().unwrap();
		(cards, values)
	}).collect::<Vec<_>>();

	let bubblesort = | values: Vec<(String, i32)> | {
		let mut values = values.clone();
		for i in 0..values.len() - 1 {
			let j = i + 1;
			let a = &values[i];
			let b = &values[j];
			let astr = power(count_unique(a.0.clone()));
			let bstr = power(count_unique(b.0.clone()));
			if bstr > astr {
				values.swap(i, j);
				//println!("Swapping {}:{} and {}:{}", i, astr, j, bstr)
			} else if bstr == astr {
				let astr = a.0.strength();
				let bstr = b.0.strength();
				if bstr > astr {
					values.swap(i, j);
					//println!("Swapping {}:{} and {}:{}", i, astr, j, bstr)
				}
			}
		}
		values
	};

	let check_sort = | values: &Vec<(String, i32)> | {
		let values = values.clone();
		let mut unsort = 0;
		for i in 0..values.len() - 1 {
			let j = i + 1;
			let a = &values[i];
			let b = &values[j];
			let astr = power(count_unique(a.0.clone()));
			let bstr = power(count_unique(b.0.clone()));
			if bstr > astr {
				unsort += 1;
				//println!("Swapping {}:{} and {}:{}", i, astr, j, bstr)
			} else if bstr == astr {
				let astr = a.0.strength();
				let bstr = b.0.strength();
				if bstr > astr {
					unsort += 1;
					//println!("Swapping {}:{} and {}:{}", i, astr, j, bstr)
				}
			}
		}
		let sort_percent = 100.0 - (unsort as f64 / values.len() as f64) * 10.0; 
		println!("{}% sorted", sort_percent);
		return unsort < 1;
	};

	while !check_sort(&values) {
		values = bubblesort(values);
	}

	let mut result = 0;
	for i in 0..values.len() {
		let pow = values.len() - i;
		result += values[i].1 * pow as i32;
	}

    result
}
