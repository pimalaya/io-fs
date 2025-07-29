//! I/O-free coroutine to create multiple filesystem directories.

use std::{collections::HashSet, path::PathBuf};

use log::{debug, trace};

use crate::{
    error::{FsError, FsResult},
    io::FsIo,
};

/// I/O-free coroutine to create multiple filesystem directories.
#[derive(Debug)]
pub struct CreateDirs {
    paths: Option<HashSet<PathBuf>>,
}

impl CreateDirs {
    /// Creates a new coroutine from the given directory path.
    pub fn new(paths: impl IntoIterator<Item = PathBuf>) -> Self {
        let paths = Some(paths.into_iter().collect());
        Self { paths }
    }

    /// Makes the coroutine progress.
    pub fn resume(&mut self, arg: Option<FsIo>) -> FsResult {
        let Some(arg) = arg else {
            let Some(paths) = self.paths.take() else {
                return FsResult::Err(FsError::MissingInput);
            };

            trace!("wants I/O to create directories: {paths:?}");
            return FsResult::Io(FsIo::CreateDirs(Err(paths)));
        };

        debug!("resume after creating directories");

        let FsIo::CreateDirs(io) = arg else {
            let err = FsError::InvalidArgument("create dirs output", arg);
            return FsResult::Err(err);
        };

        match io {
            Ok(()) => FsResult::Ok(()),
            Err(path) => FsResult::Io(FsIo::CreateDirs(Err(path))),
        }
    }
}
