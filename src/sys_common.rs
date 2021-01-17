// https://github.com/rust-lang/rust/blob/master/library/std/src/sys_common/mod.rs
/// A trait for viewing representations from std types
#[doc(hidden)]
pub trait AsInner<Inner: ?Sized> {
    fn as_inner(&self) -> &Inner;
}

/// A trait for viewing representations from std types
#[doc(hidden)]
pub trait AsInnerMut<Inner: ?Sized> {
    fn as_inner_mut(&mut self) -> &mut Inner;
}

/// A trait for extracting representations from std types
#[doc(hidden)]
pub trait IntoInner<Inner> {
    fn into_inner(self) -> Inner;
}

/// A trait for creating std types from internal representations
#[doc(hidden)]
pub trait FromInner<Inner> {
    fn from_inner(inner: Inner) -> Self;
}

// https://github.com/rust-lang/rust/blob/master/library/std/src/sys_common/fs.rs
use crate::fs;
use std::io::{self, Error, ErrorKind};
use std::path::Path;

#[allow(dead_code)] // TODO: use where it is supposed to be used
pub fn copy(from: &Path, to: &Path) -> io::Result<u64> {
    let mut reader = fs::File::open(from)?;
    let metadata = reader.metadata()?;

    if !metadata.is_file() {
        return Err(Error::new(
            ErrorKind::InvalidInput,
            "the source path is not an existing regular file",
        ));
    }

    let mut writer = fs::File::create(to)?;
    let perm = metadata.permissions();

    let ret = io::copy(&mut reader, &mut writer)?;
    writer.set_permissions(perm)?;
    Ok(ret)
}

pub fn remove_dir_all(path: &Path) -> io::Result<()> {
    let filetype = fs::symlink_metadata(path)?.file_type();
    if filetype.is_symlink() { fs::remove_file(path) } else { remove_dir_all_recursive(path) }
}

fn remove_dir_all_recursive(path: &Path) -> io::Result<()> {
    for child in fs::read_dir(path)? {
        let child = child?;
        if child.file_type()?.is_dir() {
            remove_dir_all_recursive(&child.path())?;
        } else {
            fs::remove_file(&child.path())?;
        }
    }
    fs::remove_dir(path)
}
