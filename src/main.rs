mod experiments;
mod metrics;
mod scheduler;
mod task;
mod worker;

use experiments::{balanced_workload, stressed_workload};
use scheduler::run_scheduler;

fn main() {
    let args: Vec<String> = std::env::args().collect();

    if args.len() < 2 {
        println!("Usage:");
        println!("cargo run -- balanced");
        println!("cargo run -- stressed");
        return;
    }

    match args[1].as_str() {
        "balanced" => run_scheduler(balanced_workload()),
        "stressed" => run_scheduler(stressed_workload()),
        _ => {
            println!("Unknown experiment");
        }
    }
}