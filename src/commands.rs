use crate::utils::run;

pub fn help() {
    println!("Usage: pymon <file_name>");
    println!("Example: pymon example.py");
}

pub fn init(file_name: &str) {
    run(file_name);
}
