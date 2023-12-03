#![allow(dead_code)]

fn get(
    x: i32,
    y: i32,
    input: &Vec<&str>,
    ignore: bool,
    add: bool,
    ignore_vec: &mut Vec<(i32, i32)>,
) -> char {
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
            if add {
                ignore_vec.push((x, y));
            }
            value
        }
    } else {
        value
    }
}

fn get_section(x: i32, y: i32, input: &Vec<&str>, ignore_vec: &mut Vec<(i32, i32)>) -> i32 {
    // now find adjacent numbers
    let mut value = 0;
    let mut i = 0;
    loop {
        let char = get(x - i, y, &input, true, true, ignore_vec);
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
        let char = get(x + i, y, &input, true, true, ignore_vec);
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
    value
}

pub fn gears(input: &str) -> i32 {
    let input = input.split('\n').collect::<Vec<_>>();
    let mut sum = 0;
    let mut ignore_vec = Vec::new();

    for y in 0..input.len() {
        for x in 0..input[y].len() {
            let x = x as i32;
            let y = y as i32;

            if get(x, y, &input, false, false, &mut ignore_vec) != '*' {
                continue;
            }

            let mut numbers = Vec::new();

            for i in -1..2 {
                for j in -1..2 {
                    if i == 0 && j == 0 {
                        continue;
                    }

                    if get(x + i, y + j, &input, true, false, &mut ignore_vec).is_digit(10) {
                        numbers.push(get_section(x + i, y + j, &input, &mut ignore_vec));
                    }
                }
            }

            if numbers.len() == 2 {
                sum += numbers.get(0).unwrap() * numbers.get(1).unwrap();
            }
        }
    }
    sum
}
