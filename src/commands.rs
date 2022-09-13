extern crate notify;
use notify::{INotifyWatcher, RecursiveMode, Result, Watcher};
use std::env::consts::OS;
use std::env::{args, Args};
use std::path::Path;
use std::process::{Command, ExitStatus};

pub fn help() {
    println!("Usage: pymon <file_name>");
    println!("Example: pymon example.py");
}

pub fn run(file_name: &str) {
    if Path::new(file_name).exists() {
        println!("PyMon v2.0");
        println!("Watching for file changes...");

        match OS {
            "linux" => {
                let stdout: ExitStatus = Command::new("python3")
                    .arg(file_name)
                    .status()
                    .expect("[pymon] Error: Failed to run file");
                let output: &str = &stdout.to_string() as &str;
                println!("{}", output);

                loop {
                    watch().err();
                }
            }
            "windows" => {
                todo!("Add functionality for Windows")
            }
            _ => panic!("[pymon] Error: Operating System not supported"),
        }
    } else {
        panic!(
            "[pymon] Error: No files matching the pattern '{}' were found.",
            file_name
        )
    }
}

fn watch() -> Result<()> {
    let mut watcher: INotifyWatcher =
        notify::recommended_watcher(|res: notify::Result<notify::Event>| match res {
            Ok(_) => {
                let mut args: Args = args();
                let file_name: &str = &args.nth(1).unwrap() as &str;
                run(file_name);
            }
            Err(err) => println!("watch error: {:?}", err),
        })?;

    watcher.watch(Path::new("."), RecursiveMode::Recursive)?;

    Ok(())
}
