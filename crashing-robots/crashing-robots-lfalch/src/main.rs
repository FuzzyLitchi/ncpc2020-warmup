use std::io::{self, BufRead};

#[derive(Debug, Copy, Clone)]
struct Robot {
    position: (i8, i8),
    orientation: Orientation,
}

#[derive(Debug, Copy, Clone)]
struct Instruction {
    robot_i: usize,
    action: Action,
    repeat: u8,
}

#[derive(Debug, Copy, Clone)]
enum Orientation {
    N,
    E,
    S,
    W
}

impl Orientation {
    fn right(self) -> Self {
        use self::Orientation::*;
        match self {
            N => E,
            E => S,
            S => W,
            W => N,
        }
    }
    fn left(self) -> Self {
        use self::Orientation::*;
        match self {
            N => W,
            W => S,
            S => E,
            E => N,
        }
    }
    fn position_offset(self) -> (i8, i8) {
        use self::Orientation::*;
        match self {
            N => (0, 1),
            S => (0, -1),
            E => (1, 0),
            W => (-1, 0),
        }
    }
}

#[derive(Debug, Copy, Clone)]
enum Action {
    L,
    R,
    F
}

#[derive(Debug)]
struct TestCase {
    width: i8,
    height: i8,
    robots: Vec<Robot>,
    instructions: Vec<Instruction>,
}

fn read_test_cases() -> Vec<TestCase> {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines().map(|l| l.unwrap());

    let tests_num = lines.next().unwrap().parse().unwrap();

    let mut test_cases = Vec::with_capacity(tests_num);
    for _ in 0..tests_num {
        let (width, height) = {
            let line = lines.next().unwrap();
            let mut dims = line.split(' ');
            
            (dims.next().unwrap().parse().unwrap(),
                dims.next().unwrap().parse().unwrap())
        };
        let (robots_num, instructions_num) = {
            let line = lines.next().unwrap();
            let mut ab = line.split(' ');
            
            (ab.next().unwrap().parse().unwrap(),
                ab.next().unwrap().parse().unwrap())
        };

        let mut robots = Vec::with_capacity(robots_num);

        for _ in 0..robots_num {
            let line = lines.next().unwrap();
            let mut start = line.split(' ');

            let x = start.next().unwrap().parse().unwrap();
            let y = start.next().unwrap().parse().unwrap();
            let o = start.next().unwrap();

            let orientation = match o {
                "N" => Orientation::N,
                "S" => Orientation::S,
                "E" => Orientation::E,
                "W" => Orientation::W,
                _ => unreachable!(),
            };

            robots.push(Robot {
                position: (x, y),
                orientation,
            })
        }
        let mut instructions = Vec::with_capacity(instructions_num);
        for _ in 0..instructions_num {
            let line = lines.next().unwrap();
            let mut ins = line.split(' ');
            
            let robot_i = ins.next().unwrap().parse::<usize>().unwrap() - 1;
            let action = match ins.next().unwrap() {
                "L" => Action::L,
                "R" => Action::R,
                "F" => Action::F,
                _ => unreachable!(),
            };
            let repeat = ins.next().unwrap().parse().unwrap();

            instructions.push(Instruction {
                robot_i,
                action,
                repeat
            })
        }
        test_cases.push(TestCase {
            width,
            height,
            instructions,
            robots,
        })
    }
    test_cases
}

enum Crash {
    Wall(usize),
    Robot(usize, usize),
}

fn run_test_cases() -> impl Iterator<Item=Result<(), Crash>> {
    read_test_cases().into_iter().map(|TestCase{width, height, mut robots, instructions}| {
        for Instruction { robot_i, action, repeat } in instructions {
            let robot = &mut robots[robot_i];
            match action {
                Action::R => for _ in 0..repeat {
                    robot.orientation = robot.orientation.right()
                }
                Action::L => for _ in 0..repeat {
                    robot.orientation = robot.orientation.left()
                }
                Action::F => {
                    let offset = robot.orientation.position_offset();
                    drop(robot);

                    for _ in 0..repeat {
                        let (x, y) = {
                            let (ref mut x, ref mut y) = robots[robot_i].position;
                            *x += offset.0;
                            *y += offset.1;
                            (*x, *y)
                        };
                        for (i, robot) in robots.iter().enumerate() {
                            if i != robot_i && x == robot.position.0 && y == robot.position.1 {
                                return Err(Crash::Robot(robot_i, i));
                            }
                        }
                        if x < 1 || y < 1 || x > width || y > height {
                            return Err(Crash::Wall(robot_i));
                        }
                    }
                }
            }
        }
        Ok(())
    })
}

fn main() {
    for res in run_test_cases() {
        match res {
            Ok(()) => println!("OK"),
            Err(Crash::Wall(i)) => println!("Robot {} crashes into the wall", i+1),
            Err(Crash::Robot(i, o)) => println!("Robot {} crashes into robot {}", i+1, o+1),
        }
    }
}