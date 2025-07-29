//! The standard, blocking filesystem runtime.

use std::{
    collections::{HashMap, HashSet},
    fs, io,
    path::PathBuf,
};

use log::debug;

use crate::io::FsIo;

/// The standard, blocking filesystem runtime handler.
///
/// This handler makes use of standard modules [`std::fs`] and
/// [`std::io`] to process [`FsIo`].
pub fn handle(input: FsIo) -> io::Result<FsIo> {
    match input {
        FsIo::CreateDir(input) => create_dir(input),
        FsIo::CreateDirs(input) => create_dirs(input),
        FsIo::CreateFile(input) => create_file(input),
        FsIo::CreateFiles(input) => create_files(input),
        FsIo::ReadDir(input) => read_dir(input),
        FsIo::ReadFile(input) => read_file(input),
        FsIo::ReadFiles(input) => read_files(input),
        FsIo::RemoveDir(input) => remove_dir(input),
        FsIo::RemoveDirs(input) => remove_dirs(input),
        FsIo::RemoveFile(input) => remove_file(input),
        FsIo::RemoveFiles(input) => remove_files(input),
        FsIo::Rename(input) => rename(input),
    }
}

pub fn create_dir(input: Result<(), PathBuf>) -> io::Result<FsIo> {
    let Err(path) = input else {
        let kind = io::ErrorKind::InvalidInput;
        return Err(io::Error::new(kind, "missing directory path"));
    };

    fs::create_dir(path)?;

    Ok(FsIo::CreateDir(Ok(())))
}

pub fn create_dirs(input: Result<(), HashSet<PathBuf>>) -> io::Result<FsIo> {
    let Err(paths) = input else {
        let kind = io::ErrorKind::InvalidInput;
        return Err(io::Error::new(kind, "missing directory paths"));
    };

    for path in paths {
        fs::create_dir(path)?;
    }

    Ok(FsIo::CreateDirs(Ok(())))
}

pub fn create_file(input: Result<(), (PathBuf, Vec<u8>)>) -> io::Result<FsIo> {
    let Err((path, contents)) = input else {
        let kind = io::ErrorKind::InvalidInput;
        return Err(io::Error::new(kind, "missing file contents"));
    };

    fs::write(path, contents)?;

    Ok(FsIo::CreateFile(Ok(())))
}

pub fn create_files(input: Result<(), HashMap<PathBuf, Vec<u8>>>) -> io::Result<FsIo> {
    let Err(contents) = input else {
        let kind = io::ErrorKind::InvalidInput;
        return Err(io::Error::new(kind, "missing file contents"));
    };

    for (path, contents) in contents {
        fs::write(path, contents)?;
    }

    Ok(FsIo::CreateFiles(Ok(())))
}

pub fn read_dir(input: Result<HashSet<PathBuf>, PathBuf>) -> io::Result<FsIo> {
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

    Ok(FsIo::ReadDir(Ok(paths)))
}

pub fn read_file(input: Result<Vec<u8>, PathBuf>) -> io::Result<FsIo> {
    let Err(path) = input else {
        let kind = io::ErrorKind::InvalidInput;
        return Err(io::Error::new(kind, "missing file path"));
    };

    let contents = fs::read(path)?;

    Ok(FsIo::ReadFile(Ok(contents)))
}

pub fn read_files(input: Result<HashMap<PathBuf, Vec<u8>>, HashSet<PathBuf>>) -> io::Result<FsIo> {
    let Err(paths) = input else {
        let kind = io::ErrorKind::InvalidInput;
        return Err(io::Error::new(kind, "missing file paths"));
    };

    let mut contents = HashMap::new();

    for path in paths {
        let content = fs::read(&path)?;
        contents.insert(path, content);
    }

    Ok(FsIo::ReadFiles(Ok(contents)))
}

pub fn remove_dir(input: Result<(), PathBuf>) -> io::Result<FsIo> {
    let Err(path) = input else {
        let kind = io::ErrorKind::InvalidInput;
        return Err(io::Error::new(kind, "missing directory path"));
    };

    fs::remove_dir_all(path)?;

    Ok(FsIo::RemoveDir(Ok(())))
}

pub fn remove_dirs(input: Result<(), HashSet<PathBuf>>) -> io::Result<FsIo> {
    let Err(paths) = input else {
        let kind = io::ErrorKind::InvalidInput;
        return Err(io::Error::new(kind, "missing directory paths"));
    };

    for path in paths {
        fs::remove_dir_all(path)?;
    }

    Ok(FsIo::RemoveDirs(Ok(())))
}

pub fn remove_file(input: Result<(), PathBuf>) -> io::Result<FsIo> {
    let Err(path) = input else {
        let kind = io::ErrorKind::InvalidInput;
        return Err(io::Error::new(kind, "missing file path"));
    };

    fs::remove_file(path)?;

    Ok(FsIo::RemoveFile(Ok(())))
}

pub fn remove_files(input: Result<(), HashSet<PathBuf>>) -> io::Result<FsIo> {
    let Err(paths) = input else {
        let kind = io::ErrorKind::InvalidInput;
        return Err(io::Error::new(kind, "missing file paths"));
    };

    for path in paths {
        fs::remove_file(path)?;
    }

    Ok(FsIo::RemoveFiles(Ok(())))
}

pub fn rename(input: Result<(), Vec<(PathBuf, PathBuf)>>) -> io::Result<FsIo> {
    let Err(paths) = input else {
        let kind = io::ErrorKind::InvalidInput;
        return Err(io::Error::new(kind, "missing file paths"));
    };

    for (from, to) in paths {
        fs::rename(from, to)?;
    }

    Ok(FsIo::Rename(Ok(())))
}
