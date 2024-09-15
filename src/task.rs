use std::{
    fmt,
    str::FromStr,
};

use chrono::{DateTime, Local};
use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq,Clone,Serialize,Deserialize)]
pub enum TaskStatus {
    ToDo,
    InProgress,
    Done,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Task {
    pub id: i32,
    pub description: String,
    pub status: TaskStatus,
    pub created_at: DateTime<Local>,
    pub updated_at: DateTime<Local>,
}


impl FromStr for TaskStatus{
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "done" => Ok(TaskStatus::Done),
            "in-progress" => Ok(TaskStatus::InProgress),
            "todo" => Ok(TaskStatus::ToDo),
            _ => Err("invalid status")
        }
    }
}

impl fmt::Display for TaskStatus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}
