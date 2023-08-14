use std::path::Path;
use crate::projects::current_project;
use crate::storage;

use colored::Colorize;


pub fn list(base_dir: &Path) {
    let project = current_project(base_dir).unwrap();
    let projects = storage::load_state(base_dir).unwrap();

    let total_tasks = projects.projects.iter().map(|p| p.tasks.len()).sum::<usize>();
    let total_notes = projects.projects.iter().map(|p| p.notes.len()).sum::<usize>();
    // We output in one single line:
    // - Current project
    // - # of projects
    // - total # of tasks
    // - total # of notes
    // Use bold text for the current project
    println!("{}", format!("Current project: {} \t # projects: {}  \t # tasks {} \t # notes {} ",
                format!("({}) {}", project.id , project.name.blue().bold()),
                projects.projects.len().to_string().blue().bold(),
                total_tasks.to_string().blue().bold(),
                total_notes.to_string().blue().bold()).white().bold()


             );
}