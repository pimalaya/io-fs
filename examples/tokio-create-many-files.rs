#![cfg(feature = "tokio")]

use std::{
    io::{stdin, stdout, Write as _},
    time::Instant,
};

use io_fs::{coroutines::CreateFiles, runtimes::tokio::handle};
use tempdir::TempDir;

#[tokio::main]
async fn main() {
    env_logger::init();

    let tmp = TempDir::new("tokio-create-files").unwrap();

    let n = read_line("How many temp files to create?")
        .parse::<usize>()
        .unwrap();

    let start = Instant::now();

    let mut output = None;
    let mut coroutine = CreateFiles::new(
        (0..n).map(|n| (tmp.path().join(n.to_string()), b"Hello, world!".to_vec())),
    );

    while let Err(io) = coroutine.resume(output) {
        output = Some(handle(io).await.unwrap());
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
