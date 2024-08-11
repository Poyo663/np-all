use core::panic;

use cli_app::builds::langs::{
    c::create_c_app, java::create_java_app, react::create_react_app, rust::create_rust_app,
};
use dialoguer::Select;

fn main() {
    let name = match std::env::args().nth(1) {
        Some(string) => string,
        None => {
            eprintln!("Please provide a project name");
            std::process::exit(0)
        }
    };

    let items = ["React", "Rust", "C", "Java"];

    let selection = Select::new()
        .with_prompt("Please select what project you would like to build")
        .items(&items)
        .interact()
        .unwrap();

    match selection {
        0 => {
            create_react_app(name);
        }
        1 => {
            create_rust_app(name);
        }
        2 => {
            create_c_app(name);
        }
        3 => {
            create_java_app(name);
        }
        _ => {
            panic!("Another case needs to be added in the match statement");
        }
    }
}
