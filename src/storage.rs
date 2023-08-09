use std::fs::{File, OpenOptions};
use std::io::{Read, Write};
use std::path::{Path, PathBuf};
use serde::{Serialize, Deserialize};
use toml;

#[derive(Serialize, Deserialize, Default)]
pub struct Projects {
    current_project: Option<String>,
    pub(crate) projects: Vec<Project>,
}

#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub struct Project {
    pub name: String,
}


pub fn load_projects(base_dir: &Path) -> Result<Projects, Box<dyn std::error::Error>> {
    let path = get_projects_path(base_dir);
    let mut file = File::open(&path)?;
    let mut data = String::new();
    file.read_to_string(&mut data)?;
    let projects: Projects = toml::from_str(&data)?;
    Ok(projects)
}


pub fn save_projects(base_dir: &Path, projects: &Projects) -> Result<(), Box<dyn std::error::Error>> {
    let path = get_projects_path(base_dir);
    let mut file = OpenOptions::new()
        .write(true)
        .create(true)
        .open(&path)?;
    let toml = toml::to_string(projects)?;
    file.write_all(toml.as_bytes())?;
    Ok(())
}

pub fn set_current_project(name: &str) {
    // logic to update the current project
}

fn get_projects_path(base_dir: &Path) -> PathBuf {
    base_dir.join("projects.toml")
}


#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::tempdir;

    #[test]
    fn test_save_and_load_projects() -> Result<(), Box<dyn std::error::Error>> {
        // Create a temporary directory.
        let dir = tempdir()?;
        let base_dir = dir.path();

        // Create a Projects object and save it.
        let projects = Projects {
            current_project: Some("test_project".into()),
            projects: vec![Project { name: "test".into() }],
        };
        save_projects(base_dir, &projects)?;

        // Load the Projects object and check that it's the same as what was saved.
        let loaded_projects = load_projects(base_dir)?;
        assert_eq!(projects.current_project, loaded_projects.current_project);
        assert_eq!(projects.projects, loaded_projects.projects);

        // Clean up.
        dir.close()?;

        Ok(())
    }
}
