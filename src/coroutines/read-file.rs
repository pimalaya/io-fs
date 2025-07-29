//! I/O-free coroutine to read filesystem file contents.

use std::path::PathBuf;

use log::{debug, trace};

use crate::{
    error::{FsError, FsResult},
    io::FsIo,
};

/// I/O-free coroutine to read filesystem file contents.
#[derive(Debug)]
pub struct ReadFile {
    path: Option<PathBuf>,
}

impl ReadFile {
    /// Creates a new coroutine from the given file path.
    pub fn new(path: impl Into<PathBuf>) -> Self {
        let path = Some(path.into());
        Self { path }
    }

    /// Makes the coroutine progress.
    pub fn resume(&mut self, arg: Option<FsIo>) -> FsResult<Vec<u8>> {
        let Some(arg) = arg else {
            let Some(path) = self.path.take() else {
                return FsResult::Err(FsError::MissingInput);
            };

            trace!("wants I/O to read file at {}", path.display());
            return FsResult::Io(FsIo::ReadFile(Err(path)));
        };

        debug!("resume after reading file");

        let FsIo::ReadFile(io) = arg else {
            let err = FsError::InvalidArgument("read file output", arg);
            return FsResult::Err(err);
        };

        match io {
            Ok(contents) => FsResult::Ok(contents),
            Err(path) => FsResult::Io(FsIo::ReadFile(Err(path))),
        }
    }
}
