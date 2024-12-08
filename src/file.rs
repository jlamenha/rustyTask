use crate::models::Project;
use serde_json::{self};
use std::fs;

const FILE_NAME: &str = "projects.json";

pub fn save_projects(projects: &Vec<Project>) -> Result<(), String> {
    let data = serde_json::to_string_pretty(&projects).map_err(|e| e.to_string())?;
    fs::write(FILE_NAME, data).map_err(|e| e.to_string())?;
    Ok(())
}

pub fn load_projects() -> Result<Vec<Project>, String> {
    let data = fs::read_to_string(FILE_NAME).map_err(|e| e.to_string())?;
    serde_json::from_str(&data).map_err(|e| e.to_string())
}
