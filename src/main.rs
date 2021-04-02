// inspiration: https://web.eecs.umich.edu/~baveja/RLMasses/RL.html
//
use std::fmt;

pub enum Direction {
    Up,
    Down,
    Rest,
}

type Floor = usize;

pub struct TravelRequest {
    origin: Floor,
    direction: Direction,
    destination: Floor,
}

impl fmt::Display for Direction {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // ▲ ▼ ▮
        Ok(match self {
            Direction::Up => write!(f, "▲")?,
            Direction::Down => write!(f, "▼")?,
            Direction::Rest => write!(f, "x")?,
        })
    }
}

pub struct Elevator {
    min_floor: Floor,
    max_floor: Floor,
    current_floor: Floor,
    direction: Direction,
    floor_backlog: Vec<Floor>,
    requests: Vec<TravelRequest>,
}

impl Elevator {
    fn step(&mut self) {
        self.r#move()
    }

    fn r#move(&mut self) {
        match self.direction {
            Direction::Rest => {}
            Direction::Up => {
                if self.current_floor < self.max_floor {
                    self.current_floor += 1
                } else {
                    self.direction = Direction::Rest
                }
            }
            Direction::Down => {
                if self.current_floor > self.min_floor {
                    self.current_floor -= 1
                } else {
                    self.direction = Direction::Rest
                }
            }
        }
    }

    fn add_request(&mut self, t: TravelRequest) {
        self.floor_backlog.push(t.origin);
        self.requests.push(t);
    }
}

impl fmt::Display for Elevator {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // ▲ ▼ ▮
        writeln!(f, "-----")?; // top line
        for i in (0..(self.max_floor + 1)).rev() {
            // render floor no
            write!(f, "{} ", i)?;

            // render carriage
            if i == self.current_floor {
                write!(f, "{}▮", self.direction)?;
            } else {
                write!(f, " |")?;
            }

            // render requests
            if self.floor_backlog.contains(&i) {
                write!(f, "!")?;
            }

            // finish with new line
            writeln!(f, "")?;
        }
        writeln!(f, "-----") // bottom line
    }
}

fn main() {
    let mut e = Elevator {
        min_floor: 0,
        max_floor: 3,
        current_floor: 0,
        direction: Direction::Up,
        floor_backlog: vec![],
        requests: vec![],
    };

    e.add_request(TravelRequest {
        origin: 2,
        direction: Direction::Down,
        destination: 0,
    });

    println!("The elevator Problem:");

    for i in 0..5 {
        println!("Step {}", i);
        println!("{}", e);
        e.step();
    }
}
