#![allow(dead_code)]

pub fn ferry(input: &str) -> i32 {
    let input = input.lines().collect::<Vec<_>>();
    let a = |x: usize| input[x].split(':').collect::<Vec<_>>()[1];
    let b = |x: &str| {
        x.split_whitespace()
            .into_iter()
            .filter(|e| e.len() > 0)
            .fold(String::new(), |mut a, e| {
                a.push_str(e);
                a
            })
            .parse::<i128>()
            .unwrap()
    };

    let time = b(a(0));
    let distance = b(a(1));

    let mut count = 0;
    for j in 1..=time {
        let x = j;
        let y = time - x;
        let xy = x * y;
        if xy > distance {
            count += 1;
        }
    }

    count
}
