use crate::utils::run;
use crate::utils::print_colored_text;

pub fn help() {
    println!("Usage: pymon <file_name>");
    println!("Example: pymon example.py");
}

pub fn init(file_name: &str) {
    print_colored_text("success", "PyMon v2.0\n").err();
    print_colored_text("warning", "Watching for file changes...\n").err();
    run(file_name);
}
