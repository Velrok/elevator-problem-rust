use std::fmt;

#[derive(Copy, Clone)]
pub enum Direction {
    Up,
    Down,
    Rest,
}

type Floor = usize;
type Time = usize;

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
    start: Time,
    entered_at: Option<Time>,
}

pub struct Elevator {
    pub min_floor: Floor,
    pub max_floor: Floor,
    pub current_floor: Floor,
    pub direction: Direction,
    pub floor_backlog: Vec<Floor>,
    jobs: Vec<Job>,
    time: Time,
    pub wait_times: Vec<Time>,
}

pub trait DirectionStrategy {
    fn new_direction(e: &Elevator) -> Direction;
}

#[test]
fn test_elevator() {
    use crate::strategies::AlwaysDown;

    let mut e = Elevator::new(0, 1, 1);
    e.add_request(TravelRequest{
       origin: 1,
       destination: 0,
       direction: Direction::Down,
    });
    for _ in 0..5 {
        e.step::<AlwaysDown>();
    }
    assert_eq!(0, e.jobs.len());
    assert_eq!(0, e.floor_backlog.len());
    assert_eq!(1, e.wait_times.len());
    assert_eq!(3 , e.wait_times[0]); // 1t = travel time + 1t = enter src + 1t exit dst
    assert!(4 <= e.time);
}

#[test]
fn test_elevator_is_no_teleporter() {
    use crate::strategies::AlwaysDown;

    let mut e = Elevator::new(0, 1, 0);
    e.add_request(TravelRequest{
        origin: 1,
        destination: 0,
        direction: Direction::Down,
    });
    e.add_request(TravelRequest{
        origin: 0,
        destination: 1,
        direction: Direction::Up,
    });
    e.step::<AlwaysDown>();
    e.step::<AlwaysDown>();
    e.step::<AlwaysDown>();
    e.step::<AlwaysDown>();


    assert_eq!(0, e.wait_times.len());
    assert_eq!(2, e.jobs.len());
    assert_eq!(2, e.floor_backlog.len());
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
        for job in &mut self.jobs {
            if job.travel_request.origin == self.current_floor {
                self.floor_backlog.push(job.travel_request.destination);
                job.entered_at = Some(self.time);
            }
        }

        // record completed job wait times
        for job in &self.jobs {
            if job.travel_request.destination == self.current_floor && job.entered_at.is_some() {
                let wait = self.time - job.start;
                self.wait_times.push(wait);
            }
        }

        // remove completed jobs
        let mut jobs = self.jobs.clone();
        jobs.retain(|j| j.travel_request.destination != self.current_floor || j.entered_at.is_none());
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
            entered_at: None,
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
