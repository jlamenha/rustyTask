mod cli;
mod features;
mod models;

use crate::features::{add_project, add_task, list_tasks, load_projects, save_projects};
use crate::models::{Project, Task, TaskStatus};
use clap::Parser;
use cli::{Commands, CLI};
use std::io::{self, Write};

fn main() {
    let args = CLI::parse();
    let file_path = "projects.json";

    let mut projects = load_projects(file_path);

    match args.command {
        Commands::AddProject => {
            println!("Enter the name of the project:");
            io::stdout().flush().unwrap();
            let mut name = String::new();
            io::stdin().read_line(&mut name).unwrap();
            let name = name.trim().to_string();

            println!("Enter the description of the project:");
            io::stdout().flush().unwrap();
            let mut description = String::new();
            io::stdin().read_line(&mut description).unwrap();
            let description = description.trim().to_string();

            let new_project = Project {
                id: projects.len() as u32 + 1, //Create new ID for each Project
                name: name.clone(),
                description: description.clone(),
                tasks: Vec::new(),
            };
            add_project(&mut projects, new_project);
            save_projects(file_path, &projects);
            println!("Project '{}' added successfully.", name);
        }
        Commands::AddTask { project_id } => {
            println!("Enter task title:");
            io::stdout().flush().unwrap();
            let mut title = String::new();
            io::stdin().read_line(&mut title).unwrap();
            let title = title.trim().to_string();

            println!("Enter task description:");
            io::stdout().flush().unwrap();
            let mut description = String::new();
            io::stdin().read_line(&mut description).unwrap();
            let description = description.trim().to_string();

            println!("Enter task status (Pending, InProgress, Completed):");

            io::stdout().flush().unwrap();
            let mut status = String::new();
            io::stdin().read_line(&mut status).unwrap();
            let status = status.trim().to_string();

            let status = match status.to_lowercase().as_str() {
                "pending" => TaskStatus::Pending,
                "inprogress" | "in_progress" => TaskStatus::InProgress,
                "completed" => TaskStatus::Completed,
                _ => {
                    println!("Invalid status. Defaulting to Pending.");
                    TaskStatus::Pending
                }
            };

            let new_task = Task {
                id: projects.iter().flat_map(|p| &p.tasks).count() as u32 + 1, // New ID
                title: title.clone(),
                status: status,
                description: description.clone(),
            };
            add_task(&mut projects, project_id, new_task);
            save_projects(file_path, &projects);
            println!("Task '{}' added to project {}.", title, project_id);
        }
        Commands::ListProjects => {
            println!("Listing projects...");

            if projects.is_empty() {
                println!("No projects found.");
            } else {
                for project in &projects {
                    println!("Project ID: {}", project.id);
                    println!("  Name: {}", project.name);
                    println!("  Description: {}", project.description);
                    println!("  Number of Tasks: {}", project.tasks.len());
                    println!();
                }
            }
        }
        Commands::ListTasks { project_id } => {
            println!("Listing tasks for project {}", project_id);
            list_tasks(&projects, project_id);
        }
    }
}
