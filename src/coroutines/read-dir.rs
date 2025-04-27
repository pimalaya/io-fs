//! Module dedicated to the [`ReadDir`] I/O-free coroutine.

use std::{collections::HashSet, path::PathBuf};

use log::debug;

use crate::Io;

/// I/O-free coroutine for reading directory entries.
#[derive(Debug)]
pub struct ReadDir {
    input: Option<PathBuf>,
}

impl ReadDir {
    /// Reads a new coroutine from the given directory path.
    pub fn new(path: impl Into<PathBuf>) -> Self {
        let input = Some(path.into());
        Self { input }
    }

    /// Makes the coroutine progress.
    pub fn resume(&mut self, arg: Option<Io>) -> Result<HashSet<PathBuf>, Io> {
        let Some(arg) = arg else {
            let Some(input) = self.input.take() else {
                return Err(Io::error("read dir input already consumed"));
            };

            debug!("break: need I/O to read directory");
            return Err(Io::ReadDir(Err(input)));
        };

        debug!("resume after reading directory");

        let Io::ReadDir(Ok(paths)) = arg else {
            let msg = format!("expected read dir output, got {arg:?}");
            return Err(Io::error(msg));
        };

        Ok(paths)
    }
}
