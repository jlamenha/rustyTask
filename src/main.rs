mod cli;
mod features;
mod file;
mod logic;
mod models;

use crate::models::TaskStatus;

use crate::features::{add_project, add_task, list_tasks, load_projects, save_projects};
use crate::models::{Project, Task};
use clap::Parser;
use cli::{Commands, CLI};

fn main() {
    let args = CLI::parse();
    let file_path = "projects.json";

    let mut projects = load_projects(file_path);

    match args.command {
        Commands::AddProject { name, description } => {
            println!("Adding project: {} - {}", name, description);

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
        Commands::AddTask {
            project_id,
            title,
            description,
            deadline,
            status,
        } => {
            println!(
                "Adding task: {} - {} to project {}",
                title, description, project_id
            );
            let new_task = Task {
                id: projects.iter().flat_map(|p| &p.tasks).count() as u32 + 1, // New ID
                title: title.clone(),
                description: description.clone(),
                deadline: None, // Optional for now
                status: status,
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
