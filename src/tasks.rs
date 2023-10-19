use ansi_term::Colour::Green;
use chrono::{serde::ts_seconds, DateTime, Local, Utc};
use serde::{Serialize, Deserialize};
use std::fmt;
use std::fs::{File, OpenOptions};
use std::io::{Error, ErrorKind, Result, Seek, SeekFrom};
use std::path::PathBuf;


#[derive(Debug, Deserialize, Serialize)]
pub struct Task {
    pub title: String,

    #[serde(with = "ts_seconds")]
    pub creation_date: DateTime<Utc>,

    pub done: bool,
}

impl Task {
    pub fn new(title: String) -> Task {
        Task {
            title,
            creation_date: Utc::now(),
            done: false,
        }
    }
}

impl fmt::Display for Task {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let creation_date = self.creation_date.with_timezone(&Local).format("%F %H:%M");
        // Format the task as a string of length 25 on left followed by the creation date.
        write!(f, "{:<25} | {}", self.title, creation_date)
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
    let still_undone_tasks = tasks_list.iter().filter(|task| !task.done).count();
    tasks_list.insert(still_undone_tasks, task);
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
        return Err(Error::new(
            ErrorKind::InvalidInput,
            "Task number out of range",
        ));
    } else {
        tasks_list.remove(task_number - 1);
    };

    // Clear the file before writing to it -> or it will append to the file thus corrupting it.
    file.set_len(0)?;
    // Write the updated tasks list to the file.
    serde_json::to_writer_pretty(file, &tasks_list)?;
    Ok(())
}


pub fn list_tasks(file_name: PathBuf) -> Result<()> {
    // Open file in read-only mode.
    let file = OpenOptions::new().read(true).open(file_name)?;

    // Read the file and parse the tasks list into a vector.
    let tasks_list: Vec<Task> = collect_tasks_from_file(&file)?;

    if tasks_list.is_empty() {
        // If the tasks list is empty, print a message.
        println!("No tasks in the list.");
    } else {
        // Loop through the tasks list and print each task.
        for (i, task) in tasks_list.iter().enumerate() {
            if task.done {
                let done_task = format!("[{}|Done] {}", i + 1, task);
                println!("{}", Green.paint(done_task));
                continue;
            }
            println!("[{}] {}", i + 1, task);
        }
    }
    Ok(())
}


pub fn complete_task(file_name: PathBuf, task_number: usize) -> Result<()> {
    // Open the file in read-write mode, creating it if it doesn't exist already.
    let file = OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .open(file_name)?;

    // Read the file and parse the tasks list into a vector.
    let mut tasks_list: Vec<Task> = collect_tasks_from_file(&file)?;

    // Mark the task as done or return an error if the task number is out of range.
    if task_number > tasks_list.len() || task_number == 0 {
        return Err(Error::new(
            ErrorKind::InvalidInput,
            "Task number out of list's range",
        ));
    } else {
        let mut task = tasks_list.remove(task_number - 1);
        if task.done {
            task.done = false;
        } else {
            task.done = true;
        }
        task.creation_date = Utc::now();
        tasks_list.push(task);
    };

    // Clear the file before writing to it -> or it will append to the file thus corrupting it.
    file.set_len(0)?;
    // Write the updated tasks list to the file.
    serde_json::to_writer_pretty(file, &tasks_list)?;
    Ok(())
}


pub fn move_task(file_name: PathBuf, task_number: usize, new_position: usize) -> Result<()> {
    // Open the file in read-write mode, creating it if it doesn't exist already.
    let file = OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .open(file_name)?;

    // Read the file and parse the tasks list into a vector.
    let mut tasks_list: Vec<Task> = collect_tasks_from_file(&file)?;

    // Move the task to the new position in the vector or return an error if the task number or new position is out of range.
    if new_position > tasks_list.len()
        || new_position == 0
        || task_number > tasks_list.len()
        || task_number == 0
    {
        return Err(Error::new(
            ErrorKind::InvalidInput,
            "Index out of list's range",
        ));
    } else {
        let still_undone_tasks = tasks_list.iter().filter(|task| !task.done).count();
        if new_position <= still_undone_tasks {
            let task = tasks_list.remove(task_number - 1);
            tasks_list.push(task);
            println!("New position out of undone tasks' range, moving to the end of the list.")
        } else {
            let task = tasks_list.remove(task_number - 1);
            tasks_list.insert(new_position - 1, task);
        }
    };

    // Clear the file before writing to it -> or it will append to the file thus corrupting it.
    file.set_len(0)?;
    // Write the updated tasks list to the file.
    serde_json::to_writer_pretty(file, &tasks_list)?;
    Ok(())
}


pub fn modify_task(file_name: PathBuf, task_number: usize, title: String) -> Result<()> {
    // Open the file in read-write mode, creating it if it doesn't exist already.
    let file = OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .open(file_name)?;

    // Read the file and parse the tasks list into a vector.
    let mut tasks_list: Vec<Task> = collect_tasks_from_file(&file)?;

    // Return an error if the task number is out of range.
    if task_number > tasks_list.len() || task_number == 0 {
        return Err(Error::new(
            ErrorKind::InvalidInput,
            "Task number out of list's range",
        ));
    };
    // Modify the task's title if it's not done, otherwise print an error message.
    if !tasks_list[task_number - 1].done {
        tasks_list[task_number - 1].title = title;
    } else {
        println!("Can't modify a completed task.")
    };

    // Clear the file before writing to it -> or it will append to the file thus corrupting it.
    file.set_len(0)?;
    // Write the updated tasks list to the file.
    serde_json::to_writer_pretty(file, &tasks_list)?;
    Ok(())
}
