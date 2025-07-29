//! I/O-free coroutine to remove a filesystem directory.

use std::path::PathBuf;

use log::{debug, trace};

use crate::{
    error::{FsError, FsResult},
    io::FsIo,
};

/// I/O-free coroutine to remove a filesystem directory.
#[derive(Debug)]
pub struct RemoveDir {
    path: Option<PathBuf>,
}

impl RemoveDir {
    /// Creates a new coroutine from the given directory path.
    pub fn new(path: impl Into<PathBuf>) -> Self {
        let path = Some(path.into());
        Self { path }
    }

    /// Makes remove dir progress.
    pub fn resume(&mut self, arg: Option<FsIo>) -> FsResult {
        let Some(arg) = arg else {
            let Some(path) = self.path.take() else {
                return FsResult::Err(FsError::MissingInput);
            };

            trace!("wants I/O to remove directory at {}", path.display());
            return FsResult::Io(FsIo::RemoveDir(Err(path)));
        };

        debug!("resume after creating directory");

        let FsIo::RemoveDir(io) = arg else {
            let err = FsError::InvalidArgument("remove dir output", arg);
            return FsResult::Err(err);
        };

        match io {
            Ok(()) => FsResult::Ok(()),
            Err(path) => FsResult::Io(FsIo::RemoveDir(Err(path))),
        }
    }
}
