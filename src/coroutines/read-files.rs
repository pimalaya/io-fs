//! I/O-free coroutine to read multiple filesystem files contents.

use std::{
    collections::{HashMap, HashSet},
    path::PathBuf,
};

use log::{debug, trace};

use crate::{
    error::{FsError, FsResult},
    io::FsIo,
};

/// I/O-free coroutine to read multiple filesystem files contents.
#[derive(Debug)]
pub struct ReadFiles {
    paths: Option<HashSet<PathBuf>>,
}

impl ReadFiles {
    /// Creates a new coroutine from the given file paths.
    pub fn new(paths: impl IntoIterator<Item = impl Into<PathBuf>>) -> Self {
        let paths = Some(paths.into_iter().map(Into::into).collect());
        Self { paths }
    }

    /// Makes the coroutine progress.
    pub fn resume(&mut self, arg: Option<FsIo>) -> FsResult<HashMap<PathBuf, Vec<u8>>> {
        let Some(arg) = arg else {
            let Some(path) = self.paths.take() else {
                return FsResult::Err(FsError::MissingInput);
            };

            trace!("wants I/O to read files");
            return FsResult::Io(FsIo::ReadFiles(Err(path)));
        };

        debug!("resume after reading files");

        let FsIo::ReadFiles(io) = arg else {
            let err = FsError::InvalidArgument("read files output", arg);
            return FsResult::Err(err);
        };

        match io {
            Ok(contents) => FsResult::Ok(contents),
            Err(path) => FsResult::Io(FsIo::ReadFiles(Err(path))),
        }
    }
}
