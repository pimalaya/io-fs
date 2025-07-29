//! I/O-free coroutine to remove a filesystem file.

use std::path::PathBuf;

use log::{debug, trace};

use crate::{
    error::{FsError, FsResult},
    io::FsIo,
};

/// I/O-free coroutine to remove a filesystem file.
#[derive(Debug)]
pub struct RemoveFile {
    path: Option<PathBuf>,
}

impl RemoveFile {
    /// Creates a new coroutine from the given file path.
    pub fn new(path: impl Into<PathBuf>) -> Self {
        let path = Some(path.into());
        Self { path }
    }

    /// Makes remove file progress.
    pub fn resume(&mut self, arg: Option<FsIo>) -> FsResult {
        let Some(arg) = arg else {
            let Some(path) = self.path.take() else {
                return FsResult::Err(FsError::MissingInput);
            };

            trace!("wants I/O to remove fileectory at {}", path.display());
            return FsResult::Io(FsIo::RemoveFile(Err(path)));
        };

        debug!("resume after creating fileectory");

        let FsIo::RemoveFile(io) = arg else {
            let err = FsError::InvalidArgument("remove file output", arg);
            return FsResult::Err(err);
        };

        match io {
            Ok(()) => FsResult::Ok(()),
            Err(path) => FsResult::Io(FsIo::RemoveFile(Err(path))),
        }
    }
}
