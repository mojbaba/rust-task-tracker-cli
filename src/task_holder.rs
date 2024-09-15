
use std::{
    collections::HashMap,
    fs::File,
    io::Write
};

use chrono::Local;
use serde::{Deserialize, Serialize};

use crate::task::{Task, TaskStatus};

#[derive(Serialize,Deserialize)]
pub struct TaskHolder {
    tasks: HashMap<i32, Task>,
    next_id: i32,
}

impl TaskHolder {
    pub fn new() -> Self {
        Self {
            tasks: HashMap::new(),
            next_id: 0,
        }
    }

    pub fn update(&mut self, id: i32, description: &str) -> Result<(), &'static str> {
        let task = self.tasks.get_mut(&id).ok_or("id not found")?;
        task.description = description.to_string();
        task.updated_at = Local::now();
        Ok(())
    }

    pub fn change_status(&mut self, id: i32, status: TaskStatus) -> Result<(), &'static str> {
        let task = self.tasks.get_mut(&id).ok_or("id not found")?;
        task.status = status;
        task.updated_at = Local::now();
        Ok(())
    }

    pub fn add(&mut self, mut task: Task) -> i32 {
        let id = self.next_id;
        if task.id == 0 {
            task.id = id;
            self.next_id += 1;
        }
        self.tasks.insert(task.id, task);
        id
    }

    pub fn delete(&mut self, id: i32) -> Option<Task> {
        self.tasks.remove(&id)
    }

    pub fn list(&self) -> Vec<&Task> {
        self.tasks.iter().map(|(_, v)| v).collect()
    }

    pub fn list_by(&self, status: &TaskStatus) -> Vec<&Task> {
        self.tasks
            .iter()
            .map(|(_, v)| v)
            .filter(|v| &v.status == status)
            .collect()
    }

    pub fn save(&self, file_name: &str) -> Result<(), &'static str> {
        let mut file = File::create(file_name).map_err(|_| "Error in opening file")?;

        let json = serde_json::to_string(self).unwrap();

        write!(file,"{}", json).unwrap();

        Ok(())
    }

    pub fn load(file_name: &str) -> TaskHolder {

        let tasks = match File::open(file_name) {
            Ok(file) => serde_json::from_reader(file).unwrap(),
            Err(_) => {
                let tasks = TaskHolder::new();
                tasks.save(file_name).unwrap();
                tasks
            }
        };

        tasks
    }
}
