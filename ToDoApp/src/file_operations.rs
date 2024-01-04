use std::fs::{File};
use std::io::{self, BufRead, BufReader, Write};
use crate::task::Task;

pub fn read_tasks_from_file(file_path: &str) -> Vec<Task> {
    let mut tasks = Vec::new();

    if let Ok(file) = File::open(file_path) {
        let reader = BufReader::new(file);

        for line in reader.lines() {
            if let Ok(description) = line {
                let (description, completed) = parse_task_description(&description);
                tasks.push(Task::new(&description, completed));
            }
        }
    }

    tasks
}

pub fn parse_task_description(description: &str) -> (String, bool) {
    let mut iter = description.trim().rsplitn(2, ' ');

    if let Some(completed_str) = iter.next() {
        let description = iter.next().unwrap_or_default().trim().to_string();
        let completed = match completed_str.trim() {
            "X" => false,
            "$" => true,
            _ => false,
        };
        (description, completed)
    } else {
        (description.trim().to_string(), false)
    }
}

pub fn save_tasks_to_file(tasks: &[Task], file_path: &str) -> io::Result<()> {
    let mut file = File::create(file_path)?;

    for task in tasks {
        let status = if task.completed { "$" } else { "X" };
        writeln!(&mut file, "{} {}", task.description, status)?;
    }

    Ok(())
}