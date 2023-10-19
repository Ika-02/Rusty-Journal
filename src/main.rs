use std::path::PathBuf;
use structopt::StructOpt;
use anyhow::anyhow;
use ansi_term::Colour::Red;

mod cli;
mod tasks;
use cli::{Action::*, CliOptions};
use tasks::Task;


fn find_default_file() -> Option<PathBuf> {
    home::home_dir().map(|mut path| {
        path.push(".rusty-journal-list.json");
        path
    })
}


fn try_main() -> anyhow::Result<()> {
    // Parse the command line arguments.
    let CliOptions { 
        action, 
        file,
    } = CliOptions::from_args();

    // Get the path to the JSON file or default file, print an error message if it fails.
    let file_name = file
    .or_else(find_default_file)
    .expect("Couldn't locate the default file");

    // Perform the action or print an error message.
    match action {
        Add { title } => tasks::add_task(file_name, Task::new(title)),
        Complete { task_number } => tasks::complete_task(file_name, task_number),
        Remove { task_number } => tasks::remove_task(file_name, task_number),
        Move { task_number, new_position } => tasks::move_task(file_name, task_number, new_position),
        Modify { task_number, title } => tasks::modify_task(file_name, task_number, title),
        List => tasks::list_tasks(file_name),
    }.map_err(|err| anyhow!("Failed to perform action -> {}", err))?; // Map the error to an anyhow error.
    Ok(())
}


fn main() {
    if let Err(e) = try_main() { // Try to run the main function.
        let error_message = Red.bold().paint("Error:");
        eprintln!("{} {:#?}", error_message, e); // Print the error message.
        std::process::exit(0) // Exit the program correctly.
    }
}
