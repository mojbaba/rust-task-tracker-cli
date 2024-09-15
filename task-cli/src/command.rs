use std::str::FromStr;

use crate::task::TaskStatus;

#[derive(PartialEq,Debug)]
pub enum Command {
    List,
    Update(i32, String),
    Delete(i32),
    Mark(i32, TaskStatus),
    ListBy(TaskStatus),
    Add(String),
}

pub fn get_command(mut args: impl Iterator<Item = String>) -> Result<Command, &'static str> {

    _ = args.next();
    
    match (args.next().as_deref(), args.next().as_deref(), args.next().as_deref()) {
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


#[cfg(test)]
mod test {
    use crate::command::Command;

    use super::get_command;

    #[test]
    fn list_command() {
        let mut args = Vec::new();
        args.push("task-cli".to_string());
        args.push("list".to_string());
        
        let command = get_command(args.into_iter());

        assert_eq!(command, Ok(Command::List));
    }

    #[test]
    fn list_of_command_in_progress() {
        let mut args = Vec::new();
        args.push("task-cli".to_string());
        args.push("list".to_string());
        args.push("in-progress".to_string());
        
        let command = get_command(args.into_iter());

        assert_eq!(command, Ok(Command::ListBy(crate::task::TaskStatus::InProgress)));
    }


    #[test]
    fn list_of_command_done() {
        let mut args = Vec::new();
        args.push("task-cli".to_string());
        args.push("list".to_string());
        args.push("done".to_string());
        
        let command = get_command(args.into_iter());

        assert_eq!(command, Ok(Command::ListBy(crate::task::TaskStatus::Done)));
    }

    #[test]
    fn list_of_command_todo() {
        let mut args = Vec::new();
        args.push("task-cli".to_string());
        args.push("list".to_string());
        args.push("todo".to_string());
        
        let command = get_command(args.into_iter());

        assert_eq!(command, Ok(Command::ListBy(crate::task::TaskStatus::ToDo)));
    }

    #[test]
    fn delete() {
        let mut args = Vec::new();
        args.push("task-cli".to_string());
        args.push("delete".to_string());
        args.push("5".to_string());
        
        let command = get_command(args.into_iter());

        assert_eq!(command, Ok(Command::Delete(5)));
    }

    #[test]
    fn edit() {
        let mut args = Vec::new();
        args.push("task-cli".to_string());
        args.push("update".to_string());
        args.push("5".to_string());
        args.push("Hello there".to_string());
        
        let command = get_command(args.into_iter());

        assert_eq!(command, Ok(Command::Update(5,"Hello there".to_string())));
    }

    #[test]
    fn mark_in_progress() {
        let mut args = Vec::new();
        args.push("task-cli".to_string());
        args.push("mark-in-progress".to_string());
        args.push("5".to_string());
        
        let command = get_command(args.into_iter());

        assert_eq!(command, Ok(Command::Mark(5,crate::task::TaskStatus::InProgress)));
    }

    #[test]
    fn mark_done() {
        let mut args = Vec::new();
        args.push("task-cli".to_string());
        args.push("mark-done".to_string());
        args.push("5".to_string());
        
        let command = get_command(args.into_iter());

        assert_eq!(command, Ok(Command::Mark(5,crate::task::TaskStatus::Done)));
    }
    
    
}