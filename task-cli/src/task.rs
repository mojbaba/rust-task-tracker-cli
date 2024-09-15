use std::{
    borrow::BorrowMut,
    fmt,
    ops::{Deref, DerefMut}, str::FromStr,
};

use chrono::{DateTime, Utc};
use json_macros::{Deserialize, Serialize};
use json_module::{Deserialize, Serialize};

#[derive(Debug, PartialEq)]
pub struct TaskTime(DateTime<Utc>);

#[derive(Debug, PartialEq)]
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
    pub created_at: TaskTime,
    pub updated_at: TaskTime,
}

impl Serialize for TaskTime {
    fn serialize(&self) -> String {
        format!("\"{}\"", self.0.format("%Y-%m-%dT%H:%M:%SZ"))
    }
}

impl Deserialize for TaskTime {
    fn deserialize(json: &str) -> Result<Self, &'static str> {
        let date_time: DateTime<Utc> = json.parse().unwrap();
        Ok(Self(date_time.into()))
    }
}

impl Serialize for TaskStatus {
    fn serialize(&self) -> String {
        self.to_string()
    }
}

impl Deserialize for TaskStatus {
    fn deserialize(json: &str) -> Result<Self, &'static str> {
        match json {
            "ToDo" => Ok(TaskStatus::ToDo),
            "Done" => Ok(TaskStatus::Done),
            "InProgress" => Ok(TaskStatus::InProgress),
            _ => Err("Invalid task status"),
        }
    }
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

impl Deref for TaskTime {
    type Target = DateTime<Utc>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for TaskTime {
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.0.borrow_mut()
    }
}

impl TaskTime {
    pub fn now() -> Self {
        Self(Utc::now())
    }
}

#[cfg(test)]
mod task {
    use super::*;

    #[test]
    fn test_test(){
        let task = Task{
            created_at : TaskTime::now(),
            updated_at : TaskTime::now(),
            description : "test task description".to_string(),
            id : 7,
            status : TaskStatus::ToDo
        };

        let json = task.serialize();

        let recovered = Task::deserialize(&json).unwrap();

        assert_eq!(json, recovered.serialize());
    }
}

#[cfg(test)]
mod task_time {
    use super::*;

    #[test]
    fn task_time_now() {
        let now = TaskTime::now();

        let serialized = now.serialize();

        let serialized = serialized.trim_matches('"');

        let deserialized = TaskTime::deserialize(&serialized).unwrap();

        assert_eq!(serialized, deserialized.serialize().trim_matches('"'));
    }
}

#[cfg(test)]
mod task_status {
    use super::*;

    #[test]
    fn task_status_0() {
        let t_a = TaskStatus::InProgress;
        let json = t_a.to_string();
        let des = TaskStatus::deserialize(&json).unwrap();
        assert_eq!(t_a, des);
    }

    #[test]
    fn task_status_1() {
        let t_a = TaskStatus::Done;
        let json = t_a.to_string();
        let des = TaskStatus::deserialize(&json).unwrap();
        assert_eq!(t_a, des);
    }

    #[test]
    fn task_status_2() {
        let t_a = TaskStatus::Done;
        let json = t_a.to_string();
        let des = TaskStatus::deserialize(&json).unwrap();
        assert_eq!(t_a, des);
    }
}
