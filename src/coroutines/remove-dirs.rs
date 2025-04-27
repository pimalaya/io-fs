//! Module dedicated to the [`RemoveDirs`] I/O-free coroutine.

use std::{collections::HashSet, path::PathBuf};

use log::debug;

use crate::Io;

/// I/O-free coroutine for removing directories.
#[derive(Debug)]
pub struct RemoveDirs {
    input: Option<HashSet<PathBuf>>,
}

impl RemoveDirs {
    /// Creates a new coroutine from the given directory paths.
    pub fn new(paths: impl IntoIterator<Item = impl Into<PathBuf>>) -> RemoveDirs {
        let input = Some(paths.into_iter().map(Into::into).collect());
        Self { input }
    }

    /// Makes the coroutine progress.
    pub fn resume(&mut self, arg: Option<Io>) -> Result<(), Io> {
        let Some(arg) = arg else {
            let Some(input) = self.input.take() else {
                return Err(Io::error("remove dirs input already consumed"));
            };

            debug!("break: need I/O to remove directories");
            return Err(Io::RemoveDirs(Err(input)));
        };

        debug!("resume after removing dirs");

        let Io::RemoveDir(Ok(())) = arg else {
            let msg = format!("expected remove dirs output, got {arg:?}");
            return Err(Io::error(msg));
        };

        Ok(())
    }
}
