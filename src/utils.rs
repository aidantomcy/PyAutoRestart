extern crate colored;
use colored::*;
use notify::{RecursiveMode, Watcher};
use std::{
    env::{args, consts::OS, Args},
    path::Path,
    process::Command,
};

fn watch() -> notify::Result<()> {
    let mut watcher =
        notify::recommended_watcher(|res: notify::Result<notify::Event>| match res {
            Ok(event) => {
                for file in &event.paths {
                    if let Some(extension) = file.extension() {
                        if extension == "py" {
                            print_colored_text(ResultType::Warning, "Restarting due to file changes...\n");
                            let mut args: Args = args();
                            let file_name: &str = &args.nth(1).unwrap() as &str;
                            run(file_name);
                        } else if file.extension().is_none() {
                        }
                    }
                }
            }
            Err(err) => println!("watch error: {err:?}"),
        })?;

    watcher.watch(Path::new("."), RecursiveMode::Recursive)?;

    Ok(())
}

pub(crate) fn run(file_name: &str) {
    if Path::new(file_name).exists() {
        match OS {
            "linux" | "macos" => {
                let stdout = Command::new("python3")
                    .arg(file_name)
                    .status()
                    .expect("[pymon] Error: Failed to run file");
                let output: &str = &stdout.to_string() as &str;
                println!("{output}");

                loop {
                    watch().err();
                }
            }
            "windows" => {
                let stdout = Command::new("python")
                    .arg(file_name)
                    .status()
                    .expect("[pymon] Error: Failed to run file");
                let output: &str = &stdout.to_string() as &str;
                println!("{output}");

                loop {
                    watch().err();
                }
            }
            _ => panic!("[pymon] Error: Operating System not supported"),
        }
    } else {
        panic!("[pymon] Error: No files matching the pattern '{file_name}' were found.")
    }
}

pub(crate) enum ResultType {
    Success,
    Warning,
    Error
}

pub(crate) fn print_colored_text(result_type: ResultType, msg: &str) {
    match result_type {
        ResultType::Success => println!("{}", msg.green()),
        ResultType::Warning => println!("{}", msg.yellow()),
        ResultType::Error => println!("{}", msg.red()),
    }
}
