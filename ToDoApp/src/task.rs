// src/task.rs

pub struct Task {
    pub description: String,
    pub completed: bool,
}

impl Task {
    pub fn new(description: &str, completed: bool) -> Task {
        Task {
            description: String::from(description),
            completed,
        }
    }
}