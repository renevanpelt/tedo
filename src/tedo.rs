use std::path::Path;
use crate::projects::current_project;
use crate::storage;

use prettytable::row;
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

    // List all projects in a table

    let mut table = prettytable::Table::new();

    // define a list 'shorthands' that contains the shortest possible unique
    // letter combination to identify each project in the list

    let mut shorthands: Vec<String> = Vec::new();
    for project in projects.projects.iter() {
        let mut shorthand = String::new();
        for c in project.name.chars() {
            shorthand.push(c);
            if !shorthands.contains(&shorthand) {
                shorthands.push(shorthand.clone());
                break;
            }
        }
    }

    table.add_row(row!["ID", "Project Name", "Tasks", "Notes"]);
    for project in projects.projects {
        // table.add_row(row![project.id, project.name, project.tasks.len(), project.notes.len()]);
        // include the shorthand
        let mut project_name = format!("({}) {}", shorthands.remove(0), project.name).white();
        // make bold and white if current project

        if project.id == current_project(base_dir).unwrap().id {
            project_name = project_name.white().bold()
        }

        table.add_row(row![project.id, project_name, project.tasks.len(), project.notes.len()]);
    }
    table.printstd();
}