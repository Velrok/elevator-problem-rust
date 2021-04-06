use crate::elevator::{Direction, Elevator, TravelRequest};
use crate::strategies::AlwaysDown;

pub struct ResidentialFlats {
    steps: usize,
    print_states: bool,
}

impl ResidentialFlats {
    pub fn new(total_steps: usize) -> Self {
        Self {
            steps: total_steps,
            print_states: false,
        }
    }

    pub fn enable_debug(&mut self) {
        self.print_states = true;
    }

    pub fn run(&self) -> (Vec<usize>, usize) {
        let mut e = Elevator::new(0, 3, 3);

        for t in 0..self.steps {
            if let Some(tr) = Self::gen_travel_request(t) {
                e.add_request(tr);
            }
            if self.print_states {
                println!("{}", e);
            }
            e.step(AlwaysDown);
        }

        let back_l = e.backlog();
        (e.wait_times, back_l)
    }

    fn gen_travel_request(t: usize) -> Option<TravelRequest> {
        match t {
            0 => Some(TravelRequest {
                origin: 3,
                direction: Direction::Down,
                destination: 0,
            }),
            1 => Some(TravelRequest {
                origin: 2,
                direction: Direction::Down,
                destination: 0,
            }),
            2 => Some(TravelRequest {
                origin: 1,
                direction: Direction::Down,
                destination: 0,
            }),
            _ => None,
        }
    }
}
