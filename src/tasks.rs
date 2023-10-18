use chrono::{serde::ts_seconds, DateTime, Local, Utc};
use serde::Serialize;
use serde::Deserialize;
use std::io::{Result, Seek, SeekFrom, Error, ErrorKind};
use std::path::PathBuf;
use std::fs::{File, OpenOptions};
use std::fmt;


#[derive(Debug, Deserialize, Serialize)]
pub struct Task {
    pub title: String,

    #[serde(with = "ts_seconds")]
    pub creation_date: DateTime<Utc>,
}

impl Task {
    pub fn new(title: String) -> Task {
        Task {
            title,
            creation_date: Utc::now(),
        }
    }
}

impl fmt::Display for Task {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let creation_date = self.creation_date.with_timezone(&Local).format("%F %H:%M");
        write!(f, "{:<30} | {}", self.title, creation_date)
    }
}


fn collect_tasks_from_file(mut file: &File) -> Result<Vec<Task>> {
    // Reset the cursor to the beginning of the file before reading.
    file.seek(SeekFrom::Start(0))?;

    // Read the file and parse the tasks list into a vector.
    let tasks_list: Vec<Task> = match serde_json::from_reader(file) {
        Ok(tasks_list) => tasks_list,
        Err(e) if e.is_eof() => Vec::new(),
        Err(e) => Err(e)?,
    };
    
    file.seek(SeekFrom::Start(0))?;
    Ok(tasks_list)
}


pub fn add_task(file_name: PathBuf, task: Task) -> Result<()> {
    // Open the file in read-write mode, creating it if it doesn't exist already.
    let file = OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .open(file_name)?;

    // Read the file and parse the tasks list into a vector.
    let mut tasks_list: Vec<Task> = collect_tasks_from_file(&file)?;

    // Write the updated tasks list to the file.
    tasks_list.push(task);
    serde_json::to_writer_pretty(file, &tasks_list)?;

    Ok(())
}


pub fn remove_task(file_name: PathBuf, task_number: usize) -> Result<()> {
    // Open the file in read-write mode, creating it if it doesn't exist already.
    let file = OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .open(file_name)?;

    // Read the file and parse the tasks list into a vector.
    let mut tasks_list: Vec<Task> = collect_tasks_from_file(&file)?;    

    // Remove the task from the vector or return an error if the task number is out of range.
    if task_number > tasks_list.len() || task_number == 0 {
        return Err(Error::new(ErrorKind::InvalidInput, "Task number out of range"));
    } else {
        tasks_list.remove(task_number - 1);
    };

    // Write the updated tasks list to the file.
    serde_json::to_writer_pretty(file, &tasks_list)?;
    Ok(())
}


pub fn list_tasks(file_name: PathBuf) -> Result<()> {
    // Open file in read-only mode.
    let file = OpenOptions::new()
        .read(true)
        .open(file_name)?;

    // Read the file and parse the tasks list into a vector.
    let tasks_list: Vec<Task> = collect_tasks_from_file(&file)?;

    if tasks_list.is_empty() { // If the tasks list is empty, print a message.
        println!("No tasks in the list.");
    } else {
        // Loop through the tasks list and print each task.
        for (i, task) in tasks_list.iter().enumerate() {
            println!("[{}] {}", i + 1, task);
        }
    }
    Ok(())
}
