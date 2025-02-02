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
        println!("  delete <task>   Delete the task");
        println!("  list            Lists all tasks");
        println!("  done <number>   Marks a task as done");
        return;
    }

    let mut todo = Todo::load(file_path);

    match args[1].as_str() {
        "add" => {
            if args.len() < 3 {
                println!("Please provide a task to add.");
            } else {
                todo.add_task(file_path, args[2..].join(" "));
            }
        }
        "delete" => {
            if args.len() < 3 {
                println!("Please provide a task to delete.");
            }  else if let Ok(index) = args[2].parse::<usize>() {
                todo.delete_task(file_path, index);
            } else {
                println!("Please provide a valid task number.");
            }
        }
        "list" => {
            todo.list_task();
        },
        "done" => {
            if args.len() < 3 {
                println!("Please provide a task number to mark as done.");
            } else if let Ok(index) = args[2].parse::<usize>() {
                todo.mark_done(file_path, index);
            } else {
                println!("Please provide a valid task number.");
            }
        }
        _ => println!("Unknown command."),
    }
}