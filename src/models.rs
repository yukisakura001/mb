use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Category {
    pub name: String,
    pub commands: Vec<CommandEntry>,
}

#[derive(Serialize, Deserialize)]
pub struct CommandEntry {
    pub name: String,
    pub run: String,
    pub description: String,
}
