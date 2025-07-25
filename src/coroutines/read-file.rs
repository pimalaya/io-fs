//! Module dedicated to the [`ReadFile`] I/O-free coroutine.

use std::path::PathBuf;

use log::{debug, trace};
use thiserror::Error;

use crate::io::FsIo;

#[derive(Clone, Debug, Error)]
pub enum ReadFileError {
    #[error("Missing input: path missing or already consumed")]
    MissingInput,
    #[error("Invalid argument: expected {0}, got {1:?}")]
    InvalidArgument(&'static str, FsIo),
}

#[derive(Clone, Debug)]
pub enum ReadFileResult {
    Ok(Vec<u8>),
    Err(ReadFileError),
    Io(FsIo),
}

/// I/O-free coroutine for reading file contents.
#[derive(Debug)]
pub struct ReadFile {
    path: Option<PathBuf>,
}

impl ReadFile {
    /// Reads a new coroutine from the given file path.
    pub fn new(path: impl Into<PathBuf>) -> Self {
        let path = Some(path.into());
        Self { path }
    }

    /// Makes the coroutine progress.
    pub fn resume(&mut self, arg: Option<FsIo>) -> ReadFileResult {
        let Some(arg) = arg else {
            let Some(path) = self.path.take() else {
                return ReadFileResult::Err(ReadFileError::MissingInput);
            };

            trace!("wants I/O to read file at {}", path.display());
            return ReadFileResult::Io(FsIo::ReadFile(Err(path)));
        };

        debug!("resume after reading file");

        let FsIo::ReadFile(io) = arg else {
            let err = ReadFileError::InvalidArgument("read file output", arg);
            return ReadFileResult::Err(err);
        };

        match io {
            Ok(contents) => ReadFileResult::Ok(contents),
            Err(path) => ReadFileResult::Io(FsIo::ReadFile(Err(path))),
        }
    }
}
