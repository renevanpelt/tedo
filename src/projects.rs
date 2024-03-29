use std::path::Path;

use crate::storage;
use crate::storage::{Project, TedoState};
use crate::storage::save_state;

impl Project {
    pub fn find(base_dir: &Path, identifier: &str) -> Option<Project> {
        let state = storage::load_state(base_dir).unwrap_or_default();
        if identifier.parse::<u32>().is_ok() {
            return Project::find_by_id(state, identifier.parse::<u32>().unwrap());
        } else {
            return Project::find_by_name(state, identifier.to_string());
        }
    }


    pub fn find_by_id(state: TedoState, id: u32) -> Option<Project> {
        return state.projects.into_iter().find(|p| p.id == id);
    }

    pub fn find_by_name(state: TedoState, name_start: String) -> Option<Project> {
        return state.projects.into_iter().find(|p| p.name.starts_with(&name_start));
    }

}


pub fn current_project(base_dir: &Path) -> Option<Project> {
    let tedo_state = storage::load_state(base_dir).unwrap_or_default();
    let current_project_name = tedo_state.current_project.unwrap_or_default();
    return Project::find(base_dir, &current_project_name);
}


pub fn create_project(base_dir: &Path, name: &str, switch: bool) {
    let mut tedo_state = storage::load_state(base_dir).unwrap_or_default();
    if tedo_state.projects.iter().any(|p| p.name == name) {
        println!("Project with name {} already exists", name);
        return;
    }
    let project_id = tedo_state.projects.len() as u32 + 1;
    tedo_state.projects.push(Project { id: project_id, name: name.into(), tasks: Vec::new(), notes: Vec::new() });
    save_state(base_dir, &tedo_state).expect("Failed to save projects");

    if switch {
        switch_project(base_dir, name);
    }
}


pub fn switch_project(base_dir: &Path, name: &str) {
    let tedo_state = storage::load_state(base_dir).unwrap_or_default();

    if tedo_state.projects.iter().any(|p| p.name == name) {
        println!("Switching to project {}", name);
        storage::set_current_project(base_dir, name);
    } else {
        println!("Project with name {} does not exist", name);
    }
}


pub fn list_projects(base_dir: &Path, mode: &str) {
    let projects = storage::load_state(base_dir).unwrap_or_default();

    let current_project = current_project(base_dir);

    if mode == "table" {
        println!("+ {:^21} + {:^20}  + {:^20} +", "------------------", "----------", "----------");
        println!("| {:^21} + {:^20}  + {:^20} |", "Projects", "Tasks", "Notes");
        println!("+ {:^21} + {:^20}  + {:^20} +", "------------------", "----------", "----------");
        for project in projects.projects {
            if let Some(current_project) = &current_project {
                if project.name == current_project.name {
                    println!("| {:^21} + {:^20}  + {:^20} |", format!("{} (current)", project.name), project.tasks.len(), project.notes.len());
                } else {
                    println!("| {:^21} + {:^20}  + {:^20} |", project.name, project.tasks.len(), project.notes.len());
                }
            }
        }
        println!("+ {:^21} + {:^20}  + {:^20} +", "------------------", "----------", "----------");
        return;
    }
    for project in projects.projects {
        println!("({}) {}", project.id, project.name);
    }
}


#[cfg(test)]
mod tests {
    use tempfile::tempdir;

    use super::*;

    #[test]
    fn test_create_project() {
        let dir = tempdir().unwrap();
        let base_dir = dir.path();

        create_project(base_dir, "test_project", false);
        let projects = storage::load_state(base_dir).unwrap();
        assert_eq!(projects.projects.len(), 1);
        assert_eq!(projects.projects[0].name, "test_project");
    }

    #[test]
    fn test_create_project_with_existing_name() {
        let dir = tempdir().unwrap();
        let base_dir = dir.path();

        create_project(base_dir, "test_project", false);
        create_project(base_dir, "test_project", false);
        let projects = storage::load_state(base_dir).unwrap();
        assert_eq!(projects.projects.len(), 1);
        assert_eq!(projects.projects[0].name, "test_project");
    }

    #[test]
    fn test_create_project_and_switch() {
        let dir = tempdir().unwrap();
        let base_dir = dir.path();

        create_project(base_dir, "test_project", true);
        let projects = storage::load_state(base_dir).unwrap();
        assert_eq!(projects.projects.len(), 1);
        assert_eq!(projects.projects[0].name, "test_project");
        assert_eq!(projects.current_project.unwrap(), "test_project");
    }

    #[test]
    fn test_list_projects() {
        let dir = tempdir().unwrap();
        let base_dir = dir.path();

        create_project(base_dir, "test_project_1", false);
        create_project(base_dir, "test_project_2", false);
        create_project(base_dir, "test_project_3", false);
        let projects = storage::load_state(base_dir).unwrap();
        assert_eq!(projects.projects.len(), 3);
        assert_eq!(projects.projects[0].name, "test_project_1");
        assert_eq!(projects.projects[1].name, "test_project_2");
        assert_eq!(projects.projects[2].name, "test_project_3");
    }
}

