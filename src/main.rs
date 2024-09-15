use std::env::args;

use chrono::Local;
use command::get_command;
use task::{Task, TaskStatus};
use task_holder::TaskHolder;

mod command;
mod task;
mod task_holder;

fn main() {
    let mut tasks = TaskHolder::load("/tmp/tasks.json");

    let strings = args().map(|s| s);

    let command = match get_command(strings) {
        Ok(command) => command,
        Err(err) => panic!("{}", err),
    };


    match command {
        command::Command::Add(description) => add(&mut tasks, &description),
        command::Command::Delete(id) => delete(&mut tasks, id),
        command::Command::List => list(&tasks),
        command::Command::ListBy(status) => list_by(&tasks, &status),
        command::Command::Mark(id, status) => change_status(&mut tasks, id, &status),
        command::Command::Update(id, description) => update(&mut tasks, id, &description)
    };

    tasks.save("/tmp/tasks.json").unwrap();
}

fn add(tasks: &mut TaskHolder, description: &str) {
    let id = tasks.add(Task {
        id: 0,
        description: description.to_string(),
        status: task::TaskStatus::ToDo,
        created_at: Local::now(),
        updated_at: Local::now(),
    });

    println!("Task added successfully (ID: {})", id);
}

fn update(tasks: &mut TaskHolder, id: i32, description: &str) {
    if let Err(err) = tasks.update(id, description) {
        println!("Error: {}", err);
    } else {
        println!("Updated successfully!");
    }
}

fn delete(tasks: &mut TaskHolder, id: i32) {
    if let Some(task) = tasks.delete(id) {
        println!("Task ({}) = {}  | deleted", id, task.description);
    } else {
        println!("Task not found");
    }
}

fn list(tasks: &TaskHolder) {
    for task in tasks.list() {
        println!("{}", serde_json::to_string(task).unwrap());
    }
}

fn list_by(tasks: &TaskHolder, status: &TaskStatus) {
    for task in tasks.list_by(status) {
        println!("{}", serde_json::to_string(task).unwrap());
    }
}

fn change_status(tasks: &mut TaskHolder, id: i32, status: &TaskStatus) {
    if let Err(err) = tasks.change_status(id, status.clone()) {
        println!("Error: {}", err);
    } else {
        println!("Status changed successfully!");
    }
}
