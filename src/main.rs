// inspiration: https://web.eecs.umich.edu/~baveja/RLMasses/RL.html
//
use std::fmt;

#[derive(Copy, Clone)]
pub enum Direction {
    Up,
    Down,
    Rest,
}

type Floor = usize;

#[derive(Copy, Clone)]
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

#[derive(Copy, Clone)]
struct Job {
    travel_request: TravelRequest,
    start: usize,
}

pub struct Elevator {
    min_floor: Floor,
    max_floor: Floor,
    current_floor: Floor,
    direction: Direction,
    floor_backlog: Vec<Floor>,
    jobs: Vec<Job>,
    time: usize,
    wait_times: Vec<usize>,
}

trait DirectionStrategy {
    fn new_direction(&self, e: &Elevator) -> Direction;
}

impl Elevator {
    fn is_busy(&self) -> bool {
        !self.floor_backlog.is_empty()
    }

    fn step<T: DirectionStrategy>(&mut self, s: T) {
        self.progress_time();
        self.direction = s.new_direction(&self);
        if self.floor_backlog.contains(&self.current_floor) {
            self.open_doors();
        } else {
            self.r#move();
        }
    }

    fn progress_time(&mut self) {
        self.time += 1;
    }

    fn open_doors(&mut self) {
        // remove current floor from backlog
        self.floor_backlog = self
            .floor_backlog
            .iter()
            .filter(|&&floor| floor != self.current_floor)
            .map(|f| *f)
            .collect();

        // job enters the elevator
        for job in &self.jobs {
            if job.travel_request.origin == self.current_floor {
                self.floor_backlog.push(job.travel_request.destination);
            }
        }

        // record completed job wait times
        for job in &self.jobs {
            if job.travel_request.destination == self.current_floor {
                let wait = self.time - job.start;
                self.wait_times.push(wait);
            }
        }

        // remove completed jobs
        let mut jobs = self.jobs.clone();
        jobs.retain(|j| j.travel_request.destination != self.current_floor);
        self.jobs = jobs;
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
        self.jobs.push(Job {
            travel_request: t,
            start: self.time,
        });
    }
}

impl fmt::Display for Elevator {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // ▲ ▼ ▮
        writeln!(f, "T: {}", self.time)?; // top line
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

struct AlwaysDown;
impl DirectionStrategy for AlwaysDown {
    fn new_direction(&self, _: &Elevator) -> Direction {
        Direction::Down
    }
}

fn main() {
    let mut e = Elevator {
        min_floor: 0,
        max_floor: 3,
        current_floor: 3,
        direction: Direction::Up,
        floor_backlog: vec![],
        jobs: vec![],
        time: 0,
        wait_times: vec![],
    };

    e.add_request(TravelRequest {
        origin: 3,
        direction: Direction::Down,
        destination: 0,
    });

    println!("The elevator Problem:");

    println!("{}", e);
    let mut t = 0;
    while e.is_busy() {
        match t {
            1 => e.add_request(TravelRequest {
                origin: 2,
                direction: Direction::Down,
                destination: 0,
            }),
            2 => {
                e.add_request(TravelRequest {
                    origin: 1,
                    direction: Direction::Down,
                    destination: 0,
                });
            }
            _ => {}
        }
        e.step(AlwaysDown);
        println!("{}", e);
        t += 1;
        if t > 1000000 {
            println!("Engaging failsafe after {} steps.", t);
            break;
        }
    }

    println!("Wait times:");
    println!("{:?}", e.wait_times);
}
