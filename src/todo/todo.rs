use std::fs::{OpenOptions, read_to_string};
use std::io::{Write};
use serde::{Deserialize, Serialize};
use crate::todo::todo_item::TodoItem;

#[derive(Serialize, Deserialize, Debug)]
pub(crate) struct Todo {
    tasks: Vec<TodoItem>,
}

impl Todo {
    pub fn new() -> Self {
        Self { tasks: Vec::new() }
    }

    pub fn load(file_path: &str) -> Self {
        match read_to_string(file_path) {
            Ok(data) => serde_json::from_str(&data).unwrap_or_else(|_| Self::new()),
            Err(_) => Self::new(),
        }
    }

    fn save(&self, file_path: &str) {
        let data = serde_json::to_string_pretty(&self).expect("Failed to serialize todos");
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
        self.tasks.push(TodoItem {
            task,
            done: false,
        });
        self.save(file_path);
    }

    pub fn delete_task(&mut self, file_path: &str, index: usize) {
        if self.tasks.is_empty() {
            println!("Task list is empty. Nothing to delete.");
            return;
        }
        if index == 0 || index > self.tasks.len() {
            println!("Invalid task number. Please enter a number between 1 and {}.",  self.tasks.len());
            return;
        }
        self.tasks.remove(index - 1);
        self.save(file_path);
        println!("Task {} deleted successfully.", index);
    }

    pub fn list_task(&mut self) {
        if self.tasks.is_empty() {
            println!("No tasks found.");
        } else {
            for (i, item) in self.tasks.iter().enumerate() {
                println!("{}. [{}] {}", i + 1, if item.done { "x" } else { " " }, item.task);
            }
        }
    }

    pub fn mark_done(&mut self, file_path: &str, index: usize) {
        if index == 0 || index > self.tasks.len() {
            println!("Invalid task number.");
            return;
        }
        self.tasks[index - 1].done = true;
        self.save(file_path);
        println!("Task marked as done.");
    }
}