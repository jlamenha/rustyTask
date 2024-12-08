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
    AddProject,
    AddTask { project_id: u32 },
    ListProjects,
    ListTasks { project_id: u32 },
}
