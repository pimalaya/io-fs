//! Filesystem I/O requests and responses.

use std::{
    collections::{HashMap, HashSet},
    fmt,
    path::PathBuf,
};

/// The filesystem I/O request and response enum, emitted by
/// [coroutines] and processed by [runtimes].
///
/// Represents all the possible I/O requests that a filesystem
/// coroutine can emit. Runtimes should be able to handle all
/// variants.
///
/// [coroutines]: crate::coroutines
/// [runtimes]: crate::runtimes
#[derive(Clone)]
pub enum FsIo {
    /// I/O request to create a filesystem directory.
    ///
    /// Input: directory path
    ///
    /// Output: none
    CreateDir(Result<(), PathBuf>),

    /// I/O request to create multiple filesystem directories.
    ///
    /// Input: set of directory paths
    ///
    /// Output: none
    CreateDirs(Result<(), HashSet<PathBuf>>),

    /// I/O request to create a filesystem file.
    ///
    /// Input: tuple of file path and raw contents (bytes)
    ///
    /// Output: none
    CreateFile(Result<(), (PathBuf, Vec<u8>)>),

    /// I/O request to create multiple filesystem files.
    ///
    /// Input: map of path and raw contents (bytes)
    ///
    /// Output: none
    CreateFiles(Result<(), HashMap<PathBuf, Vec<u8>>>),

    /// I/O request to read entries from a filesystem directory.
    ///
    /// Input: directory path
    ///
    /// Output: set of entry paths
    ReadDir(Result<HashSet<PathBuf>, PathBuf>),

    /// I/O request to read a filesystem file.
    ///
    /// Input: file path
    ///
    /// Output: raw contents (bytes)
    ReadFile(Result<Vec<u8>, PathBuf>),

    /// I/O request to read multiple filesystem files.
    ///
    /// Input: set of file paths
    ///
    /// Output: map of path and raw contents (bytes)
    ReadFiles(Result<HashMap<PathBuf, Vec<u8>>, HashSet<PathBuf>>),

    /// I/O request to remove a filesystem directory.
    ///
    /// Input: directory path
    ///
    /// Output: none
    RemoveDir(Result<(), PathBuf>),

    /// I/O request to remove multiple filesystem directories.
    ///
    /// Input: set of directory paths
    ///
    /// Output: none
    RemoveDirs(Result<(), HashSet<PathBuf>>),

    /// I/O request to remove a filesystem file.
    ///
    /// Input: file path
    ///
    /// Output: none
    RemoveFile(Result<(), PathBuf>),

    /// I/O request to remove multiple filesystem files.
    ///
    /// Input: set of file paths
    ///
    /// Output: none
    RemoveFiles(Result<(), HashSet<PathBuf>>),

    /// I/O request to rename multiple filesystem files and/or
    /// directories.
    ///
    /// Input: set of directory and/or file paths
    ///
    /// Output: none
    Rename(Result<(), Vec<(PathBuf, PathBuf)>>),
}

impl fmt::Debug for FsIo {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::CreateDir(Ok(_)) => f.write_str("create dir output"),
            Self::CreateDir(Err(_)) => f.write_str("create dir input"),

            Self::CreateDirs(Ok(_)) => f.write_str("create dirs output"),
            Self::CreateDirs(Err(_)) => f.write_str("create dirs input"),

            Self::CreateFile(Ok(_)) => f.write_str("create file output"),
            Self::CreateFile(Err(_)) => f.write_str("create file input"),

            Self::CreateFiles(Ok(_)) => f.write_str("create files output"),
            Self::CreateFiles(Err(_)) => f.write_str("create files input"),

            Self::ReadDir(Ok(_)) => f.write_str("read dir output"),
            Self::ReadDir(Err(_)) => f.write_str("read dir input"),

            Self::ReadFile(Ok(_)) => f.write_str("read file output"),
            Self::ReadFile(Err(_)) => f.write_str("read file input"),

            Self::ReadFiles(Ok(_)) => f.write_str("read files output"),
            Self::ReadFiles(Err(_)) => f.write_str("read files input"),

            Self::RemoveDir(Ok(_)) => f.write_str("remove dir output"),
            Self::RemoveDir(Err(_)) => f.write_str("remove dir input"),

            Self::RemoveDirs(Ok(_)) => f.write_str("remove dirs output"),
            Self::RemoveDirs(Err(_)) => f.write_str("remove dirs input"),

            Self::RemoveFile(Ok(_)) => f.write_str("remove file output"),
            Self::RemoveFile(Err(_)) => f.write_str("remove file input"),

            Self::RemoveFiles(Ok(_)) => f.write_str("remove files output"),
            Self::RemoveFiles(Err(_)) => f.write_str("remove files input"),

            Self::Rename(Ok(_)) => f.write_str("rename output"),
            Self::Rename(Err(_)) => f.write_str("rename input"),
        }
    }
}
