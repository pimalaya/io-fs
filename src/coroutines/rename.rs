//! I/O-free coroutine to rename multiple filesystem files and/or
//! directories.

use std::path::PathBuf;

use log::{debug, trace};

use crate::{
    error::{FsError, FsResult},
    io::FsIo,
};

/// I/O-free coroutine to rename multiple filesystem files and/or
/// directories.
#[derive(Debug)]
pub struct Rename {
    sources: Option<Vec<(PathBuf, PathBuf)>>,
}

impl Rename {
    /// Creates a new coroutine from the given file paths and
    /// contents.
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
    pub fn resume(&mut self, arg: Option<FsIo>) -> FsResult {
        let Some(arg) = arg else {
            let Some(sources) = self.sources.take() else {
                return FsResult::Err(FsError::MissingInput);
            };

            trace!("wants I/O to rename files");
            return FsResult::Io(FsIo::Rename(Err(sources)));
        };

        debug!("resume after renaming files");

        let FsIo::Rename(io) = arg else {
            let err = FsError::InvalidArgument("rename output", arg);
            return FsResult::Err(err);
        };

        match io {
            Ok(()) => FsResult::Ok(()),
            Err(path) => FsResult::Io(FsIo::Rename(Err(path))),
        }
    }
}
