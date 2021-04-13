use crate::elevator::{Direction, DirectionStrategy, Elevator, TravelRequest};
use rand::prelude::*;
use std::marker::PhantomData;

pub struct ResidentialFlats<T: DirectionStrategy> {
    print_states: bool,
    request_likelyhood: f32,
    rng: StdRng,
    elevator: Elevator,
    phantom: PhantomData<T>
}

impl<T: DirectionStrategy> ResidentialFlats<T> {
    pub fn new(seed: u64, print_states: bool) -> Self {
        let mut rng = StdRng::seed_from_u64(seed);

        Self {
            print_states,
            request_likelyhood: 0.1,
            rng,
            phantom: PhantomData,
            elevator: Elevator::new(0, 3, 3),
        }
    }

    pub fn enable_debug(&mut self) {
        self.print_states = true;
    }

    pub fn run(&mut self, total_steps: usize) -> (Vec<usize>, usize) {
        for _ in 0..total_steps {
            if let Some(tr) = self.gen_travel_request() {
                self.elevator.add_request(tr);
            }
            if self.print_states {
                println!("{}", self.elevator);
            }
            self.elevator.step::<T>();
        }

        let back_l = self.elevator.backlog();
        (self.elevator.wait_times.clone(), back_l)
    }

    fn gen_travel_request(&mut self) -> Option<TravelRequest> {
        // todo
        // generate a request at random
        // but not on every call
        // and all requests either start or end at ground floor
        // because residents are only interrested in getting to their flats.
        if self.rng.gen::<f32>() <= self.request_likelyhood {
            let origin = self.rng.gen::<usize>() % (self.elevator.max_floor + 1);

            let destination = if origin != 0 {
                0
            } else {
                // ensure that destination is not 0 in this case
                (self.rng.gen::<usize>() % self.elevator.max_floor) + 1
            };

            let direction = if destination < origin {
                Direction::Down
            } else {
                Direction::Up
            };

            Some(TravelRequest {
                direction,
                origin,
                destination,
            })
        } else {
            None
        }
    }
}
