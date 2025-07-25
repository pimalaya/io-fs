//! Module dedicated to the Tokio-based, async runtime.

use std::{
    collections::{HashMap, HashSet},
    io,
    path::PathBuf,
};

use tokio::fs;

use crate::io::FsIo;

/// The Tokio-based, async I/O handler.
///
/// This handler makes use of standard module [`std::io`] and Tokio
/// module [`tokio::io`] to process [`FsIo`] filesystems.
pub async fn handle(input: FsIo) -> io::Result<FsIo> {
    match input {
        FsIo::CreateDir(input) => create_dir(input).await,
        FsIo::CreateDirs(input) => create_dirs(input).await,
        FsIo::CreateFile(input) => create_file(input).await,
        FsIo::CreateFiles(input) => create_files(input).await,
        FsIo::ReadDir(input) => read_dir(input).await,
        FsIo::ReadFile(input) => read_file(input).await,
        FsIo::ReadFiles(input) => read_files(input).await,
        FsIo::RemoveDir(input) => remove_dir(input).await,
        FsIo::RemoveDirs(input) => remove_dirs(input).await,
        FsIo::RemoveFile(input) => remove_file(input).await,
        FsIo::RemoveFiles(input) => remove_files(input).await,
        FsIo::Rename(input) => rename(input).await,
    }
}

pub async fn create_dir(input: Result<(), PathBuf>) -> io::Result<FsIo> {
    let Err(path) = input else {
        let kind = io::ErrorKind::InvalidInput;
        return Err(io::Error::new(kind, "missing directory path"));
    };

    fs::create_dir(path).await?;

    Ok(FsIo::CreateDir(Ok(())))
}

pub async fn create_dirs(input: Result<(), HashSet<PathBuf>>) -> io::Result<FsIo> {
    let Err(paths) = input else {
        let kind = io::ErrorKind::InvalidInput;
        return Err(io::Error::new(kind, "missing directory paths"));
    };

    for path in paths {
        fs::create_dir(path).await?;
    }

    Ok(FsIo::CreateDirs(Ok(())))
}

pub async fn create_file(input: Result<(), (PathBuf, Vec<u8>)>) -> io::Result<FsIo> {
    let Err((path, contents)) = input else {
        let kind = io::ErrorKind::InvalidInput;
        return Err(io::Error::new(kind, "missing file contents"));
    };

    fs::write(path, contents).await?;

    Ok(FsIo::CreateFile(Ok(())))
}

pub async fn create_files(input: Result<(), HashMap<PathBuf, Vec<u8>>>) -> io::Result<FsIo> {
    let Err(contents) = input else {
        let kind = io::ErrorKind::InvalidInput;
        return Err(io::Error::new(kind, "missing file contents"));
    };

    for (path, contents) in contents {
        fs::write(path, contents).await?;
    }

    Ok(FsIo::CreateFiles(Ok(())))
}

pub async fn read_dir(input: Result<HashSet<PathBuf>, PathBuf>) -> io::Result<FsIo> {
    let Err(path) = input else {
        let kind = io::ErrorKind::InvalidInput;
        return Err(io::Error::new(kind, "missing directory path"));
    };

    let mut paths = HashSet::new();
    let mut dir = fs::read_dir(path).await?;

    while let Some(entry) = dir.next_entry().await? {
        paths.insert(entry.path());
    }

    Ok(FsIo::ReadDir(Ok(paths)))
}

pub async fn read_file(input: Result<Vec<u8>, PathBuf>) -> io::Result<FsIo> {
    let Err(path) = input else {
        let kind = io::ErrorKind::InvalidInput;
        return Err(io::Error::new(kind, "missing file path"));
    };

    let contents = fs::read(path).await?;

    Ok(FsIo::ReadFile(Ok(contents)))
}

pub async fn read_files(
    input: Result<HashMap<PathBuf, Vec<u8>>, HashSet<PathBuf>>,
) -> io::Result<FsIo> {
    let Err(paths) = input else {
        let kind = io::ErrorKind::InvalidInput;
        return Err(io::Error::new(kind, "missing file paths"));
    };

    let mut contents = HashMap::new();

    for path in paths {
        let content = fs::read(&path).await?;
        contents.insert(path, content);
    }

    Ok(FsIo::ReadFiles(Ok(contents)))
}

pub async fn remove_dir(input: Result<(), PathBuf>) -> io::Result<FsIo> {
    let Err(path) = input else {
        let kind = io::ErrorKind::InvalidInput;
        return Err(io::Error::new(kind, "missing directory path"));
    };

    fs::remove_dir_all(path).await?;

    Ok(FsIo::RemoveDir(Ok(())))
}

pub async fn remove_dirs(input: Result<(), HashSet<PathBuf>>) -> io::Result<FsIo> {
    let Err(paths) = input else {
        let kind = io::ErrorKind::InvalidInput;
        return Err(io::Error::new(kind, "missing directory paths"));
    };

    for path in paths {
        fs::remove_dir_all(path).await?;
    }

    Ok(FsIo::RemoveDirs(Ok(())))
}

pub async fn remove_file(input: Result<(), PathBuf>) -> io::Result<FsIo> {
    let Err(path) = input else {
        let kind = io::ErrorKind::InvalidInput;
        return Err(io::Error::new(kind, "missing file path"));
    };

    fs::remove_file(path).await?;

    Ok(FsIo::RemoveFile(Ok(())))
}

pub async fn remove_files(input: Result<(), HashSet<PathBuf>>) -> io::Result<FsIo> {
    let Err(paths) = input else {
        let kind = io::ErrorKind::InvalidInput;
        return Err(io::Error::new(kind, "missing file paths"));
    };

    for path in paths {
        fs::remove_file(path).await?;
    }

    Ok(FsIo::RemoveFiles(Ok(())))
}

pub async fn rename(input: Result<(), Vec<(PathBuf, PathBuf)>>) -> io::Result<FsIo> {
    let Err(paths) = input else {
        let kind = io::ErrorKind::InvalidInput;
        return Err(io::Error::new(kind, "missing file paths"));
    };

    for (from, to) in paths {
        fs::rename(from, to).await?;
    }

    Ok(FsIo::Rename(Ok(())))
}
