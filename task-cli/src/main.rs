use std::env::args;

use command::get_command;

mod task;
mod command;

fn main() {
    let strings = args().map(|s| s);

    let _command = get_command(strings).unwrap();
}

