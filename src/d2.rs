#![allow(dead_code)]

pub fn scratchcard(input: &str) -> i32 {
	let input = input.split('\n').collect::<Vec<_>>();
	let mut vec: Vec<i32> = Vec::with_capacity(input.len());
	for i in 0..input.len() {
		vec.push(0);
	}

	let mut i = 0;
	for element in input {
		let sides = element.split(':').collect::<Vec<_>>()[1].split('|').collect::<Vec<_>>();
		let a = sides[0].split(' ').filter(|e| e.len() > 0).map(|n| n.parse::<i32>().unwrap()).collect::<Vec<_>>();
		let b = sides[1].split(' ').filter(|e| e.len() > 0).map(|n| n.parse::<i32>().unwrap()).collect::<Vec<_>>();
		let mut win_count = 0;
		for n in &b {
			if a.contains(n) {
				win_count += 1;
			}
		}

		vec[i] += 1;

		for j in 1..win_count + 1 {
			if i + j < vec.len() {
				let mut x = vec[i];
				if x < 1 {
					x = 1;
				}
				vec[i + j] += x;
			}
		}

		i += 1;
	}
	
	vec.iter().sum()
}