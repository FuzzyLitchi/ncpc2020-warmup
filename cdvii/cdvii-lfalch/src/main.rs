use std::str::FromStr;
use std::io::{self, BufRead};
use std::collections::HashMap;

enum Action {
    Enter,
    Exit,
}

impl FromStr for Action {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "enter" => Ok(Action::Enter),
            "exit" => Ok(Action::Exit),
            _ => Err(()),
        }
    }
}

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

    let mut data: Vec<(String, DateTime, Action, u32)> = Vec::new();

    for line in lines {
        let mut line = line.split_whitespace();
        let plate = line.next().unwrap();
        let time = parse_datetime(line.next().unwrap());
        let action = line.next().unwrap().parse().unwrap();
        let pos = line.next().unwrap().parse::<u32>().unwrap();

        // dbg!(plate, time, action, pos);
        data.push((plate.to_owned(), time, action, pos));
    }

    data.sort_by_key(|&(_, date, _, _)| date);

    let mut enters: HashMap<String, (DateTime, u32)> = HashMap::new();
    let mut bills: HashMap<String, u32> = HashMap::new();

    for (plate, time, action, pos) in data {
        match action {
            Action::Enter => {
                // if there was an old entry, let's keep that one
                enters.entry(plate).or_insert((time, pos));
            },
            Action::Exit => {
                if let Some((enter_time, enter_pos)) = enters.remove(&plate) {
                    // Enter time has to be smaller
                    assert!(enter_time < time);

                    let rate = rates[enter_time.hour as usize];
                    let cost = (enter_pos as i32 - pos as i32).abs() as u32 * rate + 100;
                    (*bills.entry(plate).or_insert(0)) += cost;
                }
            },
        }
    }
    
    let mut bills = bills.into_iter().collect::<Vec<(String, u32)>>();
    bills.sort_by(|a,b| a.0.cmp(&b.0));
    for (plate, pennies) in bills {
        let pennies = pennies + 200;
        println!("{} ${}.{}", plate, pennies/100, pennies%100);
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
struct DateTime {
    month: u32,
    day: u32,
    hour: u32,
    minute: u32,
}

impl DateTime {
    fn minutes(&self) -> u32 {
        self.minute + 60*(self.hour + 24*(self.day + 31*self.month))
    }
}

use std::cmp::{Ordering, PartialOrd, Ord};
impl PartialOrd for DateTime {
    fn partial_cmp(&self, other: &DateTime) -> Option<Ordering> {
        self.minutes().partial_cmp(&other.minutes())
    }
}
impl Ord for DateTime {
    fn cmp(&self, other: &DateTime) -> Ordering {
        self.minutes().cmp(&other.minutes())
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
