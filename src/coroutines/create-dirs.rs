//! Module dedicated to the [`CreateDirs`] I/O-free coroutine.

use std::{collections::HashSet, path::PathBuf};

use log::debug;

use crate::Io;

/// I/O-free coroutine for creating directories.
#[derive(Debug)]
pub struct CreateDirs {
    input: Option<HashSet<PathBuf>>,
}

impl CreateDirs {
    /// Creates a new coroutine from the given directory paths.
    pub fn new(paths: impl IntoIterator<Item = impl Into<PathBuf>>) -> CreateDirs {
        let input = Some(paths.into_iter().map(Into::into).collect());
        Self { input }
    }

    /// Makes the coroutine progress.
    pub fn resume(&mut self, arg: Option<Io>) -> Result<(), Io> {
        let Some(arg) = arg else {
            let Some(input) = self.input.take() else {
                return Err(Io::error("create dirs input already consumed"));
            };

            debug!("break: need I/O to create directories");
            return Err(Io::CreateDirs(Err(input)));
        };

        debug!("resume after creating directories");

        let Io::CreateDirs(Ok(())) = arg else {
            let msg = format!("expected create dirs output, got {arg:?}");
            return Err(Io::error(msg));
        };

        Ok(())
    }
}
