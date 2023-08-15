use std::path::Path;
use crate::projects::current_project;

use crate::storage::Project;

mod storage;
mod projects;
mod tasks;
mod notes;
mod tedo;


static PROJECT_SHORTHANDS: [&str; 4] = ["project", "pr", "proj", "pro"];
fn main() {
    let base_dir = dirs::home_dir().unwrap().join(".tedo");
    let args: Vec<String> = std::env::args().collect();

    let known_subcommands = vec!["project", "init", "create", "list", "edit", "switch", "table"];
    let processed_args;
    let mut clap_args = args.clone();


    if args.len() > 3 && PROJECT_SHORTHANDS.contains(&args[1].as_str()) {
        clap_args.truncate(3); // Only take the first 3 arguments for clap
    }

    if clap_args.len() > 1 && !known_subcommands.contains(&clap_args[1].as_str()) {
        // Convert shortcuts to potential subcommands
        let new_args = arguments_from_shortcut(&args);

        // Merge the new args with the old ones
        let mut merged_args = vec![clap_args[0].clone()];
        merged_args.extend(new_args);
        merged_args.extend(clap_args[2..].to_vec());

        processed_args = merged_args;
    } else {
        processed_args = clap_args.clone();
    }

    let matches = process_matches(&processed_args);
    handle_arguments(&base_dir, &matches, &args);
}


fn initialize_tedo() {
    use std::fs;
    use std::io;

    println!("Are you sure you want to initialize tedo on your machine? A folder named .tedo will be created in your home directory (y/n)");

    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();

    if input.trim() == "y" {
        let path = dirs::home_dir().unwrap().join(".tedo");
        fs::create_dir_all(&path).expect("Failed to create .tedo directory");
        println!(".tedo directory has been created successfully!");
    } else {
        println!("Initialization aborted.");
    }
}

fn handle_arguments(base_dir: &Path, matches: &clap::ArgMatches, args: &[String]) {

    // Handling the init subcommand
    if matches.subcommand_matches("init").is_some() {
        initialize_tedo();
        return;
    }


    if Path::new(&base_dir).exists() {

        // Create

        if let Some(matches) = matches.subcommand_matches("create") {
            if let Some(note_matches) = matches.subcommand_matches("note") {
                let note_description: Vec<&str> = note_matches
                    .values_of("note_description")
                    .unwrap()
                    .collect();
                let note_description = note_description.join(" ");
                notes::create_note(&base_dir, &note_description, "");
            }
            if let Some(project_matches) = matches.subcommand_matches("project") {
                let project_name = project_matches.value_of("project_name").unwrap();
                let switch = project_matches.is_present("switch");
                projects::create_project(&base_dir, project_name, switch);
            }

            if let Some(task_matches) = matches.subcommand_matches("task") {
                let task_description: Vec<&str> = task_matches
                    .values_of("task_description")
                    .unwrap()
                    .collect();
                let task_description = task_description.join(" ");
                tasks::create_task(&base_dir, &task_description);
            }


        // List

        } else if let Some(matches) = matches.subcommand_matches("list") {
            if let Some(_project_matches) = matches.subcommand_matches("projects") {
                projects::list_projects(&base_dir, "list");
            } else if let Some(task_matches) = matches.subcommand_matches("tasks") {


                if let Some(_) = task_matches.subcommand_matches("all") {
                    println!("All tasks");
                    tasks::list_tasks(&base_dir, "list");
                } else {
                    let project = current_project(&base_dir);
                    if let Some(project) = project {
                        project.list_tasks("list");
                    } else {
                        println!("No selected project. Please switch to a project before listing tasks.");
                    }
                }
            } else if let Some(_note_matches) = matches.subcommand_matches("notes") {
                notes::list_notes(&base_dir, "list");
            } else {
                tedo::list(&base_dir);
            }


        //  Table

        } else if let Some(matches) = matches.subcommand_matches("table") {
            if let Some(_project_matches) = matches.subcommand_matches("projects") {
                projects::list_projects(&base_dir, "table");
            } else if let Some(task_matches) = matches.subcommand_matches("tasks") {
                if let Some(project_matches) = task_matches.subcommand_matches("project") {
                    let project_identifier = project_matches
                        .value_of("project_identifier");

                    if let Some(project_identifier) = project_identifier {
                        let project = Project::find(base_dir, project_identifier);
                        if let Some(project) = project {
                            project.list_tasks("table")
                        }
                    }

                }
                // tasks::list_tasks(&base_dir, "table");
            } else if let Some(_note_matches) = matches.subcommand_matches("notes") {
                notes::list_notes(&base_dir, "table");
            }


        // Switch


        } else if let Some(matches) = matches.subcommand_matches("switch") {
            let project_name = matches.value_of("project_name").unwrap();
            projects::switch_project(&base_dir, project_name);



        // Edit

        } else if let Some(matches) = matches.subcommand_matches("edit") {
            if let Some(note_matches) = matches.subcommand_matches("note") {
                let note_identifier = note_matches.value_of("note_identifier").unwrap();
                let note_id = note_identifier.parse::<u32>().expect("Failed to parse note identifier");
                notes::edit_note(&base_dir, note_id);
            }
            // Project
        } else if let Some(matches) = matches.subcommand_matches("project") {
            let project_identifier = matches.value_of("project_identifier").unwrap();


            let project = Project::find(base_dir, project_identifier);
            if let Some(project) = project {
                println!("{} {}", project.id, project.name);
            }

            println!("Project: {:?}", args);
            if args.len() > 3 {
                let additional_args: Vec<String> = args[2..].to_vec();
                println!("{:?}", additional_args);
                // Now additional_args contains your ["here", "are", "more", "arguments"]
                // Do something with additional_args...
                handle_arguments(&base_dir, &process_matches(&additional_args), &additional_args);
            }



        } else {
            println!("Invalid command. Use `tedo --help` to see the list of available commands.");
        }






    } else {
        println!("You can initialize Tedo using `tedo init`");
    }
}


fn arguments_from_shortcut(args: &[String]) -> Vec<String> {
    return args[1].chars().map(|c| c.to_string()).collect();
}


fn process_matches(args: &[String]) -> clap::ArgMatches {
    let matches = clap::App::new("Tedo")
        .version("1.0")
        .about("Productivity Manager CLI")
        .subcommand(
            clap::SubCommand::with_name("init")
                .about("Initialize Tedo in the current directory"),
        )


        // List

        .subcommand(
            clap::SubCommand::with_name("list")
                .aliases(&["ls", "l"])
                .about("List objects like projects, tasks, etc.")
                .subcommand(
                    clap::SubCommand::with_name("notes")
                        .aliases(&["n", "nt", "note"])
                        .about("List all notes"),
                )
                .subcommand(
                    clap::SubCommand::with_name("projects")
                        .aliases(&PROJECT_SHORTHANDS)
                        .about("List all projects"),
                )
                .subcommand(
                    clap::SubCommand::with_name("tasks")
                        .aliases(&["t", "ts", "task"])
                        .about("List all tasks")
                        .subcommand(
                            clap::SubCommand::with_name("all")
                                .aliases(&["a", "al"])
                                .about("List all tasks"),
                        ),

                )

        )

        // Edit

        .subcommand(
            clap::SubCommand::with_name("edit")
                .aliases(&["e", "ed"])
                .about("Edit objects like projects, tasks, etc.")
                .subcommand(
                    clap::SubCommand::with_name("note")
                        .aliases(&["n", "nt"])
                        .about("Edit a note")
                        .arg(
                            clap::Arg::with_name("note_identifier")
                                .help("ID or slug of the note")
                                .required(true),
                        ),
                )
        )

        // Table

        .subcommand(
            clap::SubCommand::with_name("table")
                .aliases(&["t", "tb", "tbl", "ta", "tab"])
                .about("List objects like projects, tasks, etc.")
                .subcommand(
                    clap::SubCommand::with_name("projects")
                        .aliases(&PROJECT_SHORTHANDS)
                        .about("List all projects"),
                )
                .subcommand(
                    clap::SubCommand::with_name("tasks")
                        .aliases(&["t", "ts", "task"])
                        .about("List all tasks")
                        .subcommand(
                            clap::SubCommand::with_name("project")
                                .aliases(&PROJECT_SHORTHANDS)
                                .help("Filter by project")

                                .arg(
                                    clap::Arg::with_name("project_identifier")
                                        .help("Filter by project"),
                                ),
                        ),
                )
                .subcommand(
                    clap::SubCommand::with_name("notes")
                        .aliases(&["n", "nt", "note"])
                        .about("List all notes")
                        .arg(
                            clap::Arg::with_name("project")
                                .short("p")
                                .takes_value(true)
                                .long("project")
                                .help("Filter by project"),
                        ),
                ),
        )

        // Switch

        .subcommand(
            clap::SubCommand::with_name("switch")
                .aliases(&["s", "sw"])
                .about("Switch context to a different project")
                .arg(
                    clap::Arg::with_name("project_name")
                        .help("Name of the project")
                        .required(true),
                ),
        )

        // Create

        .subcommand(
            clap::SubCommand::with_name("create")
                .aliases(&["c", "cr"])
                .about("Create new objects like projects, tasks, etc.")
                .subcommand(
                    clap::SubCommand::with_name("note")
                        .aliases(&["n", "nt"])
                        .about("Create a new note")
                        .arg(
                            clap::Arg::with_name("note_description")
                                .help("Description of the note")
                                .required(true)
                                .multiple(true),
                        ),
                )
                .subcommand(
                    clap::SubCommand::with_name("task")
                        .about("Create a new task")
                        .aliases(&["t", "ts"])
                        .arg(
                            clap::Arg::with_name("task_description")
                                .help("Description of the task")
                                .required(true)
                                .multiple(true),
                        ),
                )

                .subcommand(
                    clap::SubCommand::with_name("project")
                        .aliases(&["p", "pr"])
                        .about("Create a new project")
                        .arg(
                            clap::Arg::with_name("project_name")
                                .help("Name of the project")
                                .required(true),
                        )
                        .arg(
                            clap::Arg::with_name("switch")
                                .short("s")
                                .long("switch")
                                .help("Switch to the new project"),
                        ),
                )
        )

        // Project

        .subcommand(
            clap::SubCommand::with_name("project")
                .aliases(&["p", "pr"])
                .about("Manage projects")
                .arg(
                    clap::Arg::with_name("project_identifier")
                        .help("Identifier (id, name or shorthand) of the project")
                        .required(true)

                )
        )

        .get_matches_from(args);

    return matches;
}