//! Module dedicated to the [`CreateFiles`] I/O-free coroutine.

use std::{collections::HashMap, path::PathBuf};

use log::{debug, trace};
use thiserror::Error;

use crate::io::FsIo;

#[derive(Clone, Debug, Error)]
pub enum CreateFilesError {
    #[error("Missing input: contents missing or already consumed")]
    MissingInput,
    #[error("Invalid argument: expected {0}, got {1:?}")]
    InvalidArgument(&'static str, FsIo),
}

#[derive(Clone, Debug)]
pub enum CreateFilesResult {
    Ok,
    Err(CreateFilesError),
    Io(FsIo),
}

/// I/O-free coroutine for creating a file.
#[derive(Debug)]
pub struct CreateFiles {
    contents: Option<HashMap<PathBuf, Vec<u8>>>,
}

impl CreateFiles {
    /// Creates a new coroutine from the given file path and contents.
    pub fn new(
        contents: impl IntoIterator<Item = (impl Into<PathBuf>, impl IntoIterator<Item = u8>)>,
    ) -> Self {
        let contents = contents
            .into_iter()
            .map(|(path, contents)| (path.into(), contents.into_iter().collect()));
        let contents = Some(contents.collect());
        Self { contents }
    }

    /// Makes create files progress.
    pub fn resume(&mut self, arg: Option<FsIo>) -> CreateFilesResult {
        let Some(arg) = arg else {
            let Some(contents) = self.contents.take() else {
                return CreateFilesResult::Err(CreateFilesError::MissingInput);
            };

            trace!("wants I/O to create files");
            return CreateFilesResult::Io(FsIo::CreateFiles(Err(contents)));
        };

        debug!("resume after creating files");

        let FsIo::CreateFiles(io) = arg else {
            let err = CreateFilesError::InvalidArgument("create files output", arg);
            return CreateFilesResult::Err(err);
        };

        match io {
            Ok(()) => CreateFilesResult::Ok,
            Err(path) => CreateFilesResult::Io(FsIo::CreateFiles(Err(path))),
        }
    }
}
