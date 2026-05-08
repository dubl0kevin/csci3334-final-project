use crate::metrics::Metrics;
use crate::task::{Task, TaskType};

use std::collections::VecDeque;
use std::sync::{Arc, Mutex};
use std::sync::atomic::{AtomicBool, Ordering};
use std::thread;
use std::time::{Duration, Instant};

pub fn worker_loop(
    worker_id: usize,
    cpu_only: bool,
    cpu_queue: Arc<Mutex<VecDeque<Task>>>,
    io_queue: Arc<Mutex<VecDeque<Task>>>,
    metrics: Arc<Mutex<Metrics>>,
    shutdown: Arc<AtomicBool>,
) {
    loop {
        let mut task_opt = None;

        if cpu_only {
            let mut queue = cpu_queue.lock().unwrap();
            task_opt = queue.pop_front();
        } else {
            {
                let mut io_q = io_queue.lock().unwrap();
                task_opt = io_q.pop_front();
            }

            if task_opt.is_none() {
                let mut cpu_q = cpu_queue.lock().unwrap();
                task_opt = cpu_q.pop_front();
            }
        }

        match task_opt {
            Some(task) => {
                let start = Instant::now();

                let wait_time = start.duration_since(task.created_at);

                println!(
                    "Worker {} executing Task {} ({:?}) for {} ms",
                    worker_id,
                    task.id,
                    task.task_type,
                    task.duration_ms
                );

                thread::sleep(Duration::from_millis(task.duration_ms));

                let finish = Instant::now();
                let turnaround = finish.duration_since(task.created_at);

                let mut m = metrics.lock().unwrap();

                m.total_completed += 1;
                m.total_wait_time += wait_time;
                m.total_turnaround_time += turnaround;
                m.worker_busy_time[worker_id] += Duration::from_millis(task.duration_ms);

                if wait_time > m.max_wait_time {
                    m.max_wait_time = wait_time;
                }

                match task.task_type {
                    TaskType::Cpu => m.cpu_completed += 1,
                    TaskType::Io => m.io_completed += 1,
                }
            }
            None => {
                if shutdown.load(Ordering::SeqCst) {
                    break;
                }

                thread::sleep(Duration::from_millis(10));
            }
        }
    }

    println!("Worker {} shutting down", worker_id);
}