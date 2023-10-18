use chrono::{serde::ts_seconds, DateTime, Local, Utc};
use serde::{Deserialize, Serialize};
use std::io::{Result, Seek, SeekFrom, Error, ErrorKind};
use std::path::PathBuf;
use std::fs::{File, OpenOptions};


#[derive(Debug, Serialize, Deserialize)]
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


fn collect_tasks_from_file(mut file: &File) -> Result<Vec<Task>> {
    // Reset the cursor to the beginning of the file before reading.
    file.seek(SeekFrom::Start(0))?;

    // Read the file and parse the tasks list into a vector.
    let tasks_list: Vec<Task> = match serde_json::from_reader(&file) {
        Ok(tasks_list) => tasks_list,
        Err(e) if e.is_eof() => Vec::new(),
        Err(e) => Err(e)?,
    };
    
    file.seek(SeekFrom::Start(0))?;
    Ok(tasks_list)
}


pub fn add_task(file_name: PathBuf, task: Task) -> Result<()> {
    // Open the file in read-write mode, creating it if it doesn't exist already.
    let mut file = OpenOptions::new()
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


pub fn complete_task(file_name: PathBuf, task_number: usize) -> Result<()> {
    // Open the file in read-write mode, creating it if it doesn't exist already.
    let mut file = OpenOptions::new()
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



pub fn list_tasks(file_name: PathBuf) -> Result<()> {}
