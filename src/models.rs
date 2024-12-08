//use chrono::{DateTime, Utc};
use clap::ValueEnum;
use serde::{Deserialize, Serialize};
use std::fmt;
use std::str::FromStr;

#[derive(Serialize, Deserialize, Debug)]
pub struct Task {
    pub id: u32,
    pub title: String,
    pub description: String,
    //pub deadline: Option<DateTime<Utc>>,
    pub status: TaskStatus,
}

#[derive(Serialize, Deserialize, Debug, Clone, ValueEnum)]
pub enum TaskStatus {
    Pending,
    InProgress,
    Completed,
}

impl fmt::Display for TaskStatus {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            TaskStatus::Pending => write!(f, "Pending"),
            TaskStatus::InProgress => write!(f, "In Progress"),
            TaskStatus::Completed => write!(f, "Completed"),
        }
    }
}

impl FromStr for TaskStatus {
    type Err = String;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        match input.to_lowercase().as_str() {
            "pending" | "p" => Ok(TaskStatus::Pending),
            "inprogress" | "in_progress" | "ip" => Ok(TaskStatus::InProgress),
            "completed" | "c" => Ok(TaskStatus::Completed),
            _ => Err(format!("Invalid Task Status: {}", input)),
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Project {
    pub id: u32,
    pub name: String,
    pub description: String,
    pub tasks: Vec<Task>,
}
