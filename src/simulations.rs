use crate::elevator::{Direction, DirectionStrategy, Elevator, TravelRequest};
use rand::prelude::*;

pub struct ResidentialFlats {
    print_states: bool,
    request_likelyhood: f32,
    seed: u64,
}

impl ResidentialFlats {
    pub fn new(seed: u64, print_states: bool) -> Self {
        Self {
            print_states,
            request_likelyhood: 0.1,
            seed,
        }
    }

    pub fn enable_debug(&mut self) {
        self.print_states = true;
    }

    pub fn run<T: DirectionStrategy>(&self, total_steps: usize) -> (Vec<usize>, usize) {
        let mut e = Elevator::new(0, 3, 3);

        let mut seed = StdRng::seed_from_u64(self.seed);

        for _ in 0..total_steps {
            if let Some(tr) = self.gen_travel_request(&mut seed, &e) {
                e.add_request(tr);
            }
            if self.print_states {
                println!("{}", e);
            }
            e.step::<T>();
        }

        let back_l = e.backlog();
        (e.wait_times, back_l)
    }

    fn gen_travel_request(&self, seed: &mut StdRng, elevator: &Elevator) -> Option<TravelRequest> {
        // todo
        // generate a request at random
        // but not on every call
        // and all requests either start or end at ground floor
        // because residents are only interrested in getting to their flats.
        if seed.gen::<f32>() <= self.request_likelyhood {
            let origin = seed.gen::<usize>() % (elevator.max_floor + 1);

            let destination = if origin != 0 {
                0
            } else {
                // ensure that destination is not 0 in this case
                (seed.gen::<usize>() % elevator.max_floor) + 1
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
