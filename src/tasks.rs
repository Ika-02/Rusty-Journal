use chrono::{serde::ts_seconds, DateTime, Local, Utc};
use serde::{Deserialize, Serialize};
use std::io::Result;
use std::path::PathBuf;


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

pub fn add_task(file: PathBuf, task: Task) -> Result<()> {}

pub fn complete_task(file: PathBuf, task_number: usize) -> Result<()> {}

pub fn list_tasks(file: PathBuf) -> Result<()> {}
