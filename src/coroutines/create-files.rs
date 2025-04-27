//! Module dedicated to the [`CreateFiles`] I/O-free coroutine.

use std::{collections::HashMap, path::PathBuf};

use log::debug;

use crate::Io;

/// I/O-free coroutine for creating multiple files with their contents.
#[derive(Debug)]
pub struct CreateFiles {
    input: Option<HashMap<PathBuf, Vec<u8>>>,
}

impl CreateFiles {
    /// Creates a new coroutine from the given contents.
    pub fn new(
        contents: impl IntoIterator<Item = (impl Into<PathBuf>, impl IntoIterator<Item = u8>)>,
    ) -> Self {
        let contents = contents
            .into_iter()
            .map(|(path, contents)| (path.into(), contents.into_iter().collect()))
            .collect();

        Self {
            input: Some(contents),
        }
    }

    /// Makes the coroutine progress.
    pub fn resume(&mut self, arg: Option<Io>) -> Result<(), Io> {
        let Some(arg) = arg else {
            let Some(input) = self.input.take() else {
                return Err(Io::error("create files input already consumed"));
            };

            debug!("break: need I/O to create files");
            return Err(Io::CreateFiles(Err(input)));
        };

        debug!("resume after creating files");

        let Io::CreateFiles(Ok(())) = arg else {
            let msg = format!("expected create files output, got {arg:?}");
            return Err(Io::error(msg));
        };

        Ok(())
    }
}
