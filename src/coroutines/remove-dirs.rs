//! Module dedicated to the [`RemoveDirs`] I/O-free coroutine.

use std::{collections::HashSet, path::PathBuf};

use log::{debug, trace};
use thiserror::Error;

use crate::io::FsIo;

#[derive(Clone, Debug, Error)]
pub enum RemoveDirsError {
    #[error("Missing input: paths missing or already consumed")]
    MissingInput,
    #[error("Invalid argument: expected {0}, got {1:?}")]
    InvalidArgument(&'static str, FsIo),
}

#[derive(Clone, Debug)]
pub enum RemoveDirsResult {
    Ok,
    Err(RemoveDirsError),
    Io(FsIo),
}

/// I/O-free coroutine for creating a dirsectory.
#[derive(Debug)]
pub struct RemoveDirs {
    paths: Option<HashSet<PathBuf>>,
}

impl RemoveDirs {
    /// Removes a new coroutine from the given dirsectory path.
    pub fn new(paths: impl IntoIterator<Item = PathBuf>) -> Self {
        let paths = Some(paths.into_iter().collect());
        Self { paths }
    }

    /// Makes remove dirs progress.
    pub fn resume(&mut self, arg: Option<FsIo>) -> RemoveDirsResult {
        let Some(arg) = arg else {
            let Some(paths) = self.paths.take() else {
                return RemoveDirsResult::Err(RemoveDirsError::MissingInput);
            };

            trace!("wants I/O to remove directories: {paths:?}");
            return RemoveDirsResult::Io(FsIo::RemoveDirs(Err(paths)));
        };

        debug!("resume after creating directories");

        let FsIo::RemoveDirs(io) = arg else {
            let err = RemoveDirsError::InvalidArgument("remove dirs output", arg);
            return RemoveDirsResult::Err(err);
        };

        match io {
            Ok(()) => RemoveDirsResult::Ok,
            Err(path) => RemoveDirsResult::Io(FsIo::RemoveDirs(Err(path))),
        }
    }
}
