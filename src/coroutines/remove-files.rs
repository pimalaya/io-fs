//! Module dedicated to the [`RemoveFiles`] I/O-free coroutine.

use std::{collections::HashSet, path::PathBuf};

use log::{debug, trace};
use thiserror::Error;

use crate::io::FsIo;

#[derive(Clone, Debug, Error)]
pub enum RemoveFilesError {
    #[error("Missing input: paths missing or already consumed")]
    MissingInput,
    #[error("Invalid argument: expected {0}, got {1:?}")]
    InvalidArgument(&'static str, FsIo),
}

#[derive(Clone, Debug)]
pub enum RemoveFilesResult {
    Ok,
    Err(RemoveFilesError),
    Io(FsIo),
}

/// I/O-free coroutine for creating a filesectory.
#[derive(Debug)]
pub struct RemoveFiles {
    paths: Option<HashSet<PathBuf>>,
}

impl RemoveFiles {
    /// Removes a new coroutine from the given filesectory path.
    pub fn new(paths: impl IntoIterator<Item = PathBuf>) -> Self {
        let paths = Some(paths.into_iter().collect());
        Self { paths }
    }

    /// Makes remove files progress.
    pub fn resume(&mut self, arg: Option<FsIo>) -> RemoveFilesResult {
        let Some(arg) = arg else {
            let Some(paths) = self.paths.take() else {
                return RemoveFilesResult::Err(RemoveFilesError::MissingInput);
            };

            trace!("wants I/O to remove fileectories: {paths:?}");
            return RemoveFilesResult::Io(FsIo::RemoveFiles(Err(paths)));
        };

        debug!("resume after creating fileectories");

        let FsIo::RemoveFiles(io) = arg else {
            let err = RemoveFilesError::InvalidArgument("remove files output", arg);
            return RemoveFilesResult::Err(err);
        };

        match io {
            Ok(()) => RemoveFilesResult::Ok,
            Err(path) => RemoveFilesResult::Io(FsIo::RemoveFiles(Err(path))),
        }
    }
}
