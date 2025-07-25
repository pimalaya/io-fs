//! Module dedicated to the [`ReadDir`] I/O-free coroutine.

use std::{collections::HashSet, path::PathBuf};

use log::{debug, trace};
use thiserror::Error;

use crate::io::FsIo;

#[derive(Clone, Debug, Error)]
pub enum ReadDirError {
    #[error("Missing input: path missing or already consumed")]
    MissingInput,
    #[error("Invalid argument: expected {0}, got {1:?}")]
    InvalidArgument(&'static str, FsIo),
}

#[derive(Clone, Debug)]
pub enum ReadDirResult {
    Ok(HashSet<PathBuf>),
    Err(ReadDirError),
    Io(FsIo),
}

/// I/O-free coroutine for reading directory entries.
#[derive(Debug)]
pub struct ReadDir {
    path: Option<PathBuf>,
}

impl ReadDir {
    /// Reads a new coroutine from the given directory path.
    pub fn new(path: impl Into<PathBuf>) -> Self {
        let input = Some(path.into());
        Self { path: input }
    }

    /// Makes the coroutine progress.
    pub fn resume(&mut self, arg: Option<FsIo>) -> ReadDirResult {
        let Some(arg) = arg else {
            let Some(path) = self.path.take() else {
                return ReadDirResult::Err(ReadDirError::MissingInput);
            };

            trace!("wants I/O to read directory at {}", path.display());
            return ReadDirResult::Io(FsIo::ReadDir(Err(path)));
        };

        debug!("resume after reading directory");

        let FsIo::ReadDir(io) = arg else {
            let err = ReadDirError::InvalidArgument("read dir output", arg);
            return ReadDirResult::Err(err);
        };

        match io {
            Ok(paths) => ReadDirResult::Ok(paths),
            Err(path) => ReadDirResult::Io(FsIo::ReadDir(Err(path))),
        }
    }
}
