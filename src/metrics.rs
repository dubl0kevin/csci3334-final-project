use std::time::Duration;

pub struct Metrics {
    pub total_completed: usize,
    pub cpu_completed: usize,
    pub io_completed: usize,

    pub total_wait_time: Duration,
    pub total_turnaround_time: Duration,

    pub max_wait_time: Duration,

    pub worker_busy_time: Vec<Duration>
}

impl Metrics {
    pub fn new(worker_count: usize) -> Self {
        Self {
            total_completed: 0,
            cpu_completed: 0,
            io_completed: 0,
            total_wait_time: Duration::ZERO,
            total_turnaround_time: Duration::ZERO,
            max_wait_time: Duration::ZERO,
            worker_busy_time: vec![Duration::ZERO; worker_count],
        }
    }

    pub fn print_summary(&self, makespan: Duration) {
        println!("\n===== FINAL METRICS =====");

        println!("Total tasks completed: {}", self.total_completed);
        println!("CPU tasks completed: {}", self.cpu_completed);
        println!("IO tasks completed: {}", self.io_completed);

        println!("Maskespan: {:.2?}", makespan);

        if self.total_completed > 0 {
            let avg_wait = self.total_wait_time / self.total_completed as u32;
            let avg_turnaround = self.total_turnaround_time / self.total_completed as u32;

            println!("Average wait time: {:.2?}", avg_wait);
            println!("Average turnaround time: {:.2?}", avg_turnaround);
        }

        println!("Max wait time: {:.2?}", self.max_wait_time);

        for (i, busy) in self.worker_busy_time.iter().enumerate() {
            let utilization = busy.as_secs_f64() / makespan.as_secs_f64() * 100.0;
            println!("Worker {} utilization: {:.2}%", i, utilization);
        }
    }
}