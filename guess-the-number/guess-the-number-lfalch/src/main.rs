use std::io::{stdin, stdout, Write};
use std::cmp::Ordering::{self, *};

fn compare(guess: u32) -> Ordering {
    println!("{}", guess);
    stdout().flush().unwrap();
    let mut s = String::with_capacity(8);
    stdin().read_line(&mut s).unwrap();
    match s.trim() {
        "lower" => Less,
        "higher" => Greater,
        "correct" => Equal,
        _ => unreachable!(),
    }
}

fn main() {
    let mut lower = 1;
    let mut upper = 1000;
    loop {
        let guess = (lower+upper)/2;
        match compare(guess) {
            Less => upper = guess-1,
            Greater => lower = guess+1,
            Equal => break,
        }
    }
}
