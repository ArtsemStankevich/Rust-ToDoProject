mod task;
mod file_operations;
mod user_interface;

use std::io;
use file_operations::{read_tasks_from_file, save_tasks_to_file};
use user_interface::{print_tasks, add_task, mark_as_completed, mark_as_not_completed, remove_task};

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



