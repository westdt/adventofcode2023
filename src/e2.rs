#![allow(dead_code)]

pub fn fertilizer(input: &str) -> i64 {
    let input = input.lines().collect::<Vec<_>>();
    let seeds = input[0].split(':').collect::<Vec<_>>()[1]
        .split(' ')
        .collect::<Vec<_>>()
        .iter_mut()
        .filter(|s| s.len() > 0)
        .map(|s| s.trim().parse::<i64>().unwrap())
        .collect::<Vec<_>>();

    let mut steps: Vec<Vec<Box<dyn Fn(i64) -> i64>>> = Vec::new();
    for line in input {
        if line.contains("map") {
            steps.push(Vec::new());
        }

        let split = line
            .split(' ')
            .collect::<Vec<_>>()
            .iter_mut()
            .map(|s| match s.parse::<i64>() {
                Ok(i) => i,
                Err(_err) => -1,
            })
            .collect::<Vec<_>>();
        if split.contains(&-1) {
            continue;
        }

        let a = split[0].clone();
        let b = split[1].clone();
        let c = split[2].clone();

        steps.last_mut().unwrap().push(Box::new(move |number| {
            let x = b - a;
            if number >= b && number < b + c {
                return number - x;
            }
            number
        }));
    }

    let mut minimum = i64::MAX;
    let mut i = 0;
    while i < seeds.len() {
        let a = seeds[i];
        let b = seeds[i + 1];
        for j in a..a + b {
            let mut seed = j;
            for step in &steps {
                seed = transform(seed, step);
            }
            if seed < minimum {
                minimum = seed;
            }
        }
        i += 2;
    }

    minimum
}

fn transform(seed: i64, functions: &Vec<Box<dyn Fn(i64) -> i64>>) -> i64 {
    for function in functions {
        let map = function(seed);
        if map != seed {
            return map;
        }
    }
    seed
}
