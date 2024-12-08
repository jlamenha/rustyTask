use crate::file::{load_projects, save_projects};
use crate::models::Project;

pub fn add_project(name: String, description: String) -> Result<(), String> {
    let mut projects = load_projects().unwrap_or_else(|_| vec![]);
    let new_project = Project {
        id: (projects.len() as u32) + 1,
        name,
        description,
        tasks: vec![],
    };
    projects.push(new_project);
    save_projects(&projects)
}
