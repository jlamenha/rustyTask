use crate::models::{Project, Task};
use serde_json;
use std::fs;
use std::io::Read;

pub fn load_projects(file_path: &str) -> Vec<Project> {
    let mut file = fs::File::open(file_path).expect("Failed to load projects");
    let mut contents = String::new();
    file.read_to_string(&mut contents)
        .expect("Failed to read project file");

    serde_json::from_str(&contents).unwrap_or_else(|_| Vec::new())
}

pub fn save_projects(file_path: &str, projects: &[Project]) {
    let json_data = serde_json::to_string_pretty(projects).expect("Failed to serialize projects");
    fs::write(file_path, json_data).expect("Failed to write file");
}

pub fn add_project(projects: &mut Vec<Project>, project: Project) {
    projects.push(project);
}

pub fn add_task(projects: &mut Vec<Project>, project_id: u32, task: Task) {
    if let Some(project) = projects.iter_mut().find(|p| p.id == project_id) {
        project.tasks.push(task);
    }
}

pub fn list_tasks(projects: &[Project], project_id: u32) {
    if let Some(project) = projects.iter().find(|p| p.id == project_id) {
        if project.tasks.is_empty() {
            println!("No tasks found for project '{}'", project.name);
        } else {
            for task in &project.tasks {
                println!("Task ID: {}", task.id);
                println!("\nTitle: {}", task.title);
                println!("\nDescription: {}", task.description);
                println!("\nStatus: {}", task.status);
                //if let Some(deadline) = task.deadline {
                //    println!("\n Deadline: {}", deadline);
                //}
                println!();
            }
        }
    } else {
        println!("Project not found.");
    }
}
