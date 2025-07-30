#![cfg(feature = "tokio")]

use std::{
    env,
    io::{stdin, stdout, Write as _},
    time::Instant,
};

use io_fs::{coroutines::create_files::CreateFiles, error::FsResult, runtimes::tokio::handle};
use tempdir::TempDir;

#[tokio::main]
async fn main() {
    let _ = env_logger::try_init();

    let tmp = TempDir::new("tokio-create-many-files").unwrap();

    let n: usize = match env::var("N") {
        Ok(n) => n.parse().unwrap(),
        Err(_) => read_line("How many temp files to create?").parse().unwrap(),
    };

    let start = Instant::now();

    let mut arg = None;
    let mut coroutine =
        CreateFiles::new((0..n).map(|n| (tmp.path().join(n.to_string()), *b"Hello, world!")));

    loop {
        match coroutine.resume(arg) {
            FsResult::Ok(()) => break,
            FsResult::Err(err) => panic!("{err}"),
            FsResult::Io(io) => arg = Some(handle(io).await.unwrap()),
        }
    }

    let duration = start.elapsed();

    println!("Created {n} temp files in {duration:?}!");

    tmp.close().unwrap();
}

fn read_line(prompt: &str) -> String {
    print!("{prompt} ");
    stdout().flush().unwrap();
    let mut line = String::new();
    stdin().read_line(&mut line).unwrap();
    line.trim().to_owned()
}
