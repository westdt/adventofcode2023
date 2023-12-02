pub fn trebuchet(input: &str) -> i32 {
	let result = input.chars().fold((0, 0, 0), | (s, a, b), char | {
		match char {
			'\n' => {
				(s + format!("{}{}", a, b).parse::<i32>().unwrap(), 0, 0)
			}
			_ => {
				match char.to_digit(10) {
					Some(value) => {
						match a {
							0 => {
								(s, value, value)
							}
							_ => {
								(s, a, value)
							}
						}
					}
					None => {
						(s, a, b)
					}
				}
			}
		}
	});

	result.0 + format!("{}{}", result.1, result.2).parse::<i32>().unwrap()
}