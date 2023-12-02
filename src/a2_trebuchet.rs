pub fn trebuchet(input: &str) -> i32 {
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

	let numbers = vec![
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

	let mut full_output = String::new();
	for line in lines {
		let mut output = String::new();
		let strlen = line.len();
		let mut matching;
		let mut i = 0;
		for char in line.chars() { 
			matching = false;
			for j in 0..9 {
				let len = numbers[j].len();
				if strlen >= i + len {
					let substr = line.get(i..i + len).unwrap_or("");
					if substr == numbers[j] {
						matching = true;
						output.push_str(format!("{}", j + 1).as_str());
						break;
					}
				}
			}
			if !matching {
				output.push(char);
			}
			i += 1;
		}
		full_output.push_str(output.as_str());
		full_output.push('\n');
	}

	crate::a1_trebuchet::trebuchet(&full_output.as_str())
}