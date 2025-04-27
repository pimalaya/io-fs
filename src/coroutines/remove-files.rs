//! Module dedicated to the [`RemoveFiles`] I/O-free coroutine.

use std::{collections::HashSet, path::PathBuf};

use log::debug;

use crate::Io;

/// I/O-free coroutine for removing files.
#[derive(Debug)]
pub struct RemoveFiles {
    input: Option<HashSet<PathBuf>>,
}

impl RemoveFiles {
    /// Creates a new coroutine from the given file paths.
    pub fn new(paths: impl IntoIterator<Item = impl Into<PathBuf>>) -> RemoveFiles {
        let input = Some(paths.into_iter().map(Into::into).collect());
        Self { input }
    }

    /// Makes the coroutine progress.
    pub fn resume(&mut self, arg: Option<Io>) -> Result<(), Io> {
        let Some(arg) = arg else {
            let Some(input) = self.input.take() else {
                return Err(Io::error("remove files input already consumed"));
            };

            debug!("break: need I/O to remove files");
            return Err(Io::RemoveFiles(Err(input)));
        };

        debug!("resume after removing files");

        let Io::RemoveFiles(Ok(())) = arg else {
            let msg = format!("expected remove files output, got {arg:?}");
            return Err(Io::error(msg));
        };

        Ok(())
    }
}
