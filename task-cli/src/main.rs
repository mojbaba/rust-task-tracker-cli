use std::{env::{args, Args}, string};

use command::get_command;

mod task;
mod command;

fn main() {
    for a in args() {
        println!("{}",a);
    }

    let strings: Vec<&str> = args().map(|s| s.as_str()).collect();

    let command = get_command(strings).unwrap();
}

