//! Module dedicated to the [`RemoveFile`] I/O-free coroutine.

use std::path::PathBuf;

use log::debug;

use crate::Io;

/// I/O-free coroutine for removing a file.
#[derive(Debug)]
pub struct RemoveFile {
    input: Option<PathBuf>,
}

impl RemoveFile {
    /// Creates a new coroutine from the given file path.
    pub fn new(path: impl Into<PathBuf>) -> Self {
        let input = Some(path.into());
        Self { input }
    }

    /// Makes the coroutine progress.
    pub fn resume(&mut self, arg: Option<Io>) -> Result<(), Io> {
        let Some(arg) = arg else {
            let Some(input) = self.input.take() else {
                return Err(Io::error("remove file input already consumed"));
            };

            debug!("break: need I/O to remove file");
            return Err(Io::RemoveFile(Err(input)));
        };

        debug!("resume after removing file");

        let Io::RemoveFile(Ok(())) = arg else {
            let msg = format!("expected remove file output, got {arg:?}");
            return Err(Io::error(msg));
        };

        Ok(())
    }
}
