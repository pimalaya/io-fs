//! I/O-free coroutine to create multiple filesystem files.

use std::{collections::HashMap, path::PathBuf};

use log::{debug, trace};

use crate::{
    error::{FsError, FsResult},
    io::FsIo,
};

/// I/O-free coroutine to create multiple filesystem files.
#[derive(Debug)]
pub struct CreateFiles {
    contents: Option<HashMap<PathBuf, Vec<u8>>>,
}

impl CreateFiles {
    /// Creates a new coroutine from the given file paths and contents.
    pub fn new(
        contents: impl IntoIterator<Item = (impl Into<PathBuf>, impl IntoIterator<Item = u8>)>,
    ) -> Self {
        let contents = contents
            .into_iter()
            .map(|(path, contents)| (path.into(), contents.into_iter().collect()));
        let contents = Some(contents.collect());
        Self { contents }
    }

    /// Makes the coroutine progress.
    pub fn resume(&mut self, arg: Option<FsIo>) -> FsResult {
        let Some(arg) = arg else {
            let Some(contents) = self.contents.take() else {
                return FsResult::Err(FsError::MissingInput);
            };

            trace!("wants I/O to create files");
            return FsResult::Io(FsIo::CreateFiles(Err(contents)));
        };

        debug!("resume after creating files");

        let FsIo::CreateFiles(io) = arg else {
            let err = FsError::InvalidArgument("create files output", arg);
            return FsResult::Err(err);
        };

        match io {
            Ok(()) => FsResult::Ok(()),
            Err(path) => FsResult::Io(FsIo::CreateFiles(Err(path))),
        }
    }
}
