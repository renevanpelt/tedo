use std::fs::{File, OpenOptions};
use std::io::{Read, Write};
use std::path::{Path, PathBuf};
use serde::{Serialize, Deserialize};
use toml;

#[derive(Serialize, Deserialize, Default, Debug)]
pub struct TedoState {
    pub(crate) current_project: Option<String>,
    pub(crate) projects: Vec<Project>,
    // pub(crate) editor: Option<String>,

}

#[derive(Serialize, Deserialize, Debug)]
pub struct Project {
    pub name: String,
    pub tasks: Vec<Task>,
    pub notes: Vec<Note>,  // List of notes for each project

}

#[derive(Serialize, Deserialize, Debug)]
pub struct Note {
    pub id: u32,
    pub description: String,
    pub content: String,
}


impl PartialEq for Project {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name
        // Compare other fields as needed
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Task {
    pub id: u32,
    pub description: String,
    // Add other task properties here, such as status, due date, etc.
}


pub fn load_state(base_dir: &Path) -> Result<TedoState, Box<dyn std::error::Error>> {
    let path = get_state_path(base_dir);
    let mut file = File::open(&path)?;
    let mut data = String::new();
    file.read_to_string(&mut data)?;
    let tedo_state: TedoState = toml::from_str(&data)?;
    println!("Loaded state: {:?}", tedo_state);
    Ok(tedo_state)
}


pub fn save_state(base_dir: &Path, tedo_state: &TedoState) -> Result<(), Box<dyn std::error::Error>> {
    let path = get_state_path(base_dir);
    let mut file = OpenOptions::new()
        .write(true)
        .create(true)
        .open(&path)?;
    let toml = toml::to_string(tedo_state)?;
    file.write_all(toml.as_bytes())?;
    println!("Saved state: {:?}", tedo_state);
    Ok(())
}

pub fn set_current_project(base_dir: &Path, name: &str) {
    let mut tedo_state = load_state(base_dir).unwrap_or_default();
    if tedo_state.projects.iter().any(|p| p.name == name) {
        tedo_state.current_project = Some(name.into());
        save_state(base_dir, &tedo_state).expect("Failed to save projects");
    } else {
        println!("Project with name {} does not exist", name);
    }
}

fn get_state_path(base_dir: &Path) -> PathBuf {
    base_dir.join("tedo_state.toml")
}


#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::tempdir;

    #[test]
    fn test_save_and_load_state() -> Result<(), Box<dyn std::error::Error>> {
        // Create a temporary directory.
        let dir = tempdir()?;
        let base_dir = dir.path();

        // Create a Projects object and save it.
        let tedo_state = TedoState {
            current_project: Some("test_project".into()),
            projects: vec![Project { name: "test".into(), tasks: Vec::new(), notes: Vec::new() }],
        };
        save_state(base_dir, &tedo_state)?;

        // Load the Projects object and check that it's the same as what was saved.
        let loaded_state = load_state(base_dir)?;
        assert_eq!(tedo_state.current_project, loaded_state.current_project);
        assert_eq!(tedo_state.projects, loaded_state.projects);

        // Clean up.
        dir.close()?;

        Ok(())
    }
}
