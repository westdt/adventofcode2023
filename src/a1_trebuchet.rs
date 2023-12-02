pub fn trebuchet(input: &str) -> i32 {
	let mut i = 1;
	let result = input.chars().fold((0, 0, 0), | (sum, a, b), char | {
		match char {
			'\n' => {
				println!("{}: {} + {}: {} ... {}", i, a, b, a * 10 + b, sum + a * 10 + b);
				i += 1;
				(sum + a * 10 + b, 0, 0)
			}
			_ => {
				match char.to_digit(10) {
					Some(value) => {
						match a {
							0 => {
								(sum, value, value)
							}
							_ => {
								(sum, a, value)
							}
						}
					}
					None => {
						(sum, a, b)
					}
				}
			}
		}
	});

	(result.0 + (result.1 * 10 + result.2)) as i32
}