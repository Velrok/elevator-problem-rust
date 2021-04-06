// inspiration: https://web.eecs.umich.edu/~baveja/RLMasses/RL.html
mod elevator;
mod strategies;

use elevator::{Direction, Elevator, TravelRequest};
use strategies::AlwaysDown;

fn main() {
    let mut e = Elevator::new(0, 3, 3);

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
