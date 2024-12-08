# project-manager
Rust-based CLI project manager application 


This project is a Rust learning project for me!

I tried to replicate a project manager CLI interface using Rust:

To add a project:
cargo run -- add-project

To add a task:
cargo run -- add-task <projectID>

To see all projects:
cargo run -- list-projects

To see all tasks in a project:
cargo run -- list-tasks <projectID>

Make sure you have a projects.json file at all times (otherwise this WILL NOT RUN)
