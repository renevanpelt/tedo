use std::path::Path;

mod storage;
mod projects;


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
                    clap::SubCommand::with_name("projects")
                        .aliases(&["p", "pr", "project"])
                        .about("List all projects"),
                )
        )

        .subcommand(
            clap::SubCommand::with_name("table")
                .aliases(&["t", "tb", "tbl", "ta"])
                .about("List objects like projects, tasks, etc.")
                .subcommand(
                    clap::SubCommand::with_name("projects")
                        .aliases(&["p", "pr", "project"])
                        .about("List all projects"),
                )
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
            if let Some(project_matches) = matches.subcommand_matches("project") {
                let project_name = project_matches.value_of("project_name").unwrap();
                let switch = project_matches.is_present("switch");
                projects::create_project(&base_dir, project_name, switch);
            }
        } else if let Some(matches) = matches.subcommand_matches("list") {
            if let Some(_project_matches) = matches.subcommand_matches("projects") {
                projects::list_projects(&base_dir, "list");
            }
        } else if let Some(matches) = matches.subcommand_matches("table") {
            if let Some(_project_matches) = matches.subcommand_matches("projects") {
                projects::list_projects(&base_dir, "table");
            }
        } else if let Some(matches) = matches.subcommand_matches("switch") {
                let project_name = matches.value_of("project_name").unwrap();
                projects::switch_project(&base_dir, project_name);
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
