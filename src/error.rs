//! Filesystem coroutines errors.

use thiserror::Error;

use crate::io::FsIo;

/// Errors that can occur during any filesystem coroutine progression.
///
/// Only coroutine misuses should lead to these error variants.
#[derive(Clone, Debug, Error)]
pub enum FsError {
    /// The coroutine input is missing or has already been used.
    ///
    /// Occurs when the coroutine is called twice without processing
    /// I/O requests, which should not happen if the runtime process
    /// correctly I/O requests.
    #[error("Missing input: path missing or already consumed")]
    MissingInput,

    /// The coroutine received an invalid argument.
    ///
    /// Occurs when the coroutine receives an I/O response from
    /// another coroutine, which should not happen if the runtime maps
    /// correctly the arguments.
    #[error("Invalid argument: expected {0}, got {1:?}")]
    InvalidArgument(&'static str, FsIo),
}

/// Output emitted after a coroutine finishes its progression.
#[derive(Clone, Debug)]
pub enum FsResult<T = ()> {
    /// The coroutine has successfully terminated its progression.
    Ok(T),

    /// An error occured during the coroutine progression.
    Err(FsError),

    /// A filesystem I/O needs to be performed to make the coroutine
    /// progress.
    Io(FsIo),
}
