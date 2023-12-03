#![allow(dead_code)]

use std::collections::HashMap;

pub fn cubes(input: &str) -> i32 {
    let mut values = HashMap::new();

    let lines = input.split('\n').collect::<Vec<_>>();
    let mut sum = 0;
    for line in lines {
        values.insert("red".to_owned(), 0);
        values.insert("green".to_owned(), 0);
        values.insert("blue".to_owned(), 0);

        for game in line
            .split(':')
            .collect::<Vec<_>>()
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
                let current = values.get(color).unwrap();
                if &number > current {
                    values.insert(color.to_owned(), number);
                }
            }
        }

        sum += values.iter().fold(1, |i, s| i * s.1)
    }

    sum
}
