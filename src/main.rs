extern crate clap;
use clap::{App, Arg, SubCommand};

fn main() {
    let matches = App::new("Tedo")
        .version("1.0")
        .about("Productivity Manager CLI")
        .subcommand(
            SubCommand::with_name("create")
                .about("Create new objects like projects, tasks, etc.")
                .subcommand(
                    SubCommand::with_name("project")
                        .about("Create a new project")
                        .arg(
                            Arg::with_name("project_name")
                                .help("Name of the project")
                                .required(true),
                        )
                        .arg(
                            Arg::with_name("switch")
                                .short("s")
                                .long("switch")
                                .help("Switch to the new project"),
                        ),
                ),
        )
        .get_matches();

    if let Some(matches) = matches.subcommand_matches("create") {
        if let Some(project_matches) = matches.subcommand_matches("project") {
            let project_name = project_matches.value_of("project_name").unwrap();
            let switch = project_matches.is_present("switch");

            create_project(project_name, switch);
        }
    }
}

fn create_project(name: &str, switch: bool) {
    // Print the project name to the console
    println!("Creating project: {}", name);
}