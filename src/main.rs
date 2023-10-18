mod cli;
mod tasks;

use structopt::StructOpt;

use cli::{Action::*, CliOptions};
use tasks::Task;


fn main() {
    // Parse the command line arguments.
    let CliOptions { 
        action, 
        file,
    } = CliOptions::from_args();

    // Get the path to the JSON file or print an error message.
    let file_name = file.expect("Failed to find the list file");

    // Perform the action or print an error message.
    match action {
        Add { title } => tasks::add_task(file_name, Task::new(title)),
        Remove { task_number } => tasks::remove_task(file_name, task_number),
        List => tasks::list_tasks(file_name),
    }.expect("Failed to perform action");
}
