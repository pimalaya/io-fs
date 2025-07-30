#![cfg(feature = "std")]

use std::collections::{HashMap, HashSet};

use io_fs::{
    coroutines::{
        create_dir::CreateDir, create_dirs::CreateDirs, create_file::CreateFile,
        create_files::CreateFiles, read_dir::ReadDir, read_file::ReadFile, read_files::ReadFiles,
        remove_dir::RemoveDir, remove_dirs::RemoveDirs, remove_file::RemoveFile,
        remove_files::RemoveFiles, rename::Rename,
    },
    error::FsResult,
    runtimes::std::handle,
};
use tempfile::tempdir;

#[test]
fn std() {
    let _ = env_logger::try_init();

    let workdir = tempdir().unwrap();

    // create single directory

    let mut arg = None;
    let mut coroutine = CreateDir::new(workdir.path().join("dir1"));

    loop {
        match coroutine.resume(arg) {
            FsResult::Ok(()) => break,
            FsResult::Err(err) => panic!("{err}"),
            FsResult::Io(io) => arg = Some(handle(io).unwrap()),
        }
    }

    assert!(workdir.path().join("dir1").is_dir());

    // create multiple directories

    let mut arg = None;
    let mut coroutine = CreateDirs::new([workdir.path().join("dir2"), workdir.path().join("dir3")]);

    loop {
        match coroutine.resume(arg) {
            FsResult::Ok(()) => break,
            FsResult::Err(err) => panic!("{err}"),
            FsResult::Io(io) => arg = Some(handle(io).unwrap()),
        }
    }

    assert!(workdir.path().join("dir2").is_dir());
    assert!(workdir.path().join("dir3").is_dir());

    // create single file

    let mut arg = None;
    let mut coroutine = CreateFile::new(workdir.path().join("dir1").join("file1"), *b"file1");

    loop {
        match coroutine.resume(arg) {
            FsResult::Ok(()) => break,
            FsResult::Err(err) => panic!("{err}"),
            FsResult::Io(io) => arg = Some(handle(io).unwrap()),
        }
    }

    assert!(workdir.path().join("dir1").join("file1").is_file());

    // create multiple files

    let mut arg = None;
    let mut coroutine = CreateFiles::new([
        (workdir.path().join("dir2").join("file2"), *b"file2"),
        (workdir.path().join("dir2").join("file3"), *b"file3"),
    ]);

    loop {
        match coroutine.resume(arg) {
            FsResult::Ok(()) => break,
            FsResult::Err(err) => panic!("{err}"),
            FsResult::Io(io) => arg = Some(handle(io).unwrap()),
        }
    }

    assert!(workdir.path().join("dir2").join("file2").is_file());
    assert!(workdir.path().join("dir2").join("file3").is_file());

    // read directory

    let mut arg = None;
    let mut coroutine = ReadDir::new(workdir.path().join("dir1"));

    let paths = loop {
        match coroutine.resume(arg) {
            FsResult::Ok(paths) => break paths,
            FsResult::Err(err) => panic!("{err}"),
            FsResult::Io(io) => arg = Some(handle(io).unwrap()),
        }
    };

    let expected_paths = HashSet::from_iter([workdir.path().join("dir1").join("file1")]);

    assert_eq!(paths, expected_paths);

    arg = None;
    coroutine = ReadDir::new(workdir.path().join("dir2"));

    let paths = loop {
        match coroutine.resume(arg) {
            FsResult::Ok(paths) => break paths,
            FsResult::Err(err) => panic!("{err}"),
            FsResult::Io(io) => arg = Some(handle(io).unwrap()),
        }
    };

    let expected_paths = HashSet::from_iter([
        workdir.path().join("dir2").join("file2"),
        workdir.path().join("dir2").join("file3"),
    ]);

    assert_eq!(paths, expected_paths);

    arg = None;
    coroutine = ReadDir::new(workdir.path().join("dir3"));

    let paths = loop {
        match coroutine.resume(arg) {
            FsResult::Ok(paths) => break paths,
            FsResult::Err(err) => panic!("{err}"),
            FsResult::Io(io) => arg = Some(handle(io).unwrap()),
        }
    };

    assert!(paths.is_empty());

    // read single file

    let mut arg = None;
    let mut coroutine = ReadFile::new(workdir.path().join("dir1").join("file1"));

    let contents = loop {
        match coroutine.resume(arg) {
            FsResult::Ok(contents) => break contents,
            FsResult::Err(err) => panic!("{err}"),
            FsResult::Io(io) => arg = Some(handle(io).unwrap()),
        }
    };

    assert_eq!(b"file1", contents.as_slice());

    // read multiple files

    let mut arg = None;
    let mut coroutine = ReadFiles::new([
        workdir.path().join("dir2").join("file2"),
        workdir.path().join("dir2").join("file3"),
    ]);

    let contents = loop {
        match coroutine.resume(arg) {
            FsResult::Ok(contents) => break contents,
            FsResult::Err(err) => panic!("{err}"),
            FsResult::Io(io) => arg = Some(handle(io).unwrap()),
        }
    };

    let expected_contents = HashMap::from_iter([
        (workdir.path().join("dir2").join("file2"), b"file2".to_vec()),
        (workdir.path().join("dir2").join("file3"), b"file3".to_vec()),
    ]);

    assert_eq!(contents, expected_contents);

    // rename

    let mut arg = None;
    let mut coroutine = Rename::new(Some((
        workdir.path().join("dir2").join("file3"),
        workdir.path().join("dir3").join("file3"),
    )));

    loop {
        match coroutine.resume(arg) {
            FsResult::Ok(()) => break,
            FsResult::Err(err) => panic!("{err}"),
            FsResult::Io(io) => arg = Some(handle(io).unwrap()),
        }
    }

    assert_eq!(false, workdir.path().join("dir2").join("file3").is_file());
    assert_eq!(true, workdir.path().join("dir3").join("file3").is_file());

    // remove single file

    let mut arg = None;
    let mut coroutine = RemoveFile::new(workdir.path().join("dir3").join("file3"));

    loop {
        match coroutine.resume(arg) {
            FsResult::Ok(()) => break,
            FsResult::Err(err) => panic!("{err}"),
            FsResult::Io(io) => arg = Some(handle(io).unwrap()),
        }
    }

    assert_eq!(false, workdir.path().join("dir3").join("file3").is_file());

    // remove multiple files

    let mut arg = None;
    let mut coroutine = RemoveFiles::new([
        workdir.path().join("dir1").join("file1"),
        workdir.path().join("dir2").join("file2"),
    ]);

    loop {
        match coroutine.resume(arg) {
            FsResult::Ok(()) => break,
            FsResult::Err(err) => panic!("{err}"),
            FsResult::Io(io) => arg = Some(handle(io).unwrap()),
        }
    }

    assert_eq!(false, workdir.path().join("dir1").join("file1").is_file());
    assert_eq!(false, workdir.path().join("dir2").join("file2").is_file());

    // remove single directory

    let mut arg = None;
    let mut coroutine = RemoveDir::new(workdir.path().join("dir3"));

    loop {
        match coroutine.resume(arg) {
            FsResult::Ok(()) => break,
            FsResult::Err(err) => panic!("{err}"),
            FsResult::Io(io) => arg = Some(handle(io).unwrap()),
        }
    }

    assert_eq!(false, workdir.path().join("dir3").is_dir());

    // remove multiple directories

    let mut arg = None;
    let mut coroutine = RemoveDirs::new([workdir.path().join("dir1"), workdir.path().join("dir2")]);

    loop {
        match coroutine.resume(arg) {
            FsResult::Ok(()) => break,
            FsResult::Err(err) => panic!("{err}"),
            FsResult::Io(io) => arg = Some(handle(io).unwrap()),
        }
    }

    assert_eq!(false, workdir.path().join("dir1").is_dir());
    assert_eq!(false, workdir.path().join("dir2").is_dir());
}
