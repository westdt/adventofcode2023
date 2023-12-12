#![allow(dead_code)]

pub fn ferry(input: &str) -> i32 {
    let input = input.lines().collect::<Vec<_>>();
    let a = |x: usize| input[x].split(':').collect::<Vec<_>>()[1];
    let b = |x: &str| {
        x.split_whitespace()
            .into_iter()
            .filter(|e| e.len() > 0)
            .map(|e| match e.parse::<i32>() {
                Ok(x) => x,
                Err(err) => {
                    println!("{}: {}", err, e);
                    -1
                }
            })
            .collect::<Vec<_>>()
    };

    let times = b(a(0));
    let distances = b(a(1));

    let mut result = 1;
    for i in 0..times.len() {
        let distance = distances[i];
        let time = times[i];
        let mut count = 0;
        for j in 1..=time {
            let x = j;
            let y = time - x;
            let xy = x * y;
            if xy > distance {
                count += 1;
            }
        }
        result *= count;
    }

    result
}
