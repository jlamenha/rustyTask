use crate::models::TaskStatus;
use clap::{Parser, Subcommand};

#[derive(Parser, Debug)]
#[command(name = "Personal Project Manager")]
#[command(about = "Manage projects and tasks from the terminal")]
pub struct CLI {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    AddProject {
        name: String,
        description: String,
    },
    AddTask {
        project_id: u32,
        title: String,
        description: String,
        deadline: Option<String>,
        status: TaskStatus,
    },
    ListProjects,
    ListTasks {
        project_id: u32,
    },
}
