use std::path::Path;
use crate::storage;
use crate::storage::{Task, save_state};

pub fn create_task(base_dir: &Path, project_name: &str, task_description: &str) {
    let mut tedo_state = storage::load_state(base_dir).unwrap_or_default();
    let task = Task { description: task_description.into() };
    if let Some(project) = tedo_state.projects.iter_mut().find(|p| p.name == project_name) {
        project.tasks.push(task);
        save_state(base_dir, &tedo_state).expect("Failed to save task");
        println!("Task added to project {}!", project_name);
    } else {
        println!("Project with name {} does not exist", project_name);
    }
}

pub fn list_tasks(base_dir: &Path, project_name: &str) {
    let tedo_state = storage::load_state(base_dir).unwrap_or_default();
    if let Some(project) = tedo_state.projects.iter().find(|p| p.name == project_name) {
        println!("Tasks for project {}:", project_name);
        for task in &project.tasks {
            println!("{}", task.description);
        }
    } else {
        println!("Project with name {} does not exist", project_name);
    }
}

// Add other functions to manipulate tasks as needed

#[cfg(test)]
mod tests {
    use super::*;
    use crate::projects::create_project;
    use tempfile::tempdir;

    #[test]
    fn test_create_task() {
        let dir = tempdir().unwrap();
        let base_dir = dir.path();
        let project_name = "test_project";
        let task_description = "Write code";

        create_project(base_dir, project_name, false);
        create_task(base_dir, project_name, task_description);
        let tedo_state = storage::load_state(base_dir).unwrap();

        assert_eq!(tedo_state.projects.len(), 1);
        assert_eq!(tedo_state.projects[0].tasks.len(), 1);
        assert_eq!(tedo_state.projects[0].tasks[0].description, task_description);
    }

    #[test]
    fn test_create_task_nonexistent_project() {
        let dir = tempdir().unwrap();
        let base_dir = dir.path();
        let project_name = "nonexistent_project";
        let task_description = "Write code";

        create_task(base_dir, project_name, task_description);
        let tedo_state = storage::load_state(base_dir).unwrap_or_default();

        // There should be no projects or tasks since the project does not exist
        assert_eq!(tedo_state.projects.len(), 0);
    }

    #[test]
    fn test_list_tasks() {
        let dir = tempdir().unwrap();
        let base_dir = dir.path();
        let project_name = "test_project";
        let task_description1 = "Write code";
        let task_description2 = "Write tests";

        create_project(base_dir, project_name, false);
        create_task(base_dir, project_name, task_description1);
        create_task(base_dir, project_name, task_description2);
        let tedo_state = storage::load_state(base_dir).unwrap();

        assert_eq!(tedo_state.projects.len(), 1);
        assert_eq!(tedo_state.projects[0].tasks.len(), 2);
        assert_eq!(tedo_state.projects[0].tasks[0].description, task_description1);
        assert_eq!(tedo_state.projects[0].tasks[1].description, task_description2);
    }
}
