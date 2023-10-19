mod cli;
mod tasks;

use std::path::PathBuf;
use structopt::StructOpt;
use anyhow::anyhow;

use cli::{Action::*, CliOptions};
use tasks::Task;


fn find_default_file() -> Option<PathBuf> {
    home::home_dir().map(|mut path| {
        path.push(".rusty-journal-list.json");
        path
    })
}

fn main() -> anyhow::Result<()> {
    // Parse the command line arguments.
    let CliOptions { 
        action, 
        file,
    } = CliOptions::from_args();

    // Get the path to the JSON file or default file, print an error message if it fails.
    let file_name = file
    .or_else(find_default_file)
    .ok_or(anyhow!("Failed to find the file."))?;

    // Perform the action or print an error message.
    match action {
        Add { title } => tasks::add_task(file_name, Task::new(title)),
        Complete { task_number } => tasks::complete_task(file_name, task_number),
        Remove { task_number } => tasks::remove_task(file_name, task_number),
        Move { task_number, new_position } => tasks::move_task(file_name, task_number, new_position),
        Modify { task_number, title } => tasks::modify_task(file_name, task_number, title),
        List => tasks::list_tasks(file_name),
    }.expect("Failed to perform action");
    Ok(())
}
