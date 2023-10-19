use std::path::PathBuf;
use structopt::StructOpt;


#[derive(Debug, StructOpt)]
pub enum Action {
    /// Add a new task to the list
    Add { 
        #[structopt()]
        title: String 
    },
    /// Mark a task as completed (or uncompleted again)
    Complete { 
        #[structopt()] 
        task_number: usize 
    },
    /// Remove a task from the list
    Remove {
        #[structopt()] 
        task_number: usize 
    },
    /// Move a task to another position in the list
    Move {
        #[structopt()] 
        task_number: usize,
        #[structopt()] 
        new_position: usize,
    },
    /// Modify the title of a task
    Modify {
        #[structopt()] 
        task_number: usize,
        #[structopt()] 
        title: String,
    },
    /// List all tasks
    List,
}

#[derive(Debug, StructOpt)]
#[structopt(
    name = "Rusty Journal",
    about = "A command line to-do app written in Rust"
)]
pub struct CliOptions {
    #[structopt(subcommand)]
    pub action: Action,

    #[structopt(parse(from_os_str), short, long)]
    pub file: Option<PathBuf>,
}
