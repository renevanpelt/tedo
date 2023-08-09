use std::path::Path;
use crate::storage;
use crate::storage::Project;
use crate::storage::save_projects;

pub fn create_project(base_dir: &Path, name: &str, switch: bool) {
    let mut projects = storage::load_projects(base_dir).unwrap_or_default();
    projects.projects.push(Project { name: name.into() });

    if switch {
        storage::set_current_project(name);
    }
    save_projects(base_dir, &projects).expect("Failed to save projects");

}
