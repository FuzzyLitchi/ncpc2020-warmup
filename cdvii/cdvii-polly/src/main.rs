use std::io::{self, BufRead};
use std::collections::HashMap;

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines().map(|l| l.unwrap());

    let rates = lines
        .next()
        .unwrap()
        .split_whitespace()
        .map(|s| s.parse::<u32>().unwrap())
        .collect::<Vec<u32>>();
    assert_eq!(rates.len(), 24);

    let mut data: Vec<(String, DateTime, String, u32)> = Vec::new();

    for line in lines {
        let mut line = line.split_whitespace();
        let plate = line.next().unwrap();
        let time = parse_datetime(line.next().unwrap());
        let action = line.next().unwrap();
        let pos = line.next().unwrap().parse::<u32>().unwrap();

        // dbg!(plate, time, action, pos);
        data.push((plate.to_owned(), time, action.to_owned(), pos));
    }

    data.sort_by(|(_, date, _, _), (_, other, _, _)| date.partial_cmp(other).unwrap());

    let mut enters: HashMap<String, (DateTime, u32)> = HashMap::new();
    let mut bills: HashMap<String, u32> = HashMap::new();

    for (plate, time, action, pos) in data {
        match action.as_str() {
            "enter" => {enters.insert(plate, (time, pos));},
            "exit" => {
                if let Some((enter_time, enter_pos)) = enters.remove(&plate) {
                    // Enter time has to be smaller
                    assert!(enter_time < time);

                    let rate = rates[enter_time.hour as usize];
                    let cost = (enter_pos as i32 - pos as i32).abs() as u32 * rate + 100;
                    (*bills.entry(plate).or_insert(0)) += cost;
                }
            },
            _ => panic!(),
        }
    }
    
    let mut bills = bills.into_iter().collect::<Vec<(String, u32)>>();
    bills.sort_by(|a,b| a.0.cmp(&b.0));
    for (plate, pennies) in bills {
        let pennies = pennies + 200;
        println!("{} ${}.{}", plate, pennies/100, pennies%100);
    }
}

#[derive(Debug, Copy, Clone, PartialEq)]
struct DateTime {
    month: u32,
    day: u32,
    hour: u32,
    minute: u32,
}

use std::cmp::{Ordering, PartialOrd};
impl PartialOrd for DateTime {
    fn partial_cmp(&self, other: &DateTime) -> Option<Ordering> {
        (self.minute + 60*(self.hour + 24*(self.day + 31*self.month)))
            .partial_cmp(&(other.minute + 60*(other.hour + 24*(other.day + 31*other.month))))
    }
}

fn parse_datetime(datetime: &str) -> DateTime {
    let data = datetime.split(':')
        .map(|s| s.parse::<u32>().unwrap())
        .collect::<Vec<u32>>();
    assert_eq!(data.len(), 4);

    DateTime {
        month: data[0],
        day: data[1],
        hour: data[2],
        minute: data[3],
    }
}