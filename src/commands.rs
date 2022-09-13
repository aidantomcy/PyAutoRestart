use std::path::Path;

pub fn help() {
    println!("Usage: pymon <file_name>");
    println!("Example: pymon example.py")
}

pub fn run(file_name: &str) {
    if Path::new(file_name).exists() {
        println!("PyMon v2.0");
        println!("Watching for file changes...")
    } else {
        panic!(
            "[pymon] Error: No files matching the pattern '{}' were found.",
            file_name
        )
    }
}
