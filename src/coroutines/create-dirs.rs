//! Module dedicated to the [`CreateDirs`] I/O-free coroutine.

use std::{collections::HashSet, path::PathBuf};

use log::{debug, trace};
use thiserror::Error;

use crate::io::FsIo;

#[derive(Clone, Debug, Error)]
pub enum CreateDirsError {
    #[error("Missing input: paths missing or already consumed")]
    MissingInput,
    #[error("Invalid argument: expected {0}, got {1:?}")]
    InvalidArgument(&'static str, FsIo),
}

#[derive(Clone, Debug)]
pub enum CreateDirsResult {
    Ok,
    Err(CreateDirsError),
    Io(FsIo),
}

/// I/O-free coroutine for creating a dirsectory.
#[derive(Debug)]
pub struct CreateDirs {
    paths: Option<HashSet<PathBuf>>,
}

impl CreateDirs {
    /// Creates a new coroutine from the given dirsectory path.
    pub fn new(paths: impl IntoIterator<Item = PathBuf>) -> Self {
        let paths = Some(paths.into_iter().collect());
        Self { paths }
    }

    /// Makes create dirs progress.
    pub fn resume(&mut self, arg: Option<FsIo>) -> CreateDirsResult {
        let Some(arg) = arg else {
            let Some(paths) = self.paths.take() else {
                return CreateDirsResult::Err(CreateDirsError::MissingInput);
            };

            trace!("wants I/O to create directories: {paths:?}");
            return CreateDirsResult::Io(FsIo::CreateDirs(Err(paths)));
        };

        debug!("resume after creating directories");

        let FsIo::CreateDirs(io) = arg else {
            let err = CreateDirsError::InvalidArgument("create dirs output", arg);
            return CreateDirsResult::Err(err);
        };

        match io {
            Ok(()) => CreateDirsResult::Ok,
            Err(path) => CreateDirsResult::Io(FsIo::CreateDirs(Err(path))),
        }
    }
}
