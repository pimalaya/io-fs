//! Module dedicated to the [`CreateFile`] I/O-free coroutine.

use std::path::PathBuf;

use log::debug;

use crate::Io;

/// I/O-free coroutine for creating a file with its contents.
#[derive(Debug)]
pub struct CreateFile {
    input: Option<(PathBuf, Vec<u8>)>,
}

impl CreateFile {
    /// Creates a new coroutine from the given path and contents.
    pub fn new(path: impl Into<PathBuf>, contents: impl IntoIterator<Item = u8>) -> Self {
        let input = Some((path.into(), contents.into_iter().collect()));
        Self { input }
    }

    /// Makes the coroutine progress.
    pub fn resume(&mut self, arg: Option<Io>) -> Result<(), Io> {
        let Some(arg) = arg else {
            let Some(input) = self.input.take() else {
                return Err(Io::error("create file input already consumed"));
            };

            debug!("break: need I/O to create file");
            return Err(Io::CreateFile(Err(input)));
        };

        debug!("resume after creating file");

        let Io::CreateFile(Ok(())) = arg else {
            let msg = format!("expected create file output, got {arg:?}");
            return Err(Io::error(msg));
        };

        Ok(())
    }
}
