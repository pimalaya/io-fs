//! Module dedicated to the [`RemoveDir`] I/O-free coroutine.

use std::path::PathBuf;

use log::debug;

use crate::Io;

/// I/O-free coroutine for removing a directory.
#[derive(Debug)]
pub struct RemoveDir {
    input: Option<PathBuf>,
}

impl RemoveDir {
    /// Creates a new coroutine from the given directory path.
    pub fn new(path: impl Into<PathBuf>) -> Self {
        let input = Some(path.into());
        Self { input }
    }

    /// Makes the coroutine progress.
    pub fn resume(&mut self, arg: Option<Io>) -> Result<(), Io> {
        let Some(arg) = arg else {
            let Some(input) = self.input.take() else {
                return Err(Io::error("remove dir input already consumed"));
            };

            debug!("break: need I/O to remove directory");
            return Err(Io::RemoveDir(Err(input)));
        };

        debug!("resume after removing dir");

        let Io::RemoveDir(Ok(())) = arg else {
            let msg = format!("expected remove dir output, got {arg:?}");
            return Err(Io::error(msg));
        };

        Ok(())
    }
}
