#![allow(unused_variables)]
// no-operation filesystem implementation
// based on https://github.com/rust-lang/rust/blob/master/library/std/src/sys/unix/fs.rs

use std::ffi::OsString;
use std::path::{Path, PathBuf, Component};
use std::io::{self, SeekFrom};
use std::sync::Mutex;
use std::cell::RefCell;

use lazy_static::lazy_static;

/// An open file
#[derive(Debug)]
pub struct File {
    // TODO: io::cursor
    data_file: Box<DataFile>,
}

// Private structures holding the actual filesystem data
#[derive(Debug)]
struct DataFile {
    data: Vec<u8>,
}

#[derive(Debug)]
struct Dir {
    entries: Vec<DirEntry>,
}

impl Dir {
    /// Lookup a directory entry by name.
    fn find_entry(&mut self, name: &str) -> Option<&mut Entry> {
        // TODO: optimize lookup from O(n), hash
        for entry in &mut self.entries {
            if entry.name == name {
                return Some(&mut entry.entry)
            }
        }
        None
    }
}

lazy_static! {
    static ref ROOT: Mutex<Dir> = Mutex::new(Dir {
        entries: vec![
            DirEntry {
                name: "hello.txt".to_string(),
                entry: Entry::File {
                    file: RefCell::new(DataFile {
                        data: vec![41, 42, 43],
                    })
                }
            }
        // TODO: more static entries with include_bytes!()
        ]
    });
}

impl File {
    pub fn open(path: &Path, opts: &OpenOptions) -> io::Result<File> {
        let mut dir = &mut *ROOT.lock().unwrap();

        let mut components: Vec<Component> = path.components().collect();
        let file_name = components.pop().expect("open: no path components");
        let file_name = match file_name {
            Component::Normal(file_name) => file_name,
            _ => return Err(io::Error::new(
                io::ErrorKind::NotFound,
                format!("open: last path component {:?} is not Normal", file_name))),
        }.to_str().expect(&format!("open: last path component {:?} not a valid String", file_name));

        // Traverse directory hierarchy
        for component in components {
            println!("component = {:?}", component);
            match component {
                Component::Normal(name) => {
                    if let Some(entry) = dir.find_entry(&name.to_str().expect(&format!("open: dir entry name {:?} not a valid String", name))) {
                        println!("entry = {:?}", entry);

                        match entry {
                            Entry::File{..} => {
                                return Err(io::Error::new(
                                    io::ErrorKind::NotFound,
                                    format!("open: dir entry {:?} is a file not a directory", name)));
                            },
                            Entry::Dir{dir: next_dir} => {
                                dir = next_dir;
                            }
                        }
                    } else {
                        return Err(io::Error::new(
                            io::ErrorKind::NotFound,
                            format!("open: dir entry {:?} not found in dir", name)));
                    }
                }
                Component::RootDir => todo!(), //dir = ROOT.lock().unwrap().entry,
                Component::CurDir => (),
                Component::ParentDir => todo!(),
                Component::Prefix(_) => unimplemented!(),
            }
        }

        println!("opening {:?} in dir {:?}", file_name, dir);
        let mut entry = dir.find_entry(&file_name);
        println!("entry = {:?}", entry);

        match entry {
            Some(&mut Entry::File{ref mut file}) => {
                println!("file = {:?}", file);
                todo!(); //Ok(File { data_file: Box::new(file) }), // TODO: fix expected struct `DataFile`, found `&mut DataFile
            },
            Some(&mut Entry::Dir{..}) => Err(io::Error::new(
                io::ErrorKind::NotFound,
                format!("open: file is a directory: {:?}", file_name))),
            None => Err(io::Error::new(
                  io::ErrorKind::NotFound,
                  format!("open: file {:?} not found in dir {:?}", file_name, dir))),
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
        Ok(FileAttr { size: 0, ty: FileType::File })
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
        Ok(0)
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
pub struct FileAttr {
    size: u64,
    ty: FileType,
}

impl FileAttr {
    pub fn size(&self) -> u64 {
        self.size
    }

    pub fn perm(&self) -> FilePermissions {
        FilePermissions{}
    }

    pub fn file_type(&self) -> FileType {
        self.ty
    }
}

#[derive(Debug)]
pub struct DirEntry {
    name: String,
    entry: Entry,
}

#[derive(Debug)]
enum Entry {
    File {
        file: RefCell<DataFile>,
    },
    Dir {
        dir: Dir,
    }
}

impl DirEntry {
    pub fn path(&self) -> PathBuf {
        PathBuf::new()
    }

    pub fn metadata(&self) -> io::Result<FileAttr> {
        Ok(match &self.entry {
            Entry::File{file, ..} => FileAttr {
                size: file.borrow().data.len() as u64,
                ty: FileType::File,
            },
            Entry::Dir{dir, ..} => FileAttr {
                size: 0,
                ty: FileType::Dir,
            },
        })
    }

    pub fn file_name(&self) -> OsString {
        From::from(&self.name)
    }

    pub fn file_type(&self) -> io::Result<FileType> {
        Ok(match &self.entry {
            Entry::File{..} => FileType::File,
            Entry::Dir{..} => FileType::Dir,
        })
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
pub enum FileType {
    File,
    Dir,
}

impl FileType {
    pub fn is_dir(&self) -> bool {
        match self {
            FileType::Dir => true,
            _ => false,
        }
    }

    pub fn is_file(&self) -> bool {
        match self {
            FileType::File => true,
            _ => false,
        }
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
    Ok(FileAttr { size: 0, ty: FileType::File })
}

pub fn lstat(p: &Path) -> io::Result<FileAttr> {
    stat(p)
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


