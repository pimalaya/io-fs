//! I/O-free coroutine to create a filesystem file.

use std::path::PathBuf;

use log::{debug, trace};

use crate::{
    error::{FsError, FsResult},
    io::FsIo,
};

/// I/O-free coroutine to create a filesystem file.
#[derive(Debug)]
pub struct CreateFile {
    contents: Option<(PathBuf, Vec<u8>)>,
}

impl CreateFile {
    /// Creates a new coroutine from the given file path and contents.
    pub fn new(path: impl Into<PathBuf>, contents: impl IntoIterator<Item = u8>) -> Self {
        let contents = contents.into_iter().collect();
        let contents = Some((path.into(), contents));
        Self { contents }
    }

    /// Makes the coroutine progress.
    pub fn resume(&mut self, arg: Option<FsIo>) -> FsResult {
        let Some(arg) = arg else {
            let Some((path, contents)) = self.contents.take() else {
                return FsResult::Err(FsError::MissingInput);
            };

            trace!("wants I/O to create file at {}", path.display());
            return FsResult::Io(FsIo::CreateFile(Err((path, contents))));
        };

        debug!("resume after creating file");

        let FsIo::CreateFile(io) = arg else {
            let err = FsError::InvalidArgument("create file output", arg);
            return FsResult::Err(err);
        };

        match io {
            Ok(()) => FsResult::Ok(()),
            Err(path) => FsResult::Io(FsIo::CreateFile(Err(path))),
        }
    }
}
