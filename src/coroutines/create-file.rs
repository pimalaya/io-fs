//! Module dedicated to the [`CreateFile`] I/O-free coroutine.

use std::path::PathBuf;

use log::{debug, trace};
use thiserror::Error;

use crate::io::FsIo;

#[derive(Clone, Debug, Error)]
pub enum CreateFileError {
    #[error("Missing input: contents missing or already consumed")]
    MissingInput,
    #[error("Invalid argument: expected {0}, got {1:?}")]
    InvalidArgument(&'static str, FsIo),
}

#[derive(Clone, Debug)]
pub enum CreateFileResult {
    Ok,
    Err(CreateFileError),
    Io(FsIo),
}

/// I/O-free coroutine for creating a fileectory.
#[derive(Debug)]
pub struct CreateFile {
    contents: Option<(PathBuf, Vec<u8>)>,
}

impl CreateFile {
    /// Creates a new coroutine from the given fileectory path.
    pub fn new(path: impl Into<PathBuf>, contents: impl IntoIterator<Item = u8>) -> Self {
        let contents = contents.into_iter().collect();
        let contents = Some((path.into(), contents));
        Self { contents }
    }

    /// Makes create file progress.
    pub fn resume(&mut self, arg: Option<FsIo>) -> CreateFileResult {
        let Some(arg) = arg else {
            let Some((path, contents)) = self.contents.take() else {
                return CreateFileResult::Err(CreateFileError::MissingInput);
            };

            trace!("wants I/O to create file at {}", path.display());
            return CreateFileResult::Io(FsIo::CreateFile(Err((path, contents))));
        };

        debug!("resume after creating file");

        let FsIo::CreateFile(io) = arg else {
            let err = CreateFileError::InvalidArgument("create file output", arg);
            return CreateFileResult::Err(err);
        };

        match io {
            Ok(()) => CreateFileResult::Ok,
            Err(path) => CreateFileResult::Io(FsIo::CreateFile(Err(path))),
        }
    }
}
