//! I/O-free coroutine to create a filesystem directory.

use std::path::PathBuf;

use log::{debug, trace};

use crate::{
    error::{FsError, FsResult},
    io::FsIo,
};

/// I/O-free coroutine to create a filesystem directory.
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

    /// Makes the coroutine progress.
    pub fn resume(&mut self, arg: Option<FsIo>) -> FsResult {
        let Some(arg) = arg else {
            let Some(path) = self.path.take() else {
                return FsResult::Err(FsError::MissingInput);
            };

            trace!("wants I/O to create directory at {}", path.display());
            return FsResult::Io(FsIo::CreateDir(Err(path)));
        };

        debug!("resume after creating directory");

        let FsIo::CreateDir(io) = arg else {
            let err = FsError::InvalidArgument("create dir output", arg);
            return FsResult::Err(err);
        };

        match io {
            Ok(()) => FsResult::Ok(()),
            Err(path) => FsResult::Io(FsIo::CreateDir(Err(path))),
        }
    }
}
