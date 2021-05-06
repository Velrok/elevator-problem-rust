# The elevator problem

Inspired by https://web.eecs.umich.edu/~baveja/RLMasses/RL.html

To paraphrase the problem: you are tasked to write a program that decides if an
elevator should go up or down, whilst minimising some sort of reward function.

## TODO

- [x] model an elevator
- [x] model passengers calling the elevator
- [x] model passengers setting a destination
- [x] simulate a few steps for one elevator
- [x] model passengers exiting
- [x] measure passenger travel time in steps
- [x] define and print a score
- [x] run a simulation
- [x] generate travel request for residential flats simulation
- [x] write random strategy and evaluate for different scenarios
- [ ] add elevator capacity
- [ ] generate travel request for office simulation
- [ ] write different strategies * x
- [ ] pick a winning strategy
- [ ] maximum number of people in the elevator?

## Idea Simulation Main Loop

```rust
// Program/Strategy

// Elevator #1
// Elevator #2

// Simulation/Building ->
struct Simulation {
  Backlog HashMap::<Floor, Vec<Person>>
  WaitTimes Vec<_>
  AllElevators Vec<Elevator>
}

impl Simulation {
    fn tick(&mut self) {
        let t = gen_travel_request()
        self.Backlog[t.origin].push(t) // todo: push to correct floor backlog

        if Some(e) = strategy.choose_elevator(self.AllElevators, t) {
            e.floors.push(e.origin)
        }

        for each elevator:
            if current_floor in elevator.Floors {
                elevator.OperateDoors()
            } else {
                if Some(dir) = strategy.choose_direction(self.AllElevators, elevator) {
                    elevator.setDirection(dir)
                }
            }
        }
    }
}
```
