#![allow(dead_code)]

pub fn scratchcard(input: &str) -> i32 {
    input
        .split('\n')
        .map(|element| {
            let sides = element.split(':').collect::<Vec<_>>()[1]
                .split('|')
                .collect::<Vec<_>>();
            let a = sides[0]
                .split(' ')
                .filter(|e| e.len() > 0)
                .map(|n| n.parse::<i32>().unwrap())
                .collect::<Vec<_>>();
            let b = sides[1]
                .split(' ')
                .filter(|e| e.len() > 0)
                .map(|n| n.parse::<i32>().unwrap())
                .collect::<Vec<_>>();
            let mut line_sum = 0;
            for n in &b {
                if a.contains(n) {
                    if line_sum < 1 {
                        line_sum = 1;
                    } else {
                        line_sum *= 2;
                    }
                }
            }
            line_sum
        })
        .sum()
}
