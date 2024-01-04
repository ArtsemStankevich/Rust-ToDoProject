use crate::task::Task;

pub fn print_tasks(tasks: &[Task]) {
    for (index, task) in tasks.iter().enumerate() {
        let status = if task.completed { "$" } else { "X" };
        println!("{}. [{}] {}", index + 1, status, task.description);
    }
}

pub fn add_task(tasks: &mut Vec<Task>, description: &str) {
    let new_task = Task::new(description, false);
    tasks.push(new_task);
    println!("Task added: {}", description);
}

pub fn mark_as_completed(tasks: &mut Vec<Task>, index: usize) {
    if let Some(task) = tasks.get_mut(index) {
        task.completed = true;
        println!("Task marked as completed: {}", task.description);
    } else {
        println!("Invalid task index");
    }
}

pub fn mark_as_not_completed(tasks: &mut Vec<Task>, index: usize) {
    if let Some(task) = tasks.get_mut(index) {
        task.completed = false;
        println!("Task marked as not completed: {}", task.description);
    } else {
        println!("Invalid task index");
    }
}

pub fn remove_task(tasks: &mut Vec<Task>, index: usize) {
    if index < tasks.len() {
        let removed_task = tasks.remove(index);
        println!("Task removed: {}", removed_task.description);
    } else {
        println!("Invalid task index");
    }
}