use serde::{Deserialize, Serialize};
use std::fmt;

#[derive(Serialize, Debug, Deserialize, PartialEq, Clone)]
pub struct Command {
    pub name: String,
    pub command: String,
    pub category: String,
}

impl fmt::Display for Command {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}: {}", self.name, self.command)
    }
}
