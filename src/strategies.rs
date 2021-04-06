use crate::elevator::{Direction, DirectionStrategy, Elevator};

pub struct AlwaysDown;
impl DirectionStrategy for AlwaysDown {
    fn new_direction(&self, _: &Elevator) -> Direction {
        Direction::Down
    }
}
