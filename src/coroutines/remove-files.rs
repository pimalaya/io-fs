//! I/O-free coroutine to remove multiple filesystem files.

use std::{collections::HashSet, path::PathBuf};

use log::{debug, trace};

use crate::{
    error::{FsError, FsResult},
    io::FsIo,
};

/// I/O-free coroutine to remove multiple filesystem files.
#[derive(Debug)]
pub struct RemoveFiles {
    paths: Option<HashSet<PathBuf>>,
}

impl RemoveFiles {
    /// Creates a new coroutine from the given file paths.
    pub fn new(paths: impl IntoIterator<Item = PathBuf>) -> Self {
        let paths = Some(paths.into_iter().collect());
        Self { paths }
    }

    /// Makes remove files progress.
    pub fn resume(&mut self, arg: Option<FsIo>) -> FsResult {
        let Some(arg) = arg else {
            let Some(paths) = self.paths.take() else {
                return FsResult::Err(FsError::MissingInput);
            };

            trace!("wants I/O to remove fileectories: {paths:?}");
            return FsResult::Io(FsIo::RemoveFiles(Err(paths)));
        };

        debug!("resume after creating fileectories");

        let FsIo::RemoveFiles(io) = arg else {
            let err = FsError::InvalidArgument("remove files output", arg);
            return FsResult::Err(err);
        };

        match io {
            Ok(()) => FsResult::Ok(()),
            Err(path) => FsResult::Io(FsIo::RemoveFiles(Err(path))),
        }
    }
}
