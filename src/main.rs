mod todo;

use std::env;
use crate::todo::todo::Todo;

fn main() {
    let args: Vec<String> = env::args().collect();
    let file_path = "todo_list.json";

    if args.len() < 2 {
        println!("Usage: todo <command> [arguments]");
        println!("Commands:");
        println!("  add <task>      Add a new task");
        println!("  list            Lists all tasks");
        println!("  done <number>   Marks a task as done");
    }

    match args[1].as_str() {
        "add" => {
            if args.len() < 3 {
                println!("Please provide a task to add.");
            } else {
                let mut todo = Todo::new();
                todo.add_task(file_path, args[2..].join(" "));
            }
        }
        "list" => {
            let mut todo = Todo::new();
            todo.list_task(file_path);
        },
        "done" => {
            if args.len() < 3 {
                println!("Please provide a task number to mark as done.");
            } else if let Ok(index) = args[2].parse::<usize>() {
                let mut todo = Todo::new();
                todo.mark_done(file_path, index);
            } else {
                println!("Please provide a valid task number.");
            }
        }
        _ => println!("Unknown command."),
    }
}