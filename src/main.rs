// inspiration: https://web.eecs.umich.edu/~baveja/RLMasses/RL.html
mod elevator;
mod simulations;
mod strategies;
use crate::strategies::{AlwaysDown, UpAndDownLoop};

use simulations::ResidentialFlats;

fn print_results(result: (Vec<usize>, usize)) {
    let (wait_times, backlog) = result;
    println!(">>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>");
    println!("Wait times: {:?}", wait_times);

    println!("Backlog: {:?}", backlog);
    println!("");

    let wait_score = wait_times.iter().map(|&x| x as i32).sum::<i32>();
    let backlog_score = backlog as i32 * 10;

    println!("Score lower is better: {:?}", wait_score + backlog_score);
    println!("-------------------------------");
    println!("Wait times score: {:?}", wait_score);
    println!("Backlog penalty: {:?}", backlog_score);
    println!(">>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>");
    println!("");
}

fn main() {
    println!("The elevator Problem:");

    let steps = 100;

    println!("AlwaysDown (ResidentialFlats)");
    print_results(ResidentialFlats::new(123123, true).run::<AlwaysDown>(steps));

    println!("UpAndDown (ResidentialFlats)");
    print_results(ResidentialFlats::new(123123, false).run::<UpAndDownLoop>(steps));
}
