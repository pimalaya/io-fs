//! Module dedicated to the [`ReadFile`] I/O-free coroutine.

use std::path::PathBuf;

use log::debug;

use crate::Io;

/// I/O-free coroutine for reading file contents.
#[derive(Debug)]
pub struct ReadFile {
    input: Option<PathBuf>,
}

impl ReadFile {
    /// Reads a new coroutine from the given file path.
    pub fn new(path: impl Into<PathBuf>) -> Self {
        let input = Some(path.into());
        Self { input }
    }

    /// Makes the coroutine progress.
    pub fn resume(&mut self, arg: Option<Io>) -> Result<Vec<u8>, Io> {
        let Some(arg) = arg else {
            let Some(input) = self.input.take() else {
                return Err(Io::error("read file input already consumed"));
            };

            debug!("break: need I/O to read file");
            return Err(Io::ReadFile(Err(input)));
        };

        debug!("resume after reading file");

        let Io::ReadFile(Ok(contents)) = arg else {
            let msg = format!("expected read file output, got {arg:?}");
            return Err(Io::error(msg));
        };

        Ok(contents)
    }
}
