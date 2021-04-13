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
    pub origin: Floor,
    pub direction: Direction,
    pub destination: Floor,
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
    pub min_floor: Floor,
    pub max_floor: Floor,
    pub current_floor: Floor,
    pub direction: Direction,
    pub floor_backlog: Vec<Floor>,
    jobs: Vec<Job>,
    time: usize,
    pub wait_times: Vec<usize>,
}

pub trait DirectionStrategy {
    fn new_direction(e: &Elevator) -> Direction;
}

impl Elevator {
    pub fn new(min: usize, max: usize, current: usize) -> Self {
        Elevator {
            min_floor: min,
            max_floor: max,
            current_floor: current,
            direction: Direction::Rest,
            floor_backlog: vec![],
            jobs: vec![],
            time: 0,
            wait_times: vec![],
        }
    }

    pub fn is_busy(&self) -> bool {
        !self.floor_backlog.is_empty()
    }

    pub fn step<T: DirectionStrategy>(&mut self) {
        self.progress_time();
        self.direction = T::new_direction(&self);
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

    pub fn add_request(&mut self, t: TravelRequest) {
        self.floor_backlog.push(t.origin);
        self.jobs.push(Job {
            travel_request: t,
            start: self.time,
        });
    }

    pub fn backlog(&self) -> usize {
        self.jobs.len()
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
