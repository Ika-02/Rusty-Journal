mod cli;
mod tasks;

use std::path::PathBuf;

use structopt::StructOpt;

use cli::{Action::*, CliOptions};
use tasks::Task;


fn find_default_file() -> Option<PathBuf> {
    home::home_dir().map(|mut path| {
        path.push(".rusty-journal-list.json");
        path
    })
}

fn main() {
    // Parse the command line arguments.
    let CliOptions { 
        action, 
        file,
    } = CliOptions::from_args();

    // Get the path to the JSON file or default file, print an error message if it fails.
    let file_name = file
    .or_else(find_default_file)
    .expect("Failed to find the list file");

    // Perform the action or print an error message.
    match action {
        Add { title } => tasks::add_task(file_name, Task::new(title)),
        Remove { task_number } => tasks::remove_task(file_name, task_number),
        List => tasks::list_tasks(file_name),
    }.expect("Failed to perform action");
}
