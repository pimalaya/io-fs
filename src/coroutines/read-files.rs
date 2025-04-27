//! Module dedicated to the [`ReadFiles`] I/O-free coroutine.

use std::{
    collections::{HashMap, HashSet},
    path::PathBuf,
};

use log::debug;

use crate::Io;

/// I/O-free coroutine for reading files contents.
#[derive(Debug)]
pub struct ReadFiles {
    input: Option<HashSet<PathBuf>>,
}

impl ReadFiles {
    /// Reads a new coroutine from the given files path.
    pub fn new(paths: impl IntoIterator<Item = impl Into<PathBuf>>) -> Self {
        let input = Some(paths.into_iter().map(Into::into).collect());
        Self { input }
    }

    /// Makes the coroutine progress.
    pub fn resume(&mut self, arg: Option<Io>) -> Result<HashMap<PathBuf, Vec<u8>>, Io> {
        let Some(arg) = arg else {
            let Some(input) = self.input.take() else {
                return Err(Io::error("read files input already consumed"));
            };

            debug!("break: need I/O to read files");
            return Err(Io::ReadFiles(Err(input)));
        };

        debug!("resume after reading files");

        let Io::ReadFiles(Ok(contents)) = arg else {
            let msg = format!("expected read files output, got {arg:?}");
            return Err(Io::error(msg));
        };

        Ok(contents)
    }
}
