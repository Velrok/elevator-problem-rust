use crate::elevator::{Direction, DirectionStrategy, Elevator};

pub struct AlwaysDown;
impl DirectionStrategy for AlwaysDown {
    fn new_direction(_: &Elevator) -> Direction {
        Direction::Down
    }
}

pub struct UpAndDownLoop;
impl DirectionStrategy for UpAndDownLoop {
    fn new_direction(e: &Elevator) -> Direction {
        if e.current_floor == e.max_floor {
            Direction::Down
        } else {
            if e.current_floor == e.min_floor {
                Direction::Up
            } else {
                e.direction
            }
        }
    }
}

pub struct Random;
impl DirectionStrategy for Random {
    fn new_direction(_: &Elevator) -> Direction {
        use rand::prelude::*;

        if random::<bool>() {
            Direction::Up
        } else {
            Direction::Down
        }
    }
}
