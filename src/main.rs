use std::env::{args, Args};

mod commands;
mod utils;

fn main() {
    let mut args: Args = args();
    let arg: &Option<String> = &args.nth(1);
    let file_name: &str;

    match arg {
        Some(val) => {
            file_name = val as &str;
            commands::init(file_name);
        }
        None => commands::help(),
    }
}
