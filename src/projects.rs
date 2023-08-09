use std::path::Path;

use crate::storage;
use crate::storage::Project;
use crate::storage::save_state;

pub fn create_project(base_dir: &Path, name: &str, switch: bool) {
    let mut projects = storage::load_state(base_dir).unwrap_or_default();
    if projects.projects.iter().any(|p| p.name == name) {
        println!("Project with name {} already exists", name);
        return;
    }
    projects.projects.push(Project { name: name.into() });
    save_state(base_dir, &projects).expect("Failed to save projects");

    if switch {
        storage::set_current_project(base_dir, name);
    }
}


pub fn list_projects(base_dir: &Path, mode: &str) {
    let projects = storage::load_state(base_dir).unwrap_or_default();

    if mode == "table" {
        println!("+ {:^20} +", "------------------");
        println!("| {:^21} |", "Projects 📽️");
        println!("| {:^20} |", "------------------");
        for project in projects.projects {
            println!("| {:^20} |", project.name);
        }
        println!("+ {:^20} +", "------------------");
        return;
    }
    for project in projects.projects {
        println!("{}", project.name);
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