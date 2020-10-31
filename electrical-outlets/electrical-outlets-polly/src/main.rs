use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines().map(|l| l.unwrap());

    let test_cases = lines
        .next()
        .unwrap()
        .parse::<usize>()
        .unwrap();
    // dbg!(test_cases);
    
    for (m, test_case) in lines.enumerate() {
        if m >= test_cases {
            break;
        }
        let mut numbers = test_case.split(' ');
        let _n = numbers.next().unwrap();
        // dbg!(_n);

        let numbers = numbers.map(|c| c.parse::<u32>().unwrap()).collect::<Vec<u32>>();
        println!("{}", numbers.iter().sum::<u32>() - numbers.len() as u32 + 1);
    }
}
