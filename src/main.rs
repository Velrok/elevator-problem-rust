// inspiration: https://web.eecs.umich.edu/~baveja/RLMasses/RL.html
mod elevator;
mod simulations;
mod strategies;

use simulations::ResidentialFlats;

fn main() {
    println!("The elevator Problem:");

    println!("Simulating a residential flats building.");
    let mut sim = ResidentialFlats::new(10);
    sim.enable_debug();
    let (wait_times, backlog) = sim.run();

    println!("Wait times: {:?}", wait_times);

    println!("Backlog: {:?}", backlog);
    println!("");

    let wait_score = wait_times.iter().map(|&x| x as i32).sum::<i32>();
    let backlog_score = backlog as i32 * 10;
    println!("Score lower is better: {:?}", wait_score + backlog_score);
    println!("-------------------------------");
    println!("Wait times score: {:?}", wait_score);
    println!("Backlog penalty: {:?}", backlog_score);
    println!("-------------------------------");
}
