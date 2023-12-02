pub fn trebuchet(input: &str) -> i32 {
	let patterns: Vec<&str> = vec![
		"one",
		"two",
		"three",
		"four",
		"five",
		"six",
		"seven",
		"eight",
		"nine"
	];

	let mut substr = String::new();
	let mut sum = 0;
	let mut a = 0;
	let mut b = 0;
	for char in input.chars() {
		if char == '\n' {
			let join = format!("{}{}", a, b).as_str().parse::<i32>().unwrap();
			sum += join;
			println!("{} + {} = {} ... {}", a,b,join,sum);
			a = 0;
			b = 0;
			substr = String::new();
			continue;
		}

		let value = match &char.to_digit(10) {
			Some(value) => {
				substr = String::new();
				*value
			}
			None => {
				substr = format!("{}{}", substr, char);
				let mut value: u32 = 0;
				let mut valid = false;
				let len = substr.len();
				for i in 0..9 {
					if len < patterns[i].len() {
						if &substr.as_str() == &&(patterns[i][0..len]) {
							valid = true;
						}
					}
					if &substr.as_str() == &patterns[i] {
						value = (i + 1) as u32;
					}
				}
				if !valid {
					substr = String::new();
				}
				value
			}
		} as i32;

		if value != 0 {
			match a {
				0 => {
					a = value;
					b = value;
				}
				_ => {
					b = value;
				}
			}
		}
	}

	sum += format!("{}{}", a, b).as_str().parse::<i32>().unwrap();
	sum
}