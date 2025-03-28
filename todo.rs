use std::fs::{OpenOptions, read_to_string};
use std::io::{Write, stdin};

const FILE_NAME: &str = "tasks.txt";

fn main() {
    loop {
        println!("\n1. Add Task");
        println!("2. View Tasks");
        println!("3. Exit");
        print!("Choose an option: ");

        let mut choice = String::new();
        stdin().read_line(&mut choice).expect("Failed to read input");

        match choice.trim() {
            "1" => add_task(),
            "2" => view_tasks(),
            "3" => {
                println!("Goodbye!");
                break;
            }
            _ => println!("Invalid option! Try again."),
        }
    }
}

fn add_task() {
    print!("Enter a new task: ");
    let mut task = String::new();
    stdin().read_line(&mut task).expect("Failed to read input");

    let mut file = OpenOptions::new().append(true).create(true).open(FILE_NAME).expect("Failed to open file");
    writeln!(file, "{}", task.trim()).expect("Failed to write to file");

    println!("Task added!");
}

fn view_tasks() {
    match read_to_string(FILE_NAME) {
        Ok(contents) => {
            if contents.is_empty() {
                println!("No tasks yet!");
            } else {
                println!("\nYour Tasks:");
                for (i, task) in contents.lines().enumerate() {
                    println!("{}. {}", i + 1, task);
                }
            }
        }
        Err(_) => println!("No tasks found!"),
    }
}
