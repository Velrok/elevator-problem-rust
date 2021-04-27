// inspiration: https://web.eecs.umich.edu/~baveja/RLMasses/RL.html
mod elevator;
mod simulations;
mod strategies;
use crate::strategies::{AlwaysDown, UpAndDownLoop, Random};

use simulations::ResidentialFlats;

fn print_results(label: &str, result: (Vec<usize>, usize)) {
    let (wait_times, backlog) = result;
    println!("{}", label);
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

#[derive(Debug,Clone,PartialEq)]
struct Score {
    name: String,
    wait_score: i32,
    backlog_score: i32,
}
fn print_progress(scores: Vec<Score>) {
    const BAR : &str = "#######################################";

    let max_wait = scores.iter().map(|s| s.wait_score).max().unwrap();
    let max_backlog = scores.iter().map(|s| s.backlog_score).max().unwrap();

    let mut scores = scores.clone();
    scores.sort_by_key(|s| (s.backlog_score, s.wait_score));

    for score in &scores {
        let backlog_bar = (score.backlog_score as f32 / max_backlog as f32 * 30.0) as usize;
        let wait_bar = (score.wait_score as f32 / max_wait as f32 * 30.0) as usize;

        let backlog_graph = &BAR[0..backlog_bar];
        let wait_graph = &BAR[0..wait_bar];
        println!("{: <15} B:{: >8} {: <40} W:{: >8} {: <40}", score.name, score.backlog_score, backlog_graph, score.wait_score, wait_graph)
    }
    println!("");
}

fn main() {
    println!("The elevator Problem:");

    let steps = 100;
    let seed = 123123;

    // let mut sim1 = ResidentialFlats::<AlwaysDown>::new(seed, false);
    let mut sim2 = ResidentialFlats::<UpAndDownLoop>::new(seed, false);
    let mut sim3 = ResidentialFlats::<Random>::new(seed, false);

    for _ in 0..steps {
        // let sim1r = sim1.run(10);
        let sim2r = sim2.run(10);
        let sim3r = sim3.run(10);

        print_progress(vec![
            // Score{name: "AlwaysDown".into(), wait_score: sim1r.0.iter().map(|&x| x as i32).sum::<i32>(), backlog_score: sim1r.1 as i32},
            Score{name: "UpAndDownLoop".into(), wait_score: sim2r.0.iter().map(|&x| x as i32).sum::<i32>(), backlog_score: sim2r.1 as i32},
            Score{name: "Random".into(), wait_score: sim3r.0.iter().map(|&x| x as i32).sum::<i32>(), backlog_score: sim3r.1 as i32},
        ])
    }

    // print_results(ResidentialFlats::<AlwaysDown>::new(seed, false).run(steps));
    // println!("AlwaysDown (ResidentialFlats)");
    //

    // println!("UpAndDown (ResidentialFlats)");
    // print_results(ResidentialFlats::new(seed, false).run::<UpAndDownLoop>(steps));
}
