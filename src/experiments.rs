pub struct ExperimentConfig {
    pub name: String, 
    pub total_tasks: usize,
    pub cpu_ratio: f64,
    pub cpu_min: u64,
    pub cpu_max: u64,
    pub io_min: u64, 
    pub io_max: u64,
}

pub fn balanced_workload() -> ExperimentConfig {
    ExperimentConfig {
        name: "Balanced Workload". to_string(),
        total_tasks: 500,
        cpu_ratio: 0.5,
        cpu_min: 300,
        cpu_max: 700,
        io_min: 50,
        io_max: 200,
    }
}

pub fn stressed_workload() -> ExperimentConfig {
    ExperimentConfig {
        name: "Stressed Workload".to_string(),
        total_tasks: 750,
        cpu_ratio: 0.85,
        cpu_min: 800,
        cpu_max: 1500,
        io_min: 50,
        io_max: 100,
    }
}