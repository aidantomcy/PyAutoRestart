use notify::{INotifyWatcher, RecursiveMode, Result, Watcher};
use std::env::consts::OS;
use std::env::{args, Args};
use std::io::{self, Write};
use std::path::Path;
use std::process::{Command, ExitStatus};
use termcolor::{Color, ColorChoice, ColorSpec, StandardStream, WriteColor};

pub fn help() {
    println!("Usage: pymon <file_name>");
    println!("Example: pymon example.py");
}

pub fn run(file_name: &str) {
    if Path::new(file_name).exists() {
        print_colored_text("success", "PyMon v2.0").err();
        print_colored_text("warning", "Watching for file changes...").err();
        reset_stdout_color();

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
                print_colored_text("warning", "Restarting due to file changes...").err();
                reset_stdout_color();
                let mut args: Args = args();
                let file_name: &str = &args.nth(1).unwrap() as &str;
                run(file_name);
            }
            Err(err) => println!("watch error: {:?}", err),
        })?;

    watcher.watch(Path::new("."), RecursiveMode::Recursive)?;

    Ok(())
}

fn print_colored_text(output_type: &str, msg: &str) -> io::Result<()> {
    let mut stdout: StandardStream = StandardStream::stdout(ColorChoice::Always);

    match output_type {
        "success" => {
            stdout.set_color(ColorSpec::new().set_fg(Some(Color::Green)))?;
            writeln!(&mut stdout, "{}", msg)
        }
        "warning" => {
            stdout.set_color(ColorSpec::new().set_fg(Some(Color::Yellow)))?;
            writeln!(&mut stdout, "{}", msg)
        }
        "error" => {
            stdout.set_color(ColorSpec::new().set_fg(Some(Color::Red)))?;
            writeln!(&mut stdout, "{}", msg)
        }
        _ => panic!("Error: Invalid output type provided"),
    }
}

fn reset_stdout_color() {
    let mut stdout: StandardStream = StandardStream::stdout(ColorChoice::Always);
    stdout
        .set_color(ColorSpec::new().set_fg(Some(Color::White)))
        .err();
}
