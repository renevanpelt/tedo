use std::path::Path;

use crate::projects::current_project;
use crate::storage;
use crate::storage::{Project, save_state};
use crate::storage::Task;

use prettytable::row;


pub fn create_task(base_dir: &Path, description: &str) {
    let mut tedo_state = storage::load_state(base_dir).unwrap_or_default();
    let current_project_name = tedo_state.current_project.clone().unwrap_or_default();

    let project = tedo_state.projects.iter_mut().find(|p| p.name == current_project_name);

    if let Some(project) = project {
        let next_id = project.tasks.len() as u32 + 1;
        project.tasks.push(Task { id: next_id, description: description.into() });
        save_state(base_dir, &tedo_state).expect("Failed to save task");
    } else {
        println!("No selected project. Please switch to a project before creating a task.");
    }
}


impl Project {
    pub fn list_tasks(&self, mode: &str) {
        if mode == "table" {
            // println!("+ {:^10} + {:^40} +", "----------", "---------------------------------------");
            // println!("| {:^10} | {:^40} |", "ID", "Description");
            // println!("| {:^10} | {:^40} |", "----------", "---------------------------------------");
            // for task in &self.tasks {
            //     println!("| {:^10} | {:^40} |", task.id, task.description);
            // }
            // println!("+ {:^10} + {:^40} +", "----------", "---------------------------------------");

            // Use prettytable crate


            let mut table = prettytable::Table::new();
            table.add_row(row!["ID", "Description"]);
            for task in &self.tasks {
                table.add_row(row![task.id, task.description]);
            }
            table.printstd();


            return;
        }
        for task in &self.tasks {
            println!("{} {}", task.id, task.description);
        }
    }
}


pub fn list_tasks(base_dir: &Path, mode: &str) {

    let tedo_state = storage::load_state(base_dir).unwrap_or_default();
    let all_tasks: Vec<&Task> = tedo_state.projects.iter().flat_map(|p| &p.tasks).collect();
    println!("{} tasks found", all_tasks.len());
    println!("");
    // Display table or list of tasks including project identifier, name and shorthand

    if mode == "table" {
        let mut table = prettytable::Table::new();
        table.add_row(row!["ID", "Description", "Project"]);
        for task in all_tasks {
            let project = tedo_state.projects.iter().find(|p| p.tasks.contains(task)).unwrap();
            table.add_row(row![task.id, task.description, project.name]);
        }
        table.printstd();
        return;
    }

    for task in all_tasks {
        let project = tedo_state.projects.iter().find(|p| p.tasks.contains(task)).unwrap();
        println!("{}\t{}\t{}", project.name, task.id, task.description);
    }
}



#[cfg(test)]
mod tests {
    use tempfile::tempdir;

    use crate::projects::{create_project, switch_project};

    use super::*;

    #[test]
    fn test_create_task_in_selected_project() {
        let dir = tempdir().unwrap();
        let base_dir = dir.path();

        create_project(base_dir, "test_project", false);
        switch_project(base_dir, "test_project");

        create_task(base_dir, "test_task");
        let state = storage::load_state(base_dir).unwrap();

        assert_eq!(state.projects.len(), 1);
        assert_eq!(state.projects[0].name, "test_project");
        assert_eq!(state.projects[0].tasks.len(), 1);
        assert_eq!(state.projects[0].tasks[0].id, 1);
        assert_eq!(state.projects[0].tasks[0].description, "test_task");
    }

    #[test]
    fn test_create_task_without_selected_project() {
        let dir = tempdir().unwrap();
        let base_dir = dir.path();

        create_project(base_dir, "test_project", false);

        create_task(base_dir, "test_task");
        let state = storage::load_state(base_dir).unwrap();

        assert_eq!(state.projects.len(), 1);
        assert_eq!(state.projects[0].tasks.len(), 0); // No task should be added
    }

    #[test]
    fn test_create_multiple_tasks_in_selected_project() {
        let dir = tempdir().unwrap();
        let base_dir = dir.path();

        create_project(base_dir, "test_project", false);
        switch_project(base_dir, "test_project");

        create_task(base_dir, "test_task_1");
        create_task(base_dir, "test_task_2");
        let state = storage::load_state(base_dir).unwrap();
        assert_eq!(state.projects[0].tasks.len(), 2);
        assert_eq!(state.projects[0].tasks[0].description, "test_task_1");
        assert_eq!(state.projects[0].tasks[1].description, "test_task_2");
    }


    #[test]
    fn test_list_tasks() {
        let dir = tempdir().unwrap();
        let base_dir = dir.path();

        create_project(base_dir, "test_project", false);
        switch_project(base_dir, "test_project");

        create_task(base_dir, "test_task_1");
        create_task(base_dir, "test_task_2");
        create_task(base_dir, "test_task_3");
        let state = storage::load_state(base_dir).unwrap();

        assert_eq!(state.projects[0].tasks.len(), 3);
        assert_eq!(state.projects[0].tasks[0].description, "test_task_1");
        assert_eq!(state.projects[0].tasks[1].description, "test_task_2");
        assert_eq!(state.projects[0].tasks[2].description, "test_task_3");
    }
}
