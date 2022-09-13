# PyMon


PyMon is a CLI tool that helps develop Python applications by re-running the application whenever file changes are detected in the directory.
It is written in the Rust programming language and will be installable on Windows and Linux when production ready.

## Installation

- Build the code with Cargo, optimized for production:

```
cargo build --release
```

This will create a binary named `pymon` in the target/release directory. Add this binary to a location on your system which is on your PATH environment variable.

## Usage

You can run and watch any Python file by simply running `pymon <your_app.py>`. If you do not specify a file name, it will print a help message.

## Credits

This project has been heavily inspired by [nodemon](https://github.com/remy/nodemon).
