use crate::experiments::ExperimentConfig;
use crate::metrics::Metrics;
use crate::task::{Task, TaskType};
use crate::worker::worker_loop;

use rand::Rng;

use std::collections::VecDeque;
use std::sync::{Arc, Mutex};
use std::sync::atomic::{AtomicBool, Ordering};
use std::thread;
use std::time::{Duration, Instant};

pub fn run_scheduler(config: ExperimentConfig) {
    println!("\n===== {} =====", config.name);

    let worker_count = 6;

    let cpu_queue = Arc::new(Mutex::new(VecDeque::<Task>::new()));
    let io_queue = Arc::new(Mutex::new(VecDeque::<Task>::new()));

    let metrics = Arc::new(Mutex::new(Metrics::new(worker_count)));

    let shutdown = Arc::new(AtomicBool::new(false));

    let start_time = Instant::now();

    let mut workers = vec![];

    for id in 0..worker_count {
        let cpu_q = Arc::clone(&cpu_queue);
        let io_q = Arc::clone(&io_queue);
        let metrics_clone = Arc::clone(&metrics);
        let shutdown_clone = Arc::clone(&shutdown);

        let cpu_only = id < 2;

        let handle = thread::spawn(move || {
            worker_loop(
                id,
                cpu_only,
                cpu_q,
                io_q,
                metrics_clone,
                shutdown_clone,
            )
        });

        workers.push(handle);
    }

    let cpu_queue_gen = Arc::clone(&cpu_queue);
    let io_queue_gen = Arc::clone(&io_queue);

    let generator = thread::spawn(move || {
        let mut rng = rand::thread_rng();

        for id in 0..config.total_tasks {
            let cpu_task = rng.gen_bool(config.cpu_ratio);

            let (task_type, duration) = if cpu_task {
                (
                    TaskType::Cpu,
                    rng.gen_range(config.cpu_min..=config.cpu_max),
                )
            } else {
                (
                    TaskType::Io,
                    rng.gen_range(config.io_min..=config.io_max),
                )
            };

            let task = Task::new(id, task_type.clone(), duration);

            match task_type {
                TaskType::Cpu => {
                    cpu_queue_gen.lock().unwrap().push_back(task);
                }
                TaskType::Io => {
                    io_queue_gen.lock().unwrap().push_back(task);
                }
            }

            thread::sleep(Duration::from_millis(20));
        }
    });

    generator.join().unwrap();

    loop {
        let cpu_empty = cpu_queue.lock().unwrap().is_empty();
        let io_empty = io_queue.lock().unwrap().is_empty();

        let completed = metrics.lock().unwrap().total_completed;

        if cpu_empty && io_empty && completed >= config.total_tasks {
            break;
        }

        thread::sleep(Duration::from_millis(50));
    }

    shutdown.store(true, Ordering::SeqCst);

    for worker in workers {
        worker.join().unwrap();
    }

    let makespan = start_time.elapsed();

    metrics.lock().unwrap().print_summary(makespan);
}
