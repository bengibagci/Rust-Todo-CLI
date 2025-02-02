use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct TodoItem {
    pub task: String,
    pub done: bool,
}