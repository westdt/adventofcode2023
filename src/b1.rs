#![allow(dead_code)]

use std::collections::HashMap;

pub fn cubes(input: &str) -> i32 {
    let mut values = HashMap::new();
    values.insert("red", 12);
    values.insert("green", 13);
    values.insert("blue", 14);

    let lines = input.split('\n').collect::<Vec<_>>();
    let mut sum = 0;
    for line in lines {
        let mut valid = true;
        let split_colon = line.split(':').collect::<Vec<_>>();
        let id = (split_colon
            .get(0)
            .unwrap()
            .split(' ')
            .filter(|element| element.len() > 0)
            .collect::<Vec<_>>()[1])
            .parse::<i32>()
            .unwrap();
        for game in split_colon
            .get(1)
            .unwrap()
            .split(';')
            .filter(|element| element.len() > 0)
            .collect::<Vec<_>>()
        {
            for cube in game
                .split(',')
                .filter(|element| element.len() > 0)
                .collect::<Vec<_>>()
            {
                let cube_split = cube
                    .split(' ')
                    .filter(|element| element.len() > 0)
                    .collect::<Vec<_>>();
                let number = cube_split[0].parse::<i32>().unwrap();
                let color = cube_split[1];
                let max = values.get(color).unwrap();
                if &number > max {
                    valid = false;
                    break;
                }
            }
            if !valid {
                break;
            }
        }
        if valid {
            sum += id
        }
    }

    sum
}
