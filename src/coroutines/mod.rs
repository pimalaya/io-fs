//! Collection of I/O-free, resumable and composable filesystem state
//! machines.
//!
//! Coroutines emit [I/O] requests that need to be processed by
//! [runtimes] in order to continue their progression.
//!
//! [I/O]: crate::io
//! [runtimes]: crate::runtimes

#[path = "create-dir.rs"]
pub mod create_dir;
#[path = "create-dirs.rs"]
pub mod create_dirs;
#[path = "create-file.rs"]
pub mod create_file;
#[path = "create-files.rs"]
pub mod create_files;
#[path = "read-dir.rs"]
pub mod read_dir;
#[path = "read-file.rs"]
pub mod read_file;
#[path = "read-files.rs"]
pub mod read_files;
#[path = "remove-dir.rs"]
pub mod remove_dir;
#[path = "remove-dirs.rs"]
pub mod remove_dirs;
#[path = "remove-file.rs"]
pub mod remove_file;
#[path = "remove-files.rs"]
pub mod remove_files;
pub mod rename;
