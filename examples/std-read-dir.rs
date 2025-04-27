#![cfg(feature = "std")]

use std::io::{stdin, stdout, Write as _};

use io_fs::{coroutines::ReadDir, runtimes::std::handle};

fn main() {
    env_logger::init();

    let path = read_line("Which directory to read?");

    let mut output = None;
    let mut coroutine = ReadDir::new(&path);

    let paths = loop {
        match coroutine.resume(output) {
            Ok(paths) => break paths,
            Err(io) => output = Some(handle(io).unwrap()),
        }
    };

    println!("Entries inside {path}:");

    for path in paths {
        println!(" - {}", path.display());
    }
}

fn read_line(prompt: &str) -> String {
    print!("{prompt} ");
    stdout().flush().unwrap();
    let mut line = String::new();
    stdin().read_line(&mut line).unwrap();
    line.trim().to_owned()
}
