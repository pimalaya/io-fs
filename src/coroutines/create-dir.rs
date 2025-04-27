//! Module dedicated to the [`CreateDir`] I/O-free coroutine.

use std::path::PathBuf;

use log::debug;

use crate::Io;

/// I/O-free coroutine for creating a directory.
#[derive(Debug)]
pub struct CreateDir {
    input: Option<PathBuf>,
}

impl CreateDir {
    /// Creates a new coroutine from the given directory path.
    pub fn new(path: impl Into<PathBuf>) -> Self {
        let input = Some(path.into());
        Self { input }
    }

    /// Makes the coroutine progress.
    pub fn resume(&mut self, arg: Option<Io>) -> Result<(), Io> {
        let Some(arg) = arg else {
            let Some(input) = self.input.take() else {
                return Err(Io::error("create dir input already consumed"));
            };

            debug!("break: need I/O to create directory");
            return Err(Io::CreateDir(Err(input)));
        };

        debug!("resume after creating directory");

        let Io::CreateDir(Ok(())) = arg else {
            let msg = format!("expected create dir output, got {arg:?}");
            return Err(Io::error(msg));
        };

        Ok(())
    }
}
