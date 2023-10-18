use std::path::PathBuf;
use structopt::StructOpt;


#[derive(Debug, StructOpt)]
pub enum Action {
    /// Add a new task to the list
    Add { 
        #[structopt()]
        task: String 
    },
    /// Remove an item from the list
    Remove {
        #[structopt()] 
        task_number: usize 
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
