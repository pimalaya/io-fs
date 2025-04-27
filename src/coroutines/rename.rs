//! Module dedicated to the [`Rename`] I/O-free coroutine.

use std::path::PathBuf;

use log::debug;

use crate::Io;

/// I/O-free coroutine for renaming files or directories.
#[derive(Debug)]
pub struct Rename {
    input: Option<Vec<(PathBuf, PathBuf)>>,
}

impl Rename {
    /// Reads a new coroutine from the given source and destination paths.
    pub fn new(
        sources: impl IntoIterator<Item = (impl Into<PathBuf>, impl Into<PathBuf>)>,
    ) -> Self {
        let sources = sources
            .into_iter()
            .map(|(from, to)| (from.into(), to.into()))
            .collect();

        Self {
            input: Some(sources),
        }
    }

    /// Makes the coroutine progress.
    pub fn resume(&mut self, arg: Option<Io>) -> Result<(), Io> {
        let Some(arg) = arg else {
            let Some(input) = self.input.take() else {
                return Err(Io::error("rename input already consumed"));
            };

            debug!("break: need I/O to rename files or directories");
            return Err(Io::Rename(Err(input)));
        };

        debug!("resume after renaming files or directories");

        let Io::Rename(Ok(())) = arg else {
            let msg = format!("expected rename output, got {arg:?}");
            return Err(Io::error(msg));
        };

        Ok(())
    }
}
