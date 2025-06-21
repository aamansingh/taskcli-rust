use std::fs::{File, OpenOptions};
use std::io::{self, BufReader, Write};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
struct Task {
    description: String,
    completed: bool,
}

fn load_tasks() -> Vec<Task> {
    let file = File::open("tasks.json").unwrap_or_else(|_| File::create("tasks.json").unwrap());
    let reader = BufReader::new(file);
    serde_json::from_reader(reader).unwrap_or_else(|_| vec![])
}

fn save_tasks(tasks: &Vec<Task>) {
    let json = serde_json::to_string_pretty(tasks).unwrap();
    let mut file = File::create("tasks.json").unwrap();
    file.write_all(json.as_bytes()).unwrap();
}

fn main() {
    let mut tasks = load_tasks();

    println!("Welcome to Rusty Todo!");
    println!("1. Add task\n2. List tasks\n3. Complete task\nEnter choice: ");

    let mut choice = String::new();
    io::stdin().read_line(&mut choice).unwrap();

    match choice.trim() {
        "1" => {
            println!("Enter task description:");
            let mut desc = String::new();
            io::stdin().read_line(&mut desc).unwrap();
            tasks.push(Task {
                description: desc.trim().to_string(),
                completed: false,
            });
            save_tasks(&tasks);
            println!("Task added!");
        }
        "2" => {
            for (i, task) in tasks.iter().enumerate() {
                println!(
                    "{}. [{}] {}",
                    i + 1,
                    if task.completed { "x" } else { " " },
                    task.description
                );
            }
        }
        "3" => {
            println!("Enter task number to mark complete:");
            let mut num = String::new();
            io::stdin().read_line(&mut num).unwrap();
            if let Ok(i) = num.trim().parse::<usize>() {
                if i > 0 && i <= tasks.len() {
                    tasks[i - 1].completed = true;
                    save_tasks(&tasks);
                    println!("Task marked as complete!");
                } else {
                    println!("Invalid number.");
                }
            }
        }
        _ => println!("Invalid option."),
    }
}
