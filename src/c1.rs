#![allow(dead_code)]

fn get(x: i32, y: i32, input: &Vec<&str>, ignore: bool, ignore_vec: &mut Vec<(i32, i32)>) -> char {
    if x < 0 || y < 0 || y >= input.len() as i32 || x >= input[0].len() as i32 {
        return '.';
    }

    let value = *input
        .get(y as usize)
        .unwrap()
        .chars()
        .collect::<Vec<_>>()
        .get(x as usize)
        .unwrap();
    if ignore {
        if ignore_vec.contains(&(x, y)) {
            '.'
        } else {
            ignore_vec.push((x, y));
            value
        }
    } else {
        value
    }
}

pub fn gears(input: &str) -> i32 {
    let input = input.split('\n').collect::<Vec<_>>();
    let mut sum = 0;
    let mut ignore_vec = Vec::new();

    for y in 0..input.len() {
        for x in 0..input[y].len() {
            let x = x as i32;
            let y = y as i32;

            if !get(x, y, &input, false, &mut ignore_vec).is_digit(10) {
                continue;
            }

            // first see if there are any adjacent symbols
            let mut symbol = false;
            for i in -1..2 {
                for j in -1..2 {
                    if i == 0 && j == 0 {
                        continue;
                    }
                    let char = get(x + i, y + j, &input, false, &mut ignore_vec);
                    if char == '.' || char.is_digit(10) {
                        continue;
                    }
                    symbol = true;
                    break;
                }
                if symbol {
                    break;
                }
            }

            if symbol {
                // now find adjacent numbers
                let mut value = 0;
                let mut i = 0;
                loop {
                    let char = get(x - i, y, &input, true, &mut ignore_vec);
                    match char.to_digit(10) {
                        Some(number) => {
                            value += ((10 as i32).pow(i as u32)) * number as i32;
                        }
                        None => {
                            break;
                        }
                    }
                    i += 1;
                }
                let mut i = 1;
                loop {
                    let char = get(x + i, y, &input, true, &mut ignore_vec);
                    match char.to_digit(10) {
                        Some(number) => {
                            value = value * 10 + number as i32;
                        }
                        None => {
                            break;
                        }
                    }
                    i += 1;
                }
                sum += value;
            }
        }
    }
    sum
}
