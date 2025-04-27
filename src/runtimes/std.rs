//! Module dedicated to the standard, blocking runtime.

use std::{
    collections::{HashMap, HashSet},
    fs, io,
    path::PathBuf,
};

use log::debug;

use crate::Io;

/// The main runtime I/O handler.
///
/// This handler makes use of standard modules [`std::fs`] and
/// [`std::io`] to process filesystems [`Io`].
pub fn handle(input: Io) -> io::Result<Io> {
    match input {
        Io::Error(err) => Err(io::Error::new(io::ErrorKind::Other, err)),
        Io::CreateDir(input) => create_dir(input),
        Io::CreateDirs(input) => create_dirs(input),
        Io::CreateFile(input) => create_file(input),
        Io::CreateFiles(input) => create_files(input),
        Io::ReadDir(input) => read_dir(input),
        Io::ReadFile(input) => read_file(input),
        Io::ReadFiles(input) => read_files(input),
        Io::RemoveDir(input) => remove_dir(input),
        Io::RemoveDirs(input) => remove_dirs(input),
        Io::RemoveFile(input) => remove_file(input),
        Io::RemoveFiles(input) => remove_files(input),
        Io::Rename(input) => rename(input),
    }
}

pub fn create_dir(input: Result<(), PathBuf>) -> io::Result<Io> {
    let Err(path) = input else {
        let kind = io::ErrorKind::InvalidInput;
        return Err(io::Error::new(kind, "missing directory path"));
    };

    fs::create_dir(path)?;

    Ok(Io::CreateDir(Ok(())))
}

pub fn create_dirs(input: Result<(), HashSet<PathBuf>>) -> io::Result<Io> {
    let Err(paths) = input else {
        let kind = io::ErrorKind::InvalidInput;
        return Err(io::Error::new(kind, "missing directory paths"));
    };

    for path in paths {
        fs::create_dir(path)?;
    }

    Ok(Io::CreateDirs(Ok(())))
}

pub fn create_file(input: Result<(), (PathBuf, Vec<u8>)>) -> io::Result<Io> {
    let Err((path, contents)) = input else {
        let kind = io::ErrorKind::InvalidInput;
        return Err(io::Error::new(kind, "missing file contents"));
    };

    fs::write(path, contents)?;

    Ok(Io::CreateFile(Ok(())))
}

pub fn create_files(input: Result<(), HashMap<PathBuf, Vec<u8>>>) -> io::Result<Io> {
    let Err(contents) = input else {
        let kind = io::ErrorKind::InvalidInput;
        return Err(io::Error::new(kind, "missing file contents"));
    };

    for (path, contents) in contents {
        fs::write(path, contents)?;
    }

    Ok(Io::CreateFiles(Ok(())))
}

pub fn read_dir(input: Result<HashSet<PathBuf>, PathBuf>) -> io::Result<Io> {
    let Err(path) = input else {
        let kind = io::ErrorKind::InvalidInput;
        return Err(io::Error::new(kind, "missing directory path"));
    };

    let mut paths = HashSet::new();
    let dir = fs::read_dir(path)?;

    for entry in dir {
        match entry {
            Ok(entry) => {
                paths.insert(entry.path());
            }
            Err(err) => {
                debug!("ignore invalid directory entry: {err}");
                continue;
            }
        };
    }

    Ok(Io::ReadDir(Ok(paths)))
}

pub fn read_file(input: Result<Vec<u8>, PathBuf>) -> io::Result<Io> {
    let Err(path) = input else {
        let kind = io::ErrorKind::InvalidInput;
        return Err(io::Error::new(kind, "missing file path"));
    };

    let contents = fs::read(path)?;

    Ok(Io::ReadFile(Ok(contents)))
}

pub fn read_files(input: Result<HashMap<PathBuf, Vec<u8>>, HashSet<PathBuf>>) -> io::Result<Io> {
    let Err(paths) = input else {
        let kind = io::ErrorKind::InvalidInput;
        return Err(io::Error::new(kind, "missing file paths"));
    };

    let mut contents = HashMap::new();

    for path in paths {
        let content = fs::read(&path)?;
        contents.insert(path, content);
    }

    Ok(Io::ReadFiles(Ok(contents)))
}

pub fn remove_dir(input: Result<(), PathBuf>) -> io::Result<Io> {
    let Err(path) = input else {
        let kind = io::ErrorKind::InvalidInput;
        return Err(io::Error::new(kind, "missing directory path"));
    };

    fs::remove_dir_all(path)?;

    Ok(Io::RemoveDir(Ok(())))
}

pub fn remove_dirs(input: Result<(), HashSet<PathBuf>>) -> io::Result<Io> {
    let Err(paths) = input else {
        let kind = io::ErrorKind::InvalidInput;
        return Err(io::Error::new(kind, "missing directory paths"));
    };

    for path in paths {
        fs::remove_dir_all(path)?;
    }

    Ok(Io::RemoveDirs(Ok(())))
}

pub fn remove_file(input: Result<(), PathBuf>) -> io::Result<Io> {
    let Err(path) = input else {
        let kind = io::ErrorKind::InvalidInput;
        return Err(io::Error::new(kind, "missing file path"));
    };

    fs::remove_file(path)?;

    Ok(Io::RemoveFile(Ok(())))
}

pub fn remove_files(input: Result<(), HashSet<PathBuf>>) -> io::Result<Io> {
    let Err(paths) = input else {
        let kind = io::ErrorKind::InvalidInput;
        return Err(io::Error::new(kind, "missing file paths"));
    };

    for path in paths {
        fs::remove_file(path)?;
    }

    Ok(Io::RemoveFiles(Ok(())))
}

pub fn rename(input: Result<(), Vec<(PathBuf, PathBuf)>>) -> io::Result<Io> {
    let Err(paths) = input else {
        let kind = io::ErrorKind::InvalidInput;
        return Err(io::Error::new(kind, "missing file paths"));
    };

    for (from, to) in paths {
        fs::rename(from, to)?;
    }

    Ok(Io::Rename(Ok(())))
}
