//! Collection of I/O-free, resumable and composable filesystem state
//! machines.
//!
//! Coroutines emit [`Io`] requests that need to be processed by
//! [runtimes] in order to continue their progression.
//!
//! [`Io`]: crate::Io
//! [runtimes]: crate::runtimes

#[path = "create-dir.rs"]
mod create_dir;
#[path = "create-dirs.rs"]
mod create_dirs;
#[path = "create-file.rs"]
mod create_file;
#[path = "create-files.rs"]
mod create_files;
#[path = "read-dir.rs"]
mod read_dir;
#[path = "read-file.rs"]
mod read_file;
#[path = "read-files.rs"]
mod read_files;
#[path = "remove-dir.rs"]
mod remove_dir;
#[path = "remove-dirs.rs"]
mod remove_dirs;
#[path = "remove-file.rs"]
mod remove_file;
#[path = "remove-files.rs"]
mod remove_files;
mod rename;

#[doc(inline)]
pub use self::{
    create_dir::CreateDir, create_dirs::CreateDirs, create_file::CreateFile,
    create_files::CreateFiles, read_dir::ReadDir, read_file::ReadFile, read_files::ReadFiles,
    remove_dir::RemoveDir, remove_dirs::RemoveDirs, remove_file::RemoveFile,
    remove_files::RemoveFiles, rename::Rename,
};
