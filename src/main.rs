use std::path::Path;

mod storage;
mod projects;
mod tasks;
mod notes;

fn main() {
    let base_dir = dirs::home_dir().unwrap().join(".tedo");

    let matches = clap::App::new("Tedo")
        .version("1.0")
        .about("Productivity Manager CLI")
        .subcommand(
            clap::SubCommand::with_name("init")
                .about("Initialize Tedo in the current directory"),
        )

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
                        .aliases(&["p", "pr", "project"])
                        .about("List all projects"),
                )
                .subcommand(
                    clap::SubCommand::with_name("tasks")
                        .aliases(&["t", "ts", "task"])
                        .about("List all tasks"),
                ),
        )
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
        .subcommand(
            clap::SubCommand::with_name("table")
                .aliases(&["t", "tb", "tbl", "ta", "tab"])
                .about("List objects like projects, tasks, etc.")
                .subcommand(
                    clap::SubCommand::with_name("projects")
                        .aliases(&["p", "pr", "project"])
                        .about("List all projects"),
                )
                .subcommand(
                    clap::SubCommand::with_name("tasks")
                        .aliases(&["t", "ts", "task"])
                        .about("List all tasks"),
                )
                .subcommand(
                    clap::SubCommand::with_name("notes")
                        .aliases(&["n", "nt", "note"])
                        .about("List all notes"),
                ),
        )


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
                ),
        )
        .get_matches();


    // Handling the init subcommand
    if matches.subcommand_matches("init").is_some() {
        initialize_tedo();
        return;
    }


    if Path::new(&base_dir).exists() {
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
        } else if let Some(matches) = matches.subcommand_matches("list") {
            if let Some(_project_matches) = matches.subcommand_matches("projects") {
                projects::list_projects(&base_dir, "list");
            } else if let Some(_task_matches) = matches.subcommand_matches("tasks") {
                tasks::list_tasks(&base_dir, "list");
            }

        } else if let Some(matches) = matches.subcommand_matches("table") {
            if let Some(_project_matches) = matches.subcommand_matches("projects") {
                projects::list_projects(&base_dir, "table");
            } else if let Some(_task_matches) = matches.subcommand_matches("tasks") {
                tasks::list_tasks(&base_dir, "table");
            }

        } else if let Some(matches) = matches.subcommand_matches("switch") {
            let project_name = matches.value_of("project_name").unwrap();
            projects::switch_project(&base_dir, project_name);
        } else if let Some(matches) = matches.subcommand_matches("edit") {
            if let Some(note_matches) = matches.subcommand_matches("note") {
                let note_identifier = note_matches.value_of("note_identifier").unwrap();
                let note_id = note_identifier.parse::<u32>().expect("Failed to parse note identifier");
                notes::edit_note(&base_dir, note_id);

            }
        } else {
            println!("Invalid command. Use `tedo --help` to see the list of available commands.");
        }

    } else {
        println!("You can initialize Tedo using `tedo init`");
    }
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
