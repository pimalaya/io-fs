//! Module dedicated to the [`RemoveDir`] I/O-free coroutine.

use std::path::PathBuf;

use log::{debug, trace};
use thiserror::Error;

use crate::io::FsIo;

#[derive(Clone, Debug, Error)]
pub enum RemoveDirError {
    #[error("Missing input: path missing or already consumed")]
    MissingInput,
    #[error("Invalid argument: expected {0}, got {1:?}")]
    InvalidArgument(&'static str, FsIo),
}

#[derive(Clone, Debug)]
pub enum RemoveDirResult {
    Ok,
    Err(RemoveDirError),
    Io(FsIo),
}

/// I/O-free coroutine for creating a directory.
#[derive(Debug)]
pub struct RemoveDir {
    path: Option<PathBuf>,
}

impl RemoveDir {
    /// Removes a new coroutine from the given directory path.
    pub fn new(path: impl Into<PathBuf>) -> Self {
        let path = Some(path.into());
        Self { path }
    }

    /// Makes remove dir progress.
    pub fn resume(&mut self, arg: Option<FsIo>) -> RemoveDirResult {
        let Some(arg) = arg else {
            let Some(path) = self.path.take() else {
                return RemoveDirResult::Err(RemoveDirError::MissingInput);
            };

            trace!("wants I/O to remove directory at {}", path.display());
            return RemoveDirResult::Io(FsIo::RemoveDir(Err(path)));
        };

        debug!("resume after creating directory");

        let FsIo::RemoveDir(io) = arg else {
            let err = RemoveDirError::InvalidArgument("remove dir output", arg);
            return RemoveDirResult::Err(err);
        };

        match io {
            Ok(()) => RemoveDirResult::Ok,
            Err(path) => RemoveDirResult::Io(FsIo::RemoveDir(Err(path))),
        }
    }
}
