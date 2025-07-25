//! Module dedicated to the [`CreateDir`] I/O-free coroutine.

use std::path::PathBuf;

use log::{debug, trace};
use thiserror::Error;

use crate::io::FsIo;

#[derive(Clone, Debug, Error)]
pub enum CreateDirError {
    #[error("Missing input: path missing or already consumed")]
    MissingInput,
    #[error("Invalid argument: expected {0}, got {1:?}")]
    InvalidArgument(&'static str, FsIo),
}

#[derive(Clone, Debug)]
pub enum CreateDirResult {
    Ok,
    Err(CreateDirError),
    Io(FsIo),
}

/// I/O-free coroutine for creating a directory.
#[derive(Debug)]
pub struct CreateDir {
    path: Option<PathBuf>,
}

impl CreateDir {
    /// Creates a new coroutine from the given directory path.
    pub fn new(path: impl Into<PathBuf>) -> Self {
        let path = Some(path.into());
        Self { path }
    }

    /// Makes create dir progress.
    pub fn resume(&mut self, arg: Option<FsIo>) -> CreateDirResult {
        let Some(arg) = arg else {
            let Some(path) = self.path.take() else {
                return CreateDirResult::Err(CreateDirError::MissingInput);
            };

            trace!("wants I/O to create directory at {}", path.display());
            return CreateDirResult::Io(FsIo::CreateDir(Err(path)));
        };

        debug!("resume after creating directory");

        let FsIo::CreateDir(io) = arg else {
            let err = CreateDirError::InvalidArgument("create dir output", arg);
            return CreateDirResult::Err(err);
        };

        match io {
            Ok(()) => CreateDirResult::Ok,
            Err(path) => CreateDirResult::Io(FsIo::CreateDir(Err(path))),
        }
    }
}
