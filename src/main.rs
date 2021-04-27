// inspiration: https://web.eecs.umich.edu/~baveja/RLMasses/RL.html
mod elevator;
mod simulations;
mod strategies;
use crate::strategies::{Random, UpAndDownLoop};
use simulations::ResidentialFlats;

#[derive(Debug, Clone, PartialEq)]
struct Score {
    name: String,
    wait_score: i32,
    backlog_score: i32,
}
fn print_progress(scores: Vec<Score>) {
    let max_wait = scores.iter().map(|s| s.wait_score).max().unwrap();
    let max_backlog = scores.iter().map(|s| s.backlog_score).max().unwrap();

    let mut scores = scores.clone();
    scores.sort_by_key(|s| (s.backlog_score, s.wait_score));

    for score in &scores {
        let backlog_graph = render_graph(score.backlog_score, max_backlog);
        let wait_graph = render_graph(score.wait_score, max_wait);
        println!(
            "{: <15} B:{: >8} {: <40} W:{: >8} {: <40}",
            score.name, score.backlog_score, backlog_graph, score.wait_score, wait_graph
        )
    }
    println!("");
}

fn render_graph(progress: i32, max: i32) -> String {
    const BAR: &str = "#######################################";
    let bar_width = (progress as f32 / max as f32 * 30.0) as usize;
    return BAR[0..bar_width].into();
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
            Score {
                name: "UpAndDownLoop".into(),
                wait_score: sim2r.0.iter().map(|&x| x as i32).sum::<i32>(),
                backlog_score: sim2r.1 as i32,
            },
            Score {
                name: "Random".into(),
                wait_score: sim3r.0.iter().map(|&x| x as i32).sum::<i32>(),
                backlog_score: sim3r.1 as i32,
            },
        ]);

        use std::{thread, time};
        let wait_ms = time::Duration::from_millis(200);
        thread::sleep(wait_ms);
    }
}
