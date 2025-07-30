#![cfg(feature = "std")]

use std::{
    env,
    io::{stdin, stdout, Write as _},
    path::PathBuf,
};

use io_fs::{coroutines::read_dir::ReadDir, error::FsResult, runtimes::std::handle};

fn main() {
    let _ = env_logger::try_init();

    let path: PathBuf = match env::var("DIR") {
        Ok(dir) => dir.into(),
        Err(_) => read_line("Directory to read?").into(),
    };

    let mut arg = None;
    let mut coroutine = ReadDir::new(&path);

    let paths = loop {
        match coroutine.resume(arg) {
            FsResult::Ok(paths) => break paths,
            FsResult::Err(err) => panic!("{err}"),
            FsResult::Io(io) => arg = Some(handle(io).unwrap()),
        }
    };

    println!("Entries inside {}:", path.display());

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
