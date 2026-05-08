# CSCI 3334 Final Project

### Project Summary
This project is a multithreaded task scheduling simulator written in Rust. I models a small runtime system capable of executing CPU-bound and IO-bound workloads using a bounded worker pool and queue-based scheduling.

The system uses:

- concurrent worker threads
- CPU and IO task queues
- FIFO scheduling
- reserved CPU workers
- metrics collection
- clean shutdown


## Build Instructions

### Build
```bash
cargo build
```

### Run
```bash
cargo run -- balanced
cargo run -- stressed
```

## Tool Use Disclosure
ChatGPT was used for brainstorming architecture ideas and reviewing Rust concurrency patterns.
- One accepted idea ChatGPT suggested was separating CPU and IO queues for clearer scheduling behavior.
- One rejected idea ChatGPT suggested was spawning a thread per task becuase it violated the bounded worker pool requirement.