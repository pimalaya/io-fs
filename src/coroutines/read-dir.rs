//! I/O-free coroutine to read entries contained inside a filesystem
//! directory.

use std::{collections::HashSet, path::PathBuf};

use log::{debug, trace};

use crate::{
    error::{FsError, FsResult},
    io::FsIo,
};

/// I/O-free coroutine to read entries contained inside a filesystem
/// directory.
#[derive(Debug)]
pub struct ReadDir {
    path: Option<PathBuf>,
}

impl ReadDir {
    /// Creates a new coroutine from the given directory path.
    pub fn new(path: impl Into<PathBuf>) -> Self {
        let input = Some(path.into());
        Self { path: input }
    }

    /// Makes the coroutine progress.
    pub fn resume(&mut self, arg: Option<FsIo>) -> FsResult<HashSet<PathBuf>> {
        let Some(arg) = arg else {
            let Some(path) = self.path.take() else {
                return FsResult::Err(FsError::MissingInput);
            };

            trace!("wants I/O to read directory at {}", path.display());
            return FsResult::Io(FsIo::ReadDir(Err(path)));
        };

        debug!("resume after reading directory");

        let FsIo::ReadDir(io) = arg else {
            let err = FsError::InvalidArgument("read dir output", arg);
            return FsResult::Err(err);
        };

        match io {
            Ok(paths) => FsResult::Ok(paths),
            Err(path) => FsResult::Io(FsIo::ReadDir(Err(path))),
        }
    }
}
