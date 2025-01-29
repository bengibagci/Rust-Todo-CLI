use std::fs::{OpenOptions, read_to_string};
use std::io::{Write};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]

pub struct Todo {
    task: String,
    done: bool,
}

type TodoList = Vec<Todo>;

impl Todo {
    pub fn new() -> Todo {
        Todo {
            task: String::new(),
            done: false,
        }
    }

    fn load_todos(file_path: &str) -> TodoList {
        match read_to_string(file_path) {
            Ok(data) => serde_json::from_str(&data).unwrap_or_else(|_| Vec::new()),
            Err(_) => Vec::new(),
        }
    }

    fn save_todos(file_path: &str, todos: &TodoList) {
        let data = serde_json::to_string_pretty(todos).expect("Failed to serialize todos");
        let mut file = OpenOptions::new()
            .write(true)
            .create(true)
            .truncate(true)
            .open(file_path)
            .expect("Failed to open file");
        file.write_all(data.as_bytes())
            .expect("Failed to write to file");
    }

    pub fn add_task(&mut self, file_path: &str, task: String) {
        let mut todos = Self::load_todos(file_path);
        todos.push(Todo {
            task,
            done: false,
        });
        Self::save_todos(file_path, &todos);
    }

    pub fn list_task(&mut self, file_path: &str) {
        let todos = Self::load_todos(file_path);
        if todos.is_empty() {
            println!("No tasks found.");
        } else {
            for (i, todo) in todos.iter().enumerate() {
                println!("{}. [{}] {}", i + 1, if todo.done { "x" } else { " " }, todo.task);
            }
        }
    }

    pub fn mark_done(&mut self, file_path: &str, index: usize) {
        let mut todos = Self::load_todos(file_path);
        if index == 0 || index > todos.len() {
            println!("Invalid task number.");
            return;
        }
        todos[index - 1].done = true;
        Self::save_todos(file_path, &todos);
        println!("Task marked as done.");
    }
}