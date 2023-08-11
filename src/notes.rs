use std::path::Path;
use crate::storage;
use crate::storage::save_state;
use crate::storage::Note;
use std::fs::File;
use std::io::Read;
use std::io::Write;

use std::process::Command;


pub fn edit_note(base_dir: &Path, id: u32) {
    let mut tedo_state = storage::load_state(base_dir).unwrap_or_default();


    let current_project_name = tedo_state.current_project.clone().unwrap_or_default();
    let project = tedo_state.projects.iter_mut().find(|p| p.name == current_project_name);

    if let Some(project) = project {
        if let Some(note) = project.notes.iter_mut().find(|n| n.id == id) {
            // Create a temporary file and write the current content of the note to it
            let mut temp_file = tempfile::NamedTempFile::new().expect("Failed to create temporary file");
            write!(temp_file, "{}", note.content).expect("Failed to write to temporary file");

            // Launch the VI editor to edit the file
            Command::new("vi")
                .arg(temp_file.path())
                .status()
                .expect("Failed to launch editor");

            // Read the edited content back from the temporary file
            let mut new_content = String::new();
            let mut file = File::open(temp_file.path()).expect("Failed to open temporary file");
            file.read_to_string(&mut new_content).expect("Failed to read from temporary file");

            // Update the content of the note
            note.content = new_content;

            // Save the state
            save_state(base_dir, &tedo_state).expect("Failed to save note");
        } else {
            println!("Note with id {} not found.", id);
        }
    } else {
        println!("No selected project. Please switch to a project before editing a note.");
    }
}



pub fn create_note(base_dir: &Path, description: &str, content: &str) {
    let mut tedo_state = storage::load_state(base_dir).unwrap_or_default();

    let current_project_name = tedo_state.current_project.clone().unwrap_or_default();
    let project = tedo_state.projects.iter_mut().find(|p| p.name == current_project_name);

    if let Some(project) = project {
        let next_id = project.notes.len() as u32 + 1;
        project.notes.push(Note { id: next_id, description: description.into(), content: content.into() });
        save_state(base_dir, &tedo_state).expect("Failed to save note");
    } else {
        println!("No selected project. Please switch to a project before creating a note.");
    }
}



#[cfg(test)]
mod tests {
    use tempfile::tempdir;

    use crate::projects::{create_project, switch_project};

    use super::*;

    #[test]
    fn test_create_note_in_selected_project() {
        let dir = tempdir().unwrap();
        let base_dir = dir.path();

        create_project(base_dir, "test_project", false);
        switch_project(base_dir, "test_project");

        create_note(base_dir, "test_note", "test_content");
        let state = storage::load_state(base_dir).unwrap();

        assert_eq!(state.projects.len(), 1);
        assert_eq!(state.projects[0].name, "test_project");
        assert_eq!(state.projects[0].notes.len(), 1);
        assert_eq!(state.projects[0].notes[0].id, 1);
        assert_eq!(state.projects[0].notes[0].description, "test_note");
        assert_eq!(state.projects[0].notes[0].content, "test_content");
    }
}
