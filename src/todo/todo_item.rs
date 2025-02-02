use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct TodoItem {
    pub(crate) task: String,
    pub(crate) done: bool,
}