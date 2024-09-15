use std::str::FromStr;

use crate::{
    command,
    task::{Task, TaskStatus},
};

#[derive(Debug)]
pub enum Command {
    List,
    Update(i32, String),
    Delete(i32),
    Mark(i32, TaskStatus),
    ListBy(TaskStatus),
    Add(String),
}

pub fn get_command<'a>(mut args: impl Iterator<Item = &'a str>) -> Result<Command, &'static str> {
    match (args.next(), args.next(), args.next()) {
        (Some("add"), Some(name), _) => Ok(Command::Add(name.to_string())),
        (Some("update"), Some(id), Some(name)) => Ok(Command::Update(
            id.parse().map_err(|_| "cannot parse id")?,
            name.to_string(),
        )),
        (Some("delete"), Some(id), _) => {
            Ok(Command::Delete(id.parse().map_err(|_| "cannot parse id")?))
        }
        (Some("list"), Some(status), _) => Ok(Command::ListBy(TaskStatus::from_str(&status)?)),
        (Some("list"), _, _) => Ok(Command::List),
        (Some("mark-in-progress"), Some(id), _) => Ok(Command::Mark(
            id.parse().map_err(|_| "cannot parse id")?,
            TaskStatus::InProgress,
        )),
        (Some("mark-done"), Some(id), _) => Ok(Command::Mark(
            id.parse().map_err(|_| "cannot parse id")?,
            TaskStatus::Done,
        )),

        _ => Err("Invalid arguments"),
    }
}
