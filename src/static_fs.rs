#![allow(unused_variables, dead_code)]
// no-operation filesystem implementation
// based on https://github.com/rust-lang/rust/blob/master/library/std/src/sys/unix/fs.rs

use std::ffi::OsString;
use std::path::{Path, PathBuf};
use std::io::{self, SeekFrom, Cursor, Read, Write};
use std::collections::HashMap;
use lazy_static::lazy_static;

lazy_static! {
    static ref FILE_DATA: HashMap<&'static str, Vec<u8>> = {
        let mut m = HashMap::new();
        m.insert("abc.txt", vec![41, 42, 43]);
        m.insert("hello.txt", include_bytes!("hello.txt").to_vec());
        m
    };
}

#[derive(Debug)]
pub struct File {
    cursor: Cursor<Vec<u8>>,
}

impl File {
    pub fn open(path: &Path, opts: &OpenOptions) -> io::Result<File> {
        println!("open {:?}", path);

        let path_str = path.to_str().expect("path is not a valid OsString");

        if let Some(data) = FILE_DATA.get(path_str) {
            let cursor = Cursor::new(data.to_vec());
            Ok(File{ cursor })
        } else {
            Err(io::Error::new(
                io::ErrorKind::NotFound,
                "file not found"))
        }
    }

    pub fn fsync(&self) -> io::Result<()> {
        Ok(())
    }

    pub fn datasync(&self) -> io::Result<()> {
        Ok(())
    }

    pub fn truncate(&self, size: u64) -> io::Result<()> {
        Ok(())
    }

    pub fn file_attr(&self) -> io::Result<FileAttr> {
        Ok(FileAttr{})
    }

    pub fn duplicate(&self) -> io::Result<File> {
        Err(io::Error::new(
            io::ErrorKind::Other,
            "duplicate is not available on this platform"))
    }

    pub fn set_permissions(&self, perm: FilePermissions) -> io::Result<()> {
        Ok(())
    }

    pub fn read(&self, buf: &mut [u8]) -> io::Result<usize> {
        self.cursor.read(buf)
    }

    pub fn write(&self, buf: &[u8]) -> io::Result<usize> {
        Ok(buf.len())
    }

    pub fn flush(&self) -> io::Result<()> {
        Ok(())
    }

    pub fn seek(&self, pos: SeekFrom) -> io::Result<u64> {
        Ok(0)
    }
}

#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
pub struct FileAttr {}

impl FileAttr {
    pub fn size(&self) -> u64 {
        0
    }

    pub fn perm(&self) -> FilePermissions {
        FilePermissions{}
    }

    pub fn file_type(&self) -> FileType {
        FileType{}
    }
}

#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
pub struct DirEntry {}

impl DirEntry {
    pub fn path(&self) -> PathBuf {
        PathBuf::new()
    }

    pub fn metadata(&self) -> io::Result<FileAttr> {
        Ok(FileAttr{})
    }

    pub fn file_name(&self) -> OsString {
        OsString::new()
    }

    pub fn file_type(&self) -> io::Result<FileType> {
        Ok(FileType{})
    }
}

#[derive(Clone, Debug)]
pub struct OpenOptions {
    read: bool,
    write: bool,
    append: bool,
    truncate: bool,
    create: bool,
    create_new: bool,
}

impl OpenOptions {
    pub fn new() -> OpenOptions {
        OpenOptions {
            read: false,
            write: false,
            append: false,
            truncate: false,
            create: false,
            create_new: false,
        }
    }

    pub fn read(&mut self, read: bool) {
        self.read = read;
    }
    pub fn write(&mut self, write: bool) {
        self.write = write;
    }
    pub fn append(&mut self, append: bool) {
        self.append = append;
    }
    pub fn truncate(&mut self, truncate: bool) {
        self.truncate = truncate;
    }
    pub fn create(&mut self, create: bool) {
        self.create = create;
    }
    pub fn create_new(&mut self, create_new: bool) {
        self.create_new = create_new;
    }
}

#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
pub struct FilePermissions {}

impl FilePermissions {
    pub fn readonly(&self) -> bool {
        false
    }

    pub fn set_readonly(&mut self, _readonly: bool) {
    }

}

#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
pub struct FileType {}

impl FileType {
    pub fn is_dir(&self) -> bool {
        false
    }
    pub fn is_file(&self) -> bool {
        false
    }
    pub fn is_symlink(&self) -> bool {
        false
    }
}

#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
pub struct DirBuilder {}

impl DirBuilder {
    pub fn new() -> DirBuilder {
        DirBuilder{}
    }

    pub fn mkdir(&self, p: &Path) -> io::Result<()> {
        Ok(())
    }
}

#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
pub struct ReadDir {}

impl Iterator for ReadDir {
    type Item = io::Result<DirEntry>;

    fn next(&mut self) -> Option<io::Result<DirEntry>> {
        None
    }
}

pub fn unlink(p: &Path) -> io::Result<()> {
    Ok(())
}

pub fn stat(p: &Path) -> io::Result<FileAttr> {
    Ok(FileAttr{})
}

pub fn lstat(p: &Path) -> io::Result<FileAttr> {
    Ok(FileAttr{})
}

pub fn rename(old: &Path, new: &Path) -> io::Result<()> {
    Ok(())
}

pub fn copy(from: &Path, to: &Path) -> io::Result<u64> {
    Ok(0)
}


pub fn link(original: &Path, link: &Path) -> io::Result<()> {
    Ok(())
}

pub fn symlink(original: &Path, link: &Path) -> io::Result<()> {
    Ok(())
}


pub fn readlink(p: &Path) -> io::Result<PathBuf> {
    Ok(PathBuf::new())
}

pub fn canonicalize(p: &Path) -> io::Result<PathBuf> {
    Ok(PathBuf::new())
}


pub fn rmdir(p: &Path) -> io::Result<()> {
    Ok(())
}


pub use crate::sys_common::remove_dir_all;

pub fn readdir(p: &Path) -> io::Result<ReadDir> {
    Ok(ReadDir{})
}


pub fn set_perm(p: &Path, perm: FilePermissions) -> io::Result<()> {
    Ok(())
}


