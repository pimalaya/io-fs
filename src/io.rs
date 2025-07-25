use std::{
    collections::{HashMap, HashSet},
    fmt,
    path::PathBuf,
};

/// The filesystems I/O request enum, emitted by [coroutines] and
/// processed by [runtimes].
///
/// Represents all the possible I/O requests that a filesystem
/// coroutine can emit. Runtimes should be able to handle all
/// variants.
///
/// [coroutines]: crate::coroutines
/// [runtimes]: crate::runtimes
#[derive(Clone)]
pub enum FsIo {
    CreateDir(Result<(), PathBuf>),
    CreateDirs(Result<(), HashSet<PathBuf>>),
    CreateFile(Result<(), (PathBuf, Vec<u8>)>),
    CreateFiles(Result<(), HashMap<PathBuf, Vec<u8>>>),
    ReadDir(Result<HashSet<PathBuf>, PathBuf>),
    ReadFile(Result<Vec<u8>, PathBuf>),
    ReadFiles(Result<HashMap<PathBuf, Vec<u8>>, HashSet<PathBuf>>),
    RemoveDir(Result<(), PathBuf>),
    RemoveDirs(Result<(), HashSet<PathBuf>>),
    RemoveFile(Result<(), PathBuf>),
    RemoveFiles(Result<(), HashSet<PathBuf>>),
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
