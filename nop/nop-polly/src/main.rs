use std::io::{self, Read};

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();

    let split = input
        .trim()
        .split(|c: char| c.is_uppercase())
        .skip(1)
        .collect::<Vec<&str>>();

    let nops = split[..split.len()-1]
        .iter()
        .map(|s| 3 - (s.len() as u32 % 4))
        .sum::<u32>();
    println!("{}", nops);
}
