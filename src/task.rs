use std::time::Instant;

#[derive(Clone, Debug)]
pub enum TaskType {
    Cpu, 
    Io
}

# [derive(Clone, Debug)]
pub struct Task {
    pub id: usize, 
    pub task_type: TaskType, 
    pub duration_ms: u64,
    pub created_at: Instant,
}

impl Task {
    pub fn new(id: usize, task_type: TaskType, duration_ms: u64) -> Self {
        Self {
            id,
            task_type,
            duration_ms,
            created_at: Instant::now(),
        }
    }
}