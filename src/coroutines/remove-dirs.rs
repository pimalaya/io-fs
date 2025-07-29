//! I/O-free coroutine to remove multiple filesystem directories.

use std::{collections::HashSet, path::PathBuf};

use log::{debug, trace};

use crate::{
    error::{FsError, FsResult},
    io::FsIo,
};

/// I/O-free coroutine to remove multiple filesystem directories.
#[derive(Debug)]
pub struct RemoveDirs {
    paths: Option<HashSet<PathBuf>>,
}

impl RemoveDirs {
    /// Creates a new coroutine from the given directory paths.
    pub fn new(paths: impl IntoIterator<Item = PathBuf>) -> Self {
        let paths = Some(paths.into_iter().collect());
        Self { paths }
    }

    /// Makes remove dirs progress.
    pub fn resume(&mut self, arg: Option<FsIo>) -> FsResult {
        let Some(arg) = arg else {
            let Some(paths) = self.paths.take() else {
                return FsResult::Err(FsError::MissingInput);
            };

            trace!("wants I/O to remove directories: {paths:?}");
            return FsResult::Io(FsIo::RemoveDirs(Err(paths)));
        };

        debug!("resume after creating directories");

        let FsIo::RemoveDirs(io) = arg else {
            let err = FsError::InvalidArgument("remove dirs output", arg);
            return FsResult::Err(err);
        };

        match io {
            Ok(()) => FsResult::Ok(()),
            Err(path) => FsResult::Io(FsIo::RemoveDirs(Err(path))),
        }
    }
}
