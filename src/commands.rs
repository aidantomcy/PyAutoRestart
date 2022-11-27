use std::{
    env::{args, consts::OS, Args},
    io::stdout,
    path::Path,
    process::{Command, ExitStatus},
};
use crossterm::{
    style::{Color, Print, ResetColor, SetForegroundColor},
    ExecutableCommand,
};
use notify::{INotifyWatcher, RecursiveMode, Result, Watcher};

pub fn help() {
    println!("Usage: pymon <file_name>");
    println!("Example: pymon example.py");
}

pub fn run(file_name: &str) {
    if Path::new(file_name).exists() {
        print_colored_text("success", "PyMon v2.0\n").err();
        print_colored_text("warning", "Watching for file changes...\n").err();

        match OS {
            "linux" | "macos" => {
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
                print_colored_text("warning", "Restarting due to file changes...\n").err();
                let mut args: Args = args();
                let file_name: &str = &args.nth(1).unwrap() as &str;
                run(file_name);
            }
            Err(err) => println!("watch error: {:?}", err),
        })?;

    watcher.watch(Path::new("."), RecursiveMode::Recursive)?;

    Ok(())
}

fn print_colored_text(output_type: &str, msg: &str) -> crossterm::Result<()> {
    match output_type {
        "success" => {
            stdout()
                .execute(SetForegroundColor(Color::Green))?
                .execute(Print(msg))?
                .execute(ResetColor)?;
            Ok(())
        }
        "warning" => {
            stdout()
                .execute(SetForegroundColor(Color::Yellow))?
                .execute(Print(msg))?
                .execute(ResetColor)?;
            Ok(())
        }
        "error" => {
            stdout()
                .execute(SetForegroundColor(Color::Red))?
                .execute(Print(msg))?
                .execute(ResetColor)?;
            Ok(())
        }
        _ => panic!("Error: Invalid output type provided"),
    }
}
