use crate::utils::{print_colored_text, run, ResultType};

pub fn help() {
    println!("Usage: pymon <file_name>");
    println!("Example: pymon example.py");
}

pub fn init(file_name: &str) {
    print_colored_text(ResultType::Success, "pymon v2.0");
    print_colored_text(ResultType::Warning, "Watching for file changes...\n");
    run(file_name);
}
