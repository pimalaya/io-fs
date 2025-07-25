//! Module dedicated to the [`ReadFiles`] I/O-free coroutine.

use std::{
    collections::{HashMap, HashSet},
    path::PathBuf,
};

use log::{debug, trace};
use thiserror::Error;

use crate::io::FsIo;

#[derive(Clone, Debug, Error)]
pub enum ReadFilesError {
    #[error("Missing input: path missing or already consumed")]
    MissingInput,
    #[error("Invalid argument: expected {0}, got {1:?}")]
    InvalidArgument(&'static str, FsIo),
}

#[derive(Clone, Debug)]
pub enum ReadFilesResult {
    Ok(HashMap<PathBuf, Vec<u8>>),
    Err(ReadFilesError),
    Io(FsIo),
}

/// I/O-free coroutine for reading files contents.
#[derive(Debug)]
pub struct ReadFiles {
    paths: Option<HashSet<PathBuf>>,
}

impl ReadFiles {
    /// Makes the coroutine progress.
    pub fn resume(&mut self, arg: Option<FsIo>) -> ReadFilesResult {
        let Some(arg) = arg else {
            let Some(path) = self.paths.take() else {
                return ReadFilesResult::Err(ReadFilesError::MissingInput);
            };

            trace!("wants I/O to read files");
            return ReadFilesResult::Io(FsIo::ReadFiles(Err(path)));
        };

        debug!("resume after reading files");

        let FsIo::ReadFiles(io) = arg else {
            let err = ReadFilesError::InvalidArgument("read files output", arg);
            return ReadFilesResult::Err(err);
        };

        match io {
            Ok(contents) => ReadFilesResult::Ok(contents),
            Err(path) => ReadFilesResult::Io(FsIo::ReadFiles(Err(path))),
        }
    }
}
