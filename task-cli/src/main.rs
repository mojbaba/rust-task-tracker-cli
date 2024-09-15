extern crate json_macros;
extern crate json_module;

use json_macros::{Deserialize, Serialize};
use json_module::{Deserialize, Serialize};

fn main() {
    let p = Person {
        first_name: String::from("Mojtaba"),
        last_name: String::from("Goodarzi"),
    };

    let json = p.serialize();

    println!("{}", &json);
}

#[derive(Serialize, Deserialize)]
struct Person {
    first_name: String,
    last_name: String,
}
