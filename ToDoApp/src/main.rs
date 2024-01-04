use std::fs::{self, File};
use std::io::{self, BufRead, BufReader, Write};

struct Task {
    description: String,
    completed: bool,
}

fn main() {
    let file_path = "tasks.txt";
    let mut tasks = read_tasks_from_file(file_path);

    loop {
        println!("\nWhat do you want to do?");
        println!("1. Show tasks");
        println!("2. Add task");
        println!("3. Mark task as completed");
        println!("4. Mark task as not completed");
        println!("5. Remove task");
        println!("6. Save tasks to file");
        println!("7. Quit");

        let mut choice = String::new();
        io::stdin().read_line(&mut choice).expect("Error reading input");

        match choice.trim().parse() {
            Ok(1) => print_tasks(&tasks),
            Ok(2) => {
                println!("Enter task description:");
                let mut description = String::new();
                io::stdin().read_line(&mut description).expect("Error reading input");
                add_task(&mut tasks, &description.trim());
            }
            Ok(3) => {
                println!("Enter task index to mark as completed:");
                let mut index_str = String::new();
                io::stdin().read_line(&mut index_str).expect("Error reading input");
                if let Ok(index) = index_str.trim().parse::<usize>() {
                    mark_as_completed(&mut tasks, index - 1);
                } else {
                    println!("Invalid input. Please enter a valid index.");
                }
            }
            Ok(4) => {
                println!("Enter task index to mark as not completed:");
                let mut index_str = String::new();
                io::stdin().read_line(&mut index_str).expect("Error reading input");
                if let Ok(index) = index_str.trim().parse::<usize>() {
                    mark_as_not_completed(&mut tasks, index - 1);
                } else {
                    println!("Invalid input. Please enter a valid index.");
                }
            }
            Ok(5) => {
                println!("Enter task index to remove:");
                let mut index_str = String::new();
                io::stdin().read_line(&mut index_str).expect("Error reading input");
                if let Ok(index) = index_str.trim().parse::<usize>() {
                    remove_task(&mut tasks, index - 1);
                } else {
                    println!("Invalid input. Please enter a valid index.");
                }
            }
            Ok(6) => {
                if let Err(err) = save_tasks_to_file(&tasks, file_path) {
                    eprintln!("Error saving tasks to file: {}", err);
                } else {
                    println!("Tasks saved to file: {}", file_path);
                }
            }
            Ok(7) => {
                println!("Goodbye!");
                break;
            }
            _ => println!("Invalid choice. Please try again."),
        }
    }
}


impl Task {
    fn new(description: &str) -> Task {
        Task {
            description: String::from(description),
            completed: false,
        }
    }
}

fn read_tasks_from_file(file_path: &str) -> Vec<Task> {
    let mut tasks = Vec::new();

    if let Ok(file) = File::open(file_path) {
        let reader = BufReader::new(file);

        for line in reader.lines() {
            if let Ok(description) = line {
                tasks.push(Task::new(&description));
            }
        }
    }

    tasks
}

fn print_tasks(tasks: &[Task]) {
    for (index, task) in tasks.iter().enumerate() {
        let status = if task.completed { "Done" } else { "Not Done" };
        println!("{}. [{}] {}", index + 1, status, task.description);
    }
}

fn add_task(tasks: &mut Vec<Task>, description: &str) {
    let new_task = Task::new(description);
    tasks.push(new_task);
    println!("Task added: {}", description);
}

fn mark_as_completed(tasks: &mut Vec<Task>, index: usize) {
    if let Some(task) = tasks.get_mut(index) {
        task.completed = true;
        println!("Task marked as completed: {}", task.description);
    } else {
        println!("Invalid task index");
    }
}

fn mark_as_not_completed(tasks: &mut Vec<Task>, index: usize) {
    if let Some(task) = tasks.get_mut(index) {
        task.completed = false;
        println!("Task marked as not completed: {}", task.description);
    } else {
        println!("Invalid task index");
    }
}

fn remove_task(tasks: &mut Vec<Task>, index: usize) {
    if index < tasks.len() {
        let removed_task = tasks.remove(index);
        println!("Task removed: {}", removed_task.description);
    } else {
        println!("Invalid task index");
    }
}

fn save_tasks_to_file(tasks: &[Task], file_path: &str) -> io::Result<()> {
    let mut file = File::create(file_path)?;

    for task in tasks {
        writeln!(&mut file, "{}", task.description)?;
    }

    Ok(())
}

