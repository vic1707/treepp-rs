use core::result::Result;
use std::{io, path::PathBuf};
use thiserror::Error;

use crate::{FilterManager, SorterManager};

mod dir;
mod file;
mod symlink;

pub use dir::Dir;
pub use file::File;
pub use symlink::SymbolicLink;

#[derive(Debug)]
pub enum FSNode {
  File(File),
  Directory(Dir),
  SymbolicLink(SymbolicLink),
}

#[derive(Error, Debug)]
pub enum FSNodeError {
  #[error("`{0}` - Unknown node type")]
  UnknownNodeType(PathBuf),
  #[error("`{0}` - Not found")]
  NotFound(PathBuf),
  #[error("`{0}` - Not enough permissions")]
  NoPermissions(PathBuf),
  #[error("`{0}` - Not a directory")]
  NotADirectory(PathBuf),
  #[error("`{0}` - Modified date not available")]
  ModifiedNotAvailable(PathBuf),
}

pub type FSNodeRes = Result<FSNode, FSNodeError>;

impl FSNode {
  pub fn build<P: Into<PathBuf>>(
    path_: P,
    filter_manager: &FilterManager,
    sorter_manager: &SorterManager,
  ) -> FSNodeRes {
    let path: PathBuf = path_.into();
    if path.is_symlink() {
      Ok(Self::SymbolicLink(SymbolicLink::build(path)?))
    } else if path.is_dir() {
      Ok(Self::Directory(Dir::build(
        path,
        filter_manager,
        sorter_manager,
      )?))
    } else if path.is_file() {
      Ok(Self::File(File::build(path)?))
    } else {
      Err(FSNodeError::UnknownNodeType(path))
    }
  }

  pub const fn size(&self) -> &i128 {
    match *self {
      Self::File(ref file) => file.size(),
      Self::Directory(ref dir) => dir.size(),
      Self::SymbolicLink(ref symlink) => symlink.size(),
    }
  }

  pub const fn path(&self) -> &PathBuf {
    match *self {
      Self::File(ref file) => file.path(),
      Self::Directory(ref dir) => dir.path(),
      Self::SymbolicLink(ref symlink) => symlink.path(),
    }
  }
}

impl FSNodeError {
  pub fn metadata(path: PathBuf, err: &io::Error) -> Self {
    match err.kind() {
      io::ErrorKind::NotFound => Self::NotFound(path),
      io::ErrorKind::PermissionDenied => Self::NoPermissions(path),
      _ => panic!("Unknown error: {err}"),
    }
  }

  pub fn read_dir(path: PathBuf, err: &io::Error) -> Self {
    match err.kind() {
      io::ErrorKind::NotFound => Self::NotFound(path),
      io::ErrorKind::PermissionDenied => Self::NoPermissions(path),
      // only available on nightly -- issue #86442
      // io::ErrorKind::NotADirectory => Self::NotADirectory(path),
      _ => panic!("Unknown error: {err}"),
    }
  }

  pub fn DirEntry(_path: &PathBuf, err: &io::Error) -> Self {
    panic!("Unknown error: {err}")
  }

  pub fn modified(path: PathBuf, err: &io::Error) -> Self {
    match err.kind() {
      io::ErrorKind::Unsupported => Self::ModifiedNotAvailable(path),
      _ => panic!("Unknown error: {err}"),
    }
  }

  pub fn read_link(path: PathBuf, err: &io::Error) -> Self {
    match err.kind() {
      io::ErrorKind::NotFound => Self::NotFound(path),
      _ => panic!("Unknown error: {err}"),
    }
  }
}
