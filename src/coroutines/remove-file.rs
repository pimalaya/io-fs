//! Module dedicated to the [`RemoveFile`] I/O-free coroutine.

use std::path::PathBuf;

use log::{debug, trace};
use thiserror::Error;

use crate::io::FsIo;

#[derive(Clone, Debug, Error)]
pub enum RemoveFileError {
    #[error("Missing input: path missing or already consumed")]
    MissingInput,
    #[error("Invalid argument: expected {0}, got {1:?}")]
    InvalidArgument(&'static str, FsIo),
}

#[derive(Clone, Debug)]
pub enum RemoveFileResult {
    Ok,
    Err(RemoveFileError),
    Io(FsIo),
}

/// I/O-free coroutine for creating a fileectory.
#[derive(Debug)]
pub struct RemoveFile {
    path: Option<PathBuf>,
}

impl RemoveFile {
    /// Removes a new coroutine from the given fileectory path.
    pub fn new(path: impl Into<PathBuf>) -> Self {
        let path = Some(path.into());
        Self { path }
    }

    /// Makes remove file progress.
    pub fn resume(&mut self, arg: Option<FsIo>) -> RemoveFileResult {
        let Some(arg) = arg else {
            let Some(path) = self.path.take() else {
                return RemoveFileResult::Err(RemoveFileError::MissingInput);
            };

            trace!("wants I/O to remove fileectory at {}", path.display());
            return RemoveFileResult::Io(FsIo::RemoveFile(Err(path)));
        };

        debug!("resume after creating fileectory");

        let FsIo::RemoveFile(io) = arg else {
            let err = RemoveFileError::InvalidArgument("remove file output", arg);
            return RemoveFileResult::Err(err);
        };

        match io {
            Ok(()) => RemoveFileResult::Ok,
            Err(path) => RemoveFileResult::Io(FsIo::RemoveFile(Err(path))),
        }
    }
}
