//! Module dedicated to the [`Rename`] I/O-free coroutine.

use std::path::PathBuf;

use log::{debug, trace};
use thiserror::Error;

use crate::io::FsIo;

#[derive(Clone, Debug, Error)]
pub enum RenameError {
    #[error("Missing input: paths missing or already consumed")]
    MissingInput,
    #[error("Invalid argument: expected {0}, got {1:?}")]
    InvalidArgument(&'static str, FsIo),
}

#[derive(Clone, Debug)]
pub enum RenameResult {
    Ok,
    Err(RenameError),
    Io(FsIo),
}

/// I/O-free coroutine for renaming files or directories.
#[derive(Debug)]
pub struct Rename {
    sources: Option<Vec<(PathBuf, PathBuf)>>,
}

impl Rename {
    /// Reads a new coroutine from the given source and destination paths.
    pub fn new(
        sources: impl IntoIterator<Item = (impl Into<PathBuf>, impl Into<PathBuf>)>,
    ) -> Self {
        let sources = sources
            .into_iter()
            .map(|(from, to)| (from.into(), to.into()))
            .collect();

        Self {
            sources: Some(sources),
        }
    }

    /// Makes the coroutine progress.
    pub fn resume(&mut self, arg: Option<FsIo>) -> RenameResult {
        let Some(arg) = arg else {
            let Some(sources) = self.sources.take() else {
                return RenameResult::Err(RenameError::MissingInput);
            };

            trace!("wants I/O to rename files");
            return RenameResult::Io(FsIo::Rename(Err(sources)));
        };

        debug!("resume after renaming files");

        let FsIo::Rename(io) = arg else {
            let err = RenameError::InvalidArgument("rename output", arg);
            return RenameResult::Err(err);
        };

        match io {
            Ok(()) => RenameResult::Ok,
            Err(path) => RenameResult::Io(FsIo::Rename(Err(path))),
        }
    }
}
